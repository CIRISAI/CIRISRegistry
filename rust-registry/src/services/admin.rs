//! RegistryAdminService implementation

use std::sync::Arc;
use std::time::{Instant, SystemTime, UNIX_EPOCH};

use tonic::{Request, Response, Status};
use tracing::info;

use crate::config::Environment;
use crate::crypto::HybridCrypto;
use crate::db::{self, Database};
use crate::middleware::auth::Claims;
use crate::proto::registry_admin_service_server::RegistryAdminService as AdminServiceTrait;
use crate::proto::*;

pub struct AdminService {
    db: Database,
    crypto: Arc<HybridCrypto>,
    environment: i32,
}

impl AdminService {
    pub fn new(db: Database, crypto: Arc<HybridCrypto>, environment: Environment) -> Self {
        Self {
            db,
            crypto,
            environment: environment.to_proto_i32(),
        }
    }

    /// Extract operator ID from JWT claims stored in request extensions.
    /// Falls back to "unknown" if claims are not present (should not happen
    /// on admin endpoints since auth middleware enforces JWT).
    fn extract_operator_id<T>(request: &Request<T>) -> String {
        request
            .extensions()
            .get::<Claims>()
            .map(|c| c.sub.clone())
            .unwrap_or_else(|| "unknown".to_string())
    }

    /// Extract org_id from JWT claims stored in request extensions.
    /// Falls back to "default" if no org_id claim is present.
    fn extract_org_id<T>(request: &Request<T>) -> String {
        request
            .extensions()
            .get::<Claims>()
            .map(|c| c.org_id.clone())
            .filter(|id| !id.is_empty())
            .unwrap_or_else(|| "default".to_string())
    }

    fn response_context(
        &self,
        request_id: Option<String>,
        start_time: Option<Instant>,
    ) -> ResponseContext {
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
            environment: self.environment,
        }
    }

    fn admin_response(
        &self,
        success: bool,
        message: &str,
        request_id: Option<String>,
    ) -> AdminResponse {
        AdminResponse {
            success,
            message: message.to_string(),
            error: if success {
                None
            } else {
                Some(ErrorDetail {
                    code: RegistryErrorCode::RegistryErrorInternal as i32,
                    message: message.to_string(),
                    retry_status: Retryable::RetryNo as i32,
                    retry_after_seconds: 0,
                    metadata: Default::default(),
                    cause: None,
                })
            },
            context: Some(self.response_context(request_id, None)),
        }
    }
}

#[tonic::async_trait]
impl AdminServiceTrait for AdminService {
    async fn register_agent(
        &self,
        request: Request<RegisterAgentRequest>,
    ) -> Result<Response<AdminResponse>, Status> {
        let req = request.into_inner();
        let request_id = req.context.as_ref().map(|c| c.request_id.clone());

        let agent = req
            .agent
            .ok_or_else(|| Status::invalid_argument("agent is required"))?;

        info!(
            agent_hash = hex::encode(&agent.agent_hash),
            "Registering agent"
        );

        db::register_agent(self.db.pool(), &agent)
            .await
            .map_err(|e| Status::internal(e.to_string()))?;

        Ok(Response::new(self.admin_response(
            true,
            "Agent registered successfully",
            request_id,
        )))
    }

    async fn list_registered_agents(
        &self,
        request: Request<ListRegisteredAgentsRequest>,
    ) -> Result<Response<ListRegisteredAgentsResponse>, Status> {
        let req = request.into_inner();
        let request_id = req.context.as_ref().map(|c| c.request_id.clone());

        let page_size = if req.page_size <= 0 || req.page_size > 100 {
            50
        } else {
            req.page_size
        };

        // Parse page token as offset
        let offset: i32 = req.page_token.parse().unwrap_or(0);

        let agent_type = if req.agent_type == 0 {
            None
        } else {
            Some(req.agent_type)
        };

        let status = if req.status == 0 {
            None
        } else {
            Some(req.status)
        };

        let version_prefix = if req.version_prefix.is_empty() {
            None
        } else {
            Some(req.version_prefix.as_str())
        };

        let search_query = if req.search_query.is_empty() {
            None
        } else {
            Some(req.search_query.as_str())
        };

        let order_by = if req.order_by.is_empty() {
            "registered_at"
        } else {
            &req.order_by
        };

        let result = db::list_registered_agents(
            self.db.pool(),
            agent_type,
            status,
            version_prefix,
            search_query,
            req.include_test_records,
            page_size,
            offset,
            order_by,
            req.descending,
        )
        .await
        .map_err(|e| Status::internal(e.to_string()))?;

        // Calculate next page token
        let next_page_token = if result.agents.len() as i32 == page_size {
            (offset + page_size).to_string()
        } else {
            String::new()
        };

        Ok(Response::new(ListRegisteredAgentsResponse {
            agents: result.agents.iter().map(|a| a.to_proto()).collect(),
            next_page_token,
            total_count: result.total_count,
            active_count: result.active_count,
            deprecated_count: result.deprecated_count,
            revoked_count: result.revoked_count,
            context: Some(self.response_context(request_id, None)),
        }))
    }

    async fn batch_register_agents(
        &self,
        request: Request<BatchRegisterAgentsRequest>,
    ) -> Result<Response<BatchRegisterAgentsResponse>, Status> {
        let req = request.into_inner();
        let request_id = req.context.as_ref().map(|c| c.request_id.clone());

        if req.agents.len() > 1000 {
            return Err(Status::invalid_argument("Maximum batch size is 1000"));
        }

        let mut succeeded = 0;
        let mut failed = 0;
        let mut created = Vec::new();
        let mut errors = Vec::new();

        for agent in req.agents {
            match db::register_agent(self.db.pool(), &agent).await {
                Ok(_) => {
                    succeeded += 1;
                    created.push(agent);
                }
                Err(e) => {
                    failed += 1;
                    errors.push(batch_register_agents_response::Error {
                        agent: Some(agent),
                        error: Some(ErrorDetail {
                            code: RegistryErrorCode::RegistryErrorInternal as i32,
                            message: e.to_string(),
                            retry_status: Retryable::RetryNo as i32,
                            retry_after_seconds: 0,
                            metadata: Default::default(),
                            cause: None,
                        }),
                    });
                }
            }
        }

        Ok(Response::new(BatchRegisterAgentsResponse {
            succeeded,
            failed,
            created,
            errors,
            batch_id: uuid::Uuid::new_v4().to_string(),
            context: Some(self.response_context(request_id, None)),
        }))
    }

    async fn register_partner(
        &self,
        request: Request<RegisterPartnerRequest>,
    ) -> Result<Response<AdminResponse>, Status> {
        let req = request.into_inner();
        let request_id = req.context.as_ref().map(|c| c.request_id.clone());

        let partner = req
            .partner
            .ok_or_else(|| Status::invalid_argument("partner is required"))?;

        info!(partner_id = %partner.partner_id, "Registering partner");

        db::register_partner(self.db.pool(), &partner)
            .await
            .map_err(|e| Status::internal(e.to_string()))?;

        Ok(Response::new(self.admin_response(
            true,
            "Partner registered successfully",
            request_id,
        )))
    }

    async fn revoke_entity(
        &self,
        request: Request<RevokeEntityRequest>,
    ) -> Result<Response<AdminResponse>, Status> {
        let req = request.into_inner();
        let request_id = req.context.as_ref().map(|c| c.request_id.clone());

        match RevocationType::try_from(req.target_type) {
            Ok(RevocationType::AgentHash) => {
                let hash = hex::decode(&req.target_id)
                    .map_err(|_| Status::invalid_argument("Invalid agent hash hex"))?;
                db::revoke_agent(self.db.pool(), &hash, &req.reason_detail)
                    .await
                    .map_err(|e| Status::internal(e.to_string()))?;
            }
            _ => {
                return Err(Status::unimplemented("Revocation type not yet implemented"));
            }
        }

        Ok(Response::new(self.admin_response(
            true,
            "Entity revoked successfully",
            request_id,
        )))
    }

    async fn mass_revoke(
        &self,
        request: Request<MassRevokeRequest>,
    ) -> Result<Response<MassRevokeResponse>, Status> {
        let operator_id = Self::extract_operator_id(&request);
        let req = request.into_inner();
        let request_id = req.context.as_ref().map(|c| c.request_id.clone());

        info!(
            agent_hashes_count = req.agent_hashes.len(),
            partner_ids_count = req.partner_ids.len(),
            version_prefix = %req.agent_version_prefix,
            severity = ?req.severity,
            is_dry_run = req.is_dry_run,
            "Processing mass revoke request"
        );

        let mut revoked_count = 0;
        let mut errors = Vec::new();

        let reason_code = req.severity; // Use severity as reason code
        let reason_detail = if req.incident_details.is_empty() {
            &req.incident_summary
        } else {
            &req.incident_details
        };

        // Revoke by agent hashes
        if !req.agent_hashes.is_empty() && !req.is_dry_run {
            let hashes: Vec<Vec<u8>> = req.agent_hashes.iter().map(|h| h.to_vec()).collect();
            match db::mass_revoke_agents(self.db.pool(), &hashes, reason_code, reason_detail).await
            {
                Ok(count) => revoked_count += count,
                Err(e) => errors.push(format!("Failed to revoke agents by hash: {}", e)),
            }
        } else if !req.agent_hashes.is_empty() {
            // Dry run - just count
            revoked_count += req.agent_hashes.len() as i32;
        }

        // Revoke by version prefix
        if !req.agent_version_prefix.is_empty() && !req.is_dry_run {
            match db::mass_revoke_by_version_prefix(
                self.db.pool(),
                &req.agent_version_prefix,
                reason_code,
                reason_detail,
            )
            .await
            {
                Ok(count) => revoked_count += count,
                Err(e) => errors.push(format!("Failed to revoke by version prefix: {}", e)),
            }
        } else if !req.agent_version_prefix.is_empty() {
            // Dry run - count matching agents
            match db::count_agents_by_version_prefix(self.db.pool(), &req.agent_version_prefix)
                .await
            {
                Ok(count) => revoked_count += count,
                Err(e) => errors.push(format!("Failed to count agents for dry run: {}", e)),
            }
        }

        // Revoke by partner IDs
        if !req.partner_ids.is_empty() && !req.is_dry_run {
            match db::mass_revoke_partners(
                self.db.pool(),
                &req.partner_ids,
                reason_code,
                reason_detail,
            )
            .await
            {
                Ok(count) => revoked_count += count,
                Err(e) => errors.push(format!("Failed to revoke partners: {}", e)),
            }
        } else if !req.partner_ids.is_empty() {
            // Dry run
            revoked_count += req.partner_ids.len() as i32;
        }

        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs() as i64;

        Ok(Response::new(MassRevokeResponse {
            revoked_count,
            affected_deployments: 0, // Would need to track this
            dry_run_count: if req.is_dry_run { revoked_count } else { 0 },
            agents_revoked: if !req.agent_hashes.is_empty() || !req.agent_version_prefix.is_empty()
            {
                revoked_count
            } else {
                0
            },
            partners_revoked: if !req.partner_ids.is_empty() {
                revoked_count
            } else {
                0
            },
            licenses_revoked: 0, // License revocation not implemented
            incident_id: req.incident_id.clone(),
            executed_at: now,
            operator_id,
            audit_log_entry_id: uuid::Uuid::new_v4().to_string(),
            response_signature: None, // Would need to sign
            context: Some(self.response_context(request_id, None)),
        }))
    }

    async fn set_emergency_shutdown(
        &self,
        request: Request<EmergencyShutdownRequest>,
    ) -> Result<Response<EmergencyShutdownResponse>, Status> {
        let operator_id = Self::extract_operator_id(&request);
        let req = request.into_inner();
        let request_id = req.context.as_ref().map(|c| c.request_id.clone());

        info!(
            reason = %req.reason,
            severity = ?req.severity,
            "Setting emergency shutdown"
        );

        db::set_emergency_shutdown(
            self.db.pool(),
            &req.reason,
            req.severity,
            req.lock_duration_seconds,
            &req.allowed_operations,
            &operator_id,
        )
        .await
        .map_err(|e| Status::internal(e.to_string()))?;

        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs() as i64;

        Ok(Response::new(EmergencyShutdownResponse {
            enabled: true,
            locked_at: now,
            locked_until: if req.lock_duration_seconds > 0 {
                now + req.lock_duration_seconds
            } else {
                0
            },
            operator_id,
            context: Some(self.response_context(request_id, None)),
        }))
    }

    async fn clear_emergency_shutdown(
        &self,
        request: Request<ClearEmergencyShutdownRequest>,
    ) -> Result<Response<AdminResponse>, Status> {
        let req = request.into_inner();
        let request_id = req.context.as_ref().map(|c| c.request_id.clone());

        info!(reason = %req.reason, "Clearing emergency shutdown");

        db::clear_emergency_shutdown(self.db.pool())
            .await
            .map_err(|e| Status::internal(e.to_string()))?;

        Ok(Response::new(self.admin_response(
            true,
            "Emergency shutdown cleared",
            request_id,
        )))
    }

    async fn rotate_signing_key(
        &self,
        request: Request<RotateSigningKeyRequest>,
    ) -> Result<Response<RotateSigningKeyResponse>, Status> {
        let operator_id = Self::extract_operator_id(&request);
        let req = request.into_inner();
        let request_id = req.context.as_ref().map(|c| c.request_id.clone());

        info!(new_key_id = %req.new_key_id, "Rotating registry signing key");

        // Get current active signing key
        let old_key = db::get_active_signing_key(self.db.pool())
            .await
            .map_err(|e| Status::internal(e.to_string()))?
            .ok_or_else(|| Status::failed_precondition("No active signing key to rotate"))?;

        // Generate new key pair
        let new_crypto =
            HybridCrypto::generate_ephemeral().map_err(|e| Status::internal(e.to_string()))?;

        let ed25519_pubkey = new_crypto.ed25519_public_key();
        let mldsa_pubkey = new_crypto.mldsa_public_key();
        let ed25519_fp = HybridCrypto::fingerprint(&ed25519_pubkey);
        let mldsa_fp = HybridCrypto::fingerprint(&mldsa_pubkey);

        // Create new key in pending state
        let new_key_id = db::create_signing_key(
            self.db.pool(),
            req.target_storage,
            &ed25519_pubkey,
            &ed25519_fp,
            &mldsa_pubkey,
            &mldsa_fp,
            None, // HSM slot
            None, // HSM label
        )
        .await
        .map_err(|e| Status::internal(e.to_string()))?;

        // Perform rotation
        db::rotate_signing_key(self.db.pool(), &old_key.key_id, &new_key_id, &operator_id)
            .await
            .map_err(|e| Status::internal(e.to_string()))?;

        // Fetch the new key
        let new_key = db::get_signing_key(self.db.pool(), &new_key_id)
            .await
            .map_err(|e| Status::internal(e.to_string()))?
            .ok_or_else(|| Status::internal("New signing key not found"))?;

        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs() as i64;

        Ok(Response::new(RotateSigningKeyResponse {
            new_key_id: new_key_id.clone(),
            old_key_id: old_key.key_id.clone(),
            new_key: Some(new_key.to_proto()),
            cutover_time: now,
            is_dual_signing: false, // Immediate cutover
            context: Some(self.response_context(request_id, None)),
        }))
    }

    async fn get_active_signing_key(
        &self,
        request: Request<GetActiveSigningKeyRequest>,
    ) -> Result<Response<GetActiveSigningKeyResponse>, Status> {
        let req = request.into_inner();
        let request_id = req.context.as_ref().map(|c| c.request_id.clone());

        let key = db::get_active_signing_key(self.db.pool())
            .await
            .map_err(|e| Status::internal(e.to_string()))?;

        Ok(Response::new(GetActiveSigningKeyResponse {
            key: key.map(|k| k.to_proto()),
            dual_signing_active: false,
            secondary_key_id: String::new(),
            context: Some(self.response_context(request_id, None)),
        }))
    }

    async fn list_signing_keys(
        &self,
        request: Request<ListSigningKeysRequest>,
    ) -> Result<Response<ListSigningKeysResponse>, Status> {
        let req = request.into_inner();
        let request_id = req.context.as_ref().map(|c| c.request_id.clone());

        let keys = db::list_signing_keys(self.db.pool(), req.include_retired)
            .await
            .map_err(|e| Status::internal(e.to_string()))?;

        Ok(Response::new(ListSigningKeysResponse {
            keys: keys.iter().map(|k| k.to_proto()).collect(),
            context: Some(self.response_context(request_id, None)),
        }))
    }

    async fn test_hsm_connection(
        &self,
        request: Request<TestHsmConnectionRequest>,
    ) -> Result<Response<TestHsmConnectionResponse>, Status> {
        let req = request.into_inner();
        let request_id = req.context.as_ref().map(|c| c.request_id.clone());

        // For now, we don't have actual HSM integration
        // Return success if using file-based keys, or test Vault connection if configured

        // TODO: Add actual HSM/Vault connection test
        Ok(Response::new(TestHsmConnectionResponse {
            connected: true,
            status: "SIMULATED - No actual HSM connection".to_string(),
            hsm_model: "Simulated HSM".to_string(),
            available_slots: 10,
            context: Some(self.response_context(request_id, None)),
        }))
    }

    async fn register_build_attestation(
        &self,
        request: Request<RegisterBuildAttestationRequest>,
    ) -> Result<Response<AdminResponse>, Status> {
        let req = request.into_inner();
        let request_id = req.context.as_ref().map(|c| c.request_id.clone());

        let attestation = req
            .attestation
            .ok_or_else(|| Status::invalid_argument("attestation required"))?;

        info!(
            agent_hash = hex::encode(&req.agent_hash),
            "Registering build attestation"
        );

        db::register_attestation(self.db.pool(), &req.agent_hash, &attestation)
            .await
            .map_err(|e| Status::internal(e.to_string()))?;

        Ok(Response::new(self.admin_response(
            true,
            "Build attestation registered successfully",
            request_id,
        )))
    }

    async fn register_webhook(
        &self,
        request: Request<RegisterWebhookRequest>,
    ) -> Result<Response<AdminResponse>, Status> {
        let org_id = Self::extract_org_id(&request);
        let req = request.into_inner();
        let request_id = req.context.as_ref().map(|c| c.request_id.clone());

        let config = req
            .config
            .ok_or_else(|| Status::invalid_argument("config is required"))?;

        info!(url = %config.url, org_id = %org_id, "Registering webhook");

        let (webhook_id, _signing_secret) = db::register_webhook(
            self.db.pool(),
            &org_id,
            &config.url,
            &config.subscribed_events,
        )
        .await
        .map_err(|e| Status::internal(e.to_string()))?;

        Ok(Response::new(self.admin_response(
            true,
            &format!("Webhook registered with ID: {}", webhook_id),
            request_id,
        )))
    }

    async fn list_webhooks(
        &self,
        request: Request<ListWebhooksRequest>,
    ) -> Result<Response<ListWebhooksResponse>, Status> {
        let org_id = Self::extract_org_id(&request);
        let req = request.into_inner();
        let request_id = req.context.as_ref().map(|c| c.request_id.clone());

        let webhooks = db::list_webhooks(self.db.pool(), &org_id)
            .await
            .map_err(|e| Status::internal(e.to_string()))?;

        Ok(Response::new(ListWebhooksResponse {
            webhooks: webhooks.iter().map(|w| w.to_proto()).collect(),
            context: Some(self.response_context(request_id, None)),
        }))
    }

    async fn delete_webhook(
        &self,
        request: Request<DeleteWebhookRequest>,
    ) -> Result<Response<AdminResponse>, Status> {
        let req = request.into_inner();
        let request_id = req.context.as_ref().map(|c| c.request_id.clone());

        info!(webhook_id = %req.webhook_id, "Deleting webhook");

        // First, get the webhook to find its org_id
        let webhook = db::get_webhook(self.db.pool(), &req.webhook_id)
            .await
            .map_err(|e| Status::internal(e.to_string()))?
            .ok_or_else(|| Status::not_found("Webhook not found"))?;

        let deleted = db::delete_webhook(self.db.pool(), &req.webhook_id, &webhook.org_id)
            .await
            .map_err(|e| Status::internal(e.to_string()))?;

        if deleted {
            Ok(Response::new(self.admin_response(
                true,
                "Webhook deleted successfully",
                request_id,
            )))
        } else {
            Ok(Response::new(self.admin_response(
                false,
                "Webhook not found",
                request_id,
            )))
        }
    }

    async fn list_expiring_licenses(
        &self,
        request: Request<ListExpiringLicensesRequest>,
    ) -> Result<Response<ListExpiringLicensesResponse>, Status> {
        let req = request.into_inner();
        let request_id = req.context.as_ref().map(|c| c.request_id.clone());

        let partners = db::list_expiring_licenses(
            self.db.pool(),
            req.expiring_within_days,
            req.include_expired,
        )
        .await
        .map_err(|e| Status::internal(e.to_string()))?;

        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs() as i64;

        let licenses: Vec<ExpiringLicense> = partners
            .iter()
            .map(|p| ExpiringLicense {
                partner_id: p.partner_id.clone(),
                organization_name: p.organization_name.clone(),
                license_id: p.license_id.clone(),
                expires_at: p.expires_at.unix_timestamp(),
                days_remaining: ((p.expires_at.unix_timestamp() - now) / 86400) as i32,
                technical_contact: p.technical_contact.clone().unwrap_or_default(),
                compliance_contact: p.compliance_contact.clone().unwrap_or_default(),
                renewal_status: "PENDING".to_string(),
            })
            .collect();

        let count_expired = licenses.iter().filter(|l| l.days_remaining < 0).count() as i32;
        let count_expiring = licenses.iter().filter(|l| l.days_remaining >= 0).count() as i32;

        Ok(Response::new(ListExpiringLicensesResponse {
            licenses,
            count_expiring_soon: count_expiring,
            count_already_expired: count_expired,
            context: Some(self.response_context(request_id, None)),
        }))
    }

    async fn get_partner_activity(
        &self,
        request: Request<GetPartnerActivityRequest>,
    ) -> Result<Response<PartnerActivityResponse>, Status> {
        let req = request.into_inner();
        let request_id = req.context.as_ref().map(|c| c.request_id.clone());

        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs() as i64;

        // Get audit log entries for this partner in the last 30 days
        let thirty_days_ago = now - (30 * 24 * 60 * 60);
        let action_types: Vec<i32> = vec![]; // All action types
        let (audit_entries, _) = db::get_audit_log(
            self.db.pool(),
            &req.partner_id,
            thirty_days_ago,
            now,
            &action_types,
            1000,
            0,
        )
        .await
        .map_err(|e| Status::internal(e.to_string()))?;

        // Count lookups in the last 30 days (agent registrations, partner lookups)
        let lookups_last_30_days = audit_entries.len() as i64;

        // Last lookup timestamp
        let last_lookup_at = audit_entries
            .iter()
            .map(|e| e.timestamp.unix_timestamp())
            .max()
            .unwrap_or(0);

        // Find last portal login
        let last_portal_login = audit_entries
            .iter()
            .filter(|e| e.action == AuditActionType::AuditUserLogin as i32)
            .map(|e| e.timestamp.unix_timestamp())
            .max()
            .unwrap_or(0);

        // Find last key rotation
        let last_key_rotation = audit_entries
            .iter()
            .filter(|e| e.action == AuditActionType::AuditKeyRotated as i32)
            .map(|e| e.timestamp.unix_timestamp())
            .max()
            .unwrap_or(0);

        let days_since_key_rotation = if last_key_rotation > 0 {
            ((now - last_key_rotation) / 86400) as i32
        } else {
            -1 // Never rotated
        };

        // Determine health status
        let health_status = if last_lookup_at > now - (7 * 24 * 60 * 60) {
            "HEALTHY".to_string()
        } else if last_lookup_at > now - (30 * 24 * 60 * 60) {
            "IDLE".to_string()
        } else {
            "INACTIVE".to_string()
        };

        let recommendations = if days_since_key_rotation > 90 {
            "Consider rotating API keys for security".to_string()
        } else if health_status == "INACTIVE" {
            "No activity in 30+ days, verify integration".to_string()
        } else {
            String::new()
        };

        Ok(Response::new(PartnerActivityResponse {
            partner_id: req.partner_id,
            last_lookup_at,
            lookups_last_30_days,
            last_portal_login,
            active_users: 0, // Would need to query users table
            last_key_rotation,
            days_since_key_rotation,
            health_status,
            recommendations,
            context: Some(self.response_context(request_id, None)),
        }))
    }

    async fn cleanup_test_records(
        &self,
        request: Request<CleanupTestRecordsRequest>,
    ) -> Result<Response<CleanupTestRecordsResponse>, Status> {
        let req = request.into_inner();
        let request_id = req.context.as_ref().map(|c| c.request_id.clone());

        let removed = db::cleanup_test_records(self.db.pool(), &req.test_tag)
            .await
            .map_err(|e| Status::internal(e.to_string()))?;

        Ok(Response::new(CleanupTestRecordsResponse {
            records_removed: removed as i32,
            context: Some(self.response_context(request_id, None)),
        }))
    }

    async fn register_build(
        &self,
        request: Request<RegisterBuildRequest>,
    ) -> Result<Response<AdminResponse>, Status> {
        let req = request.into_inner();
        let request_id = req.context.as_ref().map(|c| c.request_id.clone());

        let build = req
            .build
            .ok_or_else(|| Status::invalid_argument("build is required"))?;

        info!(
            version = %build.version,
            build_hash = %build.build_hash,
            file_count = build.file_manifest_count,
            "Registering build"
        );

        let build_id = db::register_build(self.db.pool(), &build)
            .await
            .map_err(|e| Status::internal(e.to_string()))?;

        Ok(Response::new(self.admin_response(
            true,
            &format!("Build registered successfully with ID: {}", build_id),
            request_id,
        )))
    }

    async fn get_build(
        &self,
        request: Request<GetBuildRequest>,
    ) -> Result<Response<GetBuildResponse>, Status> {
        let req = request.into_inner();
        let request_id = req.context.as_ref().map(|c| c.request_id.clone());

        let version = if req.version.is_empty() {
            None
        } else {
            Some(req.version.as_str())
        };
        let build_hash = if req.build_hash.is_empty() {
            None
        } else {
            Some(req.build_hash.as_str())
        };

        let row = db::get_build(self.db.pool(), version, build_hash)
            .await
            .map_err(|e| Status::internal(e.to_string()))?;

        Ok(Response::new(GetBuildResponse {
            build: row.as_ref().map(|r| r.to_proto()),
            found: row.is_some(),
            context: Some(self.response_context(request_id, None)),
        }))
    }

    async fn list_builds(
        &self,
        request: Request<ListBuildsRequest>,
    ) -> Result<Response<ListBuildsResponse>, Status> {
        let req = request.into_inner();
        let request_id = req.context.as_ref().map(|c| c.request_id.clone());

        let status = if req.status.is_empty() {
            None
        } else {
            Some(req.status.as_str())
        };

        let (rows, total) = db::list_builds(
            self.db.pool(),
            status,
            req.page_size,
            if req.page_token.is_empty() {
                None
            } else {
                Some(req.page_token.as_str())
            },
        )
        .await
        .map_err(|e| Status::internal(e.to_string()))?;

        Ok(Response::new(ListBuildsResponse {
            builds: rows.iter().map(|r| r.to_proto()).collect(),
            total_count: total as i32,
            next_page_token: String::new(),
            context: Some(self.response_context(request_id, None)),
        }))
    }
}
