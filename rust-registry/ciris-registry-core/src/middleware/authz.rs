//! Policy authorization for authenticated handlers.
//!
//! Wire-level auth (`middleware/auth.rs`) validates JWTs and inserts
//! `Claims` into request extensions. This module is the policy layer that
//! every authenticated handler should call to enforce per-org access rules.
//!
//! Closes THREAT_MODEL.md AV-15 (PortalService cross-org access — handlers
//! trust `req.org_id`, ignore JWT claims). Foundation for Phase 4 handler
//! sweep + Phase 5 audit-actor integrity (W1) + Phase 6 SYSTEM_ADMIN
//! gating (W2).
//!
//! ## Convention for handler authors
//!
//! Every authenticated handler must:
//!
//! 1. Extract claims *before* `request.into_inner()`:
//!    ```ignore
//!    let claims = claims_from_request(&request)?;
//!    ```
//! 2. Call `authorize_org_access` (or `authorize_system_admin`) against
//!    the same field used for the DB write. **Inner-proto fields**
//!    (e.g., `req.user.org_id`, `req.organization.org_id`,
//!    `req.sign_request.org_id`) must be authz'd against the inner value
//!    — see W3 in the AV-15 investigation report.
//! 3. Derive `actor_user_id` from `claims.sub`, never from request body
//!    fields like `req.requester_user_id` (W1).
//!
//! See `CLAUDE.md` "Handler convention" section for the canonical preamble.

use sqlx::PgPool;
use tonic::{Request, Status};

use crate::db;
use crate::middleware::auth::Claims;
use crate::proto::AuditActionType;

/// System admin role value (matches `auth::ROLE_SYSTEM_ADMIN`).
pub const ROLE_SYSTEM_ADMIN: i32 = 1;

/// Org-membership roles, mirroring `migrations/002_role_hierarchy.sql:75`.
///
/// **Lower numeric value = higher privilege.** A caller with role
/// `OrgAdmin (1)` satisfies any `required_role >= 1`. A caller with role
/// `Viewer (4)` only satisfies `required_role == 4`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum OrgRole {
    /// Full admin within an org. Can create/update users, manage keys, etc.
    OrgAdmin = 1,
    /// Manage cryptographic keys for the org.
    KeyManager = 2,
    /// Operate (sign, request signatures, request key recovery).
    Operator = 3,
    /// Read-only access.
    Viewer = 4,
}

/// Extract validated `Claims` from a tonic request.
///
/// The auth middleware (`middleware/auth.rs`) inserts `Claims` into the
/// request extensions when the JWT validates successfully. Returns
/// `Status::unauthenticated` if claims are missing — should never happen
/// on a properly auth-gated handler, but a safe failure mode.
pub fn claims_from_request<T>(request: &Request<T>) -> Result<&Claims, Status> {
    request
        .extensions()
        .get::<Claims>()
        .ok_or_else(|| Status::unauthenticated("missing JWT claims"))
}

/// Authorize the caller for access to a target organization at the given
/// minimum role.
///
/// Logic:
/// 1. SYSTEM_ADMIN passes unconditionally (cross-org by design).
/// 2. Empty `target_org_id` is rejected with `InvalidArgument`.
/// 3. Otherwise look up the caller's role in `target_org_id` via
///    `db::memberships::get_user_role_in_org`.
/// 4. Pass if the caller has a role and it meets `required_role`
///    (numerically less-than-or-equal — lower = higher privilege).
/// 5. Otherwise deny with `PermissionDenied` and write an
///    `AUDIT_ACCESS_DENIED` audit-log entry.
///
/// **Important**: pass `target_org_id` exactly as the DB write will use
/// it. Inner-proto fields (e.g., `req.user.org_id`) must be authz'd
/// against the inner value, not a separate top-level `req.org_id`.
pub async fn authorize_org_access(
    db: &PgPool,
    claims: &Claims,
    target_org_id: &str,
    required_role: OrgRole,
) -> Result<(), Status> {
    // (1) SYSTEM_ADMIN cross-org override.
    if claims.role == ROLE_SYSTEM_ADMIN {
        return Ok(());
    }

    // (2) Reject empty target.
    if target_org_id.is_empty() {
        return Err(Status::invalid_argument(
            "org_id is required",
        ));
    }

    // (3) Membership lookup.
    let role = db::get_user_role_in_org(db, &claims.sub, target_org_id)
        .await
        .map_err(|e| Status::internal(format!("authz lookup failed: {}", e)))?;

    // (4) Role check (lower = higher privilege).
    let required = required_role as i32;
    match role {
        Some(r) if r > 0 && r <= required => Ok(()),
        _ => {
            // (5) Audit the denial. Best-effort — don't fail the
            // PermissionDenied response if audit write itself errors.
            let _ = db::create_audit_entry(
                db,
                AuditActionType::AuditAccessDenied,
                Some(&claims.sub),
                Some(target_org_id),
                None,
                Some("portal_rpc"),
                None,
                &format!(
                    "Cross-org access denied: caller_org={}, target_org={}, role={:?}, required={:?}",
                    claims.org_id, target_org_id, role, required_role
                ),
                None,
            )
            .await;

            Err(Status::permission_denied(
                "not authorized for this organization",
            ))
        }
    }
}

/// Authorize the caller as a SYSTEM_ADMIN. Used for PortalService methods
/// that operate cross-org by design (`create_organization`,
/// `create_system_user`, `link_*_oauth`, `upgrade_to_partner`, etc.).
///
/// Closes W2 — the AuthLayer only gates SYSTEM_ADMIN on
/// `RegistryAdminService`. PortalService god-mode methods need an
/// explicit check.
pub async fn authorize_system_admin(
    db: &PgPool,
    claims: &Claims,
) -> Result<(), Status> {
    if claims.role == ROLE_SYSTEM_ADMIN {
        return Ok(());
    }

    let _ = db::create_audit_entry(
        db,
        AuditActionType::AuditAccessDenied,
        Some(&claims.sub),
        Some(&claims.org_id),
        None,
        Some("portal_rpc"),
        None,
        "System-admin access denied: caller is not SYSTEM_ADMIN",
        None,
    )
    .await;

    Err(Status::permission_denied("system admin role required"))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn claims(sub: &str, org_id: &str, role: i32) -> Claims {
        Claims {
            sub: sub.to_string(),
            org_id: org_id.to_string(),
            role,
            exp: (std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs() as usize)
                + 3600,
            iat: 0,
            iss: "test".to_string(),
        }
    }

    #[test]
    fn org_role_numeric_values_match_proto() {
        // Sanity check against migrations/002_role_hierarchy.sql:75
        assert_eq!(OrgRole::OrgAdmin as i32, 1);
        assert_eq!(OrgRole::KeyManager as i32, 2);
        assert_eq!(OrgRole::Operator as i32, 3);
        assert_eq!(OrgRole::Viewer as i32, 4);
    }

    #[test]
    fn role_constants_match_auth_module() {
        // Make sure we agree with auth.rs::ROLE_SYSTEM_ADMIN.
        assert_eq!(ROLE_SYSTEM_ADMIN, 1);
    }

    #[test]
    fn claims_from_request_returns_extensions() {
        let mut req = Request::new(());
        let c = claims("user-1", "org-a", 0);
        req.extensions_mut().insert(c.clone());
        let extracted = claims_from_request(&req).unwrap();
        assert_eq!(extracted.sub, "user-1");
    }

    #[test]
    fn claims_from_request_missing_returns_unauthenticated() {
        let req: Request<()> = Request::new(());
        let err = claims_from_request(&req).unwrap_err();
        assert_eq!(err.code(), tonic::Code::Unauthenticated);
    }

    // Note: authorize_org_access requires a live DB pool to test the
    // membership lookup path. Integration tests in tests/portal_authz.rs
    // (Phase 4) will exercise the SYSTEM_ADMIN pass + cross-org deny +
    // home-org membership pass paths against a seeded database.
}
