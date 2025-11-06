use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use anyhow::Result;

pub mod settings;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    pub server: ServerConfig,
    pub models: ModelsConfig,
    pub inference: InferenceConfig,
    pub preprocessing: PreprocessingConfig,
    pub monitoring: MonitoringConfig,
    pub cache: CacheConfig,
    pub performance: PerformanceConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
    pub workers: usize,
    pub request_timeout_ms: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelsConfig {
    pub default: String,
    pub cache_dir: PathBuf,
    pub auto_download: bool,
    pub available_models: Vec<ModelInfo>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelInfo {
    pub name: String,
    pub task: String,
    pub repo: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InferenceConfig {
    pub batch_size: usize,
    pub max_length: usize,
    pub device: String,
    pub num_threads: usize,
    pub enable_gpu: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PreprocessingConfig {
    pub lowercase: bool,
    pub remove_special_chars: bool,
    pub max_input_length: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MonitoringConfig {
    pub enable_metrics: bool,
    pub metrics_port: u16,
    pub log_level: String,
    pub log_format: String,
    pub log_file: PathBuf,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CacheConfig {
    pub enable: bool,
    pub ttl_seconds: u64,
    pub max_entries: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceConfig {
    pub async_workers: usize,
    pub queue_size: usize,
    pub enable_benchmarking: bool,
}

impl AppConfig {
    pub fn load() -> Result<Self> {
        dotenv::dotenv().ok();
        
        let config = config::Config::builder()
            .add_source(config::File::with_name("config"))
            .add_source(config::Environment::with_prefix("APP"))
            .build()?;
        
        let app_config: AppConfig = config.try_deserialize()?;
        
        Ok(app_config)
    }
}
