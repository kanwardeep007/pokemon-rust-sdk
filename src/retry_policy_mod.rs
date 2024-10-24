use std::time::Duration;

use reqwest_retry::policies::{ExponentialBackoff, ExponentialBackoffTimed};

#[derive(Debug)]
pub enum RetryStrategy {
    ExponentialBackoffTimed { max_duration: Duration },
}

impl RetryStrategy {
    pub fn get_policy(&self) -> ExponentialBackoffTimed {
        match self {
            RetryStrategy::ExponentialBackoffTimed {
                max_duration: max_retry_interval,
            } => ExponentialBackoff::builder()
                .build_with_total_retry_duration_and_max_retries(*max_retry_interval),
        }
    }
}
