use conveyor_belt::{builder::Builder, retry_strategy::RetryStrategy};

#[tokio::main]
async fn main() {
    // use builder to create a queue
    let _queue = Builder::<usize>::new()
        .name("test".to_string())
        .retry_strategy(RetryStrategy::None)
        .timeout(std::time::Duration::from_secs(1))
        .build();

    // TODO: finish this example
}
