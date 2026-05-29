//! JWT minting + tonic request helpers for Phase 4+ integration tests.
//!
//! Mirrors the encoding recipe at `src/middleware/auth.rs:251-302`
//! (existing JWT validation tests). Test code that needs to exercise
//! auth-gated PortalService handlers should:
//!
//! ```ignore
//! let jwt = mint_jwt("test-secret", "user-1", "org-a", 0, 3600);
//! let req = with_jwt(MyRequest { org_id: "org-a".into(), ... }, "test-secret", "user-1", "org-a", 0);
//! my_service.my_method(req).await
//! ```
//!
//! `with_jwt` injects both the `Authorization: Bearer ...` header (so the
//! tower AuthLayer would validate it if present) AND pre-inserts the
//! `Claims` struct into request extensions (so unit-style tests that
//! bypass the middleware stack still get authenticated context).

use jsonwebtoken::{encode, EncodingKey, Header};
use serde::{Deserialize, Serialize};

/// Mirrors `src/middleware/auth.rs::Claims`. Duplicated here so the test
/// fixture compiles independently of the production Claims type
/// visibility (it is `pub` today; this struct shadows it for serialization
/// in fixtures).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestClaims {
    pub sub: String,
    #[serde(default)]
    pub org_id: String,
    #[serde(default)]
    pub role: i32,
    pub exp: usize,
    #[serde(default)]
    pub iat: usize,
    #[serde(default)]
    pub iss: String,
}

/// Mint a JWT with the given fields. Use the same secret as the test
/// fixture's auth setup (typically `"test-secret"`).
///
/// `exp_offset_secs` is added to the current time; pass a large positive
/// value (e.g., 3600) for a token that won't expire mid-test.
pub fn mint_jwt(
    secret: &str,
    sub: &str,
    org_id: &str,
    role: i32,
    exp_offset_secs: i64,
) -> String {
    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs() as i64;

    let claims = TestClaims {
        sub: sub.to_string(),
        org_id: org_id.to_string(),
        role,
        exp: (now + exp_offset_secs) as usize,
        iat: now as usize,
        iss: "test".to_string(),
    };

    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_bytes()),
    )
    .expect("JWT mint")
}

/// Build a tonic request carrying both the `Authorization: Bearer <jwt>`
/// header AND a pre-inserted `Claims` extension (so handlers reading via
/// `claims_from_request` get them whether or not the AuthLayer ran).
///
/// **Note**: `claims` are imported via the production `middleware::auth`
/// module. Tests must depend on the binary crate path — see
/// `tests/portal_authz.rs` for the canonical setup.
pub fn with_jwt<T>(
    inner: T,
    secret: &str,
    sub: &str,
    org_id: &str,
    role: i32,
) -> tonic::Request<T> {
    let jwt = mint_jwt(secret, sub, org_id, role, 3600);
    let mut req = tonic::Request::new(inner);
    req.metadata_mut()
        .insert("authorization", format!("Bearer {}", jwt).parse().unwrap());
    req
}

/// Convenience: SYSTEM_ADMIN role JWT for cross-org test scenarios.
pub fn mint_admin_jwt(secret: &str, sub: &str) -> String {
    mint_jwt(secret, sub, "system", 1, 3600)
}
