use axum::{
    extract::{State, Path},
    Json,
    http::StatusCode,
    response::IntoResponse,
};
use serde_json::json;

use super::{AppState, routes::*};
use crate::inference::device::get_device_info;

// Health check
pub async fn health_check() -> impl IntoResponse {
    Json(json!({
        "status": "healthy",
        "timestamp": chrono::Utc::now().to_rfc3339()
    }))
}

// Single prediction
pub async fn predict(
    State(state): State<AppState>,
    Json(request): Json<PredictRequest>,
) -> impl IntoResponse {
    let start = std::time::Instant::now();
    
    tracing::info!("Prediction request received");
    
    // Switch model if specified
    if let Some(model_name) = &request.model {
        if let Err(e) = state.model_manager.switch_model(model_name).await {
            return (
                StatusCode::BAD_REQUEST,
                Json(PredictResponse {
                    success: false,
                    result: None,
                    error: Some(format!("Failed to switch model: {}", e)),
                })
            );
        }
    }
    
    // Run inference
    match state.inference_engine.infer_single(&request.text).await {
        Ok(result) => {
            let latency = start.elapsed().as_millis();
            state.metrics.record_inference(latency as u64).await;
            
            (
                StatusCode::OK,
                Json(PredictResponse {
                    success: true,
                    result: Some(result),
                    error: None,
                })
            )
        }
        Err(e) => {
            tracing::error!("Inference failed: {}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(PredictResponse {
                    success: false,
                    result: None,
                    error: Some(e.to_string()),
                })
            )
        }
    }
}

// Batch prediction
pub async fn predict_batch(
    State(state): State<AppState>,
    Json(request): Json<BatchPredictRequest>,
) -> impl IntoResponse {
    let start = std::time::Instant::now();
    
    tracing::info!("Batch prediction request received: {} items", request.texts.len());
    
    if request.texts.is_empty() {
        return (
            StatusCode::BAD_REQUEST,
            Json(BatchPredictResponse {
                success: false,
                results: None,
                error: Some("Empty batch".to_string()),
            })
        );
    }
    
    match state.inference_engine.infer_batch(request.texts).await {
        Ok(results) => {
            let latency = start.elapsed().as_millis();
            state.metrics.record_batch_inference(results.len() as u64, latency as u64).await;
            
            (
                StatusCode::OK,
                Json(BatchPredictResponse {
                    success: true,
                    results: Some(results),
                    error: None,
                })
            )
        }
        Err(e) => {
            tracing::error!("Batch inference failed: {}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(BatchPredictResponse {
                    success: false,
                    results: None,
                    error: Some(e.to_string()),
                })
            )
        }
    }
}

// List available models
pub async fn list_models(State(state): State<AppState>) -> impl IntoResponse {
    let models = state.model_manager.list_models().await;
    Json(json!({
        "models": models
    }))
}

// Get active model
pub async fn get_active_model(State(state): State<AppState>) -> impl IntoResponse {
    match state.model_manager.get_active_model().await {
        Some(model) => Json(json!({
            "active_model": model
        })),
        None => Json(json!({
            "active_model": null,
            "message": "No active model"
        }))
    }
}

// Set active model
pub async fn set_active_model(
    State(state): State<AppState>,
    Path(name): Path<String>,
) -> impl IntoResponse {
    match state.model_manager.switch_model(&name).await {
        Ok(_) => (
            StatusCode::OK,
            Json(json!({
                "success": true,
                "message": format!("Model switched to: {}", name)
            }))
        ),
        Err(e) => (
            StatusCode::BAD_REQUEST,
            Json(json!({
                "success": false,
                "error": e.to_string()
            }))
        )
    }
}

// Get model statistics
pub async fn get_model_stats(State(state): State<AppState>) -> impl IntoResponse {
    let stats = state.model_manager.registry.get_all_stats().await;
    Json(json!({
        "model_stats": stats
    }))
}

// System information
pub async fn system_info(State(state): State<AppState>) -> impl IntoResponse {
    let device_info = get_device_info();
    
    Json(json!({
        "version": env!("CARGO_PKG_VERSION"),
        "device": device_info,
        "config": {
            "batch_size": state.config.inference.batch_size,
            "max_length": state.config.inference.max_length,
            "cache_enabled": state.config.cache.enable,
        }
    }))
}

// Get metrics
pub async fn get_metrics(State(state): State<AppState>) -> impl IntoResponse {
    let metrics = state.metrics.get_summary().await;
    Json(metrics)
}
