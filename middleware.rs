use axum::{
    body::Body,
    http::{Request, StatusCode},
    middleware::Next,
    response::Response,
};
use std::time::Instant;

pub async fn logging_middleware(
    req: Request<Body>,
    next: Next,
) -> Result<Response, StatusCode> {
    let start = Instant::now();
    let method = req.method().clone();
    let uri = req.uri().clone();
    
    let response = next.run(req).await;
    
    let latency = start.elapsed();
    let status = response.status();
    
    tracing::info!(
        method = %method,
        uri = %uri,
        status = %status,
        latency_ms = latency.as_millis(),
        "Request processed"
    );
    
    Ok(response)
}

pub async fn request_id_middleware(
    mut req: Request<Body>,
    next: Next,
) -> Response {
    let request_id = uuid::Uuid::new_v4().to_string();
    req.headers_mut().insert(
        "x-request-id",
        request_id.parse().unwrap(),
    );
    
    next.run(req).await
}
