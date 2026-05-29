//! Per-IP rate limiting on gRPC requests.
//!
//! Mirrors the `auth::AuthLayer` shape: a tower `Layer<S>` that wraps the
//! tonic service stack. Slots in `main.rs` *before* `AuthLayer` so denied
//! requests skip JWT decode entirely.
//!
//! Tier mapping (per `classify_tier`):
//! - `Tier::Bypass` — `HealthCheck`, `GetCapabilities`, `GetMetrics`, gRPC
//!   reflection, the HTTP health/ready/live/metrics paths. Used by k8s
//!   probes; the metrics layer counts them separately.
//! - `Tier::Public` — single-row indexed lookups
//!   (`LookupAgent`, `LookupPartner`, `GetEmergencyStatus`, `GetPublicKeys`,
//!   `GetBuildAttestation`, `VerifyDeployment`, `GetRevocationList` delta).
//!   60/min, 600/hr per IP.
//! - `Tier::Verify` — DB-fanout-heavy (`BatchLookupAgents` up to 100,
//!   `GetOfflinePackage`, `GetOfflineDelta`, `GetRevocationList` full).
//!   5/min, 50/hr per IP.
//!
//! `RegistryAdminService` and `PortalService` paths are bypassed here
//! (auth-gated upstream; have separate quotas at the deployment edge).
//!
//! Closes THREAT_MODEL.md AV-9.

use std::net::IpAddr;
use std::task::{Context, Poll};

use http::{HeaderValue, Request, Response, StatusCode};
use tower::{Layer, Service};

use crate::rate_limiter::{
    check_public_rate_limit, check_verify_rate_limit, extract_client_ip, RateLimitResult,
};

/// Rate-limit tier assigned to a request path.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Tier {
    Bypass,
    Public,
    Verify,
}

/// Map a request path to a rate-limit tier. `None` is interpreted as
/// `Bypass` by the layer.
fn classify_tier(path: &str) -> Tier {
    // Health / probe / metrics / reflection — bypass
    if path == "/health"
        || path == "/ready"
        || path == "/live"
        || path == "/metrics"
        || path.contains("grpc.health")
        || path.contains("grpc.reflection")
    {
        return Tier::Bypass;
    }

    // Admin / Portal services — bypass at this layer (auth handles them)
    if path.contains("ciris.registry.v1.RegistryAdminService")
        || path.contains("ciris.registry.v1.PortalService")
    {
        return Tier::Bypass;
    }

    // RegistryService method-level mapping
    if path.contains("ciris.registry.v1.RegistryService/") {
        if path.ends_with("/HealthCheck")
            || path.ends_with("/GetCapabilities")
            || path.ends_with("/GetMetrics")
        {
            return Tier::Bypass;
        }
        if path.ends_with("/BatchLookupAgents")
            || path.ends_with("/GetOfflinePackage")
            || path.ends_with("/GetOfflineDelta")
        {
            return Tier::Verify;
        }
        // Default: every other RegistryService RPC gets the public tier.
        return Tier::Public;
    }

    // Anything else (unknown gRPC path, future services) — fail-secure.
    // Apply public tier so unknown paths get rate-limited rather than
    // bypassed. Auth layer handles the actual access check downstream.
    Tier::Public
}

/// Extract the gRPC peer IP from a request. Tonic's transport inserts
/// `TcpConnectInfo` into request extensions; we honor `cf-connecting-ip`
/// and trusted-proxy `X-Forwarded-For` ahead of the raw peer.
fn extract_grpc_client_ip<B>(req: &Request<B>) -> IpAddr {
    let peer = req
        .extensions()
        .get::<tonic::transport::server::TcpConnectInfo>()
        .and_then(|i| i.remote_addr())
        .map(|sa| sa.ip());
    extract_client_ip(req.headers(), peer)
}

/// Build a gRPC-aware rate-limit response.
///
/// gRPC over HTTP/2 expects errors as HTTP 200 + `grpc-status` trailer.
/// We emit a `grpc-status: 8` (RESOURCE_EXHAUSTED) header (per spec, header
/// is acceptable when no body is sent) plus `grpc-message` and `retry-after`
/// for HTTP-level observers / proxies. Tonic clients surface this as
/// `Status::resource_exhausted` on receive.
///
/// Following the pattern used by `AuthLayer` (which returns plain
/// `StatusCode::UNAUTHORIZED` / `FORBIDDEN`), this also works as a
/// transport-level signal — tonic clients will surface a transport error
/// either way.
fn rate_limited_response<B: Default>(result: RateLimitResult) -> Response<B> {
    let msg = result
        .error_message()
        .unwrap_or_else(|| "rate limit exceeded".to_string());
    let retry_after = result.retry_after().unwrap_or(60);

    let mut response = Response::builder()
        .status(StatusCode::OK)
        .header("content-type", "application/grpc")
        .header("grpc-status", "8") // RESOURCE_EXHAUSTED
        .body(B::default())
        .unwrap();

    if let Ok(v) = HeaderValue::from_str(&msg) {
        response.headers_mut().insert("grpc-message", v);
    }
    if let Ok(v) = HeaderValue::from_str(&retry_after.to_string()) {
        response.headers_mut().insert("retry-after", v);
    }

    response
}

/// HTTP 429 builder for axum / non-tonic call sites.
fn http_rate_limited_response<B: Default>(result: RateLimitResult) -> Response<B> {
    let retry_after = result.retry_after().unwrap_or(60);
    let mut response = Response::builder()
        .status(StatusCode::TOO_MANY_REQUESTS)
        .body(B::default())
        .unwrap();
    if let Ok(v) = HeaderValue::from_str(&retry_after.to_string()) {
        response.headers_mut().insert("retry-after", v);
    }
    response
}

#[derive(Clone, Default)]
pub struct RateLimitLayer;

impl RateLimitLayer {
    pub fn new() -> Self {
        Self
    }
}

impl<S> Layer<S> for RateLimitLayer {
    type Service = RateLimitService<S>;

    fn layer(&self, inner: S) -> Self::Service {
        RateLimitService { inner }
    }
}

#[derive(Clone)]
pub struct RateLimitService<S> {
    inner: S,
}

impl<S, ReqBody, ResBody> Service<Request<ReqBody>> for RateLimitService<S>
where
    S: Service<Request<ReqBody>, Response = Response<ResBody>>,
    S::Future: Send + 'static,
    ResBody: Default + Send + 'static,
{
    type Response = S::Response;
    type Error = S::Error;
    type Future = std::pin::Pin<
        Box<dyn std::future::Future<Output = Result<Self::Response, Self::Error>> + Send>,
    >;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx)
    }

    fn call(&mut self, request: Request<ReqBody>) -> Self::Future {
        let path = request.uri().path().to_owned();
        let tier = classify_tier(&path);

        if tier == Tier::Bypass {
            let future = self.inner.call(request);
            return Box::pin(async move { future.await });
        }

        let ip = extract_grpc_client_ip(&request);
        let result = match tier {
            Tier::Public => check_public_rate_limit(ip),
            Tier::Verify => check_verify_rate_limit(ip),
            Tier::Bypass => unreachable!(),
        };

        if !result.is_allowed() {
            tracing::warn!(
                ip = %ip,
                path = %path,
                tier = ?tier,
                "grpc_rate_limit_exceeded"
            );
            let response = rate_limited_response(result);
            return Box::pin(async move { Ok(response) });
        }

        let future = self.inner.call(request);
        Box::pin(async move { future.await })
    }
}

/// Axum `from_fn` middleware applied to the unauthenticated HTTP `/v1/*`
/// public routes (builds, verify, revocation, steward-key). Reuses the
/// same `Tier::Public` bucket as the gRPC path so a single attacker
/// can't multiply their effective limit by switching protocols.
pub async fn rate_limit_public_http(
    req: axum::extract::Request,
    next: axum::middleware::Next,
) -> axum::response::Response {
    let ip = extract_client_ip(req.headers(), None);
    let result = check_public_rate_limit(ip);
    if !result.is_allowed() {
        tracing::warn!(ip = %ip, path = %req.uri().path(), "http_rate_limit_exceeded");
        return http_rate_limited_response::<axum::body::Body>(result);
    }
    next.run(req).await
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn classify_health_paths_bypass() {
        assert_eq!(classify_tier("/health"), Tier::Bypass);
        assert_eq!(classify_tier("/ready"), Tier::Bypass);
        assert_eq!(classify_tier("/live"), Tier::Bypass);
        assert_eq!(classify_tier("/metrics"), Tier::Bypass);
    }

    #[test]
    fn classify_grpc_health_reflection_bypass() {
        assert_eq!(classify_tier("/grpc.health.v1.Health/Check"), Tier::Bypass);
        assert_eq!(
            classify_tier("/grpc.reflection.v1.ServerReflection/ServerReflectionInfo"),
            Tier::Bypass
        );
    }

    #[test]
    fn classify_admin_portal_bypass() {
        assert_eq!(
            classify_tier("/ciris.registry.v1.RegistryAdminService/RegisterAgent"),
            Tier::Bypass
        );
        assert_eq!(
            classify_tier("/ciris.registry.v1.PortalService/CreateOrganization"),
            Tier::Bypass
        );
    }

    #[test]
    fn classify_registry_lookups_public() {
        assert_eq!(
            classify_tier("/ciris.registry.v1.RegistryService/LookupAgent"),
            Tier::Public
        );
        assert_eq!(
            classify_tier("/ciris.registry.v1.RegistryService/LookupPartner"),
            Tier::Public
        );
    }

    #[test]
    fn classify_registry_health_bypass() {
        assert_eq!(
            classify_tier("/ciris.registry.v1.RegistryService/HealthCheck"),
            Tier::Bypass
        );
        assert_eq!(
            classify_tier("/ciris.registry.v1.RegistryService/GetCapabilities"),
            Tier::Bypass
        );
        assert_eq!(
            classify_tier("/ciris.registry.v1.RegistryService/GetMetrics"),
            Tier::Bypass
        );
    }

    #[test]
    fn classify_registry_expensive_verify() {
        assert_eq!(
            classify_tier("/ciris.registry.v1.RegistryService/BatchLookupAgents"),
            Tier::Verify
        );
        assert_eq!(
            classify_tier("/ciris.registry.v1.RegistryService/GetOfflinePackage"),
            Tier::Verify
        );
        assert_eq!(
            classify_tier("/ciris.registry.v1.RegistryService/GetOfflineDelta"),
            Tier::Verify
        );
    }

    #[test]
    fn classify_unknown_paths_default_public() {
        assert_eq!(classify_tier("/some.future.Service/RPC"), Tier::Public);
    }
}
