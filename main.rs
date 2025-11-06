mod config;
mod model;
mod inference;
mod preprocessing;
mod api;
mod monitoring;

use anyhow::Result;
use std::sync::Arc;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> Result<()> {
    println!(" TransformerForge v{} - AI Inference Engine", env!("CARGO_PKG_VERSION"));
    println!("======================================================");
    
    // Load configuration
    let config = Arc::new(config::AppConfig::load()?);
    println!(" Configuration loaded");
    
    // Initialize logging
    monitoring::logger::init_logger(&config);
    tracing::info!("TransformerForge starting up...");
    
    // Initialize metrics collector
    let metrics = Arc::new(monitoring::MetricsCollector::new());
    tracing::info!("Metrics collector initialized");
    
    // Initialize model manager
    let model_manager = Arc::new(model::ModelManager::new(config.clone()));
    tracing::info!("Model manager initialized");
    
    // Load default model
    model_manager.load_default_model().await?;
    tracing::info!("Default model loaded successfully");
    
    // Initialize inference engine
    let inference_engine = Arc::new(
        inference::InferenceEngine::new(config.clone(), model_manager.clone()).await?
    );
    tracing::info!("Inference engine initialized");
    
    // Create application state
    let app_state = api::AppState {
        config: config.clone(),
        model_manager,
        inference_engine,
        metrics,
    };
    
    // Create router
    let app = api::create_router(app_state);
    
    // Server address
    let addr = format!("{}:{}", config.server.host, config.server.port);
    let listener = TcpListener::bind(&addr).await?;
    
    println!("======================================================");
    println!(" Server running on http://{}", addr);
    println!(" Metrics available at http://{}:{}/metrics", 
        config.server.host, 
        config.monitoring.metrics_port
    );
    println!("======================================================");
    println!("\n💡 Available endpoints:");
    println!("  POST /predict              - Single inference");
    println!("  POST /predict/batch        - Batch inference");
    println!("  GET  /models               - List models");
    println!("  GET  /models/active        - Get active model");
    println!("  POST /models/:name/activate - Switch model");
    println!("  GET  /info                 - System info");
    println!("  GET  /metrics              - Metrics summary");
    println!("  GET  /health               - Health check");
    println!("\n Ready to process requests!\n");
    
    tracing::info!("Server started on {}", addr);
    
    // Start server
    axum::serve(listener, app)
        .await?;
    
    Ok(())
}
