//! PortalService implementation (CIRISPortal operations)

use std::sync::Arc;
use std::time::{Instant, SystemTime, UNIX_EPOCH};

use tonic::{Request, Response, Status};
use tracing::info;

use crate::config::Environment;
use crate::crypto::HybridCrypto;
use crate::db::{self, Database};
use crate::middleware::authz::{
    authorize_org_access, authorize_system_admin, claims_from_request, OrgRole,
};
use crate::proto::portal_service_server::PortalService as PortalServiceTrait;
use crate::proto::{
    AuditActionType, AuditEntry, AuditExportFormat, KeyCustodyModel, KeyRotationMode, KeyStatus,
    OAuthIdentity, OrgType, User, *,
};

pub struct PortalService {
    db: Database,
    crypto: Arc<HybridCrypto>,
    environment: i32,
}

impl PortalService {
    pub fn new(db: Database, crypto: Arc<HybridCrypto>, environment: Environment) -> Self {
        Self {
            db,
            crypto,
            environment: environment.to_proto_i32(),
        }
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
        let claims = claims_from_request(&request)?.clone();
        let req = request.into_inner();
        let request_id = req.context.as_ref().map(|c| c.request_id.clone());

        // W2: PortalService god-mode method (creates new orgs cross-tenant).
        // AuthLayer only enforces SYSTEM_ADMIN on RegistryAdminService;
        // PortalService needs explicit gating per Phase 6.
        authorize_system_admin(self.db.pool(), &claims).await?;

        let org = req
            .organization
            .ok_or_else(|| Status::invalid_argument("organization required"))?;

        // Check if initial_admin is provided - use transactional creation to avoid race condition
        if let Some(admin_user) = req.initial_admin {
            info!(name = %org.name, admin_email = %admin_user.email, "Creating organization with initial admin");

            let (org_id, user_id) =
                db::create_organization_with_admin(self.db.pool(), &org, &admin_user)
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
        let claims = claims_from_request(&request)?.clone();
        let req = request.into_inner();
        let request_id = req.context.as_ref().map(|c| c.request_id.clone());

        authorize_org_access(self.db.pool(), &claims, &req.org_id, OrgRole::Viewer).await?;

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
        let claims = claims_from_request(&request)?.clone();
        let req = request.into_inner();
        let request_id = req.context.as_ref().map(|c| c.request_id.clone());

        let org = req
            .organization
            .ok_or_else(|| Status::invalid_argument("organization required"))?;

        // W3: authz against the inner-proto org.org_id (the field used by the DB write).
        authorize_org_access(self.db.pool(), &claims, &org.org_id, OrgRole::OrgAdmin).await?;

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
        let claims = claims_from_request(&request)?.clone();
        let req = request.into_inner();
        let request_id = req.context.as_ref().map(|c| c.request_id.clone());

        // W2: god-mode (lists orgs cross-tenant).
        authorize_system_admin(self.db.pool(), &claims).await?;

        let page_size = if req.page_size == 0 {
            50
        } else {
            req.page_size
        };
        let offset: i32 = req.page_token.parse().unwrap_or(0);

        let (orgs, total) =
            db::list_organizations(self.db.pool(), page_size, offset, req.include_inactive)
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
        let claims = claims_from_request(&request)?.clone();
        let req = request.into_inner();
        let request_id = req.context.as_ref().map(|c| c.request_id.clone());

        // W2: god-mode (batch creates orgs cross-tenant).
        authorize_system_admin(self.db.pool(), &claims).await?;

        if req.organizations.len() > 100 {
            return Err(Status::invalid_argument("Maximum batch size is 100"));
        }

        info!(
            count = req.organizations.len(),
            "Batch creating organizations"
        );

        let mut results = Vec::new();
        let mut successful_count = 0;
        let mut failed_count = 0;

        for (idx, org) in req.organizations.iter().enumerate() {
            match db::create_organization(self.db.pool(), org).await {
                Ok(org_id) => {
                    // Fetch the created org to return
                    let created_org = db::get_organization(self.db.pool(), &org_id)
                        .await
                        .ok()
                        .flatten();
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
        let claims = claims_from_request(&request)?.clone();
        let req = request.into_inner();
        let request_id = req.context.as_ref().map(|c| c.request_id.clone());

        let user = req
            .user
            .ok_or_else(|| Status::invalid_argument("user required"))?;

        // W3: authz against the inner-proto user.org_id (the field used by the DB write).
        authorize_org_access(self.db.pool(), &claims, &user.org_id, OrgRole::OrgAdmin).await?;

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
        let claims = claims_from_request(&request)?.clone();
        let req = request.into_inner();
        let request_id = req.context.as_ref().map(|c| c.request_id.clone());

        let user = db::get_user(self.db.pool(), &req.user_id)
            .await
            .map_err(|e| Status::internal(e.to_string()))?;

        // Authz: derive target org from the user record itself. Self-lookup
        // (claims.sub == req.user_id) is allowed regardless of org membership;
        // otherwise the caller must have at least Viewer access in the
        // target user's org.
        if let Some(ref u) = user {
            if claims.sub != req.user_id {
                authorize_org_access(self.db.pool(), &claims, &u.org_id, OrgRole::Viewer).await?;
            }
        }

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
        let claims = claims_from_request(&request)?.clone();
        let req = request.into_inner();
        let request_id = req.context.as_ref().map(|c| c.request_id.clone());

        // Authz against req.org_id when provided. Empty org_id (legacy
        // backward-compat path) requires SYSTEM_ADMIN since it can return
        // any user across orgs.
        if req.org_id.is_empty() {
            authorize_system_admin(self.db.pool(), &claims).await?;
        } else {
            authorize_org_access(self.db.pool(), &claims, &req.org_id, OrgRole::Viewer).await?;
        }

        // Use org-specific lookup to avoid returning wrong org's user
        let user = if req.org_id.is_empty() {
            // Fallback for backward compatibility (returns arbitrary first match)
            db::get_user_by_email(self.db.pool(), &req.email).await
        } else {
            // Correct: filter by both org_id AND email
            db::get_org_user_by_email(self.db.pool(), &req.org_id, &req.email).await
        }
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
        let claims = claims_from_request(&request)?.clone();
        let req = request.into_inner();
        let request_id = req.context.as_ref().map(|c| c.request_id.clone());

        let user = req
            .user
            .ok_or_else(|| Status::invalid_argument("user required"))?;

        // W3: authz against the inner-proto user.org_id (the field used by the DB write).
        authorize_org_access(self.db.pool(), &claims, &user.org_id, OrgRole::OrgAdmin).await?;

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
        let claims = claims_from_request(&request)?.clone();
        let req = request.into_inner();
        let request_id = req.context.as_ref().map(|c| c.request_id.clone());

        authorize_org_access(self.db.pool(), &claims, &req.org_id, OrgRole::Viewer).await?;

        let page_size = if req.page_size == 0 {
            50
        } else {
            req.page_size
        };
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
        let claims = claims_from_request(&request)?.clone();
        let req = request.into_inner();
        let request_id = req.context.as_ref().map(|c| c.request_id.clone());

        if req.users.len() > 100 {
            return Err(Status::invalid_argument("Maximum batch size is 100"));
        }

        authorize_org_access(self.db.pool(), &claims, &req.org_id, OrgRole::OrgAdmin).await?;

        // Cascading authz: reject any per-user record whose explicit org_id
        // doesn't match the batch-level org_id. Empty per-user org_id is
        // allowed (filled in below); a non-empty mismatched value is a
        // fail-secure rejection of the whole batch.
        for (idx, u) in req.users.iter().enumerate() {
            if !u.org_id.is_empty() && u.org_id != req.org_id {
                return Err(Status::invalid_argument(format!(
                    "user[{}] org_id={:?} does not match batch org_id={:?}",
                    idx, u.org_id, req.org_id
                )));
            }
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
        let claims = claims_from_request(&request)?.clone();
        let req = request.into_inner();
        let request_id = req.context.as_ref().map(|c| c.request_id.clone());

        authorize_org_access(self.db.pool(), &claims, &req.org_id, OrgRole::KeyManager).await?;

        info!(org_id = %req.org_id, "Generating key pair");

        // Generate ephemeral keys for this organization
        let key_pair =
            HybridCrypto::generate_ephemeral().map_err(|e| Status::internal(e.to_string()))?;

        let ed25519_pubkey = key_pair.ed25519_public_key();
        let mldsa_pubkey = key_pair.mldsa_public_key();
        let ed25519_private = key_pair.ed25519_private_key_bytes();
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
            &claims.sub,
        )
        .await
        .map_err(|e| Status::internal(e.to_string()))?;

        // Create audit entry for key generation
        // W1: actor identity derived from JWT claims.sub, NOT from
        // req.requester_user_id (which is forgeable by any authenticated
        // caller). The proto field is kept for wire-format compat but
        // ignored on the server side.
        let _ = db::create_audit_entry(
            self.db.pool(),
            AuditActionType::AuditKeyGenerated,
            Some(claims.sub.as_str()),
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

            // Create audit entry for key activation (W1: actor from claims.sub).
            let _ = db::create_audit_entry(
                self.db.pool(),
                AuditActionType::AuditKeyActivated,
                Some(claims.sub.as_str()),
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
            ed25519_private_key: ed25519_private.into(),
            context: Some(self.response_context(request_id, None)),
        }))
    }

    async fn list_keys(
        &self,
        request: Request<ListKeysRequest>,
    ) -> Result<Response<ListKeysResponse>, Status> {
        let claims = claims_from_request(&request)?.clone();
        let req = request.into_inner();
        let request_id = req.context.as_ref().map(|c| c.request_id.clone());

        authorize_org_access(self.db.pool(), &claims, &req.org_id, OrgRole::Viewer).await?;

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
        let claims = claims_from_request(&request)?.clone();
        let req = request.into_inner();
        let request_id = req.context.as_ref().map(|c| c.request_id.clone());

        info!(key_id = %req.key_id, "Activating key");

        // Fetch the key first to derive its org for authz (no org_id in
        // the request body — it must be inferred from the key record).
        let key = db::get_key(self.db.pool(), &req.key_id)
            .await
            .map_err(|e| Status::internal(e.to_string()))?;
        if let Some(ref k) = key {
            authorize_org_access(self.db.pool(), &claims, &k.org_id, OrgRole::KeyManager).await?;
        }

        let activated = db::activate_key(self.db.pool(), &req.key_id)
            .await
            .map_err(|e| Status::internal(e.to_string()))?;

        if activated {
            // W1: actor from claims.sub, not from forgeable req.requester_user_id.
            let _ = db::create_audit_entry(
                self.db.pool(),
                AuditActionType::AuditKeyActivated,
                Some(claims.sub.as_str()),
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

            Ok(Response::new(self.admin_response(
                true,
                "Key activated",
                request_id,
            )))
        } else {
            Ok(Response::new(self.admin_response(
                false,
                "Key not found or already active",
                request_id,
            )))
        }
    }

    async fn rotate_key(
        &self,
        request: Request<RotateKeyRequest>,
    ) -> Result<Response<RotateKeyResponse>, Status> {
        let claims = claims_from_request(&request)?.clone();
        let req = request.into_inner();
        let request_id = req.context.as_ref().map(|c| c.request_id.clone());

        authorize_org_access(self.db.pool(), &claims, &req.org_id, OrgRole::KeyManager).await?;

        info!(org_id = %req.org_id, mode = ?req.mode, "Rotating key");

        // Get current active key
        let old_key = db::get_active_key(self.db.pool(), &req.org_id)
            .await
            .map_err(|e| Status::internal(e.to_string()))?
            .ok_or_else(|| Status::failed_precondition("No active key to rotate"))?;

        // Generate new key pair
        let key_pair =
            HybridCrypto::generate_ephemeral().map_err(|e| Status::internal(e.to_string()))?;

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
            &claims.sub,
            grace_period,
            immediate,
        )
        .await
        .map_err(|e| Status::internal(e.to_string()))?;

        // W1: actor from claims.sub, not from forgeable req.requester_user_id.
        let _ = db::create_audit_entry(
            self.db.pool(),
            AuditActionType::AuditKeyRotated,
            Some(claims.sub.as_str()),
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
        let claims = claims_from_request(&request)?.clone();
        let req = request.into_inner();
        let request_id = req.context.as_ref().map(|c| c.request_id.clone());

        authorize_org_access(self.db.pool(), &claims, &req.org_id, OrgRole::KeyManager).await?;

        info!(key_id = %req.key_id, org_id = %req.org_id, "Revoking key");

        // Verify key belongs to org
        let key = db::get_key(self.db.pool(), &req.key_id)
            .await
            .map_err(|e| Status::internal(e.to_string()))?
            .ok_or_else(|| Status::not_found("Key not found"))?;

        if key.org_id != req.org_id {
            return Err(Status::permission_denied(
                "Key does not belong to organization",
            ));
        }

        if key.status == KeyStatus::KeyRevoked as i32 {
            return Ok(Response::new(self.admin_response(
                false,
                "Key is already revoked",
                request_id,
            )));
        }

        // Revoke the key (W1: revoker identity from claims.sub).
        let revoked = db::revoke_key(
            self.db.pool(),
            &req.key_id,
            &req.reason,
            &claims.sub,
        )
        .await
        .map_err(|e| Status::internal(e.to_string()))?;

        if revoked {
            // W1: actor from claims.sub, not from forgeable req.requester_user_id.
            let _ = db::create_audit_entry(
                self.db.pool(),
                AuditActionType::AuditKeyRevoked,
                Some(claims.sub.as_str()),
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
        let claims = claims_from_request(&request)?.clone();
        let req = request.into_inner();
        let request_id = req.context.as_ref().map(|c| c.request_id.clone());

        authorize_org_access(self.db.pool(), &claims, &req.org_id, OrgRole::OrgAdmin).await?;

        info!(key_id = %req.key_id, org_id = %req.org_id, "Requesting key escrow");

        // Verify key exists and belongs to org
        let key = db::get_key(self.db.pool(), &req.key_id)
            .await
            .map_err(|e| Status::internal(e.to_string()))?
            .ok_or_else(|| Status::not_found("Key not found"))?;

        if key.org_id != req.org_id {
            return Err(Status::permission_denied(
                "Key does not belong to organization",
            ));
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
        let claims = claims_from_request(&request)?.clone();
        let req = request.into_inner();
        let request_id = req.context.as_ref().map(|c| c.request_id.clone());

        authorize_org_access(self.db.pool(), &claims, &req.org_id, OrgRole::OrgAdmin).await?;

        info!(escrow_id = %req.escrow_id, org_id = %req.org_id, "Requesting key recovery");

        // Get escrow and verify org ownership
        let escrow = db::get_escrow(self.db.pool(), &req.escrow_id)
            .await
            .map_err(|e| Status::internal(e.to_string()))?
            .ok_or_else(|| Status::not_found("Escrow not found"))?;

        if escrow.org_id != req.org_id {
            return Err(Status::permission_denied(
                "Escrow does not belong to organization",
            ));
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
                .as_secs() as i64
                + 86400, // 24 hours from now
            context: Some(self.response_context(request_id, None)),
        }))
    }

    async fn list_key_escrows(
        &self,
        request: Request<ListKeyEscrowsRequest>,
    ) -> Result<Response<ListKeyEscrowsResponse>, Status> {
        let claims = claims_from_request(&request)?.clone();
        let req = request.into_inner();
        let request_id = req.context.as_ref().map(|c| c.request_id.clone());

        authorize_org_access(self.db.pool(), &claims, &req.org_id, OrgRole::Viewer).await?;

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
        let claims = claims_from_request(&request)?.clone();
        let req = request.into_inner();
        let request_id = req.context.as_ref().map(|c| c.request_id.clone());

        let sign_request = req
            .sign_request
            .ok_or_else(|| Status::invalid_argument("sign_request required"))?;

        // W3: authz against the inner-proto sign_request.org_id (the field used
        // for the active-key lookup).
        authorize_org_access(self.db.pool(), &claims, &sign_request.org_id, OrgRole::Operator)
            .await?;

        // Get active key for org
        let key = db::get_active_key(self.db.pool(), &sign_request.org_id)
            .await
            .map_err(|e| Status::internal(e.to_string()))?
            .ok_or_else(|| Status::failed_precondition("No active key for organization"))?;

        // Sign using the registry's crypto provider (for custodied keys)
        let signature = self
            .crypto
            .sign(&sign_request.data)
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
        let claims = claims_from_request(&request)?.clone();
        let req = request.into_inner();
        let request_id = req.context.as_ref().map(|c| c.request_id.clone());

        authorize_org_access(self.db.pool(), &claims, &req.org_id, OrgRole::Viewer).await?;

        let page_size = if req.page_size == 0 {
            50
        } else {
            req.page_size
        };
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
        let claims = claims_from_request(&request)?.clone();
        let req = request.into_inner();
        let request_id = req.context.as_ref().map(|c| c.request_id.clone());

        authorize_org_access(self.db.pool(), &claims, &req.org_id, OrgRole::OrgAdmin).await?;

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
        use sha2::{Digest, Sha256};
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
        let claims = claims_from_request(&request)?.clone();
        let req = request.into_inner();
        let request_id = req.context.as_ref().map(|c| c.request_id.clone());

        // Authz against the actor org being recorded (or system-admin
        // override for cross-org Portal operations).
        if !req.actor_org_id.is_empty() {
            authorize_org_access(self.db.pool(), &claims, &req.actor_org_id, OrgRole::Operator)
                .await?;
        } else {
            authorize_system_admin(self.db.pool(), &claims).await?;
        }

        // W1: actor_user_id must be derivable. Two cases:
        //   - SYSTEM_ADMIN may log on behalf of another user (CIRISPortal use
        //     case: "user X logged in" where Portal-system is the caller).
        //     Trust whatever req.actor_user_id is supplied (or fall back to
        //     claims.sub if empty).
        //   - Non-admin caller MUST log as themselves. If req.actor_user_id
        //     is non-empty and != claims.sub, reject with PermissionDenied
        //     (forge attempt). Otherwise force claims.sub.
        let resolved_actor: String = if claims.role
            == crate::middleware::authz::ROLE_SYSTEM_ADMIN
        {
            if req.actor_user_id.is_empty() {
                claims.sub.clone()
            } else {
                req.actor_user_id.clone()
            }
        } else if req.actor_user_id.is_empty() || req.actor_user_id == claims.sub {
            claims.sub.clone()
        } else {
            return Err(Status::permission_denied(
                "actor_user_id must equal JWT subject (or caller must be SYSTEM_ADMIN)",
            ));
        };

        info!(
            action = ?req.action,
            actor_user_id = %resolved_actor,
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
            Some(resolved_actor.as_str()),
            if req.actor_org_id.is_empty() {
                None
            } else {
                Some(&req.actor_org_id)
            },
            if req.actor_ip_address.is_empty() {
                None
            } else {
                Some(&req.actor_ip_address)
            },
            if req.target_type.is_empty() {
                None
            } else {
                Some(&req.target_type)
            },
            if req.target_id.is_empty() {
                None
            } else {
                Some(&req.target_id)
            },
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
        let claims = claims_from_request(&request)?.clone();
        let req = request.into_inner();
        let request_id = req.context.as_ref().map(|c| c.request_id.clone());

        authorize_org_access(self.db.pool(), &claims, &req.org_id, OrgRole::OrgAdmin).await?;

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
        let keys_revoked = keys
            .iter()
            .filter(|k| k.status == KeyStatus::KeyRevoked as i32)
            .count() as i32;

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

        let earliest_event = audit_entries
            .iter()
            .map(|e| e.timestamp.unix_timestamp())
            .min()
            .unwrap_or(0);
        let latest_event = audit_entries
            .iter()
            .map(|e| e.timestamp.unix_timestamp())
            .max()
            .unwrap_or(0);
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
                    if rotation_policy_compliant {
                        "COMPLIANT"
                    } else {
                        "NON-COMPLIANT"
                    },
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
        let report_data = format!(
            "{}:{}:{}:{}",
            report_id, req.org_id, req.start_time, req.end_time
        );
        let report_signature = self
            .crypto
            .sign(report_data.as_bytes())
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
        let claims = claims_from_request(&request)?.clone();
        let req = request.into_inner();
        let request_id = req.context.as_ref().map(|c| c.request_id.clone());

        // W2: bare user creation (no org binding) is SYSTEM_ADMIN-only.
        // Org-scoped user creation goes through CreateOrgUser /
        // CreateUserWithMembership which use OrgRole::OrgAdmin.
        authorize_system_admin(self.db.pool(), &claims).await?;

        let user = req
            .user
            .ok_or_else(|| Status::invalid_argument("user required"))?;

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
            user: created_user
                .map(|u| u.to_proto(memberships.iter().map(|m| m.to_proto()).collect())),
            error: None,
            context: Some(self.response_context(request_id, None)),
        }))
    }

    async fn create_user_with_membership(
        &self,
        request: Request<CreateUserWithMembershipRequest>,
    ) -> Result<Response<CreateUserWithMembershipResponse>, Status> {
        let claims = claims_from_request(&request)?.clone();
        let req = request.into_inner();
        let request_id = req.context.as_ref().map(|c| c.request_id.clone());

        authorize_org_access(self.db.pool(), &claims, &req.org_id, OrgRole::OrgAdmin).await?;

        let user = req
            .user
            .ok_or_else(|| Status::invalid_argument("user required"))?;

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
            user: created_user
                .map(|u| u.to_proto(memberships.iter().map(|m| m.to_proto()).collect())),
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
        let claims = claims_from_request(&request)?.clone();
        let req = request.into_inner();
        let request_id = req.context.as_ref().map(|c| c.request_id.clone());

        authorize_org_access(self.db.pool(), &claims, &req.org_id, OrgRole::OrgAdmin).await?;

        info!(user_id = %req.user_id, org_id = %req.org_id, role = req.role, "Adding user to org");

        db::add_user_to_org(
            self.db.pool(),
            &req.user_id,
            &req.org_id,
            req.role,
            if req.invited_by.is_empty() {
                None
            } else {
                Some(&req.invited_by)
            },
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
        let claims = claims_from_request(&request)?.clone();
        let req = request.into_inner();
        let request_id = req.context.as_ref().map(|c| c.request_id.clone());

        authorize_org_access(self.db.pool(), &claims, &req.org_id, OrgRole::OrgAdmin).await?;

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
        let claims = claims_from_request(&request)?.clone();
        let req = request.into_inner();
        let request_id = req.context.as_ref().map(|c| c.request_id.clone());

        authorize_org_access(self.db.pool(), &claims, &req.org_id, OrgRole::OrgAdmin).await?;

        info!(user_id = %req.user_id, org_id = %req.org_id, new_role = req.new_role, "Updating user org role");

        let updated =
            db::update_user_org_role(self.db.pool(), &req.user_id, &req.org_id, req.new_role)
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
        let claims = claims_from_request(&request)?.clone();
        let req = request.into_inner();
        let request_id = req.context.as_ref().map(|c| c.request_id.clone());

        authorize_org_access(self.db.pool(), &claims, &req.org_id, OrgRole::Viewer).await?;

        let page_size = if req.page_size == 0 {
            50
        } else {
            req.page_size
        };
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
        let claims = claims_from_request(&request)?.clone();
        let req = request.into_inner();
        let request_id = req.context.as_ref().map(|c| c.request_id.clone());

        // W2: only SYSTEM_ADMIN can create system users (role=1 grants
        // god-mode access to RegistryAdminService).
        authorize_system_admin(self.db.pool(), &claims).await?;

        let user = req
            .user
            .ok_or_else(|| Status::invalid_argument("user required"))?;

        // SYSTEM_ADMIN (role=1) requires @ciris.ai email
        if user.role == 1 && !user.email.ends_with("@ciris.ai") {
            return Err(Status::invalid_argument(
                "SYSTEM_ADMIN role requires @ciris.ai email",
            ));
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
        let claims = claims_from_request(&request)?.clone();
        let req = request.into_inner();
        let request_id = req.context.as_ref().map(|c| c.request_id.clone());

        // W2: system users are SYSTEM_ADMIN-only.
        authorize_system_admin(self.db.pool(), &claims).await?;

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
        let claims = claims_from_request(&request)?.clone();
        let req = request.into_inner();
        let request_id = req.context.as_ref().map(|c| c.request_id.clone());

        // W2: system users are SYSTEM_ADMIN-only.
        authorize_system_admin(self.db.pool(), &claims).await?;

        let page_size = if req.page_size == 0 {
            50
        } else {
            req.page_size
        };
        let offset: i32 = req.page_token.parse().unwrap_or(0);

        let (users, total) =
            db::list_system_users(self.db.pool(), page_size, offset, req.include_inactive)
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
        let claims = claims_from_request(&request)?.clone();
        let req = request.into_inner();
        let request_id = req.context.as_ref().map(|c| c.request_id.clone());

        // W2: system users are SYSTEM_ADMIN-only.
        authorize_system_admin(self.db.pool(), &claims).await?;

        let user = req
            .user
            .ok_or_else(|| Status::invalid_argument("user required"))?;

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
    // OAuth Identity Management (v1.2.1)
    // =============================================================================

    async fn lookup_user_by_o_auth(
        &self,
        request: Request<LookupUserByOAuthRequest>,
    ) -> Result<Response<LookupUserByOAuthResponse>, Status> {
        let claims = claims_from_request(&request)?.clone();
        let req = request.into_inner();
        let request_id = req.context.as_ref().map(|c| c.request_id.clone());

        // W2: this is the login-flow lookup endpoint — production caller is
        // the CIRISPortal backend with SYSTEM_ADMIN credentials. Restricting
        // to SYSTEM_ADMIN closes a PII / identity-existence oracle that any
        // authenticated caller could otherwise use to enumerate which OAuth
        // identities are registered. A future "self-lookup" path can be
        // added separately if user-facing OAuth introspection is needed.
        authorize_system_admin(self.db.pool(), &claims).await?;

        info!(
            oauth_provider = %req.oauth_provider,
            email = %req.email,
            "Looking up user by OAuth"
        );

        // Use the combined lookup function
        let result = db::lookup_user_for_login(
            self.db.pool(),
            &req.oauth_provider,
            &req.oauth_subject,
            &req.email,
        )
        .await
        .map_err(|e| Status::internal(e.to_string()))?;

        match result {
            db::OAuthLookupResult::FoundByOAuth(user_id) => {
                // Found by OAuth identity
                let user = db::get_multiorg_user(self.db.pool(), &user_id)
                    .await
                    .map_err(|e| Status::internal(e.to_string()))?;
                let memberships = if user.is_some() {
                    db::get_user_memberships(self.db.pool(), &user_id)
                        .await
                        .map_err(|e| Status::internal(e.to_string()))?
                } else {
                    Vec::new()
                };

                Ok(Response::new(LookupUserByOAuthResponse {
                    found: true,
                    user: user.map(|u| u.to_proto(memberships.iter().map(|m| m.to_proto()).collect())),
                    lookup_method: "oauth".to_string(),
                    should_link_oauth: false,
                    error: None,
                    context: Some(self.response_context(request_id, None)),
                }))
            }
            db::OAuthLookupResult::FoundByEmail(user_id) => {
                // Found by email - OAuth should be linked
                let user = db::get_multiorg_user(self.db.pool(), &user_id)
                    .await
                    .map_err(|e| Status::internal(e.to_string()))?;
                let memberships = if user.is_some() {
                    db::get_user_memberships(self.db.pool(), &user_id)
                        .await
                        .map_err(|e| Status::internal(e.to_string()))?
                } else {
                    Vec::new()
                };

                Ok(Response::new(LookupUserByOAuthResponse {
                    found: true,
                    user: user.map(|u| u.to_proto(memberships.iter().map(|m| m.to_proto()).collect())),
                    lookup_method: "email".to_string(),
                    should_link_oauth: true, // Caller should link the new OAuth identity
                    error: None,
                    context: Some(self.response_context(request_id, None)),
                }))
            }
            db::OAuthLookupResult::NotFound => {
                Ok(Response::new(LookupUserByOAuthResponse {
                    found: false,
                    user: None,
                    lookup_method: "not_found".to_string(),
                    should_link_oauth: false,
                    error: None,
                    context: Some(self.response_context(request_id, None)),
                }))
            }
        }
    }

    async fn link_user_o_auth(
        &self,
        request: Request<LinkUserOAuthRequest>,
    ) -> Result<Response<AdminResponse>, Status> {
        let claims = claims_from_request(&request)?.clone();
        let req = request.into_inner();
        let request_id = req.context.as_ref().map(|c| c.request_id.clone());

        // W2: linking OAuth identity to a user. Allow self-link (user
        // attaching their own OAuth identity) or SYSTEM_ADMIN-on-behalf
        // (Portal flow). Cross-user linking by non-admin is rejected.
        if claims.role != crate::middleware::authz::ROLE_SYSTEM_ADMIN
            && claims.sub != req.user_id
        {
            return Err(Status::permission_denied(
                "OAuth linking requires SYSTEM_ADMIN or self (claims.sub == user_id)",
            ));
        }

        info!(
            user_id = %req.user_id,
            oauth_provider = %req.oauth_provider,
            "Linking OAuth identity to user"
        );

        db::link_user_oauth(
            self.db.pool(),
            &req.user_id,
            &req.oauth_provider,
            &req.oauth_subject,
            if req.email.is_empty() { None } else { Some(&req.email) },
        )
        .await
        .map_err(|e| Status::internal(e.to_string()))?;

        // W1: actor from claims.sub (was Some(&req.user_id) — would record
        // the LINKED user as the actor, conflating subject with operator).
        let _ = db::create_audit_entry(
            self.db.pool(),
            AuditActionType::AuditUserUpdated,
            Some(claims.sub.as_str()),
            None,
            None,
            Some("user_oauth"),
            Some(&req.user_id),
            &format!("OAuth identity linked: {} via {}", req.user_id, req.oauth_provider),
            Some(serde_json::json!({
                "oauth_provider": req.oauth_provider,
                "email": req.email,
            })),
        )
        .await;

        Ok(Response::new(self.admin_response(
            true,
            "OAuth identity linked successfully",
            request_id,
        )))
    }

    async fn list_user_o_auth_identities(
        &self,
        request: Request<ListUserOAuthIdentitiesRequest>,
    ) -> Result<Response<ListUserOAuthIdentitiesResponse>, Status> {
        let claims = claims_from_request(&request)?.clone();
        let req = request.into_inner();
        let request_id = req.context.as_ref().map(|c| c.request_id.clone());

        // W2: same self-or-admin pattern as link_user_o_auth — listing OAuth
        // identities is sensitive (reveals which providers a user has linked).
        if claims.role != crate::middleware::authz::ROLE_SYSTEM_ADMIN
            && claims.sub != req.user_id
        {
            return Err(Status::permission_denied(
                "OAuth identity listing requires SYSTEM_ADMIN or self (claims.sub == user_id)",
            ));
        }

        let identities = db::list_user_oauth_identities(self.db.pool(), &req.user_id)
            .await
            .map_err(|e| Status::internal(e.to_string()))?;

        Ok(Response::new(ListUserOAuthIdentitiesResponse {
            identities: identities
                .iter()
                .map(|i| OAuthIdentity {
                    user_id: i.user_id.clone(),
                    oauth_provider: i.oauth_provider.clone(),
                    oauth_subject: i.oauth_subject.clone(),
                    email_at_link: i.email_at_link.clone().unwrap_or_default(),
                    created_at: i.created_at.unix_timestamp(),
                    created_at_iso: i.created_at.format(&time::format_description::well_known::Rfc3339).unwrap_or_default(),
                })
                .collect(),
            context: Some(self.response_context(request_id, None)),
        }))
    }

    async fn lookup_system_user_by_o_auth(
        &self,
        request: Request<LookupSystemUserByOAuthRequest>,
    ) -> Result<Response<LookupSystemUserByOAuthResponse>, Status> {
        let claims = claims_from_request(&request)?.clone();
        let req = request.into_inner();
        let request_id = req.context.as_ref().map(|c| c.request_id.clone());

        // W2: system-user lookups are SYSTEM_ADMIN-only (could enumerate
        // ciris-staff identities by OAuth subject otherwise).
        authorize_system_admin(self.db.pool(), &claims).await?;

        info!(
            oauth_provider = %req.oauth_provider,
            email = %req.email,
            "Looking up system user by OAuth"
        );

        let result = db::lookup_system_user_for_login(
            self.db.pool(),
            &req.oauth_provider,
            &req.oauth_subject,
            &req.email,
        )
        .await
        .map_err(|e| Status::internal(e.to_string()))?;

        match result {
            db::OAuthLookupResult::FoundByOAuth(user_id) => {
                let user = db::get_system_user(self.db.pool(), &user_id)
                    .await
                    .map_err(|e| Status::internal(e.to_string()))?;

                Ok(Response::new(LookupSystemUserByOAuthResponse {
                    found: true,
                    user: user.map(|u| u.to_proto()),
                    lookup_method: "oauth".to_string(),
                    should_link_oauth: false,
                    error: None,
                    context: Some(self.response_context(request_id, None)),
                }))
            }
            db::OAuthLookupResult::FoundByEmail(user_id) => {
                let user = db::get_system_user(self.db.pool(), &user_id)
                    .await
                    .map_err(|e| Status::internal(e.to_string()))?;

                Ok(Response::new(LookupSystemUserByOAuthResponse {
                    found: true,
                    user: user.map(|u| u.to_proto()),
                    lookup_method: "email".to_string(),
                    should_link_oauth: true,
                    error: None,
                    context: Some(self.response_context(request_id, None)),
                }))
            }
            db::OAuthLookupResult::NotFound => {
                Ok(Response::new(LookupSystemUserByOAuthResponse {
                    found: false,
                    user: None,
                    lookup_method: "not_found".to_string(),
                    should_link_oauth: false,
                    error: None,
                    context: Some(self.response_context(request_id, None)),
                }))
            }
        }
    }

    async fn link_system_user_o_auth(
        &self,
        request: Request<LinkSystemUserOAuthRequest>,
    ) -> Result<Response<AdminResponse>, Status> {
        let claims = claims_from_request(&request)?.clone();
        let req = request.into_inner();
        let request_id = req.context.as_ref().map(|c| c.request_id.clone());

        // W2: linking OAuth to a system user is SYSTEM_ADMIN-only.
        authorize_system_admin(self.db.pool(), &claims).await?;

        info!(
            user_id = %req.user_id,
            oauth_provider = %req.oauth_provider,
            "Linking OAuth identity to system user"
        );

        db::link_system_user_oauth(
            self.db.pool(),
            &req.user_id,
            &req.oauth_provider,
            &req.oauth_subject,
            if req.email.is_empty() { None } else { Some(&req.email) },
        )
        .await
        .map_err(|e| Status::internal(e.to_string()))?;

        // W1: actor from claims.sub (was Some(&req.user_id) before — would
        // record the LINKED user as the actor of the link op, which conflates
        // the subject with the operator).
        let _ = db::create_audit_entry(
            self.db.pool(),
            AuditActionType::AuditUserUpdated,
            Some(claims.sub.as_str()),
            None,
            None,
            Some("system_user_oauth"),
            Some(&req.user_id),
            &format!("System user OAuth identity linked: {} via {}", req.user_id, req.oauth_provider),
            Some(serde_json::json!({
                "oauth_provider": req.oauth_provider,
                "email": req.email,
                "is_system_user": true,
            })),
        )
        .await;

        Ok(Response::new(self.admin_response(
            true,
            "System user OAuth identity linked successfully",
            request_id,
        )))
    }

    async fn list_system_user_o_auth_identities(
        &self,
        request: Request<ListSystemUserOAuthIdentitiesRequest>,
    ) -> Result<Response<ListSystemUserOAuthIdentitiesResponse>, Status> {
        let claims = claims_from_request(&request)?.clone();
        let req = request.into_inner();
        let request_id = req.context.as_ref().map(|c| c.request_id.clone());

        // W2: system-user OAuth identities are SYSTEM_ADMIN-only.
        authorize_system_admin(self.db.pool(), &claims).await?;

        let identities = db::list_system_user_oauth_identities(self.db.pool(), &req.user_id)
            .await
            .map_err(|e| Status::internal(e.to_string()))?;

        Ok(Response::new(ListSystemUserOAuthIdentitiesResponse {
            identities: identities
                .iter()
                .map(|i| OAuthIdentity {
                    user_id: i.user_id.clone(),
                    oauth_provider: i.oauth_provider.clone(),
                    oauth_subject: i.oauth_subject.clone(),
                    email_at_link: i.email_at_link.clone().unwrap_or_default(),
                    created_at: i.created_at.unix_timestamp(),
                    created_at_iso: i.created_at.format(&time::format_description::well_known::Rfc3339).unwrap_or_default(),
                })
                .collect(),
            context: Some(self.response_context(request_id, None)),
        }))
    }

    // =============================================================================
    // Organization Hierarchy (v1.2.0)
    // =============================================================================

    async fn list_child_organizations(
        &self,
        request: Request<ListChildOrganizationsRequest>,
    ) -> Result<Response<ListOrganizationsResponse>, Status> {
        let claims = claims_from_request(&request)?.clone();
        let req = request.into_inner();
        let request_id = req.context.as_ref().map(|c| c.request_id.clone());

        // Authz against parent_org_id — requires Viewer access to the parent.
        authorize_org_access(self.db.pool(), &claims, &req.parent_org_id, OrgRole::Viewer).await?;

        let page_size = if req.page_size == 0 {
            50
        } else {
            req.page_size
        };
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
        let claims = claims_from_request(&request)?.clone();
        let req = request.into_inner();
        let request_id = req.context.as_ref().map(|c| c.request_id.clone());

        // W2: creating a licensee under a PARTNER org requires either
        // SYSTEM_ADMIN or OrgAdmin on the parent. SYSTEM_ADMIN check is
        // implicit in authorize_org_access, so a single call covers both.
        authorize_org_access(self.db.pool(), &claims, &req.parent_org_id, OrgRole::OrgAdmin)
            .await?;

        let mut org = req
            .organization
            .ok_or_else(|| Status::invalid_argument("organization required"))?;

        // Verify parent org exists and is a PARTNER type
        let parent = db::get_organization(self.db.pool(), &req.parent_org_id)
            .await
            .map_err(|e| Status::internal(e.to_string()))?
            .ok_or_else(|| Status::not_found("Parent organization not found"))?;

        if parent.org_type != OrgType::OrgPartner as i32 {
            return Err(Status::invalid_argument(
                "Only PARTNER organizations can create licensees",
            ));
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
        let claims = claims_from_request(&request)?.clone();
        let req = request.into_inner();
        let request_id = req.context.as_ref().map(|c| c.request_id.clone());

        authorize_org_access(self.db.pool(), &claims, &req.org_id, OrgRole::Viewer).await?;

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
        let claims = claims_from_request(&request)?.clone();
        let req = request.into_inner();
        let request_id = req.context.as_ref().map(|c| c.request_id.clone());

        // W2: upgrading to PARTNER tier is a billing/license-tier escalation.
        // Restrict to SYSTEM_ADMIN; org-admins can request via support, not
        // self-serve (avoids unauthorized tier changes that affect billing).
        authorize_system_admin(self.db.pool(), &claims).await?;

        // Get the org to upgrade
        let org = db::get_organization(self.db.pool(), &req.org_id)
            .await
            .map_err(|e| Status::internal(e.to_string()))?
            .ok_or_else(|| Status::not_found("Organization not found"))?;

        // Must be COMMUNITY type to upgrade
        if org.org_type != OrgType::OrgCommunity as i32 {
            return Err(Status::invalid_argument(
                "Only COMMUNITY organizations can be upgraded to PARTNER",
            ));
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
                None,                             // upgraded_by not in proto
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

    // =============================================================================
    // Self-Custody Key Management (v1.3.0)
    // =============================================================================

    async fn get_registration_challenge(
        &self,
        request: Request<GetRegistrationChallengeRequest>,
    ) -> Result<Response<GetRegistrationChallengeResponse>, Status> {
        let claims = claims_from_request(&request)?.clone();
        let req = request.into_inner();
        let request_id = req.context.as_ref().map(|c| c.request_id.clone());

        authorize_org_access(self.db.pool(), &claims, &req.org_id, OrgRole::KeyManager).await?;

        info!(org_id = %req.org_id, "Generating registration challenge for self-custody key");

        // Verify org exists
        let org = db::get_organization(self.db.pool(), &req.org_id)
            .await
            .map_err(|e| Status::internal(e.to_string()))?
            .ok_or_else(|| Status::not_found("Organization not found"))?;

        if !org.active {
            return Err(Status::failed_precondition("Organization is not active"));
        }

        // Generate 32-byte challenge
        use rand::RngCore;
        let mut challenge = [0u8; 32];
        rand::rngs::OsRng.fill_bytes(&mut challenge);

        // Store challenge with 5-minute expiry
        let expires_at = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs() as i64
            + 300;

        db::store_registration_challenge(self.db.pool(), &req.org_id, &challenge, expires_at)
            .await
            .map_err(|e| Status::internal(e.to_string()))?;

        Ok(Response::new(GetRegistrationChallengeResponse {
            challenge: challenge.to_vec(),
            expires_at,
            context: Some(self.response_context(request_id, None)),
        }))
    }

    async fn register_public_key(
        &self,
        request: Request<RegisterPublicKeyRequest>,
    ) -> Result<Response<RegisterPublicKeyResponse>, Status> {
        let claims = claims_from_request(&request)?.clone();
        let req = request.into_inner();
        let request_id = req.context.as_ref().map(|c| c.request_id.clone());

        authorize_org_access(self.db.pool(), &claims, &req.org_id, OrgRole::KeyManager).await?;

        info!(org_id = %req.org_id, "Registering self-custody public key");

        // 1. Validate Ed25519 public key length
        if req.ed25519_public_key.len() != 32 {
            return Err(Status::invalid_argument(
                "Ed25519 public key must be 32 bytes",
            ));
        }

        // 2. Validate and consume the challenge
        let stored_challenge = db::get_and_remove_registration_challenge(self.db.pool(), &req.org_id)
            .await
            .map_err(|e| Status::internal(e.to_string()))?
            .ok_or_else(|| Status::invalid_argument("Invalid or expired challenge"))?;

        if stored_challenge != req.registration_challenge {
            return Err(Status::invalid_argument("Challenge mismatch"));
        }

        // 3. Verify signature over challenge
        let public_key = ed25519_dalek::VerifyingKey::from_bytes(
            req.ed25519_public_key
                .as_slice()
                .try_into()
                .map_err(|_| Status::invalid_argument("Invalid public key length"))?,
        )
        .map_err(|_| Status::invalid_argument("Invalid Ed25519 public key"))?;

        if req.ed25519_signature.len() != 64 {
            return Err(Status::invalid_argument(
                "Ed25519 signature must be 64 bytes",
            ));
        }

        let signature = ed25519_dalek::Signature::from_bytes(
            req.ed25519_signature
                .as_slice()
                .try_into()
                .map_err(|_| Status::invalid_argument("Invalid signature length"))?,
        );

        use ed25519_dalek::Verifier;
        public_key
            .verify(&req.registration_challenge, &signature)
            .map_err(|_| Status::invalid_argument("Signature verification failed"))?;

        // 4. Check for duplicate public key across all orgs
        let pub_key_hash = HybridCrypto::fingerprint(&req.ed25519_public_key);
        if db::public_key_exists(self.db.pool(), &pub_key_hash)
            .await
            .map_err(|e| Status::internal(e.to_string()))?
        {
            return Err(Status::already_exists("Public key already registered"));
        }

        // 5. Generate ML-DSA fingerprint if provided
        let mldsa_fp = if !req.ml_dsa_65_public_key.is_empty() {
            HybridCrypto::fingerprint(&req.ml_dsa_65_public_key)
        } else {
            String::new()
        };

        // 6. Create key record (PENDING status, SELF_SOVEREIGN custody)
        let key_label = if req.key_label.is_empty() {
            None
        } else {
            Some(req.key_label.as_str())
        };
        let key_id = db::create_self_custody_key(
            self.db.pool(),
            &req.org_id,
            &req.ed25519_public_key,
            &req.ml_dsa_65_public_key,
            &pub_key_hash,
            &mldsa_fp,
            &claims.sub,
            key_label,
        )
        .await
        .map_err(|e| Status::internal(e.to_string()))?;

        // 7. Generate activation challenge
        use rand::RngCore;
        let mut activation_challenge = [0u8; 32];
        rand::rngs::OsRng.fill_bytes(&mut activation_challenge);

        let activation_expires = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs() as i64
            + 300; // 5 minutes

        db::store_activation_challenge(
            self.db.pool(),
            &key_id,
            &activation_challenge,
            activation_expires,
        )
        .await
        .map_err(|e| Status::internal(e.to_string()))?;

        // 8. Fetch the created key record
        let key_record = db::get_key(self.db.pool(), &key_id)
            .await
            .map_err(|e| Status::internal(e.to_string()))?
            .ok_or_else(|| Status::internal("Key not found after creation"))?;

        // 9. Create audit entry (W1: actor from claims.sub).
        let _ = db::create_audit_entry(
            self.db.pool(),
            AuditActionType::AuditKeyGenerated,
            Some(claims.sub.as_str()),
            Some(&req.org_id),
            None,
            Some("key"),
            Some(&key_id),
            &format!("Self-custody public key registered: {}", key_id),
            Some(serde_json::json!({
                "key_id": key_id,
                "custody_model": "SELF_SOVEREIGN",
                "ed25519_fingerprint": pub_key_hash,
                "key_label": req.key_label,
            })),
        )
        .await;

        Ok(Response::new(RegisterPublicKeyResponse {
            key_record: Some(key_record.to_proto()),
            activation_challenge: activation_challenge.to_vec(),
            error: None,
            context: Some(self.response_context(request_id, None)),
        }))
    }

    async fn activate_self_custody_key(
        &self,
        request: Request<ActivateSelfCustodyKeyRequest>,
    ) -> Result<Response<AdminResponse>, Status> {
        let claims = claims_from_request(&request)?.clone();
        let req = request.into_inner();
        let request_id = req.context.as_ref().map(|c| c.request_id.clone());

        authorize_org_access(self.db.pool(), &claims, &req.org_id, OrgRole::KeyManager).await?;

        info!(key_id = %req.key_id, org_id = %req.org_id, "Activating self-custody key");

        // 1. Get pending key record
        let key_record = db::get_key(self.db.pool(), &req.key_id)
            .await
            .map_err(|e| Status::internal(e.to_string()))?
            .ok_or_else(|| Status::not_found("Key not found"))?;

        if key_record.org_id != req.org_id {
            return Err(Status::permission_denied(
                "Key does not belong to organization",
            ));
        }

        if key_record.status != KeyStatus::KeyPending as i32 {
            return Err(Status::failed_precondition("Key not in PENDING status"));
        }

        if key_record.custody_model != KeyCustodyModel::SelfSovereign as i32 {
            return Err(Status::failed_precondition("Not a self-custody key"));
        }

        // 2. Validate and consume activation challenge
        let stored_challenge = db::get_and_remove_activation_challenge(self.db.pool(), &req.key_id)
            .await
            .map_err(|e| Status::internal(e.to_string()))?
            .ok_or_else(|| Status::invalid_argument("Invalid or expired activation challenge"))?;

        if stored_challenge != req.activation_challenge {
            return Err(Status::invalid_argument("Challenge mismatch"));
        }

        // 3. Verify activation signature
        let public_key = ed25519_dalek::VerifyingKey::from_bytes(
            key_record
                .ed25519_public_key
                .as_slice()
                .try_into()
                .map_err(|_| Status::internal("Invalid stored public key"))?,
        )
        .map_err(|_| Status::internal("Invalid stored public key"))?;

        if req.ed25519_signature.len() != 64 {
            return Err(Status::invalid_argument(
                "Ed25519 signature must be 64 bytes",
            ));
        }

        let signature = ed25519_dalek::Signature::from_bytes(
            req.ed25519_signature
                .as_slice()
                .try_into()
                .map_err(|_| Status::invalid_argument("Invalid signature"))?,
        );

        use ed25519_dalek::Verifier;
        public_key
            .verify(&req.activation_challenge, &signature)
            .map_err(|_| Status::invalid_argument("Activation signature verification failed"))?;

        // 4. Activate key
        db::activate_key(self.db.pool(), &req.key_id)
            .await
            .map_err(|e| Status::internal(e.to_string()))?;

        // 5. Optionally bind to agent hash
        if !req.agent_hash.is_empty() {
            // Store agent binding - could add a column for this if needed
            info!(key_id = %req.key_id, agent_hash = %req.agent_hash, "Key bound to agent");
        }

        // 6. Create audit entry
        let _ = db::create_audit_entry(
            self.db.pool(),
            AuditActionType::AuditKeyActivated,
            None,
            Some(&req.org_id),
            None,
            Some("key"),
            Some(&req.key_id),
            &format!("Self-custody key activated: {}", req.key_id),
            Some(serde_json::json!({
                "key_id": req.key_id,
                "custody_model": "SELF_SOVEREIGN",
                "agent_hash": req.agent_hash,
            })),
        )
        .await;

        Ok(Response::new(self.admin_response(
            true,
            "Self-custody key activated",
            request_id,
        )))
    }

    async fn rotate_self_custody_key(
        &self,
        request: Request<RotateSelfCustodyKeyRequest>,
    ) -> Result<Response<RotateKeyResponse>, Status> {
        let claims = claims_from_request(&request)?.clone();
        let req = request.into_inner();
        let request_id = req.context.as_ref().map(|c| c.request_id.clone());

        authorize_org_access(self.db.pool(), &claims, &req.org_id, OrgRole::KeyManager).await?;

        info!(org_id = %req.org_id, new_key_id = %req.new_key_id, "Rotating self-custody key");

        // 1. Get current active key
        let old_key = db::get_active_key(self.db.pool(), &req.org_id)
            .await
            .map_err(|e| Status::internal(e.to_string()))?
            .ok_or_else(|| Status::failed_precondition("No active key to rotate"))?;

        if old_key.custody_model != KeyCustodyModel::SelfSovereign as i32 {
            return Err(Status::failed_precondition(
                "Current key is not self-custody; use RotateKey for custodied keys",
            ));
        }

        // 2. Get new key (must be PENDING self-custody key)
        let new_key = db::get_key(self.db.pool(), &req.new_key_id)
            .await
            .map_err(|e| Status::internal(e.to_string()))?
            .ok_or_else(|| Status::not_found("New key not found"))?;

        if new_key.org_id != req.org_id {
            return Err(Status::permission_denied(
                "New key does not belong to organization",
            ));
        }

        if new_key.status != KeyStatus::KeyPending as i32 {
            return Err(Status::failed_precondition("New key not in PENDING status"));
        }

        if new_key.custody_model != KeyCustodyModel::SelfSovereign as i32 {
            return Err(Status::failed_precondition("New key is not self-custody"));
        }

        // 3. Verify signatures from both old and new keys over rotation challenge
        // Old key signature
        let old_pubkey = ed25519_dalek::VerifyingKey::from_bytes(
            old_key
                .ed25519_public_key
                .as_slice()
                .try_into()
                .map_err(|_| Status::internal("Invalid old public key"))?,
        )
        .map_err(|_| Status::internal("Invalid old public key"))?;

        if req.old_key_signature.len() != 64 {
            return Err(Status::invalid_argument(
                "Old key signature must be 64 bytes",
            ));
        }

        let old_signature = ed25519_dalek::Signature::from_bytes(
            req.old_key_signature
                .as_slice()
                .try_into()
                .map_err(|_| Status::invalid_argument("Invalid old key signature"))?,
        );

        use ed25519_dalek::Verifier;
        old_pubkey
            .verify(&req.rotation_challenge, &old_signature)
            .map_err(|_| Status::invalid_argument("Old key signature verification failed"))?;

        // New key signature
        let new_pubkey = ed25519_dalek::VerifyingKey::from_bytes(
            new_key
                .ed25519_public_key
                .as_slice()
                .try_into()
                .map_err(|_| Status::internal("Invalid new public key"))?,
        )
        .map_err(|_| Status::internal("Invalid new public key"))?;

        if req.new_key_signature.len() != 64 {
            return Err(Status::invalid_argument(
                "New key signature must be 64 bytes",
            ));
        }

        let new_signature = ed25519_dalek::Signature::from_bytes(
            req.new_key_signature
                .as_slice()
                .try_into()
                .map_err(|_| Status::invalid_argument("Invalid new key signature"))?,
        );

        new_pubkey
            .verify(&req.rotation_challenge, &new_signature)
            .map_err(|_| Status::invalid_argument("New key signature verification failed"))?;

        // 4. Perform rotation
        let grace_period = if req.grace_period_hours > 0 {
            req.grace_period_hours
        } else {
            24 // Default 24 hour grace period
        };

        // Activate new key
        db::activate_key(self.db.pool(), &req.new_key_id)
            .await
            .map_err(|e| Status::internal(e.to_string()))?;

        // Mark old key as rotated with grace period
        db::mark_key_rotated(self.db.pool(), &old_key.key_id, grace_period)
            .await
            .map_err(|e| Status::internal(e.to_string()))?;

        // 5. Create audit entry
        let _ = db::create_audit_entry(
            self.db.pool(),
            AuditActionType::AuditKeyRotated,
            None,
            Some(&req.org_id),
            None,
            Some("key"),
            Some(&old_key.key_id),
            &format!(
                "Self-custody key rotated: {} -> {}",
                old_key.key_id, req.new_key_id
            ),
            Some(serde_json::json!({
                "old_key_id": old_key.key_id,
                "new_key_id": req.new_key_id,
                "custody_model": "SELF_SOVEREIGN",
                "reason": req.reason,
                "grace_period_hours": grace_period,
            })),
        )
        .await;

        // 6. Fetch updated keys
        let new_key_updated = db::get_key(self.db.pool(), &req.new_key_id)
            .await
            .map_err(|e| Status::internal(e.to_string()))?;

        let old_key_updated = db::get_key(self.db.pool(), &old_key.key_id)
            .await
            .map_err(|e| Status::internal(e.to_string()))?;

        let grace_expires = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs() as i64
            + (grace_period as i64 * 3600);

        Ok(Response::new(RotateKeyResponse {
            new_key: new_key_updated.map(|k| k.to_proto()),
            old_key: old_key_updated.map(|k| k.to_proto()),
            grace_period_expires_at: grace_expires,
            rotation_id: uuid::Uuid::new_v4().to_string(),
            context: Some(self.response_context(request_id, None)),
        }))
    }
}
