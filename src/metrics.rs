use axum::{
    body::Body,
    http::Request,
    response::Response,
};
use metrics::{counter, histogram};
use std::time::Instant;
use tower::{Layer, Service};

#[derive(Clone)]
pub struct MetricsLayer;

impl MetricsLayer {
    pub fn new() -> Self {
        MetricsLayer
    }
}

impl<S> Layer<S> for MetricsLayer {
    type Service = MetricsMiddleware<S>;

    fn layer(&self, service: S) -> Self::Service {
        MetricsMiddleware { service }
    }
}

#[derive(Clone)]
pub struct MetricsMiddleware<S> {
    service: S,
}

impl<S> Service<Request<Body>> for MetricsMiddleware<S>
where
    S: Service<Request<Body>, Response = Response> + Send + 'static,
    S::Future: Send + 'static,
{
    type Response = S::Response;
    type Error = S::Error;
    type Future = futures::future::BoxFuture<'static, Result<Self::Response, Self::Error>>;

    fn poll_ready(&mut self, cx: &mut std::task::Context<'_>) -> std::task::Poll<Result<(), Self::Error>> {
        self.service.poll_ready(cx)
    }

    fn call(&mut self, req: Request<Body>) -> Self::Future {
        let start = Instant::now();
        let method = req.method().to_string();
        let path = req.uri().path().to_string();

        counter!("http_requests_total", "method" => method.clone(), "path" => path.clone()).increment(1);

        let future = self.service.call(req);

        Box::pin(async move {
            let response = future.await?;
            let duration = start.elapsed();

            histogram!("http_request_duration_seconds", "method" => method, "path" => path)
                .record(duration.as_secs_f64());

            Ok(response)
        })
    }
}