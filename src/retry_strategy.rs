use std::time::Duration;

#[derive(PartialEq, Eq, Debug, Clone)]
pub enum RetryStrategy {
    None,
    Linear(Duration),
    Exponential(Duration, usize),
}
