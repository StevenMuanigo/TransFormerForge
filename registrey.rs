use std::sync::Arc;
use tokio::sync::RwLock;
use std::collections::HashMap;
use anyhow::{Result, bail};

pub struct ModelRegistry {
    models: Arc<RwLock<HashMap<String, ModelEntry>>>,
    active_model: Arc<RwLock<Option<String>>>,
}

#[derive(Debug, Clone)]
struct ModelEntry {
    name: String,
    load_time: chrono::DateTime<chrono::Utc>,
    inference_count: u64,
}

impl ModelRegistry {
    pub fn new() -> Self {
        Self {
            models: Arc::new(RwLock::new(HashMap::new())),
            active_model: Arc::new(RwLock::new(None)),
        }
    }

    pub async fn register_model(&self, name: String) {
        let mut models = self.models.write().await;
        models.insert(
            name.clone(),
            ModelEntry {
                name: name.clone(),
                load_time: chrono::Utc::now(),
                inference_count: 0,
            },
        );
        
        tracing::info!("Model registered: {}", name);
    }

    pub async fn is_registered(&self, name: &str) -> bool {
        let models = self.models.read().await;
        models.contains_key(name)
    }

    pub async fn set_active(&self, name: &str) -> Result<()> {
        let models = self.models.read().await;
        
        if !models.contains_key(name) {
            bail!("Model not registered: {}", name);
        }
        
        let mut active = self.active_model.write().await;
        *active = Some(name.to_string());
        
        Ok(())
    }

    pub async fn get_active(&self) -> Option<String> {
        let active = self.active_model.read().await;
        active.clone()
    }

    pub async fn list_all(&self) -> Vec<String> {
        let models = self.models.read().await;
        models.keys().cloned().collect()
    }

    pub async fn increment_inference_count(&self, name: &str) {
        let mut models = self.models.write().await;
        if let Some(entry) = models.get_mut(name) {
            entry.inference_count += 1;
        }
    }

    pub async fn get_stats(&self, name: &str) -> Option<ModelStats> {
        let models = self.models.read().await;
        models.get(name).map(|entry| ModelStats {
            name: entry.name.clone(),
            load_time: entry.load_time,
            inference_count: entry.inference_count,
        })
    }

    pub async fn get_all_stats(&self) -> Vec<ModelStats> {
        let models = self.models.read().await;
        models
            .values()
            .map(|entry| ModelStats {
                name: entry.name.clone(),
                load_time: entry.load_time,
                inference_count: entry.inference_count,
            })
            .collect()
    }
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct ModelStats {
    pub name: String,
    pub load_time: chrono::DateTime<chrono::Utc>,
    pub inference_count: u64,
}
