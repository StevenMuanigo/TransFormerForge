use tokio::sync::Semaphore;
use std::sync::Arc;
use anyhow::Result;

pub struct BatchProcessor {
    semaphore: Arc<Semaphore>,
    batch_size: usize,
}

impl BatchProcessor {
    pub fn new(batch_size: usize, max_concurrent: usize) -> Self {
        Self {
            semaphore: Arc::new(Semaphore::new(max_concurrent)),
            batch_size,
        }
    }

    pub async fn process_batch<T, F, Fut>(
        &self,
        items: Vec<T>,
        processor: F,
    ) -> Result<Vec<Fut::Output>>
    where
        F: Fn(T) -> Fut + Send + Sync + 'static + Clone,
        Fut: std::future::Future + Send,
        Fut::Output: Send,
        T: Send + 'static,
    {
        let mut handles = Vec::new();

        for chunk in items.chunks(self.batch_size) {
            for item in chunk {
                let permit = self.semaphore.clone().acquire_owned().await?;
                let processor = processor.clone();
                
                let handle = tokio::spawn(async move {
                    let result = processor(item).await;
                    drop(permit);
                    result
                });
                
                handles.push(handle);
            }
        }

        let mut results = Vec::new();
        for handle in handles {
            results.push(handle.await?);
        }

        Ok(results)
    }
}
