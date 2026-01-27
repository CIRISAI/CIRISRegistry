//! RegistryService implementation (public read-only lookups)

use std::sync::Arc;
use std::time::{Instant, SystemTime, UNIX_EPOCH};

use tonic::{Request, Response, Status};
use tracing::info;

use crate::crypto::HybridCrypto;
use crate::db::Database;
use crate::proto::registry_service_server::RegistryService as RegistryServiceTrait;
use crate::proto::*;
use crate::{db, error::RegistryError};

pub struct RegistryService {
    db: Database,
    crypto: Arc<HybridCrypto>,
}

impl RegistryService {
    pub fn new(db: Database, crypto: Arc<HybridCrypto>) -> Self {
        Self { db, crypto }
    }

    fn response_context(&self, request_id: Option<String>, start_time: Option<Instant>) -> ResponseContext {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs() as i64;

        let processing_time_ms = start_time
            .map(|t| t.elapsed().as_millis() as i64)
            .unwrap_or(0);

        ResponseContext {
            request_id: request_id.unwrap_or_else(|| uuid::Uuid::new_v4().to_string()),
            server_timestamp: now,
            processing_time_ms,
            server_version: format!("registry-v{}", env!("CARGO_PKG_VERSION")),
            environment: RegistryEnvironment::EnvDevelopment as i32, // TODO: From config
        }
    }
}

#[tonic::async_trait]
impl RegistryServiceTrait for RegistryService {
    async fn health_check(
        &self,
        request: Request<HealthCheckRequest>,
    ) -> Result<Response<HealthCheckResponse>, Status> {
        let req = request.into_inner();

        let db_healthy = self.db.health_check().await.unwrap_or(false);
        let stats = self.db.pool_stats();

        let status = if db_healthy {
            HealthStatus::HealthServing
        } else {
            HealthStatus::HealthNotServing
        };

        let components = if req.include_diagnostics {
            vec![
                ComponentHealth {
                    name: "database".to_string(),
                    status: if db_healthy {
                        HealthStatus::HealthServing as i32
                    } else {
                        HealthStatus::HealthNotServing as i32
                    },
                    message: "PostgreSQL".to_string(),
                    metrics: [
                        ("active_connections".to_string(), stats.active.to_string()),
                        ("idle_connections".to_string(), stats.idle.to_string()),
                    ]
                    .into_iter()
                    .collect(),
                },
            ]
        } else {
            vec![]
        };

        Ok(Response::new(HealthCheckResponse {
            status: status as i32,
            readiness: ReadinessStatus::ReadinessLive as i32,
            components,
            version: env!("CARGO_PKG_VERSION").to_string(),
            build_commit: option_env!("GIT_COMMIT").unwrap_or("unknown").to_string(),
            uptime_seconds: 0, // TODO: Track uptime
            active_connections: stats.active as i32,
            cpu_usage_percent: 0.0,
            memory_usage_percent: 0.0,
            database_healthy: db_healthy,
            replication_lag_ms: 0,
            context: Some(self.response_context(None, None)),
        }))
    }

    async fn get_capabilities(
        &self,
        _request: Request<GetCapabilitiesRequest>,
    ) -> Result<Response<RegistryCapabilities>, Status> {
        Ok(Response::new(RegistryCapabilities {
            protocol_version: "1.1.0".to_string(),
            supports_merkle_proofs: true,
            supports_offline_mode: true,
            supports_batch_operations: true,
            supports_webhooks: true,
            supports_build_attestation: true,
            supported_algorithms: vec!["Ed25519".to_string(), "ML-DSA-65".to_string()],
            max_batch_size: 100,
            revocation_list_ttl_seconds: 3600,
            offline_package_ttl_hours: 72,
            deprecated_endpoints: vec![],
            migration_guide: Default::default(),
            context: Some(self.response_context(None, None)),
        }))
    }

    async fn get_metrics(
        &self,
        _request: Request<MetricsRequest>,
    ) -> Result<Response<MetricsResponse>, Status> {
        // TODO: Implement actual metrics collection
        Ok(Response::new(MetricsResponse {
            queries_total: 0,
            queries_by_type: Default::default(),
            query_latency_p50_ms: 0,
            query_latency_p95_ms: 0,
            query_latency_p99_ms: 0,
            errors_total: 0,
            errors_by_code: Default::default(),
            signing_operations: 0,
            db_connections_active: self.db.pool_stats().active as i64,
            db_connections_max: self.db.pool_stats().max as i64,
            timestamp: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs() as i64,
            context: Some(self.response_context(None, None)),
        }))
    }

    async fn lookup_agent(
        &self,
        request: Request<LookupAgentRequest>,
    ) -> Result<Response<LookupAgentResponse>, Status> {
        let req = request.into_inner();
        let request_id = req.context.as_ref().map(|c| c.request_id.clone());

        info!(
            agent_hash = hex::encode(&req.agent_hash),
            "Looking up agent"
        );

        let agent_row = db::lookup_agent(self.db.pool(), &req.agent_hash)
            .await
            .map_err(RegistryError::from)?;

        let (agent, found) = match agent_row {
            Some(row) => (Some(row.to_proto()), true),
            None => (None, false),
        };

        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs() as i64;

        // Sign the response if agent found
        let response_signature = if found {
            let data_to_sign = format!(
                "{}:{}:{}",
                hex::encode(&req.agent_hash),
                hex::encode(&req.request_nonce),
                now
            );
            Some(self.crypto.sign(data_to_sign.as_bytes())?)
        } else {
            None
        };

        Ok(Response::new(LookupAgentResponse {
            agent,
            found,
            query_timestamp: now,
            response_signature,
            merkle_proof: None, // TODO: Implement Merkle proofs
            error: None,
            context: Some(self.response_context(request_id, None)),
        }))
    }

    async fn batch_lookup_agents(
        &self,
        request: Request<BatchLookupAgentsRequest>,
    ) -> Result<Response<BatchLookupAgentsResponse>, Status> {
        let req = request.into_inner();
        let request_id = req.context.as_ref().map(|c| c.request_id.clone());

        if req.agent_hashes.len() > 100 {
            return Err(Status::invalid_argument("Maximum batch size is 100"));
        }

        let hashes: Vec<Vec<u8>> = req.agent_hashes.iter().map(|b| b.to_vec()).collect();
        let results = db::batch_lookup_agents(self.db.pool(), &hashes)
            .await
            .map_err(RegistryError::from)?;

        let agents: Vec<AgentRecord> = results
            .iter()
            .filter_map(|(_, row)| row.as_ref().map(|r| r.to_proto()))
            .collect();

        let found: Vec<bool> = results.iter().map(|(_, row)| row.is_some()).collect();

        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs() as i64;

        Ok(Response::new(BatchLookupAgentsResponse {
            agents,
            found,
            query_timestamp: now,
            response_signature: None,
            merkle_proofs: vec![],
            context: Some(self.response_context(request_id, None)),
        }))
    }

    async fn lookup_partner(
        &self,
        request: Request<LookupPartnerRequest>,
    ) -> Result<Response<LookupPartnerResponse>, Status> {
        let req = request.into_inner();
        let request_id = req.context.as_ref().map(|c| c.request_id.clone());

        info!(partner_id = %req.partner_id, "Looking up partner");

        let partner_row = db::lookup_partner(self.db.pool(), &req.partner_id)
            .await
            .map_err(RegistryError::from)?;

        let (partner, found) = match partner_row {
            Some(row) => (Some(row.to_proto()), true),
            None => (None, false),
        };

        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs() as i64;

        Ok(Response::new(LookupPartnerResponse {
            partner,
            found,
            query_timestamp: now,
            response_signature: None,
            merkle_proof: None,
            error: None,
            context: Some(self.response_context(request_id, None)),
        }))
    }

    async fn verify_deployment(
        &self,
        request: Request<VerifyDeploymentRequest>,
    ) -> Result<Response<VerifyDeploymentResponse>, Status> {
        let req = request.into_inner();
        let request_id = req.context.as_ref().map(|c| c.request_id.clone());

        // Lookup agent
        let agent_row = db::lookup_agent(self.db.pool(), &req.agent_hash)
            .await
            .map_err(RegistryError::from)?;

        // Lookup partner
        let partner_row = db::lookup_partner(self.db.pool(), &req.partner_id)
            .await
            .map_err(RegistryError::from)?;

        let agent_found = agent_row.is_some();
        let partner_found = partner_row.is_some();

        // Calculate effective capabilities
        let (effective_capabilities, effective_autonomy_tier) =
            match (&agent_row, &partner_row) {
                (Some(agent), Some(partner)) => {
                    let caps: Vec<String> = agent
                        .base_capabilities
                        .iter()
                        .filter(|c| partner.capabilities_granted.contains(c))
                        .filter(|c| !partner.capabilities_denied.contains(c))
                        .cloned()
                        .collect();

                    let tier = std::cmp::min(agent.max_autonomy_tier, partner.max_autonomy_tier);
                    (caps, tier)
                }
                _ => (vec![], 0),
            };

        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs() as i64;

        Ok(Response::new(VerifyDeploymentResponse {
            agent: agent_row.map(|r| r.to_proto()),
            partner: partner_row.map(|r| r.to_proto()),
            agent_found,
            partner_found,
            effective_capabilities,
            effective_autonomy_tier,
            mandatory_disclosure: String::new(),
            query_timestamp: now,
            response_signature: None,
            error: None,
            context: Some(self.response_context(request_id, None)),
        }))
    }

    async fn get_revocation_list(
        &self,
        request: Request<GetRevocationListRequest>,
    ) -> Result<Response<GetRevocationListResponse>, Status> {
        let req = request.into_inner();
        let request_id = req.context.as_ref().map(|c| c.request_id.clone());

        // Get revocations since the specified version (if any)
        let since_version = if req.since_version > 0 {
            Some(req.since_version as i32)
        } else {
            None
        };

        let (entries, list_version) = db::get_revocation_list(self.db.pool(), since_version)
            .await
            .map_err(RegistryError::from)?;

        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs() as i64;

        let revocation_entries: Vec<RevocationEntry> = entries.iter().map(|e| e.to_proto()).collect();

        let revocations = RevocationList {
            entries: revocation_entries,
            list_version: list_version as i64,
            generated_at: now,
            next_update: now + 3600, // 1 hour from now
            list_signature: None, // Would need to sign the list
        };

        Ok(Response::new(GetRevocationListResponse {
            revocations: Some(revocations),
            is_delta: since_version.is_some(),
            context: Some(self.response_context(request_id, None)),
        }))
    }

    async fn get_public_keys(
        &self,
        request: Request<GetPublicKeysRequest>,
    ) -> Result<Response<GetPublicKeysResponse>, Status> {
        let req = request.into_inner();
        let request_id = req.context.as_ref().map(|c| c.request_id.clone());

        let key = if req.key_id.is_empty() {
            db::get_active_key(self.db.pool(), &req.org_id)
                .await
                .map_err(RegistryError::from)?
        } else {
            db::get_key(self.db.pool(), &req.key_id)
                .await
                .map_err(RegistryError::from)?
        };

        match key {
            Some(k) => Ok(Response::new(GetPublicKeysResponse {
                public_keys: Some(PublicKeys {
                    ed25519_public_key: k.ed25519_public_key.into(),
                    ml_dsa_65_public_key: k.ml_dsa_65_public_key.into(),
                }),
                key_id: k.key_id,
                status: k.status,
                found: true,
                error: None,
                context: Some(self.response_context(request_id, None)),
            })),
            None => Ok(Response::new(GetPublicKeysResponse {
                public_keys: None,
                key_id: String::new(),
                status: 0,
                found: false,
                error: None,
                context: Some(self.response_context(request_id, None)),
            })),
        }
    }

    async fn get_offline_package(
        &self,
        request: Request<GetOfflinePackageRequest>,
    ) -> Result<Response<GetOfflinePackageResponse>, Status> {
        let req = request.into_inner();
        let request_id = req.context.as_ref().map(|c| c.request_id.clone());

        // Get all active agents
        let agents = db::get_all_agents_for_snapshot(self.db.pool())
            .await
            .map_err(RegistryError::from)?;

        // Get all active partners
        let partners = db::get_all_partners_for_snapshot(self.db.pool())
            .await
            .map_err(RegistryError::from)?;

        // Get revocation list
        let (revocations, _) = db::get_revocation_list(self.db.pool(), None)
            .await
            .map_err(RegistryError::from)?;

        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs() as i64;

        // Serialize and compress data
        let agents_proto: Vec<AgentRecord> = agents.iter().map(|a| a.to_proto()).collect();
        let partners_proto: Vec<PartnerRecord> = partners.iter().map(|p| p.to_proto()).collect();
        let revocations_proto: Vec<RevocationEntry> = revocations.iter().map(|r| r.to_proto()).collect();

        // For now, just serialize as JSON and compress (could use protobuf serialization)
        use flate2::write::GzEncoder;
        use flate2::Compression;
        use std::io::Write;

        let agents_json = serde_json::to_vec(&agents_proto.iter().map(|a| {
            serde_json::json!({
                "agent_hash": hex::encode(&a.agent_hash),
                "agent_type": a.agent_type,
                "status": a.status
            })
        }).collect::<Vec<_>>()).unwrap_or_default();

        let mut agents_encoder = GzEncoder::new(Vec::new(), Compression::default());
        agents_encoder.write_all(&agents_json).ok();
        let agents_data = agents_encoder.finish().unwrap_or_default();

        let partners_json = serde_json::to_vec(&partners_proto.iter().map(|p| {
            serde_json::json!({
                "partner_id": &p.partner_id,
                "organization_name": &p.organization_name,
                "status": p.status
            })
        }).collect::<Vec<_>>()).unwrap_or_default();

        let mut partners_encoder = GzEncoder::new(Vec::new(), Compression::default());
        partners_encoder.write_all(&partners_json).ok();
        let partners_data = partners_encoder.finish().unwrap_or_default();

        let revocations_json = serde_json::to_vec(&revocations_proto.iter().map(|r| {
            serde_json::json!({
                "target_type": r.target_type,
                "target_id": &r.target_id,
                "revoked_at": r.revoked_at,
                "reason_code": r.reason_code
            })
        }).collect::<Vec<_>>()).unwrap_or_default();
        let mut revocations_encoder = GzEncoder::new(Vec::new(), Compression::default());
        revocations_encoder.write_all(&revocations_json).ok();
        let revocations_data = revocations_encoder.finish().unwrap_or_default();

        // Compute simple SHA-256 roots (proper Merkle tree would be better)
        use sha2::{Sha256, Digest};
        let agents_root = Sha256::digest(&agents_data).to_vec();
        let partners_root = Sha256::digest(&partners_data).to_vec();
        let revocations_root = Sha256::digest(&revocations_data).to_vec();

        // Compute snapshot root from combined roots
        let mut snapshot_hasher = Sha256::new();
        snapshot_hasher.update(&agents_root);
        snapshot_hasher.update(&partners_root);
        snapshot_hasher.update(&revocations_root);
        let snapshot_root = snapshot_hasher.finalize().to_vec();

        // Sign the snapshot root
        let package_signature = self.crypto.sign(&snapshot_root)?;

        let package = OfflineVerificationPackage {
            agents_data: agents_data.into(),
            partners_data: partners_data.into(),
            revocations_data: revocations_data.into(),
            agents_root: agents_root.into(),
            partners_root: partners_root.into(),
            revocations_root: revocations_root.into(),
            snapshot_root: snapshot_root.into(),
            package_signature: Some(package_signature),
            signer_public_key_classical: self.crypto.ed25519_public_key().to_vec().into(),
            signer_public_key_pqc: self.crypto.mldsa_public_key().to_vec().into(),
            snapshot_timestamp: now,
            api_version: "1.1.0".to_string(),
            expires_at: now + 72 * 3600, // 72 hours
            compression: "gzip".to_string(),
        };

        Ok(Response::new(GetOfflinePackageResponse {
            package: Some(package),
            context: Some(self.response_context(request_id, None)),
        }))
    }

    async fn get_offline_delta(
        &self,
        request: Request<GetOfflineDeltaRequest>,
    ) -> Result<Response<GetOfflineDeltaResponse>, Status> {
        let req = request.into_inner();
        let request_id = req.context.as_ref().map(|c| c.request_id.clone());

        let since_timestamp = req.since_snapshot_timestamp;

        // Get agents modified since the snapshot
        let modified_agents = db::get_agents_since_snapshot(self.db.pool(), since_timestamp)
            .await
            .map_err(RegistryError::from)?;

        // Get partners modified since the snapshot
        let modified_partners = db::get_partners_since_snapshot(self.db.pool(), since_timestamp)
            .await
            .map_err(RegistryError::from)?;

        // Get new revocations since the snapshot
        let (revocations, _) = db::get_revocation_list(self.db.pool(), None)
            .await
            .map_err(RegistryError::from)?;

        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs() as i64;

        // Separate added vs modified agents (those with status=Revoked are "removed")
        let added_agents: Vec<AgentRecord> = modified_agents
            .iter()
            .filter(|a| a.registered_at.unix_timestamp() > since_timestamp)
            .filter(|a| a.status != AgentStatus::AgentRevoked as i32)
            .map(|a| a.to_proto())
            .collect();

        let modified_agents_proto: Vec<AgentRecord> = modified_agents
            .iter()
            .filter(|a| a.registered_at.unix_timestamp() <= since_timestamp)
            .filter(|a| a.status != AgentStatus::AgentRevoked as i32)
            .map(|a| a.to_proto())
            .collect();

        let removed_agent_hashes: Vec<Vec<u8>> = modified_agents
            .iter()
            .filter(|a| a.status == AgentStatus::AgentRevoked as i32)
            .map(|a| a.agent_hash.clone())
            .collect();

        // Similar for partners
        let added_partners: Vec<PartnerRecord> = modified_partners
            .iter()
            .filter(|p| p.issued_at.unix_timestamp() > since_timestamp)
            .filter(|p| p.status != PartnerStatus::PartnerRevoked as i32)
            .map(|p| p.to_proto())
            .collect();

        let modified_partners_proto: Vec<PartnerRecord> = modified_partners
            .iter()
            .filter(|p| p.issued_at.unix_timestamp() <= since_timestamp)
            .filter(|p| p.status != PartnerStatus::PartnerRevoked as i32)
            .map(|p| p.to_proto())
            .collect();

        let removed_partner_ids: Vec<String> = modified_partners
            .iter()
            .filter(|p| p.status == PartnerStatus::PartnerRevoked as i32)
            .map(|p| p.partner_id.clone())
            .collect();

        // New revocations
        let new_revocations: Vec<RevocationEntry> = revocations
            .iter()
            .filter(|r| r.revoked_at.unix_timestamp() > since_timestamp)
            .map(|r| r.to_proto())
            .collect();

        let delta = OfflineSnapshotDelta {
            previous_snapshot_root: vec![].into(), // Would need previous snapshot root
            previous_snapshot_timestamp: since_timestamp,
            added_agents,
            modified_agents: modified_agents_proto,
            removed_agent_hashes,
            added_partners,
            modified_partners: modified_partners_proto,
            removed_partner_ids,
            new_revocations,
            new_agents_root: vec![].into(), // Would need to compute
            new_partners_root: vec![].into(),
            new_revocations_root: vec![].into(),
            new_snapshot_root: vec![].into(),
            delta_signature: None,
            delta_timestamp: now,
            expires_at: now + 72 * 3600, // 72 hours
        };

        Ok(Response::new(GetOfflineDeltaResponse {
            delta: Some(delta),
            next_delta_available_at: now + 300, // 5 minutes from now
            context: Some(self.response_context(request_id, None)),
        }))
    }

    async fn get_build_attestation(
        &self,
        request: Request<GetBuildAttestationRequest>,
    ) -> Result<Response<GetBuildAttestationResponse>, Status> {
        let req = request.into_inner();
        let request_id = req.context.as_ref().map(|c| c.request_id.clone());

        let attestation_row = db::get_attestation(self.db.pool(), &req.agent_hash)
            .await
            .map_err(RegistryError::from)?;

        match attestation_row {
            Some(row) => Ok(Response::new(GetBuildAttestationResponse {
                attestation: Some(row.to_proto()),
                found: true,
                independent_verification_count: row.verification_count,
                last_verified_at: row.last_verified_at.map(|t| t.unix_timestamp()).unwrap_or(0),
                context: Some(self.response_context(request_id, None)),
            })),
            None => Ok(Response::new(GetBuildAttestationResponse {
                attestation: None,
                found: false,
                independent_verification_count: 0,
                last_verified_at: 0,
                context: Some(self.response_context(request_id, None)),
            })),
        }
    }

    async fn get_emergency_status(
        &self,
        request: Request<GetEmergencyStatusRequest>,
    ) -> Result<Response<EmergencyStatusResponse>, Status> {
        let req = request.into_inner();
        let request_id = req.context.as_ref().map(|c| c.request_id.clone());

        let status = db::get_emergency_status(self.db.pool())
            .await
            .map_err(RegistryError::from)?;

        let mut response = status.to_proto();
        response.context = Some(self.response_context(request_id, None));

        Ok(Response::new(response))
    }
}

