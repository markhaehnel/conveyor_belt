use std::{collections::HashMap, time::Duration};

use crate::{queue::Queue, retry_strategy::RetryStrategy};

pub struct Builder<T> {
    name: String,
    processor: Box<dyn Fn(T)>,
    retry_strategy: RetryStrategy,
    timeout: Duration,
}

impl<T> Builder<T> {
    pub fn new() -> Self {
        Self {
            name: "default".to_owned(),
            processor: Box::new(|_| {}),
            retry_strategy: RetryStrategy::None,
            timeout: Duration::from_secs(30),
        }
    }

    pub fn name(mut self, name: String) -> Self {
        self.name = name;
        self
    }

    pub fn retry_strategy(mut self, retry_strategy: RetryStrategy) -> Self {
        self.retry_strategy = retry_strategy;
        self
    }

    pub fn timeout(mut self, timeout: Duration) -> Self {
        self.timeout = timeout;
        self
    }

    pub fn build(self) -> Queue<T> {
        Queue::<T>::new(self.name, self.retry_strategy, self.timeout)
    }
}

impl<T> Default for Builder<T> {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use std::time::Duration;

    use super::*;

    #[test]
    fn test_builder_defaults() {
        let queue = Builder::<usize>::new().build();

        assert_eq!(queue.name(), "default");
        assert_eq!(queue.retry_strategy(), &RetryStrategy::None);
        assert_eq!(queue.timeout(), Duration::from_secs(30));
    }

    fn test_builder() {
        let queue = Builder::<usize>::new()
            .name("test".to_string())
            .retry_strategy(RetryStrategy::Linear(Duration::from_secs(1)))
            .timeout(Duration::from_secs(1))
            .build();

        assert_eq!(queue.name(), "test");
        assert_eq!(
            queue.retry_strategy(),
            &RetryStrategy::Linear(Duration::from_secs(1))
        );
        assert_eq!(queue.timeout(), Duration::from_secs(1));
    }
}
