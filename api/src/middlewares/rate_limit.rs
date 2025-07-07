use dashmap::DashMap;
use std::{
    sync::Arc,
    time::{SystemTime, UNIX_EPOCH},
};

/// A thread-safe rate limiter that tracks and enforces request quotas per day.
#[derive(Clone)]
pub struct RateLimiterStore {
    request_per_day: u32,
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
    /// Creates a new `RateLimiterStore` with the specified requests per day limit.
    pub fn new(request_per_day: u32) -> Self {
        Self {
            request_per_day,
            usage: Arc::new(DashMap::new()),
        }
    }

    /// Checks if a request is allowed under the rate limit, updating the usage count.
    pub fn check(&self, token: String, extend: bool) -> RateLimitStatus {
        let limit = if extend {
            self.request_per_day * 10
        } else {
            self.request_per_day
        };

        let now_secs = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();

        let day_start = now_secs / 86400 * 86400;
        let mut usage_entry = self
            .usage
            .entry(token.clone())
            .or_insert((day_start, 0, limit));
        let usage = usage_entry.value_mut();

        if usage.0 != day_start {
            *usage = (day_start, 0, limit);
        }

        if usage.1 < limit {
            usage.1 += 1;
            RateLimitStatus {
                is_allowed: true,
                retry_after: None,
                limit,
                remaining: limit - usage.1,
                reset_after: 86400 - (now_secs - day_start),
            }
        } else {
            let wait_time = 86400 - (now_secs - usage.0);
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
