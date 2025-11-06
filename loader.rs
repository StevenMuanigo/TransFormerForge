use anyhow::{Result, Context};
use std::path::{Path, PathBuf};
use tokio::fs;
use hf_hub::{api::sync::Api, Repo, RepoType};
use tracing::{info, warn, error};

pub async fn load_model(
    model_name: &str,
    cache_dir: &Path,
    auto_download: bool,
) -> Result<PathBuf> {
    let model_path = cache_dir.join(model_name);
    
    // Check if model exists in cache
    if model_path.exists() {
        info!("Model found in cache: {:?}", model_path);
        return Ok(model_path);
    }
    
    if !auto_download {
        anyhow::bail!("Model not found and auto_download is disabled");
    }
    
    info!("Downloading model from Hugging Face: {}", model_name);
    download_model_from_hf(model_name, &model_path).await?;
    
    Ok(model_path)
}

async fn download_model_from_hf(model_name: &str, dest_path: &Path) -> Result<()> {
    // Create cache directory
    fs::create_dir_all(dest_path).await
        .context("Failed to create cache directory")?;
    
    // Download using hf-hub
    let api = Api::new()
        .context("Failed to initialize Hugging Face API")?;
    
    let repo = api.repo(Repo::new(
        model_name.to_string(),
        RepoType::Model,
    ));
    
    info!("Downloading model files for: {}", model_name);
    
    // Download required files
    let files_to_download = vec![
        "config.json",
        "model.safetensors",
        "tokenizer.json",
        "tokenizer_config.json",
        "vocab.txt",
    ];
    
    for file in files_to_download {
        match repo.get(file) {
            Ok(path) => {
                let dest = dest_path.join(file);
                fs::copy(&path, &dest).await
                    .with_context(|| format!("Failed to copy {}", file))?;
                info!("Downloaded: {}", file);
            }
            Err(e) => {
                warn!("Optional file {} not found: {}", file, e);
            }
        }
    }
    
    info!("Model download completed: {}", model_name);
    Ok(())
}

pub fn get_model_info(model_path: &Path) -> Result<ModelMetadata> {
    let config_path = model_path.join("config.json");
    
    if !config_path.exists() {
        anyhow::bail!("Model config not found");
    }
    
    let config_content = std::fs::read_to_string(&config_path)?;
    let metadata: ModelMetadata = serde_json::from_str(&config_content)?;
    
    Ok(metadata)
}

#[derive(Debug, serde::Deserialize)]
pub struct ModelMetadata {
    pub model_type: Option<String>,
    pub hidden_size: Option<usize>,
    pub num_attention_heads: Option<usize>,
    pub num_hidden_layers: Option<usize>,
}
use anyhow::{Result, Context};
use std::path::{Path, PathBuf};
use tokio::fs;
use hf_hub::{api::sync::Api, Repo, RepoType};
use tracing::{info, warn, error};

pub async fn load_model(
    model_name: &str,
    cache_dir: &Path,
    auto_download: bool,
) -> Result<PathBuf> {
    let model_path = cache_dir.join(model_name);
    
    // Check if model exists in cache
    if model_path.exists() {
        info!("Model found in cache: {:?}", model_path);
        return Ok(model_path);
    }
    
    if !auto_download {
        anyhow::bail!("Model not found and auto_download is disabled");
    }
    
    info!("Downloading model from Hugging Face: {}", model_name);
    download_model_from_hf(model_name, &model_path).await?;
    
    Ok(model_path)
}

async fn download_model_from_hf(model_name: &str, dest_path: &Path) -> Result<()> {
    // Create cache directory
    fs::create_dir_all(dest_path).await
        .context("Failed to create cache directory")?;
    
    // Download using hf-hub
    let api = Api::new()
        .context("Failed to initialize Hugging Face API")?;
    
    let repo = api.repo(Repo::new(
        model_name.to_string(),
        RepoType::Model,
    ));
    
    info!("Downloading model files for: {}", model_name);
    
    // Download required files
    let files_to_download = vec![
        "config.json",
        "model.safetensors",
        "tokenizer.json",
        "tokenizer_config.json",
        "vocab.txt",
    ];
    
    for file in files_to_download {
        match repo.get(file) {
            Ok(path) => {
                let dest = dest_path.join(file);
                fs::copy(&path, &dest).await
                    .with_context(|| format!("Failed to copy {}", file))?;
                info!("Downloaded: {}", file);
            }
            Err(e) => {
                warn!("Optional file {} not found: {}", file, e);
            }
        }
    }
    
    info!("Model download completed: {}", model_name);
    Ok(())
}

pub fn get_model_info(model_path: &Path) -> Result<ModelMetadata> {
    let config_path = model_path.join("config.json");
    
    if !config_path.exists() {
        anyhow::bail!("Model config not found");
    }
    
    let config_content = std::fs::read_to_string(&config_path)?;
    let metadata: ModelMetadata = serde_json::from_str(&config_content)?;
    
    Ok(metadata)
}

#[derive(Debug, serde::Deserialize)]
pub struct ModelMetadata {
    pub model_type: Option<String>,
    pub hidden_size: Option<usize>,
    pub num_attention_heads: Option<usize>,
    pub num_hidden_layers: Option<usize>,
}
