use dashmap::DashMap;
use std::{
    sync::Arc,
    time::{SystemTime, UNIX_EPOCH},
};

/// A thread-safe rate limiter that tracks and enforces request quotas per minute.
#[derive(Clone)]
pub struct RateLimiterStore {
    request_per_minute: u32,
    usage: Arc<DashMap<String, (u64, u32, u32)>>,
}

/// Represents the status of a rate limit check, including allowance status and quota information.
pub struct RateLimitStatus {
    /// Whether the current request is allowed under the rate limit.
    pub is_allowed: bool,
    /// Seconds until the next allowed request (only present when rate limited).
    pub retry_after: Option<u64>,
    /// The total request limit per time window.
    pub limit: u32,
    /// Remaining requests in the current time window.
    pub remaining: u32,
    /// Seconds until the current rate limit window resets.
    pub reset_after: u64,
}

impl RateLimiterStore {
    /// Creates a new `RateLimiterStore` with the specified requests per minute limit.
    pub fn new(request_per_minute: u32) -> Self {
        Self {
            request_per_minute,
            usage: Arc::new(DashMap::new()),
        }
    }

    /// Checks if a request is allowed under the rate limit, updating the usage count.
    pub fn check(&self, token: String, extend: bool) -> RateLimitStatus {
        let limit = if extend {
            self.request_per_minute * 5
        } else {
            self.request_per_minute
        };

        let now_secs = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        let minute = now_secs / 60 * 60;
        let mut usage_entry = self
            .usage
            .entry(token.clone())
            .or_insert((minute, 0, limit));
        let usage = usage_entry.value_mut();

        if usage.0 != minute {
            *usage = (minute, 0, limit);
        }

        if usage.1 < limit {
            usage.1 += 1;
            RateLimitStatus {
                is_allowed: true,
                retry_after: None,
                limit,
                remaining: limit - usage.1,
                reset_after: 60 - (now_secs - minute),
            }
        } else {
            let wait_time = 60 - (now_secs - usage.0);
            RateLimitStatus {
                is_allowed: false,
                retry_after: Some(wait_time),
                limit,
                remaining: 0,
                reset_after: wait_time,
            }
        }
    }
}
