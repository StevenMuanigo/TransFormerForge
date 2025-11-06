use anyhow::Result;
use std::path::PathBuf;
use std::sync::Arc;
use tokio::sync::RwLock;
use dashmap::DashMap;

pub mod loader;
pub mod cache;
pub mod registry;

use crate::config::AppConfig;

#[derive(Debug, Clone)]
pub struct ModelManager {
    pub registry: Arc<registry::ModelRegistry>,
    pub cache: Arc<cache::ModelCache>,
    pub config: Arc<AppConfig>,
}

impl ModelManager {
    pub fn new(config: Arc<AppConfig>) -> Self {
        let registry = Arc::new(registry::ModelRegistry::new());
        let cache = Arc::new(cache::ModelCache::new(
            config.cache.max_entries,
            config.cache.ttl_seconds,
        ));

        Self {
            registry,
            cache,
            config,
        }
    }

    pub async fn load_default_model(&self) -> Result<()> {
        let default_model = &self.config.models.default;
        tracing::info!("Loading default model: {}", default_model);
        
        loader::load_model(
            default_model,
            &self.config.models.cache_dir,
            self.config.models.auto_download,
        ).await?;
        
        self.registry.register_model(default_model.clone()).await;
        tracing::info!("Default model loaded successfully");
        
        Ok(())
    }

    pub async fn switch_model(&self, model_name: &str) -> Result<()> {
        tracing::info!("Switching to model: {}", model_name);
        
        if !self.registry.is_registered(model_name).await {
            loader::load_model(
                model_name,
                &self.config.models.cache_dir,
                self.config.models.auto_download,
            ).await?;
            
            self.registry.register_model(model_name.to_string()).await;
        }
        
        self.registry.set_active(model_name).await?;
        tracing::info!("Model switched successfully to: {}", model_name);
        
        Ok(())
    }

    pub async fn get_active_model(&self) -> Option<String> {
        self.registry.get_active().await
    }

    pub async fn list_models(&self) -> Vec<String> {
        self.registry.list_all().await
    }
}
