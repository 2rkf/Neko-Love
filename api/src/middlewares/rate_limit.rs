use dashmap::DashMap;
use std::{
    sync::Arc,
    time::{SystemTime, UNIX_EPOCH},
};

pub struct RateLimitStatus {
    pub is_allowed: bool,
    pub retry_after: Option<u64>,
    pub limit: u32,
    pub remaining: u32,
    pub reset_after: u64,
}

#[derive(Clone)]
pub struct RateLimiterStore {
    pub request_per_minute: u32,
    usage: Arc<DashMap<String, (u64, u32, u32)>>,
}

impl RateLimiterStore {
    pub fn new(request_per_minute: u32) -> Self {
        Self {
            request_per_minute,
            usage: Arc::new(DashMap::new()),
        }
    }

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
