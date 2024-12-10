use std::future::Future;
use tokio::time::{sleep, Duration};

/// Executes an asynchronous operation with retries in case of failure.
///
/// The function will attempt to execute the provided operation up to `max_retries` times,
/// waiting for the specified `delay` duration between each retry. If all attempts fail,
/// the error from the last attempt is returned.
///
/// # Arguments
/// - `operation`: A closure that returns a future resolving to a `Result<T, E>`.
/// - `max_retries`: The maximum number of retry attempts.
/// - `delay`: The duration to wait between each retry attempt.
///
/// # Type Parameters
/// - `F`: A future returned by the `operation` closure, with an output of `Result<T, E>`.
/// - `T`: The success type of the `Result` returned by the operation.
/// - `E`: The error type of the `Result`, which must implement `std::fmt::Debug`.
///
/// # Returns
/// A `Result` containing:
/// - `T`: The successful result if the operation succeeds within the retry limit.
/// - `E`: The error from the last attempt if all retries fail.
///
/// # Examples
/// ```
/// use tokio::time::Duration;
///
/// async fn unreliable_operation() -> Result<u32, &'static str> {
///     Err("Operation failed")
/// }
///
/// #[tokio::main]
/// async fn main() {
///     let result = retry(unreliable_operation, 3, Duration::from_secs(2)).await;
///     match result {
///         Ok(value) => println!("Operation succeeded with value: {}", value),
///         Err(e) => println!("Operation failed after retries: {:?}", e),
///     }
/// }
/// ```
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
