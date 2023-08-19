use std::time::Duration;

#[derive(PartialEq, Debug, Clone)]
pub enum RetryStrategy {
    None,
    Linear(Duration),
    Exponential(Duration, usize),
}
