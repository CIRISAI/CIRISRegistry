//! Authentication middleware (mTLS + JWT)
//!
//! Enforces per-service auth based on gRPC path:
//! - RegistryService: Public (read-only lookups, no auth required)
//! - PortalService: Requires valid JWT (any authenticated user)
//! - RegistryAdminService: Requires valid JWT with admin role (role=1)

use std::task::{Context, Poll};

use http::{Request, Response, StatusCode};
use jsonwebtoken::{decode, DecodingKey, Validation, Algorithm};
use serde::{Deserialize, Serialize};
use tower::{Layer, Service};

use crate::config::AuthSettings;

/// Role constants (from proto SystemRole enum)
const ROLE_SYSTEM_ADMIN: i32 = 1;

/// JWT claims structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Claims {
    /// Subject (user ID)
    pub sub: String,
    /// Organization ID
    #[serde(default)]
    pub org_id: String,
    /// User role (as integer, maps to SystemRole proto enum)
    #[serde(default)]
    pub role: i32,
    /// Expiration timestamp
    pub exp: usize,
    /// Issued at timestamp
    #[serde(default)]
    pub iat: usize,
    /// Issuer
    #[serde(default)]
    pub iss: String,
}

/// What level of auth a request path requires
#[derive(Debug, Clone, Copy, PartialEq)]
enum AuthRequirement {
    /// No auth needed (public read-only)
    None,
    /// Valid JWT required (any authenticated user)
    Authenticated,
    /// Valid JWT with admin role required
    Admin,
}

/// Determine auth requirement based on gRPC path
fn classify_path(path: &str) -> AuthRequirement {
    // Infrastructure endpoints — no auth
    if path.contains("grpc.health")
        || path.contains("grpc.reflection")
        || path == "/health"
        || path == "/ready"
        || path == "/live"
        || path == "/metrics"
    {
        return AuthRequirement::None;
    }

    // RegistryService — public read-only lookups (CIRISVerify, etc.)
    if path.contains("ciris.registry.v1.RegistryService") {
        return AuthRequirement::None;
    }

    // RegistryAdminService — admin only (register agents, revoke, emergency)
    if path.contains("ciris.registry.v1.RegistryAdminService") {
        return AuthRequirement::Admin;
    }

    // PortalService — authenticated (org management, key custody)
    if path.contains("ciris.registry.v1.PortalService") {
        return AuthRequirement::Authenticated;
    }

    // Unknown paths — require auth by default (fail-secure)
    AuthRequirement::Authenticated
}


#[derive(Clone)]
pub struct AuthLayer {
    settings: AuthSettings,
}

impl AuthLayer {
    pub fn new(settings: AuthSettings) -> Self {
        Self { settings }
    }
}

impl<S> Layer<S> for AuthLayer {
    type Service = AuthService<S>;

    fn layer(&self, inner: S) -> Self::Service {
        AuthService {
            inner,
            settings: self.settings.clone(),
        }
    }
}

#[derive(Clone)]
pub struct AuthService<S> {
    inner: S,
    settings: AuthSettings,
}

impl<S, ReqBody, ResBody> Service<Request<ReqBody>> for AuthService<S>
where
    S: Service<Request<ReqBody>, Response = Response<ResBody>>,
    S::Future: Send + 'static,
    ResBody: Default,
{
    type Response = S::Response;
    type Error = S::Error;
    type Future = std::pin::Pin<
        Box<dyn std::future::Future<Output = Result<Self::Response, Self::Error>> + Send>,
    >;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx)
    }

    fn call(&mut self, mut request: Request<ReqBody>) -> Self::Future {
        let path = request.uri().path().to_owned();
        let requirement = classify_path(&path);

        // Public endpoints — pass through without auth
        if requirement == AuthRequirement::None {
            // Still validate JWT if provided (for audit attribution)
            if let Some(claims) = self.try_extract_claims(&request) {
                request.extensions_mut().insert(claims);
            }
            let future = self.inner.call(request);
            return Box::pin(async move { future.await });
        }

        // Protected endpoints — JWT required
        let claims = match self.try_extract_claims(&request) {
            Some(claims) => claims,
            None => {
                tracing::warn!(
                    path = %path,
                    requirement = ?requirement,
                    "Unauthenticated request to protected endpoint"
                );
                return Box::pin(async move {
                    Ok(Response::builder()
                        .status(StatusCode::UNAUTHORIZED)
                        .body(ResBody::default())
                        .unwrap())
                });
            }
        };

        // Admin endpoints — check role
        if requirement == AuthRequirement::Admin && claims.role != ROLE_SYSTEM_ADMIN {
            tracing::warn!(
                path = %path,
                sub = %claims.sub,
                role = claims.role,
                "Non-admin request to admin endpoint"
            );
            return Box::pin(async move {
                Ok(Response::builder()
                    .status(StatusCode::FORBIDDEN)
                    .body(ResBody::default())
                    .unwrap())
            });
        }

        // Store validated claims for handler use
        request.extensions_mut().insert(claims);
        let future = self.inner.call(request);
        Box::pin(async move { future.await })
    }
}

impl<S> AuthService<S> {
    /// Extract and validate JWT from Authorization header.
    /// Returns None if no header or validation fails.
    fn try_extract_claims<ReqBody>(&self, request: &Request<ReqBody>) -> Option<Claims> {
        let auth_header = request
            .headers()
            .get("authorization")
            .and_then(|h| h.to_str().ok())?;

        if !auth_header.starts_with("Bearer ") {
            return None;
        }

        let token = &auth_header[7..];
        match validate_jwt(token, &self.settings.jwt_secret, &self.settings.jwt_issuer) {
            Ok(claims) => Some(claims),
            Err(e) => {
                tracing::warn!("JWT validation failed: {}", e);
                None
            }
        }
    }
}

/// Validate a JWT token
fn validate_jwt(token: &str, secret: &str, expected_issuer: &str) -> Result<Claims, String> {
    let decoding_key = DecodingKey::from_secret(secret.as_bytes());

    let mut validation = Validation::new(Algorithm::HS256);
    validation.set_issuer(&[expected_issuer]);
    validation.validate_exp = true;

    decode::<Claims>(token, &decoding_key, &validation)
        .map(|data| data.claims)
        .map_err(|e| format!("JWT decode error: {}", e))
}

/// Helper to extract claims from request extensions
pub fn get_claims<B>(request: &Request<B>) -> Option<&Claims> {
    request.extensions().get::<Claims>()
}

#[cfg(test)]
mod tests {
    use super::*;
    use jsonwebtoken::{encode, EncodingKey, Header};

    #[test]
    fn test_classify_path() {
        assert_eq!(
            classify_path("/ciris.registry.v1.RegistryService/LookupAgent"),
            AuthRequirement::None,
        );
        assert_eq!(
            classify_path("/ciris.registry.v1.RegistryAdminService/RegisterAgent"),
            AuthRequirement::Admin,
        );
        assert_eq!(
            classify_path("/ciris.registry.v1.PortalService/CreateOrganization"),
            AuthRequirement::Authenticated,
        );
        assert_eq!(
            classify_path("/health"),
            AuthRequirement::None,
        );
        assert_eq!(
            classify_path("/unknown/path"),
            AuthRequirement::Authenticated,
        );
    }

    #[test]
    fn test_validate_jwt_valid_token() {
        let secret = "test-secret";
        let issuer = "test-issuer";

        let claims = Claims {
            sub: "user123".to_string(),
            org_id: "org456".to_string(),
            role: 1,
            exp: (chrono::Utc::now().timestamp() + 3600) as usize,
            iat: chrono::Utc::now().timestamp() as usize,
            iss: issuer.to_string(),
        };

        let token = encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(secret.as_bytes()),
        )
        .unwrap();

        let result = validate_jwt(&token, secret, issuer);
        assert!(result.is_ok());
        let decoded = result.unwrap();
        assert_eq!(decoded.sub, "user123");
        assert_eq!(decoded.org_id, "org456");
    }

    #[test]
    fn test_validate_jwt_expired_token() {
        let secret = "test-secret";
        let issuer = "test-issuer";

        let claims = Claims {
            sub: "user123".to_string(),
            org_id: "org456".to_string(),
            role: 1,
            exp: (chrono::Utc::now().timestamp() - 3600) as usize, // Expired
            iat: chrono::Utc::now().timestamp() as usize,
            iss: issuer.to_string(),
        };

        let token = encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(secret.as_bytes()),
        )
        .unwrap();

        let result = validate_jwt(&token, secret, issuer);
        assert!(result.is_err());
    }

    #[test]
    fn test_admin_role_check() {
        // SYSTEM_ADMIN = 1 should pass admin check
        assert_eq!(ROLE_SYSTEM_ADMIN, 1);
    }
}
