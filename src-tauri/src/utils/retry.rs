use std::future::Future;
use tokio::time::{sleep, Duration};

pub async fn retry<F, T, E>(
    operation: impl Fn() -> F,
    max_retries: usize,
    delay: Duration,
) -> Result<T, E>
where
    F: Future<Output = Result<T, E>>, E: std::fmt::Debug
{
    for attempt in 1..=max_retries {
        println!("Attempt {}/{}", attempt, max_retries);
        match operation().await {
            Ok(result) => return Ok(result),
            Err(e) => {
                println!("Attempt failed: {:?}", e);
                if attempt < max_retries {
                    sleep(delay).await;
                } else {
                    return Err(e);
                }
            }
        }
    }

    unreachable!()
}
