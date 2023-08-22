use std::time::Duration;

use crate::retry_strategy::RetryStrategy;

pub trait Queue<T> {
    fn name(&self) -> &str;
    fn retry_strategy(&self) -> &RetryStrategy;
    fn timeout(&self) -> Duration;

    fn add_job(&self, item: T);
    fn fetch(&self) -> Option<T>;
}
