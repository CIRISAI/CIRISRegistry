//! Metrics collection middleware

use std::task::{Context, Poll};

use http::{Request, Response};
use metrics::{counter, histogram};
use tower::{Layer, Service};

#[derive(Clone, Copy)]
pub struct MetricsLayer;

impl<S> Layer<S> for MetricsLayer {
    type Service = MetricsService<S>;

    fn layer(&self, inner: S) -> Self::Service {
        MetricsService { inner }
    }
}

#[derive(Clone)]
pub struct MetricsService<S> {
    inner: S,
}

impl<S, ReqBody, ResBody> Service<Request<ReqBody>> for MetricsService<S>
where
    S: Service<Request<ReqBody>, Response = Response<ResBody>>,
    S::Future: Send + 'static,
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
        let start = std::time::Instant::now();

        // Extract method name from gRPC path (e.g., "/ciris.registry.v1.RegistryService/HealthCheck")
        let path = request.uri().path().to_string();
        let method = extract_grpc_method(&path);

        let future = self.inner.call(request);

        Box::pin(async move {
            let result = future.await;

            let elapsed = start.elapsed();
            let status = match &result {
                Ok(_) => "ok",
                Err(_) => "error",
            };

            // Record request count
            counter!(
                "ciris_registry_grpc_requests_total",
                "method" => method.clone(),
                "status" => status.to_string()
            )
            .increment(1);

            // Record request duration
            histogram!(
                "ciris_registry_grpc_request_duration_seconds",
                "method" => method.clone()
            )
            .record(elapsed.as_secs_f64());

            // Track slow requests (> 1 second)
            if elapsed.as_secs() >= 1 {
                counter!(
                    "ciris_registry_slow_requests_total",
                    "method" => method
                )
                .increment(1);
            }

            result
        })
    }
}

/// Extract gRPC method name from path
fn extract_grpc_method(path: &str) -> String {
    // Path format: /package.Service/Method
    // Extract just the Method part for cleaner metrics
    if let Some(last_slash) = path.rfind('/') {
        path[last_slash + 1..].to_string()
    } else {
        path.to_string()
    }
}
