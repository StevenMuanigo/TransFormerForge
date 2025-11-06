use std::sync::Arc;
use tokio::sync::RwLock;
use serde::{Serialize, Deserialize};
use prometheus::{IntCounter, Histogram, Registry, Encoder, TextEncoder};

pub mod logger;
pub mod benchmark;

#[derive(Clone)]
pub struct MetricsCollector {
    pub total_requests: Arc<IntCounter>,
    pub inference_latency: Arc<Histogram>,
    pub batch_size: Arc<Histogram>,
    stats: Arc<RwLock<MetricsStats>>,
    registry: Arc<Registry>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct MetricsStats {
    total_inferences: u64,
    total_batch_inferences: u64,
    avg_latency_ms: f64,
    max_latency_ms: u64,
    min_latency_ms: u64,
}

impl MetricsCollector {
    pub fn new() -> Self {
        let registry = Registry::new();
        
        let total_requests = IntCounter::new(
            "transformer_forge_total_requests",
            "Total number of inference requests"
        ).unwrap();
        
        let inference_latency = Histogram::new(
            "transformer_forge_inference_latency_ms",
            "Inference latency in milliseconds"
        ).unwrap();
        
        let batch_size = Histogram::new(
            "transformer_forge_batch_size",
            "Batch processing size"
        ).unwrap();
        
        registry.register(Box::new(total_requests.clone())).unwrap();
        registry.register(Box::new(inference_latency.clone())).unwrap();
        registry.register(Box::new(batch_size.clone())).unwrap();
        
        Self {
            total_requests: Arc::new(total_requests),
            inference_latency: Arc::new(inference_latency),
            batch_size: Arc::new(batch_size),
            stats: Arc::new(RwLock::new(MetricsStats {
                total_inferences: 0,
                total_batch_inferences: 0,
                avg_latency_ms: 0.0,
                max_latency_ms: 0,
                min_latency_ms: u64::MAX,
            })),
            registry: Arc::new(registry),
        }
    }

    pub async fn record_inference(&self, latency_ms: u64) {
        self.total_requests.inc();
        self.inference_latency.observe(latency_ms as f64);
        
        let mut stats = self.stats.write().await;
        stats.total_inferences += 1;
        stats.max_latency_ms = stats.max_latency_ms.max(latency_ms);
        stats.min_latency_ms = stats.min_latency_ms.min(latency_ms);
        
        // Update running average
        let total = stats.total_inferences as f64;
        stats.avg_latency_ms = (stats.avg_latency_ms * (total - 1.0) + latency_ms as f64) / total;
    }

    pub async fn record_batch_inference(&self, batch_size: u64, latency_ms: u64) {
        self.batch_size.observe(batch_size as f64);
        self.record_inference(latency_ms).await;
        
        let mut stats = self.stats.write().await;
        stats.total_batch_inferences += 1;
    }

    pub async fn get_summary(&self) -> MetricsSummary {
        let stats = self.stats.read().await;
        
        MetricsSummary {
            total_inferences: stats.total_inferences,
            total_batch_inferences: stats.total_batch_inferences,
            avg_latency_ms: stats.avg_latency_ms,
            max_latency_ms: stats.max_latency_ms,
            min_latency_ms: if stats.min_latency_ms == u64::MAX { 0 } else { stats.min_latency_ms },
            timestamp: chrono::Utc::now(),
        }
    }

    pub fn export_prometheus(&self) -> String {
        let encoder = TextEncoder::new();
        let metric_families = self.registry.gather();
        let mut buffer = vec![];
        encoder.encode(&metric_families, &mut buffer).unwrap();
        String::from_utf8(buffer).unwrap()
    }
}

#[derive(Debug, Serialize)]
pub struct MetricsSummary {
    pub total_inferences: u64,
    pub total_batch_inferences: u64,
    pub avg_latency_ms: f64,
    pub max_latency_ms: u64,
    pub min_latency_ms: u64,
    pub timestamp: chrono::DateTime<chrono::Utc>,
}
