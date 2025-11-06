use axum::{
    Router,
    routing::{get, post},
};
use tower_http::{
    trace::TraceLayer,
    cors::{CorsLayer, Any},
};
use std::sync::Arc;

pub mod routes;
pub mod handlers;
pub mod middleware;

use crate::{
    config::AppConfig,
    model::ModelManager,
    inference::InferenceEngine,
    monitoring::MetricsCollector,
};

#[derive(Clone)]
pub struct AppState {
    pub config: Arc<AppConfig>,
    pub model_manager: Arc<ModelManager>,
    pub inference_engine: Arc<InferenceEngine>,
    pub metrics: Arc<MetricsCollector>,
}

pub fn create_router(state: AppState) -> Router {
    Router::new()
        // Health check
        .route("/health", get(handlers::health_check))
        
        // Inference endpoints
        .route("/predict", post(handlers::predict))
        .route("/predict/batch", post(handlers::predict_batch))
        
        // Model management
        .route("/models", get(handlers::list_models))
        .route("/models/active", get(handlers::get_active_model))
        .route("/models/:name/activate", post(handlers::set_active_model))
        .route("/models/stats", get(handlers::get_model_stats))
        
        // System info
        .route("/info", get(handlers::system_info))
        .route("/metrics", get(handlers::get_metrics))
        
        // Add state
        .with_state(state)
        
        // Middleware
        .layer(TraceLayer::new_for_http())
        .layer(
            CorsLayer::new()
                .allow_origin(Any)
                .allow_methods(Any)
                .allow_headers(Any)
        )
}
