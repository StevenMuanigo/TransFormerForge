use std::time::{Duration, Instant};
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct BenchmarkResult {
    pub total_requests: usize,
    pub total_duration: Duration,
    pub avg_latency_ms: f64,
    pub min_latency_ms: u64,
    pub max_latency_ms: u64,
    pub throughput_req_per_sec: f64,
    pub p50_latency_ms: u64,
    pub p95_latency_ms: u64,
    pub p99_latency_ms: u64,
}

pub struct Benchmark {
    latencies: Vec<u64>,
    start_time: Option<Instant>,
}

impl Benchmark {
    pub fn new() -> Self {
        Self {
            latencies: Vec::new(),
            start_time: None,
        }
    }

    pub fn start(&mut self) {
        self.start_time = Some(Instant::now());
    }

    pub fn record(&mut self, latency_ms: u64) {
        self.latencies.push(latency_ms);
    }

    pub fn finish(mut self) -> BenchmarkResult {
        let total_duration = self.start_time
            .map(|start| start.elapsed())
            .unwrap_or_default();
        
        let total_requests = self.latencies.len();
        
        if total_requests == 0 {
            return BenchmarkResult {
                total_requests: 0,
                total_duration,
                avg_latency_ms: 0.0,
                min_latency_ms: 0,
                max_latency_ms: 0,
                throughput_req_per_sec: 0.0,
                p50_latency_ms: 0,
                p95_latency_ms: 0,
                p99_latency_ms: 0,
            };
        }
        
        // Sort for percentile calculation
        self.latencies.sort_unstable();
        
        let sum: u64 = self.latencies.iter().sum();
        let avg_latency_ms = sum as f64 / total_requests as f64;
        let min_latency_ms = *self.latencies.first().unwrap();
        let max_latency_ms = *self.latencies.last().unwrap();
        
        let throughput_req_per_sec = if total_duration.as_secs_f64() > 0.0 {
            total_requests as f64 / total_duration.as_secs_f64()
        } else {
            0.0
        };
        
        let p50_latency_ms = self.percentile(50.0);
        let p95_latency_ms = self.percentile(95.0);
        let p99_latency_ms = self.percentile(99.0);
        
        BenchmarkResult {
            total_requests,
            total_duration,
            avg_latency_ms,
            min_latency_ms,
            max_latency_ms,
            throughput_req_per_sec,
            p50_latency_ms,
            p95_latency_ms,
            p99_latency_ms,
        }
    }

    fn percentile(&self, p: f64) -> u64 {
        let index = ((p / 100.0) * self.latencies.len() as f64).ceil() as usize - 1;
        self.latencies[index.min(self.latencies.len() - 1)]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_benchmark() {
        let mut bench = Benchmark::new();
        bench.start();
        
        for i in 1..=100 {
            bench.record(i);
        }
        
        let result = bench.finish();
        assert_eq!(result.total_requests, 100);
        assert!(result.avg_latency_ms > 0.0);
    }
}
