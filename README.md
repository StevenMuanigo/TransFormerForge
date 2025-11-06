
# TransformerForge

**Production-Grade AI Inference Engine with Hugging Face Transformers**

A high-performance, scalable inference server built in Rust for deploying transformer models from Hugging Face Hub.

## Features

### Core Features
- ✅ **Dynamic Model Loading** - Load models from Hugging Face Hub on-the-fly
- ✅ **Hot-Swap Models** - Switch between models without server restart
- ✅ **Batch Processing** - Efficient batch inference with configurable size
- ✅ **GPU/CPU Auto-Detection** - Automatic device selection (CUDA, Metal, CPU)
- ✅ **Model Caching** - LRU cache with TTL for faster repeated inference
- ✅ **REST API** - Production-ready HTTP endpoints with Axum

### Engineering Features
- ⚡ **Async Processing** - Tokio-based async runtime for high concurrency
- 📊 **Prometheus Metrics** - Built-in metrics collection and export
- 📝 **Structured Logging** - JSON and pretty logging with tracing
- 🔧 **Configuration Management** - YAML config + environment variables
- 🧪 **Benchmarking** - Performance profiling with Criterion
- 🛡️ **Error Handling** - Comprehensive error handling with anyhow/thiserror

## Quick Start

### Prerequisites
- Rust 1.75+ ([Install Rust](https://rustup.rs))
- Optional: CUDA for GPU acceleration

### Installation

#### Windows
```batch
# Build the project
build.bat

# Run the server
run.bat
```

#### Linux/macOS
```bash
# Install dependencies
cargo build --release

# Run server
cargo run --release
```

## API Endpoints

### Inference

**Single Prediction**
```bash
POST /predict
Content-Type: application/json

{
  "text": "This product is amazing!",
  "model": "bert-base-uncased"  # optional
}
```

**Batch Prediction**
```bash
POST /predict/batch
Content-Type: application/json

{
  "texts": ["Great product!", "Not satisfied", "Excellent service"],
  "model": "bert-base-uncased"  # optional
}
```

### Model Management

**List Available Models**
```bash
GET /models
```

**Get Active Model**
```bash
GET /models/active
```

**Switch Model**
```bash
POST /models/{model_name}/activate
```

**Model Statistics**
```bash
GET /models/stats
```

### System

**Health Check**
```bash
GET /health
```

**System Information**
```bash
GET /info
```

**Metrics**
```bash
GET /metrics
```

## Configuration

Edit `config.yaml` to customize:

```yaml
server:
  host: "0.0.0.0"
  port: 8080
  workers: 4

models:
  default: "bert-base-uncased"
  cache_dir: "./models_cache"
  auto_download: true

inference:
  batch_size: 32
  max_length: 512
  device: "auto"  # auto, cpu, cuda:0
  enable_gpu: true

cache:
  enable: true
  ttl_seconds: 3600
  max_entries: 10000
```

## Performance

### Benchmarks

Run benchmarks:
```bash
cargo bench
```

### Metrics

Access Prometheus metrics at:
```
http://localhost:8080/metrics
```

Key metrics:
- `transformer_forge_total_requests` - Total inference requests
- `transformer_forge_inference_latency_ms` - Inference latency histogram
- `transformer_forge_batch_size` - Batch processing size distribution

## Architecture

```
TransformerForge/
├── src/
│   ├── config/          # Configuration management
│   ├── model/           # Model loading, caching, registry
│   ├── inference/       # Inference engine, device detection
│   ├── preprocessing/   # Text normalization, tokenization
│   ├── api/             # REST API endpoints and handlers
│   └── monitoring/      # Metrics, logging, benchmarking
├── tests/               # Integration tests
└── benches/             # Performance benchmarks
```

## Tech Stack

- **Language**: Rust 2021 Edition
- **Async Runtime**: Tokio
- **Web Framework**: Axum
- **ML Framework**: Candle, rust-bert
- **Tokenization**: tokenizers (Hugging Face)
- **Metrics**: Prometheus
- **Logging**: tracing + tracing-subscriber
- **Configuration**: config-rs, dotenv

## Advanced Features

### Model Hot-Swapping

Switch models dynamically:
```bash
curl -X POST http://localhost:8080/models/gpt2/activate
```

### Concurrent Batch Processing

Process multiple batches concurrently with semaphore-based concurrency control.

### Auto Cleanup

Automatic cache eviction based on LRU policy and TTL.

### GPU Acceleration

Automatically detects and uses CUDA/Metal if available:
```yaml
inference:
  enable_gpu: true
  device: "auto"  # Will auto-detect best device
```

## Testing

```bash
# Run all tests
cargo test

# Run with logging
RUST_LOG=debug cargo test

# Integration tests only
cargo test --test integration_test
```

## Deployment

### Docker (Coming Soon)
```bash
docker-compose up -d
```

### Cloud Platforms
- AWS ECS/EKS
- Google Cloud Run/GKE
- Azure Container Instances/AKS
- Any Kubernetes cluster

## Development

### Project Structure
- `src/main.rs` - Application entry point
- `src/config/` - Configuration loading
- `src/model/` - Model management (loader, cache, registry)
- `src/inference/` - Inference engine and device management
- `src/preprocessing/` - Text preprocessing pipeline
- `src/api/` - HTTP API layer
- `src/monitoring/` - Metrics and logging

### Adding New Models

1. Add to `config.yaml`:
```yaml
models:
  available_models:
    - name: "my-model"
      task: "classification"
      repo: "huggingface/my-model"
```

2. Activate via API:
```bash
curl -X POST http://localhost:8080/models/my-model/activate
```

## Troubleshooting

### Model Download Issues
- Check internet connection
- Verify Hugging Face Hub accessibility
- Check `models_cache/` directory permissions

### GPU Not Detected
- Verify CUDA installation
- Check `nvidia-smi` output
- Set `enable_gpu: false` in config for CPU-only mode

### High Memory Usage
- Reduce `cache.max_entries` in config
- Lower `inference.batch_size`
- Enable cache TTL cleanup

## Contributing

This is an educational project demonstrating production-grade Rust engineering practices.

## License

MIT License - For educational and research purposes.

## Acknowledgments

- Hugging Face for transformer models and tokenizers
- Candle ML framework by Hugging Face
- Rust community for excellent async ecosystem

---

**TransformerForge v1.0.0** - Where AI meets Rust performance 🦀🔥

THE ONLY CODER:STEVENMUANİGO

DEMO SİTE: https://transformeforge.netlify.app/
