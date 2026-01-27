//! Authentication middleware (mTLS + JWT)

use std::task::{Context, Poll};

use http::{Request, Response, StatusCode};
use jsonwebtoken::{decode, DecodingKey, Validation, Algorithm};
use serde::{Deserialize, Serialize};
use tower::{Layer, Service};

use crate::config::AuthSettings;

/// JWT claims structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Claims {
    /// Subject (user ID)
    pub sub: String,
    /// Organization ID
    #[serde(default)]
    pub org_id: String,
    /// User role (as integer)
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
        let path = request.uri().path();

        // Skip auth for health check and reflection endpoints
        if path.contains("grpc.health")
            || path.contains("grpc.reflection")
            || path == "/health"
            || path == "/ready"
            || path == "/live"
            || path == "/metrics"
        {
            let future = self.inner.call(request);
            return Box::pin(async move { future.await });
        }

        // Extract and validate JWT if present
        let auth_header = request
            .headers()
            .get("authorization")
            .and_then(|h| h.to_str().ok());

        if let Some(header) = auth_header {
            if header.starts_with("Bearer ") {
                let token = &header[7..];

                match validate_jwt(token, &self.settings.jwt_secret, &self.settings.jwt_issuer) {
                    Ok(claims) => {
                        // Store validated claims in request extensions for use by handlers
                        request.extensions_mut().insert(claims);
                    }
                    Err(e) => {
                        tracing::warn!("JWT validation failed: {}", e);
                        return Box::pin(async move {
                            Ok(Response::builder()
                                .status(StatusCode::UNAUTHORIZED)
                                .body(ResBody::default())
                                .unwrap())
                        });
                    }
                }
            }
        }

        // mTLS validation would go here if mtls_enabled is true
        // This requires extracting the client certificate from the TLS connection
        // which requires server-side TLS configuration changes

        let future = self.inner.call(request);
        Box::pin(async move { future.await })
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
}
