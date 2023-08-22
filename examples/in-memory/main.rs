use conveyor_belt::{memory_queue::MemoryQueue, retry_strategy::RetryStrategy};

#[tokio::main]
async fn main() {
    let _queue = MemoryQueue::<usize>::new(
        "test".to_string(),
        RetryStrategy::None,
        std::time::Duration::from_secs(1),
    );

    // TODO: finish this example
}
