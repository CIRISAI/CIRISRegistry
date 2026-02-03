//! PortalService implementation (CIRISPortal operations)

use std::sync::Arc;
use std::time::{Instant, SystemTime, UNIX_EPOCH};

use tonic::{Request, Response, Status};
use tracing::info;

use crate::crypto::HybridCrypto;
use crate::db::{self, Database};
use crate::proto::portal_service_server::PortalService as PortalServiceTrait;
use crate::proto::{
    AuditActionType, AuditEntry, AuditExportFormat, KeyRotationMode, KeyStatus, OrgType, User,
    *,
};

pub struct PortalService {
    db: Database,
    crypto: Arc<HybridCrypto>,
}

impl PortalService {
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
            environment: RegistryEnvironment::EnvDevelopment as i32,
        }
    }

    fn admin_response(&self, success: bool, message: &str, request_id: Option<String>) -> AdminResponse {
        AdminResponse {
            success,
            message: message.to_string(),
            error: None,
            context: Some(self.response_context(request_id, None)),
        }
    }
}

#[tonic::async_trait]
impl PortalServiceTrait for PortalService {
    async fn create_organization(
        &self,
        request: Request<CreateOrganizationRequest>,
    ) -> Result<Response<CreateOrganizationResponse>, Status> {
        let req = request.into_inner();
        let request_id = req.context.as_ref().map(|c| c.request_id.clone());

        let org = req.organization.ok_or_else(|| Status::invalid_argument("organization required"))?;

        // Check if initial_admin is provided - use transactional creation to avoid race condition
        if let Some(admin_user) = req.initial_admin {
            info!(name = %org.name, admin_email = %admin_user.email, "Creating organization with initial admin");

            let (org_id, user_id) = db::create_organization_with_admin(self.db.pool(), &org, &admin_user)
                .await
                .map_err(|e| Status::internal(e.to_string()))?;

            // Fetch the created records to return full objects
            let created_org = db::get_organization(self.db.pool(), &org_id)
                .await
                .map_err(|e| Status::internal(e.to_string()))?;
            let created_user = db::get_user(self.db.pool(), &user_id)
                .await
                .map_err(|e| Status::internal(e.to_string()))?;

            // Create audit entry for organization creation
            let _ = db::create_audit_entry(
                self.db.pool(),
                AuditActionType::AuditOrgCreated,
                Some(&user_id),
                Some(&org_id),
                None,
                Some("organization"),
                Some(&org_id),
                &format!("Organization created: {}", org.name),
                Some(serde_json::json!({
                    "org_name": org.name,
                    "org_type": org.org_type,
                    "initial_admin_email": admin_user.email,
                })),
            )
            .await;

            // Create audit entry for admin user creation
            let _ = db::create_audit_entry(
                self.db.pool(),
                AuditActionType::AuditUserCreated,
                Some(&user_id),
                Some(&org_id),
                None,
                Some("user"),
                Some(&user_id),
                &format!("Initial admin user created: {}", admin_user.email),
                Some(serde_json::json!({
                    "user_email": admin_user.email,
                    "role": "ORG_ADMIN",
                })),
            )
            .await;

            Ok(Response::new(CreateOrganizationResponse {
                success: true,
                message: "Organization created with initial admin".to_string(),
                org_id: org_id.clone(),
                user_id: user_id.clone(),
                organization: created_org.map(|o| o.to_proto()),
                admin_user: created_user.map(|u| u.to_proto()),
                error: None,
                context: Some(self.response_context(request_id, None)),
            }))
        } else {
            info!(name = %org.name, "Creating organization");

            let org_id = db::create_organization(self.db.pool(), &org)
                .await
                .map_err(|e| Status::internal(e.to_string()))?;

            // Fetch the created record to return full object
            let created_org = db::get_organization(self.db.pool(), &org_id)
                .await
                .map_err(|e| Status::internal(e.to_string()))?;

            // Create audit entry
            let _ = db::create_audit_entry(
                self.db.pool(),
                AuditActionType::AuditOrgCreated,
                None,
                Some(&org_id),
                None,
                Some("organization"),
                Some(&org_id),
                &format!("Organization created: {}", org.name),
                Some(serde_json::json!({
                    "org_name": org.name,
                    "org_type": org.org_type,
                })),
            )
            .await;

            Ok(Response::new(CreateOrganizationResponse {
                success: true,
                message: "Organization created".to_string(),
                org_id: org_id.clone(),
                user_id: String::new(),
                organization: created_org.map(|o| o.to_proto()),
                admin_user: None,
                error: None,
                context: Some(self.response_context(request_id, None)),
            }))
        }
    }

    async fn get_organization(
        &self,
        request: Request<GetOrganizationRequest>,
    ) -> Result<Response<GetOrganizationResponse>, Status> {
        let req = request.into_inner();
        let request_id = req.context.as_ref().map(|c| c.request_id.clone());

        let org = db::get_organization(self.db.pool(), &req.org_id)
            .await
            .map_err(|e| Status::internal(e.to_string()))?;

        let found = org.is_some();
        Ok(Response::new(GetOrganizationResponse {
            organization: org.map(|o| o.to_proto()),
            found,
            error: None,
            context: Some(self.response_context(request_id, None)),
        }))
    }

    async fn update_organization(
        &self,
        request: Request<UpdateOrganizationRequest>,
    ) -> Result<Response<AdminResponse>, Status> {
        let req = request.into_inner();
        let request_id = req.context.as_ref().map(|c| c.request_id.clone());

        let org = req.organization.ok_or_else(|| Status::invalid_argument("organization required"))?;

        info!(org_id = %org.org_id, "Updating organization");

        let updated = db::update_organization(self.db.pool(), &org.org_id, &org)
            .await
            .map_err(|e| Status::internal(e.to_string()))?;

        if updated {
            // Create audit entry
            let _ = db::create_audit_entry(
                self.db.pool(),
                AuditActionType::AuditOrgUpdated,
                None,
                Some(&org.org_id),
                None,
                Some("organization"),
                Some(&org.org_id),
                &format!("Organization updated: {}", org.org_id),
                Some(serde_json::json!({
                    "org_name": org.name,
                })),
            )
            .await;

            Ok(Response::new(self.admin_response(
                true,
                "Organization updated successfully",
                request_id,
            )))
        } else {
            Ok(Response::new(self.admin_response(
                false,
                "Organization not found",
                request_id,
            )))
        }
    }

    async fn list_organizations(
        &self,
        request: Request<ListOrganizationsRequest>,
    ) -> Result<Response<ListOrganizationsResponse>, Status> {
        let req = request.into_inner();
        let request_id = req.context.as_ref().map(|c| c.request_id.clone());

        let page_size = if req.page_size == 0 { 50 } else { req.page_size };
        let offset: i32 = req.page_token.parse().unwrap_or(0);

        let (orgs, total) = db::list_organizations(
            self.db.pool(),
            page_size,
            offset,
            req.include_inactive,
        )
        .await
        .map_err(|e| Status::internal(e.to_string()))?;

        let next_offset = offset + orgs.len() as i32;
        let next_page_token = if next_offset < total {
            next_offset.to_string()
        } else {
            String::new()
        };

        Ok(Response::new(ListOrganizationsResponse {
            organizations: orgs.iter().map(|o| o.to_proto()).collect(),
            next_page_token,
            total_count: total,
            context: Some(self.response_context(request_id, None)),
        }))
    }

    async fn batch_create_organizations(
        &self,
        request: Request<BatchCreateOrganizationsRequest>,
    ) -> Result<Response<BatchCreateOrganizationsResponse>, Status> {
        let req = request.into_inner();
        let request_id = req.context.as_ref().map(|c| c.request_id.clone());

        if req.organizations.len() > 100 {
            return Err(Status::invalid_argument("Maximum batch size is 100"));
        }

        info!(count = req.organizations.len(), "Batch creating organizations");

        let mut results = Vec::new();
        let mut successful_count = 0;
        let mut failed_count = 0;

        for (idx, org) in req.organizations.iter().enumerate() {
            match db::create_organization(self.db.pool(), org).await {
                Ok(org_id) => {
                    // Fetch the created org to return
                    let created_org = db::get_organization(self.db.pool(), &org_id).await.ok().flatten();
                    results.push(batch_create_organizations_response::Result {
                        success: true,
                        organization: created_org.map(|o| o.to_proto()),
                        error: None,
                        index: idx as i32,
                    });
                    successful_count += 1;
                }
                Err(e) => {
                    results.push(batch_create_organizations_response::Result {
                        success: false,
                        organization: None,
                        error: Some(ErrorDetail {
                            code: 0,
                            message: e.to_string(),
                            retry_status: 0,
                            retry_after_seconds: 0,
                            metadata: Default::default(),
                            cause: None,
                        }),
                        index: idx as i32,
                    });
                    failed_count += 1;
                }
            }
        }

        Ok(Response::new(BatchCreateOrganizationsResponse {
            results,
            successful_count,
            failed_count,
            batch_id: uuid::Uuid::new_v4().to_string(),
            context: Some(self.response_context(request_id, None)),
        }))
    }

    async fn create_org_user(
        &self,
        request: Request<CreateOrgUserRequest>,
    ) -> Result<Response<AdminResponse>, Status> {
        let req = request.into_inner();
        let request_id = req.context.as_ref().map(|c| c.request_id.clone());

        let user = req.user.ok_or_else(|| Status::invalid_argument("user required"))?;

        info!(email = %user.email, org_id = %user.org_id, "Creating user");

        let user_id = db::create_user(self.db.pool(), &user)
            .await
            .map_err(|e| Status::internal(e.to_string()))?;

        // Create audit entry
        let _ = db::create_audit_entry(
            self.db.pool(),
            AuditActionType::AuditUserCreated,
            None, // Requester user ID not in proto
            Some(&user.org_id),
            None,
            Some("user"),
            Some(&user_id),
            &format!("User created: {}", user.email),
            Some(serde_json::json!({
                "user_email": user.email,
                "user_name": user.name,
                "role": user.role,
            })),
        )
        .await;

        Ok(Response::new(self.admin_response(
            true,
            &format!("User created with ID: {}", user_id),
            request_id,
        )))
    }

    async fn get_org_user(
        &self,
        request: Request<GetOrgUserRequest>,
    ) -> Result<Response<GetOrgUserResponse>, Status> {
        let req = request.into_inner();
        let request_id = req.context.as_ref().map(|c| c.request_id.clone());

        let user = db::get_user(self.db.pool(), &req.user_id)
            .await
            .map_err(|e| Status::internal(e.to_string()))?;

        let found = user.is_some();
        Ok(Response::new(GetOrgUserResponse {
            user: user.map(|u| u.to_proto()),
            found,
            error: None,
            context: Some(self.response_context(request_id, None)),
        }))
    }

    async fn get_org_user_by_email(
        &self,
        request: Request<GetOrgUserByEmailRequest>,
    ) -> Result<Response<GetOrgUserResponse>, Status> {
        let req = request.into_inner();
        let request_id = req.context.as_ref().map(|c| c.request_id.clone());

        let user = db::get_user_by_email(self.db.pool(), &req.email)
            .await
            .map_err(|e| Status::internal(e.to_string()))?;

        let found = user.is_some();
        Ok(Response::new(GetOrgUserResponse {
            user: user.map(|u| u.to_proto()),
            found,
            error: None,
            context: Some(self.response_context(request_id, None)),
        }))
    }

    async fn update_org_user(
        &self,
        request: Request<UpdateOrgUserRequest>,
    ) -> Result<Response<AdminResponse>, Status> {
        let req = request.into_inner();
        let request_id = req.context.as_ref().map(|c| c.request_id.clone());

        let user = req.user.ok_or_else(|| Status::invalid_argument("user required"))?;

        info!(user_id = %user.user_id, "Updating user");

        let updated = db::update_user(self.db.pool(), &user.user_id, &user)
            .await
            .map_err(|e| Status::internal(e.to_string()))?;

        if updated {
            // Create audit entry
            let _ = db::create_audit_entry(
                self.db.pool(),
                AuditActionType::AuditUserUpdated,
                None, // Requester user ID not in proto
                Some(&user.org_id),
                None,
                Some("user"),
                Some(&user.user_id),
                &format!("User updated: {}", user.user_id),
                Some(serde_json::json!({
                    "user_email": user.email,
                    "user_name": user.name,
                    "role": user.role,
                    "active": user.active,
                })),
            )
            .await;

            Ok(Response::new(self.admin_response(
                true,
                "User updated successfully",
                request_id,
            )))
        } else {
            Ok(Response::new(self.admin_response(
                false,
                "User not found",
                request_id,
            )))
        }
    }

    async fn list_org_users(
        &self,
        request: Request<ListOrgUsersRequest>,
    ) -> Result<Response<ListOrgUsersResponse>, Status> {
        let req = request.into_inner();
        let request_id = req.context.as_ref().map(|c| c.request_id.clone());

        let page_size = if req.page_size == 0 { 50 } else { req.page_size };
        let offset: i32 = req.page_token.parse().unwrap_or(0);

        let (users, total) = db::list_org_users(
            self.db.pool(),
            &req.org_id,
            page_size,
            offset,
            req.include_inactive,
        )
        .await
        .map_err(|e| Status::internal(e.to_string()))?;

        let next_offset = offset + users.len() as i32;
        let next_page_token = if next_offset < total {
            next_offset.to_string()
        } else {
            String::new()
        };

        Ok(Response::new(ListOrgUsersResponse {
            users: users.iter().map(|u| u.to_proto()).collect(),
            next_page_token,
            total_count: total,
            context: Some(self.response_context(request_id, None)),
        }))
    }

    async fn batch_create_org_users(
        &self,
        request: Request<BatchCreateOrgUsersRequest>,
    ) -> Result<Response<BatchCreateOrgUsersResponse>, Status> {
        let req = request.into_inner();
        let request_id = req.context.as_ref().map(|c| c.request_id.clone());

        if req.users.len() > 100 {
            return Err(Status::invalid_argument("Maximum batch size is 100"));
        }

        info!(org_id = %req.org_id, count = req.users.len(), "Batch creating users");

        let mut results = Vec::new();
        let mut successful_count = 0;
        let mut failed_count = 0;

        for (idx, user) in req.users.iter().enumerate() {
            // Ensure user is assigned to the correct org
            let mut user_with_org = user.clone();
            if user_with_org.org_id.is_empty() {
                user_with_org.org_id = req.org_id.clone();
            }

            match db::create_user(self.db.pool(), &user_with_org).await {
                Ok(user_id) => {
                    // Fetch the created user to return
                    let created_user = db::get_user(self.db.pool(), &user_id).await.ok().flatten();
                    results.push(batch_create_org_users_response::Result {
                        success: true,
                        user: created_user.map(|u| u.to_proto()),
                        error: None,
                        index: idx as i32,
                    });
                    successful_count += 1;
                }
                Err(e) => {
                    results.push(batch_create_org_users_response::Result {
                        success: false,
                        user: None,
                        error: Some(ErrorDetail {
                            code: 0,
                            message: e.to_string(),
                            retry_status: 0,
                            retry_after_seconds: 0,
                            metadata: Default::default(),
                            cause: None,
                        }),
                        index: idx as i32,
                    });
                    failed_count += 1;
                }
            }
        }

        Ok(Response::new(BatchCreateOrgUsersResponse {
            results,
            successful_count,
            failed_count,
            batch_id: uuid::Uuid::new_v4().to_string(),
            context: Some(self.response_context(request_id, None)),
        }))
    }

    async fn generate_key_pair(
        &self,
        request: Request<GenerateKeyPairRequest>,
    ) -> Result<Response<GenerateKeyPairResponse>, Status> {
        let req = request.into_inner();
        let request_id = req.context.as_ref().map(|c| c.request_id.clone());

        info!(org_id = %req.org_id, "Generating key pair");

        // Generate ephemeral keys for this organization
        let key_pair = HybridCrypto::generate_ephemeral()
            .map_err(|e| Status::internal(e.to_string()))?;

        let ed25519_pubkey = key_pair.ed25519_public_key();
        let mldsa_pubkey = key_pair.mldsa_public_key();
        let ed25519_fp = HybridCrypto::fingerprint(&ed25519_pubkey);
        let mldsa_fp = HybridCrypto::fingerprint(&mldsa_pubkey);

        let key_id = db::create_key(
            self.db.pool(),
            &req.org_id,
            &ed25519_pubkey,
            &mldsa_pubkey,
            &ed25519_fp,
            &mldsa_fp,
            KeyCustodyModel::Custodied as i32,
            &req.requester_user_id,
        )
        .await
        .map_err(|e| Status::internal(e.to_string()))?;

        // Create audit entry for key generation
        let _ = db::create_audit_entry(
            self.db.pool(),
            AuditActionType::AuditKeyGenerated,
            Some(&req.requester_user_id),
            Some(&req.org_id),
            None,
            Some("key"),
            Some(&key_id),
            &format!("Key pair generated: {}", key_id),
            Some(serde_json::json!({
                "key_id": key_id,
                "ed25519_fingerprint": ed25519_fp,
                "activate_immediately": req.activate_immediately,
            })),
        )
        .await;

        // Activate immediately if requested
        if req.activate_immediately {
            db::activate_key(self.db.pool(), &key_id)
                .await
                .map_err(|e| Status::internal(e.to_string()))?;

            // Create audit entry for key activation
            let _ = db::create_audit_entry(
                self.db.pool(),
                AuditActionType::AuditKeyActivated,
                Some(&req.requester_user_id),
                Some(&req.org_id),
                None,
                Some("key"),
                Some(&key_id),
                &format!("Key activated immediately: {}", key_id),
                Some(serde_json::json!({
                    "key_id": key_id,
                })),
            )
            .await;
        }

        let key = db::get_key(self.db.pool(), &key_id)
            .await
            .map_err(|e| Status::internal(e.to_string()))?
            .ok_or_else(|| Status::internal("Key not found after creation"))?;

        Ok(Response::new(GenerateKeyPairResponse {
            key_record: Some(key.to_proto()),
            context: Some(self.response_context(request_id, None)),
        }))
    }

    async fn list_keys(
        &self,
        request: Request<ListKeysRequest>,
    ) -> Result<Response<ListKeysResponse>, Status> {
        let req = request.into_inner();
        let request_id = req.context.as_ref().map(|c| c.request_id.clone());

        let keys = db::list_keys(self.db.pool(), &req.org_id, req.include_revoked)
            .await
            .map_err(|e| Status::internal(e.to_string()))?;

        Ok(Response::new(ListKeysResponse {
            keys: keys.iter().map(|k| k.to_proto()).collect(),
            next_page_token: String::new(),
            total_count: keys.len() as i32,
            context: Some(self.response_context(request_id, None)),
        }))
    }

    async fn activate_key(
        &self,
        request: Request<ActivateKeyRequest>,
    ) -> Result<Response<AdminResponse>, Status> {
        let req = request.into_inner();
        let request_id = req.context.as_ref().map(|c| c.request_id.clone());

        info!(key_id = %req.key_id, "Activating key");

        // Fetch the key to get org_id for audit logging
        let key = db::get_key(self.db.pool(), &req.key_id)
            .await
            .map_err(|e| Status::internal(e.to_string()))?;

        let activated = db::activate_key(self.db.pool(), &req.key_id)
            .await
            .map_err(|e| Status::internal(e.to_string()))?;

        if activated {
            // Create audit entry
            let requester = if req.requester_user_id.is_empty() { None } else { Some(req.requester_user_id.as_str()) };
            let _ = db::create_audit_entry(
                self.db.pool(),
                AuditActionType::AuditKeyActivated,
                requester,
                key.as_ref().map(|k| k.org_id.as_str()),
                None,
                Some("key"),
                Some(&req.key_id),
                &format!("Key activated: {}", req.key_id),
                Some(serde_json::json!({
                    "key_id": req.key_id,
                })),
            )
            .await;

            Ok(Response::new(self.admin_response(true, "Key activated", request_id)))
        } else {
            Ok(Response::new(self.admin_response(false, "Key not found or already active", request_id)))
        }
    }

    async fn rotate_key(
        &self,
        request: Request<RotateKeyRequest>,
    ) -> Result<Response<RotateKeyResponse>, Status> {
        let req = request.into_inner();
        let request_id = req.context.as_ref().map(|c| c.request_id.clone());

        info!(org_id = %req.org_id, mode = ?req.mode, "Rotating key");

        // Get current active key
        let old_key = db::get_active_key(self.db.pool(), &req.org_id)
            .await
            .map_err(|e| Status::internal(e.to_string()))?
            .ok_or_else(|| Status::failed_precondition("No active key to rotate"))?;

        // Generate new key pair
        let key_pair = HybridCrypto::generate_ephemeral()
            .map_err(|e| Status::internal(e.to_string()))?;

        let ed25519_pubkey = key_pair.ed25519_public_key();
        let mldsa_pubkey = key_pair.mldsa_public_key();
        let ed25519_fp = HybridCrypto::fingerprint(&ed25519_pubkey);
        let mldsa_fp = HybridCrypto::fingerprint(&mldsa_pubkey);
        let new_key_id = uuid::Uuid::new_v4().to_string();

        // Determine if immediate activation based on rotation mode
        let immediate = req.mode != KeyRotationMode::RotationStaged as i32;
        let grace_period = if req.grace_period_hours > 0 {
            req.grace_period_hours
        } else {
            24 // Default 24 hour grace period
        };

        // Perform rotation
        db::rotate_key(
            self.db.pool(),
            &old_key.key_id,
            &new_key_id,
            &req.org_id,
            &ed25519_pubkey,
            &mldsa_pubkey,
            &ed25519_fp,
            &mldsa_fp,
            old_key.custody_model,
            &req.requester_user_id,
            grace_period,
            immediate,
        )
        .await
        .map_err(|e| Status::internal(e.to_string()))?;

        // Create audit entry
        let _ = db::create_audit_entry(
            self.db.pool(),
            AuditActionType::AuditKeyRotated,
            Some(&req.requester_user_id),
            Some(&req.org_id),
            None,
            Some("key"),
            Some(&old_key.key_id),
            &format!("Key rotated: {} -> {}", old_key.key_id, new_key_id),
            Some(serde_json::json!({
                "old_key_id": old_key.key_id,
                "new_key_id": new_key_id,
                "reason": req.reason,
                "mode": req.mode,
            })),
        )
        .await;

        // Fetch the new key
        let new_key = db::get_key(self.db.pool(), &new_key_id)
            .await
            .map_err(|e| Status::internal(e.to_string()))?
            .ok_or_else(|| Status::internal("New key not found after rotation"))?;

        // Fetch the old key (now rotated)
        let old_key_updated = db::get_key(self.db.pool(), &old_key.key_id)
            .await
            .map_err(|e| Status::internal(e.to_string()))?;

        // Calculate grace period expiration
        let grace_expires = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs() as i64
            + (grace_period as i64 * 3600);

        Ok(Response::new(RotateKeyResponse {
            new_key: Some(new_key.to_proto()),
            old_key: old_key_updated.map(|k| k.to_proto()),
            grace_period_expires_at: grace_expires,
            rotation_id: uuid::Uuid::new_v4().to_string(),
            context: Some(self.response_context(request_id, None)),
        }))
    }

    async fn revoke_key(
        &self,
        request: Request<RevokeKeyRequest>,
    ) -> Result<Response<AdminResponse>, Status> {
        let req = request.into_inner();
        let request_id = req.context.as_ref().map(|c| c.request_id.clone());

        info!(key_id = %req.key_id, org_id = %req.org_id, "Revoking key");

        // Verify key belongs to org
        let key = db::get_key(self.db.pool(), &req.key_id)
            .await
            .map_err(|e| Status::internal(e.to_string()))?
            .ok_or_else(|| Status::not_found("Key not found"))?;

        if key.org_id != req.org_id {
            return Err(Status::permission_denied("Key does not belong to organization"));
        }

        if key.status == KeyStatus::KeyRevoked as i32 {
            return Ok(Response::new(self.admin_response(
                false,
                "Key is already revoked",
                request_id,
            )));
        }

        // Revoke the key
        let revoked = db::revoke_key(
            self.db.pool(),
            &req.key_id,
            &req.reason,
            &req.requester_user_id,
        )
        .await
        .map_err(|e| Status::internal(e.to_string()))?;

        if revoked {
            // Create audit entry
            let _ = db::create_audit_entry(
                self.db.pool(),
                AuditActionType::AuditKeyRevoked,
                Some(&req.requester_user_id),
                Some(&req.org_id),
                None,
                Some("key"),
                Some(&req.key_id),
                &format!("Key revoked: {}", req.key_id),
                Some(serde_json::json!({
                    "key_id": req.key_id,
                    "reason": req.reason,
                })),
            )
            .await;

            Ok(Response::new(self.admin_response(
                true,
                "Key revoked successfully",
                request_id,
            )))
        } else {
            Ok(Response::new(self.admin_response(
                false,
                "Failed to revoke key",
                request_id,
            )))
        }
    }

    async fn request_key_escrow(
        &self,
        request: Request<RequestKeyEscrowRequest>,
    ) -> Result<Response<RequestKeyEscrowResponse>, Status> {
        let req = request.into_inner();
        let request_id = req.context.as_ref().map(|c| c.request_id.clone());

        info!(key_id = %req.key_id, org_id = %req.org_id, "Requesting key escrow");

        // Verify key exists and belongs to org
        let key = db::get_key(self.db.pool(), &req.key_id)
            .await
            .map_err(|e| Status::internal(e.to_string()))?
            .ok_or_else(|| Status::not_found("Key not found"))?;

        if key.org_id != req.org_id {
            return Err(Status::permission_denied("Key does not belong to organization"));
        }

        // Create escrow entry (use "steward" as default custodian)
        let escrow_id = db::create_escrow(
            self.db.pool(),
            &req.key_id,
            &req.org_id,
            req.escrow_type,
            "steward", // Default custodian
        )
        .await
        .map_err(|e| Status::internal(e.to_string()))?;

        // Get the created escrow
        let escrow = db::get_escrow(self.db.pool(), &escrow_id)
            .await
            .map_err(|e| Status::internal(e.to_string()))?
            .ok_or_else(|| Status::internal("Escrow not found after creation"))?;

        Ok(Response::new(RequestKeyEscrowResponse {
            escrow: Some(escrow.to_proto()),
            context: Some(self.response_context(request_id, None)),
        }))
    }

    async fn request_key_recovery(
        &self,
        request: Request<RequestKeyRecoveryRequest>,
    ) -> Result<Response<RequestKeyRecoveryResponse>, Status> {
        let req = request.into_inner();
        let request_id = req.context.as_ref().map(|c| c.request_id.clone());

        info!(escrow_id = %req.escrow_id, org_id = %req.org_id, "Requesting key recovery");

        // Get escrow and verify org ownership
        let escrow = db::get_escrow(self.db.pool(), &req.escrow_id)
            .await
            .map_err(|e| Status::internal(e.to_string()))?
            .ok_or_else(|| Status::not_found("Escrow not found"))?;

        if escrow.org_id != req.org_id {
            return Err(Status::permission_denied("Escrow does not belong to organization"));
        }

        // Update escrow status to "RECOVERY_REQUESTED"
        db::update_escrow_status(self.db.pool(), &req.escrow_id, "RECOVERY_REQUESTED")
            .await
            .map_err(|e| Status::internal(e.to_string()))?;

        // In a real implementation, this would trigger a recovery workflow
        // For now, just return success with status update

        Ok(Response::new(RequestKeyRecoveryResponse {
            recovery_request_id: uuid::Uuid::new_v4().to_string(),
            status: "PENDING_STEWARD_APPROVAL".to_string(),
            expires_at: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs() as i64 + 86400, // 24 hours from now
            context: Some(self.response_context(request_id, None)),
        }))
    }

    async fn list_key_escrows(
        &self,
        request: Request<ListKeyEscrowsRequest>,
    ) -> Result<Response<ListKeyEscrowsResponse>, Status> {
        let req = request.into_inner();
        let request_id = req.context.as_ref().map(|c| c.request_id.clone());

        let escrows = db::list_escrows(self.db.pool(), &req.org_id)
            .await
            .map_err(|e| Status::internal(e.to_string()))?;

        Ok(Response::new(ListKeyEscrowsResponse {
            escrows: escrows.iter().map(|e| e.to_proto()).collect(),
            context: Some(self.response_context(request_id, None)),
        }))
    }

    async fn request_signature(
        &self,
        request: Request<RequestSignatureRequest>,
    ) -> Result<Response<RequestSignatureResponse>, Status> {
        let req = request.into_inner();
        let request_id = req.context.as_ref().map(|c| c.request_id.clone());

        let sign_request = req.sign_request.ok_or_else(|| Status::invalid_argument("sign_request required"))?;

        // Get active key for org
        let key = db::get_active_key(self.db.pool(), &sign_request.org_id)
            .await
            .map_err(|e| Status::internal(e.to_string()))?
            .ok_or_else(|| Status::failed_precondition("No active key for organization"))?;

        // Sign using the registry's crypto provider (for custodied keys)
        let signature = self.crypto.sign(&sign_request.data)
            .map_err(|e| Status::internal(e.to_string()))?;

        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs() as i64;

        Ok(Response::new(RequestSignatureResponse {
            sign_response: Some(SignResponse {
                signature: Some(signature),
                key_id: key.key_id,
                signed_at: now,
            }),
            success: true,
            error: None,
            context: Some(self.response_context(request_id, None)),
        }))
    }

    async fn get_audit_log(
        &self,
        request: Request<GetAuditLogRequest>,
    ) -> Result<Response<GetAuditLogResponse>, Status> {
        let req = request.into_inner();
        let request_id = req.context.as_ref().map(|c| c.request_id.clone());

        let page_size = if req.page_size == 0 { 50 } else { req.page_size };
        let offset: i32 = req.page_token.parse().unwrap_or(0);

        let action_types: Vec<i32> = req.action_types.iter().map(|a| *a as i32).collect();

        let (entries, total) = db::get_audit_log(
            self.db.pool(),
            &req.org_id,
            req.start_time,
            req.end_time,
            &action_types,
            page_size,
            offset,
        )
        .await
        .map_err(|e| Status::internal(e.to_string()))?;

        let next_offset = offset + entries.len() as i32;
        let next_page_token = if next_offset < total {
            next_offset.to_string()
        } else {
            String::new()
        };

        Ok(Response::new(GetAuditLogResponse {
            entries: entries.iter().map(|e| e.to_proto()).collect(),
            next_page_token,
            total_count: total,
            context: Some(self.response_context(request_id, None)),
        }))
    }

    async fn export_audit_log(
        &self,
        request: Request<ExportAuditLogRequest>,
    ) -> Result<Response<ExportAuditLogResponse>, Status> {
        let req = request.into_inner();
        let request_id = req.context.as_ref().map(|c| c.request_id.clone());

        info!(org_id = %req.org_id, format = ?req.format, "Exporting audit log");

        let action_types: Vec<i32> = req.action_types.iter().map(|a| *a as i32).collect();

        // Fetch all matching entries
        let entries = db::export_audit_log(
            self.db.pool(),
            &req.org_id,
            req.start_time,
            req.end_time,
            &action_types,
            &req.actor_user_ids,
        )
        .await
        .map_err(|e| Status::internal(e.to_string()))?;

        // Convert to proto format
        let proto_entries: Vec<AuditEntry> = entries.iter().map(|e| e.to_proto()).collect();

        // Format based on requested export format
        let (data, content_type) = match req.format {
            f if f == AuditExportFormat::AuditExportJson as i32 => {
                let json = serde_json::to_vec_pretty(&serde_json::json!({
                    "org_id": req.org_id,
                    "exported_at": SystemTime::now()
                        .duration_since(UNIX_EPOCH)
                        .unwrap()
                        .as_secs(),
                    "entry_count": proto_entries.len(),
                    "entries": proto_entries.iter().map(|e| {
                        serde_json::json!({
                            "entry_id": e.entry_id,
                            "timestamp": e.timestamp,
                            "actor_user_id": e.actor_user_id,
                            "actor_org_id": e.actor_org_id,
                            "action": e.action,
                            "target_type": e.target_type,
                            "target_id": e.target_id,
                            "description": e.description,
                            "metadata": e.metadata,
                        })
                    }).collect::<Vec<_>>(),
                }))
                .map_err(|e| Status::internal(e.to_string()))?;
                (json, "application/json")
            }
            f if f == AuditExportFormat::AuditExportCsv as i32 => {
                let mut csv = String::from("entry_id,timestamp,actor_user_id,actor_org_id,action,target_type,target_id,description\n");
                for e in &proto_entries {
                    csv.push_str(&format!(
                        "{},{},{},{},{},{},{},{}\n",
                        e.entry_id,
                        e.timestamp,
                        e.actor_user_id,
                        e.actor_org_id,
                        e.action,
                        e.target_type,
                        e.target_id,
                        e.description.replace(',', ";").replace('\n', " ")
                    ));
                }
                (csv.into_bytes(), "text/csv")
            }
            f if f == AuditExportFormat::AuditExportJsonl as i32 => {
                let mut jsonl = Vec::new();
                for e in &proto_entries {
                    let line = serde_json::to_string(&serde_json::json!({
                        "entry_id": e.entry_id,
                        "timestamp": e.timestamp,
                        "actor_user_id": e.actor_user_id,
                        "actor_org_id": e.actor_org_id,
                        "action": e.action,
                        "target_type": e.target_type,
                        "target_id": e.target_id,
                        "description": e.description,
                        "metadata": e.metadata,
                    }))
                    .map_err(|e| Status::internal(e.to_string()))?;
                    jsonl.extend_from_slice(line.as_bytes());
                    jsonl.push(b'\n');
                }
                (jsonl, "application/x-ndjson")
            }
            f if f == AuditExportFormat::AuditExportSplunkHec as i32 => {
                // Splunk HEC format: each event wrapped in {"event": ...}
                let mut hec = Vec::new();
                for e in &proto_entries {
                    let event = serde_json::to_string(&serde_json::json!({
                        "time": e.timestamp,
                        "source": "ciris-registry",
                        "sourcetype": "audit",
                        "event": {
                            "entry_id": e.entry_id,
                            "actor_user_id": e.actor_user_id,
                            "actor_org_id": e.actor_org_id,
                            "action": e.action,
                            "target_type": e.target_type,
                            "target_id": e.target_id,
                            "description": e.description,
                            "metadata": e.metadata,
                        }
                    }))
                    .map_err(|e| Status::internal(e.to_string()))?;
                    hec.extend_from_slice(event.as_bytes());
                    hec.push(b'\n');
                }
                (hec, "application/json")
            }
            _ => {
                return Err(Status::invalid_argument("Unsupported export format"));
            }
        };

        // Compute SHA256 checksum
        use sha2::{Sha256, Digest};
        let mut hasher = Sha256::new();
        hasher.update(&data);
        let checksum = hex::encode(hasher.finalize());

        Ok(Response::new(ExportAuditLogResponse {
            download_url: String::new(), // Empty for inline data
            data: data.into(),
            media_type: content_type.to_string(),
            entries_count: proto_entries.len() as i64,
            export_id: uuid::Uuid::new_v4().to_string(),
            sha256_checksum: checksum,
            export_signature: None,
            context: Some(self.response_context(request_id, None)),
        }))
    }

    async fn create_audit_entry(
        &self,
        request: Request<CreateAuditEntryRequest>,
    ) -> Result<Response<CreateAuditEntryResponse>, Status> {
        let req = request.into_inner();
        let request_id = req.context.as_ref().map(|c| c.request_id.clone());

        info!(
            action = ?req.action,
            actor_user_id = %req.actor_user_id,
            actor_org_id = %req.actor_org_id,
            "Creating audit entry from Portal"
        );

        // Convert action enum to AuditActionType
        let action = AuditActionType::try_from(req.action)
            .unwrap_or(AuditActionType::AuditActionUnspecified);

        // Convert metadata map to JSON
        let metadata = if req.metadata.is_empty() {
            None
        } else {
            Some(serde_json::json!(req.metadata))
        };

        // Create the audit entry
        let entry_id = db::create_audit_entry(
            self.db.pool(),
            action,
            if req.actor_user_id.is_empty() { None } else { Some(&req.actor_user_id) },
            if req.actor_org_id.is_empty() { None } else { Some(&req.actor_org_id) },
            if req.actor_ip_address.is_empty() { None } else { Some(&req.actor_ip_address) },
            if req.target_type.is_empty() { None } else { Some(&req.target_type) },
            if req.target_id.is_empty() { None } else { Some(&req.target_id) },
            &req.description,
            metadata,
        )
        .await
        .map_err(|e| Status::internal(e.to_string()))?;

        Ok(Response::new(CreateAuditEntryResponse {
            success: true,
            entry_id,
            message: "Audit entry created".to_string(),
            error: None,
            context: Some(self.response_context(request_id, None)),
        }))
    }

    async fn generate_compliance_report(
        &self,
        request: Request<GenerateComplianceReportRequest>,
    ) -> Result<Response<ComplianceReport>, Status> {
        let req = request.into_inner();
        let request_id = req.context.as_ref().map(|c| c.request_id.clone());

        info!(org_id = %req.org_id, framework = ?req.framework, "Generating compliance report");

        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs() as i64;

        // Get key statistics
        let keys = db::list_keys(self.db.pool(), &req.org_id, true)
            .await
            .map_err(|e| Status::internal(e.to_string()))?;

        let keys_generated = keys.len() as i32;
        let keys_rotated = keys.iter().filter(|k| k.rotated_at.is_some()).count() as i32;
        let keys_revoked = keys.iter().filter(|k| k.status == KeyStatus::KeyRevoked as i32).count() as i32;

        // Calculate oldest active key age
        let oldest_active_key_age_days = keys
            .iter()
            .filter(|k| k.status == KeyStatus::KeyActive as i32)
            .filter_map(|k| {
                let age_secs = now - k.created_at.unix_timestamp();
                Some(age_secs / 86400) // Convert to days
            })
            .max()
            .unwrap_or(0);

        // Key rotation policy: compliant if no active key is older than 365 days
        let rotation_policy_compliant = oldest_active_key_age_days <= 365;

        // Get user statistics
        let (users, _) = db::list_org_users(self.db.pool(), &req.org_id, 1000, 0, true)
            .await
            .map_err(|e| Status::internal(e.to_string()))?;

        let total_users = users.len() as i32;
        let admin_users = users.iter().filter(|u| u.role >= 100).count() as i32; // Assuming role >= 100 is admin
        let mfa_enabled_users = users.iter().filter(|u| u.mfa_enabled).count() as i32;

        // Get audit statistics
        let action_types: Vec<i32> = vec![];
        let (audit_entries, total_events) = db::get_audit_log(
            self.db.pool(),
            &req.org_id,
            req.start_time,
            req.end_time,
            &action_types,
            1000,
            0,
        )
        .await
        .map_err(|e| Status::internal(e.to_string()))?;

        let earliest_event = audit_entries.iter().map(|e| e.timestamp.unix_timestamp()).min().unwrap_or(0);
        let latest_event = audit_entries.iter().map(|e| e.timestamp.unix_timestamp()).max().unwrap_or(0);
        let audit_trail_continuous = total_events > 0;

        let report_id = uuid::Uuid::new_v4().to_string();

        // Generate attestation statement based on framework
        let attestation_statement = match req.framework {
            f if f == ComplianceFramework::ComplianceSoc2 as i32 => {
                format!(
                    "SOC 2 Compliance Report for organization {}. Period: {} to {}. \
                     Key management: {} keys generated, {} rotated, {} revoked. \
                     Rotation policy compliance: {}. \
                     Access control: {} total users, {} with MFA enabled.",
                    req.org_id,
                    req.start_time,
                    req.end_time,
                    keys_generated,
                    keys_rotated,
                    keys_revoked,
                    if rotation_policy_compliant { "COMPLIANT" } else { "NON-COMPLIANT" },
                    total_users,
                    mfa_enabled_users
                )
            }
            f if f == ComplianceFramework::ComplianceIso27001 as i32 => {
                format!(
                    "ISO 27001 Compliance Report for organization {}. \
                     Information security management assessment for period {} to {}.",
                    req.org_id, req.start_time, req.end_time
                )
            }
            f if f == ComplianceFramework::ComplianceHipaa as i32 => {
                format!(
                    "HIPAA Compliance Report for organization {}. \
                     Protected health information (PHI) security assessment for period {} to {}.",
                    req.org_id, req.start_time, req.end_time
                )
            }
            _ => format!(
                "Compliance Report for organization {}. Period: {} to {}.",
                req.org_id, req.start_time, req.end_time
            ),
        };

        // Sign the report
        let report_data = format!("{}:{}:{}:{}", report_id, req.org_id, req.start_time, req.end_time);
        let report_signature = self.crypto.sign(report_data.as_bytes())
            .map_err(|e| Status::internal(e.to_string()))?;

        Ok(Response::new(ComplianceReport {
            report_id,
            framework: req.framework,
            org_id: req.org_id,
            period_start: req.start_time,
            period_end: req.end_time,
            key_management: Some(compliance_report::KeyManagementSummary {
                keys_generated,
                keys_rotated,
                keys_revoked,
                oldest_active_key_age_days,
                rotation_policy_compliant,
            }),
            access_control: Some(compliance_report::AccessControlSummary {
                total_users,
                admin_users,
                mfa_enabled_users,
                failed_login_attempts: 0, // Would need to track this
            }),
            audit: Some(compliance_report::AuditSummary {
                total_events: total_events as i64,
                audit_trail_continuous,
                earliest_event,
                latest_event,
            }),
            attestation_statement,
            report_signature: Some(report_signature),
            generated_at: now,
            context: Some(self.response_context(request_id, None)),
        }))
    }

    // =============================================================================
    // Multi-Org User Management (v1.2.0)
    // =============================================================================

    async fn create_user(
        &self,
        request: Request<CreateUserRequest>,
    ) -> Result<Response<CreateUserResponse>, Status> {
        let req = request.into_inner();
        let request_id = req.context.as_ref().map(|c| c.request_id.clone());

        let user = req.user.ok_or_else(|| Status::invalid_argument("user required"))?;

        info!(email = %user.email, "Creating multi-org user");

        let user_id = db::create_multiorg_user(self.db.pool(), &user)
            .await
            .map_err(|e| Status::internal(e.to_string()))?;

        // Fetch the created user with memberships
        let created_user = db::get_multiorg_user(self.db.pool(), &user_id)
            .await
            .map_err(|e| Status::internal(e.to_string()))?;

        let memberships = db::get_user_memberships(self.db.pool(), &user_id)
            .await
            .map_err(|e| Status::internal(e.to_string()))?;

        Ok(Response::new(CreateUserResponse {
            success: true,
            message: "User created".to_string(),
            user_id: user_id.clone(),
            user: created_user.map(|u| u.to_proto(memberships.iter().map(|m| m.to_proto()).collect())),
            error: None,
            context: Some(self.response_context(request_id, None)),
        }))
    }

    async fn create_user_with_membership(
        &self,
        request: Request<CreateUserWithMembershipRequest>,
    ) -> Result<Response<CreateUserWithMembershipResponse>, Status> {
        let req = request.into_inner();
        let request_id = req.context.as_ref().map(|c| c.request_id.clone());

        let user = req.user.ok_or_else(|| Status::invalid_argument("user required"))?;

        info!(email = %user.email, org_id = %req.org_id, role = req.role, "Creating user with membership");

        let user_id = db::create_user_with_membership(
            self.db.pool(),
            &user,
            &req.org_id,
            req.role,
            None, // invited_by not in proto
        )
        .await
        .map_err(|e| Status::internal(e.to_string()))?;

        // Create audit entry
        let _ = db::create_audit_entry(
            self.db.pool(),
            AuditActionType::AuditUserCreated,
            None,
            Some(&req.org_id),
            None,
            Some("user"),
            Some(&user_id),
            &format!("User created with membership: {}", user.email),
            Some(serde_json::json!({
                "user_email": user.email,
                "user_name": user.name,
                "org_id": req.org_id,
                "role": req.role,
            })),
        )
        .await;

        // Fetch the created user with memberships
        let created_user = db::get_multiorg_user(self.db.pool(), &user_id)
            .await
            .map_err(|e| Status::internal(e.to_string()))?;

        let memberships = db::get_user_memberships(self.db.pool(), &user_id)
            .await
            .map_err(|e| Status::internal(e.to_string()))?;

        Ok(Response::new(CreateUserWithMembershipResponse {
            success: true,
            message: "User created with membership".to_string(),
            user_id: user_id.clone(),
            user: created_user.map(|u| u.to_proto(memberships.iter().map(|m| m.to_proto()).collect())),
            error: None,
            context: Some(self.response_context(request_id, None)),
        }))
    }

    async fn get_user(
        &self,
        request: Request<GetUserRequest>,
    ) -> Result<Response<GetUserResponse>, Status> {
        let req = request.into_inner();
        let request_id = req.context.as_ref().map(|c| c.request_id.clone());

        let user = db::get_multiorg_user(self.db.pool(), &req.user_id)
            .await
            .map_err(|e| Status::internal(e.to_string()))?;

        let found = user.is_some();

        let user_proto = if let Some(u) = user {
            let memberships = db::get_user_memberships(self.db.pool(), &req.user_id)
                .await
                .map_err(|e| Status::internal(e.to_string()))?;
            Some(u.to_proto(memberships.iter().map(|m| m.to_proto()).collect()))
        } else {
            None
        };

        Ok(Response::new(GetUserResponse {
            user: user_proto,
            found,
            error: None,
            context: Some(self.response_context(request_id, None)),
        }))
    }

    async fn get_user_by_email(
        &self,
        request: Request<GetUserByEmailRequest>,
    ) -> Result<Response<GetUserResponse>, Status> {
        let req = request.into_inner();
        let request_id = req.context.as_ref().map(|c| c.request_id.clone());

        let user = db::get_multiorg_user_by_email(self.db.pool(), &req.email)
            .await
            .map_err(|e| Status::internal(e.to_string()))?;

        let found = user.is_some();

        let user_proto = if let Some(u) = user {
            let memberships = db::get_user_memberships(self.db.pool(), &u.user_id)
                .await
                .map_err(|e| Status::internal(e.to_string()))?;
            Some(u.to_proto(memberships.iter().map(|m| m.to_proto()).collect()))
        } else {
            None
        };

        Ok(Response::new(GetUserResponse {
            user: user_proto,
            found,
            error: None,
            context: Some(self.response_context(request_id, None)),
        }))
    }

    async fn add_user_to_org(
        &self,
        request: Request<AddUserToOrgRequest>,
    ) -> Result<Response<AdminResponse>, Status> {
        let req = request.into_inner();
        let request_id = req.context.as_ref().map(|c| c.request_id.clone());

        info!(user_id = %req.user_id, org_id = %req.org_id, role = req.role, "Adding user to org");

        db::add_user_to_org(
            self.db.pool(),
            &req.user_id,
            &req.org_id,
            req.role,
            if req.invited_by.is_empty() { None } else { Some(&req.invited_by) },
        )
        .await
        .map_err(|e| Status::internal(e.to_string()))?;

        Ok(Response::new(self.admin_response(
            true,
            "User added to organization",
            request_id,
        )))
    }

    async fn remove_user_from_org(
        &self,
        request: Request<RemoveUserFromOrgRequest>,
    ) -> Result<Response<AdminResponse>, Status> {
        let req = request.into_inner();
        let request_id = req.context.as_ref().map(|c| c.request_id.clone());

        info!(user_id = %req.user_id, org_id = %req.org_id, "Removing user from org");

        let removed = db::remove_user_from_org(self.db.pool(), &req.user_id, &req.org_id)
            .await
            .map_err(|e| Status::internal(e.to_string()))?;

        if removed {
            Ok(Response::new(self.admin_response(
                true,
                "User removed from organization",
                request_id,
            )))
        } else {
            Ok(Response::new(self.admin_response(
                false,
                "User not found in organization",
                request_id,
            )))
        }
    }

    async fn update_user_org_role(
        &self,
        request: Request<UpdateUserOrgRoleRequest>,
    ) -> Result<Response<AdminResponse>, Status> {
        let req = request.into_inner();
        let request_id = req.context.as_ref().map(|c| c.request_id.clone());

        info!(user_id = %req.user_id, org_id = %req.org_id, new_role = req.new_role, "Updating user org role");

        let updated = db::update_user_org_role(self.db.pool(), &req.user_id, &req.org_id, req.new_role)
            .await
            .map_err(|e| Status::internal(e.to_string()))?;

        if updated {
            Ok(Response::new(self.admin_response(
                true,
                "User role updated",
                request_id,
            )))
        } else {
            Ok(Response::new(self.admin_response(
                false,
                "User membership not found",
                request_id,
            )))
        }
    }

    async fn list_org_members(
        &self,
        request: Request<ListOrgMembersRequest>,
    ) -> Result<Response<ListOrgMembersResponse>, Status> {
        let req = request.into_inner();
        let request_id = req.context.as_ref().map(|c| c.request_id.clone());

        let page_size = if req.page_size == 0 { 50 } else { req.page_size };
        let offset: i32 = req.page_token.parse().unwrap_or(0);

        let (members, total) = db::list_org_members(
            self.db.pool(),
            &req.org_id,
            page_size,
            offset,
            req.include_inactive,
        )
        .await
        .map_err(|e| Status::internal(e.to_string()))?;

        let next_offset = offset + members.len() as i32;
        let next_page_token = if next_offset < total {
            next_offset.to_string()
        } else {
            String::new()
        };

        // Convert to proto User with their memberships
        let users: Vec<User> = members
            .into_iter()
            .map(|(user, membership)| user.to_proto(vec![membership.to_proto()]))
            .collect();

        Ok(Response::new(ListOrgMembersResponse {
            users,
            next_page_token,
            total_count: total,
            context: Some(self.response_context(request_id, None)),
        }))
    }

    // =============================================================================
    // System User Management (v1.2.0)
    // =============================================================================

    async fn create_system_user(
        &self,
        request: Request<CreateSystemUserRequest>,
    ) -> Result<Response<CreateSystemUserResponse>, Status> {
        let req = request.into_inner();
        let request_id = req.context.as_ref().map(|c| c.request_id.clone());

        let user = req.user.ok_or_else(|| Status::invalid_argument("user required"))?;

        // SYSTEM_ADMIN (role=1) requires @ciris.ai email
        if user.role == 1 && !user.email.ends_with("@ciris.ai") {
            return Err(Status::invalid_argument("SYSTEM_ADMIN role requires @ciris.ai email"));
        }

        info!(email = %user.email, role = user.role, "Creating system user");

        let user_id = db::create_system_user(
            self.db.pool(),
            &user,
            None, // created_by not in proto
        )
        .await
        .map_err(|e| Status::internal(e.to_string()))?;

        // Create audit entry for system user creation
        let _ = db::create_audit_entry(
            self.db.pool(),
            AuditActionType::AuditUserCreated,
            None,
            None, // System users are org-independent
            None,
            Some("system_user"),
            Some(&user_id),
            &format!("System user created: {}", user.email),
            Some(serde_json::json!({
                "user_email": user.email,
                "user_name": user.name,
                "system_role": user.role,
                "is_system_user": true,
            })),
        )
        .await;

        let created_user = db::get_system_user(self.db.pool(), &user_id)
            .await
            .map_err(|e| Status::internal(e.to_string()))?;

        Ok(Response::new(CreateSystemUserResponse {
            success: true,
            message: "System user created".to_string(),
            user_id: user_id.clone(),
            user: created_user.map(|u| u.to_proto()),
            error: None,
            context: Some(self.response_context(request_id, None)),
        }))
    }

    async fn get_system_user(
        &self,
        request: Request<GetSystemUserRequest>,
    ) -> Result<Response<GetSystemUserResponse>, Status> {
        let req = request.into_inner();
        let request_id = req.context.as_ref().map(|c| c.request_id.clone());

        let user = db::get_system_user(self.db.pool(), &req.user_id)
            .await
            .map_err(|e| Status::internal(e.to_string()))?;

        let found = user.is_some();

        Ok(Response::new(GetSystemUserResponse {
            user: user.map(|u| u.to_proto()),
            found,
            error: None,
            context: Some(self.response_context(request_id, None)),
        }))
    }

    async fn list_system_users(
        &self,
        request: Request<ListSystemUsersRequest>,
    ) -> Result<Response<ListSystemUsersResponse>, Status> {
        let req = request.into_inner();
        let request_id = req.context.as_ref().map(|c| c.request_id.clone());

        let page_size = if req.page_size == 0 { 50 } else { req.page_size };
        let offset: i32 = req.page_token.parse().unwrap_or(0);

        let (users, total) = db::list_system_users(
            self.db.pool(),
            page_size,
            offset,
            req.include_inactive,
        )
        .await
        .map_err(|e| Status::internal(e.to_string()))?;

        let next_offset = offset + users.len() as i32;
        let next_page_token = if next_offset < total {
            next_offset.to_string()
        } else {
            String::new()
        };

        Ok(Response::new(ListSystemUsersResponse {
            users: users.iter().map(|u| u.to_proto()).collect(),
            next_page_token,
            total_count: total,
            context: Some(self.response_context(request_id, None)),
        }))
    }

    async fn update_system_user(
        &self,
        request: Request<UpdateSystemUserRequest>,
    ) -> Result<Response<AdminResponse>, Status> {
        let req = request.into_inner();
        let request_id = req.context.as_ref().map(|c| c.request_id.clone());

        let user = req.user.ok_or_else(|| Status::invalid_argument("user required"))?;

        info!(user_id = %user.user_id, "Updating system user");

        let updated = db::update_system_user(self.db.pool(), &user.user_id, &user)
            .await
            .map_err(|e| Status::internal(e.to_string()))?;

        if updated {
            Ok(Response::new(self.admin_response(
                true,
                "System user updated",
                request_id,
            )))
        } else {
            Ok(Response::new(self.admin_response(
                false,
                "System user not found",
                request_id,
            )))
        }
    }

    // =============================================================================
    // Organization Hierarchy (v1.2.0)
    // =============================================================================

    async fn list_child_organizations(
        &self,
        request: Request<ListChildOrganizationsRequest>,
    ) -> Result<Response<ListOrganizationsResponse>, Status> {
        let req = request.into_inner();
        let request_id = req.context.as_ref().map(|c| c.request_id.clone());

        let page_size = if req.page_size == 0 { 50 } else { req.page_size };
        let offset: i32 = req.page_token.parse().unwrap_or(0);

        let (orgs, total) = db::list_child_organizations(
            self.db.pool(),
            &req.parent_org_id,
            page_size,
            offset,
            req.include_inactive,
        )
        .await
        .map_err(|e| Status::internal(e.to_string()))?;

        let next_offset = offset + orgs.len() as i32;
        let next_page_token = if next_offset < total {
            next_offset.to_string()
        } else {
            String::new()
        };

        Ok(Response::new(ListOrganizationsResponse {
            organizations: orgs.iter().map(|o| o.to_proto()).collect(),
            next_page_token,
            total_count: total,
            context: Some(self.response_context(request_id, None)),
        }))
    }

    async fn create_licensee_organization(
        &self,
        request: Request<CreateLicenseeOrganizationRequest>,
    ) -> Result<Response<CreateOrganizationResponse>, Status> {
        let req = request.into_inner();
        let request_id = req.context.as_ref().map(|c| c.request_id.clone());

        let mut org = req.organization.ok_or_else(|| Status::invalid_argument("organization required"))?;

        // Verify parent org exists and is a PARTNER type
        let parent = db::get_organization(self.db.pool(), &req.parent_org_id)
            .await
            .map_err(|e| Status::internal(e.to_string()))?
            .ok_or_else(|| Status::not_found("Parent organization not found"))?;

        if parent.org_type != OrgType::OrgPartner as i32 {
            return Err(Status::invalid_argument("Only PARTNER organizations can create licensees"));
        }

        // Set licensee org fields
        org.org_type = OrgType::OrgLicensee as i32;
        org.parent_org_id = req.parent_org_id.clone();

        info!(name = %org.name, parent_org_id = %req.parent_org_id, "Creating licensee organization");

        let org_id = db::create_organization(self.db.pool(), &org)
            .await
            .map_err(|e| Status::internal(e.to_string()))?;

        let created_org = db::get_organization(self.db.pool(), &org_id)
            .await
            .map_err(|e| Status::internal(e.to_string()))?;

        Ok(Response::new(CreateOrganizationResponse {
            success: true,
            message: "Licensee organization created".to_string(),
            org_id: org_id.clone(),
            user_id: String::new(),
            organization: created_org.map(|o| o.to_proto()),
            admin_user: None,
            error: None,
            context: Some(self.response_context(request_id, None)),
        }))
    }

    async fn get_organization_hierarchy(
        &self,
        request: Request<GetOrganizationHierarchyRequest>,
    ) -> Result<Response<GetOrganizationHierarchyResponse>, Status> {
        let req = request.into_inner();
        let request_id = req.context.as_ref().map(|c| c.request_id.clone());

        // Get the requested org
        let org = db::get_organization(self.db.pool(), &req.org_id)
            .await
            .map_err(|e| Status::internal(e.to_string()))?
            .ok_or_else(|| Status::not_found("Organization not found"))?;

        // Build ancestors chain (walk up parent links)
        let mut ancestors = Vec::new();
        let mut current_parent_id = org.parent_org_id.clone();
        while let Some(parent_id) = current_parent_id {
            if let Some(parent) = db::get_organization(self.db.pool(), &parent_id)
                .await
                .map_err(|e| Status::internal(e.to_string()))?
            {
                current_parent_id = parent.parent_org_id.clone();
                ancestors.push(parent.to_proto());
            } else {
                break;
            }
        }

        // Get children
        let (children, _) = db::list_child_organizations(
            self.db.pool(),
            &req.org_id,
            100, // Max children to return
            0,
            false, // Only active
        )
        .await
        .map_err(|e| Status::internal(e.to_string()))?;

        Ok(Response::new(GetOrganizationHierarchyResponse {
            organization: Some(org.to_proto()),
            ancestors,
            children: children.iter().map(|c| c.to_proto()).collect(),
            context: Some(self.response_context(request_id, None)),
        }))
    }

    async fn upgrade_to_partner(
        &self,
        request: Request<UpgradeToPartnerRequest>,
    ) -> Result<Response<AdminResponse>, Status> {
        let req = request.into_inner();
        let request_id = req.context.as_ref().map(|c| c.request_id.clone());

        // Get the org to upgrade
        let org = db::get_organization(self.db.pool(), &req.org_id)
            .await
            .map_err(|e| Status::internal(e.to_string()))?
            .ok_or_else(|| Status::not_found("Organization not found"))?;

        // Must be COMMUNITY type to upgrade
        if org.org_type != OrgType::OrgCommunity as i32 {
            return Err(Status::invalid_argument("Only COMMUNITY organizations can be upgraded to PARTNER"));
        }

        info!(org_id = %req.org_id, "Upgrading organization to PARTNER");

        // Create updated org with PARTNER type
        let mut updated_org = org.to_proto();
        updated_org.org_type = OrgType::OrgPartner as i32;

        let updated = db::update_organization(self.db.pool(), &req.org_id, &updated_org)
            .await
            .map_err(|e| Status::internal(e.to_string()))?;

        if updated {
            // Create audit entry
            let _ = db::create_audit_entry(
                self.db.pool(),
                AuditActionType::AuditOrgCreated, // Could add AUDIT_ORG_UPGRADED
                None, // upgraded_by not in proto
                Some(&req.org_id),
                None,
                Some("organization"),
                Some(&req.org_id),
                &format!("Organization upgraded to PARTNER: {}", req.org_id),
                Some(serde_json::json!({
                    "partner_license_type": req.partner_license_type,
                })),
            )
            .await;

            Ok(Response::new(self.admin_response(
                true,
                "Organization upgraded to PARTNER",
                request_id,
            )))
        } else {
            Ok(Response::new(self.admin_response(
                false,
                "Failed to upgrade organization",
                request_id,
            )))
        }
    }
}
