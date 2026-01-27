//! Request tracing middleware

use std::task::{Context, Poll};

use http::{Request, Response};
use tower::{Layer, Service};
use tracing::{info_span, Instrument};

#[derive(Clone, Copy)]
pub struct TracingLayer;

impl<S> Layer<S> for TracingLayer {
    type Service = TracingService<S>;

    fn layer(&self, inner: S) -> Self::Service {
        TracingService { inner }
    }
}

#[derive(Clone)]
pub struct TracingService<S> {
    inner: S,
}

impl<S, ReqBody, ResBody> Service<Request<ReqBody>> for TracingService<S>
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
        let method = request.method().clone();
        let uri = request.uri().clone();

        let span = info_span!(
            "grpc_request",
            method = %method,
            uri = %uri,
        );

        let future = self.inner.call(request);

        Box::pin(async move { future.await }.instrument(span))
    }
}
