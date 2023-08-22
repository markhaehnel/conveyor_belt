use crate::{queue::Queue, retry_strategy::RetryStrategy};
use parking_lot::Mutex;
use std::{collections::VecDeque, time::Duration};

pub struct MemoryQueue<T: 'static> {
    name: String,
    retry_strategy: RetryStrategy,
    timeout: Duration,

    items: Mutex<VecDeque<T>>,
}

impl<T> MemoryQueue<T> {
    #[must_use]
    pub fn new(name: String, retry_strategy: RetryStrategy, timeout: Duration) -> Self {
        Self {
            name,
            retry_strategy,
            timeout,
            items: Mutex::new(VecDeque::new()),
        }
    }

    fn acquire_items_lock(
        &self,
    ) -> parking_lot::lock_api::MutexGuard<'_, parking_lot::RawMutex, VecDeque<T>> {
        self.items
            .try_lock_for(Duration::from_secs(1))
            .unwrap_or_else(|| panic!("Could not acquire lock for queue items on {}", self.name))
    }
}

impl<T> Queue<T> for MemoryQueue<T> {
    fn add_job(&self, item: T) {
        self.acquire_items_lock().push_back(item);
    }

    fn fetch(&self) -> Option<T> {
        self.acquire_items_lock().pop_front()
    }

    fn name(&self) -> &str {
        &self.name
    }

    fn retry_strategy(&self) -> &RetryStrategy {
        &self.retry_strategy
    }

    fn timeout(&self) -> Duration {
        self.timeout
    }
}

#[cfg(test)]
mod tests {
    use std::time::Duration;

    use super::*;

    #[test]
    fn test_queue() {
        let queue = MemoryQueue::new(
            "test".to_string(),
            RetryStrategy::None,
            Duration::from_secs(1),
        );

        queue.add_job(1);
        queue.add_job(2);
        queue.add_job(3);

        assert_eq!(queue.fetch(), Some(1));
        assert_eq!(queue.fetch(), Some(2));
        assert_eq!(queue.fetch(), Some(3));
        assert_eq!(queue.fetch(), None);
    }
}
