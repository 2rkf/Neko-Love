use std::{num::NonZeroU32, sync::Arc};

use dashmap::DashMap;
use governor::{
    Quota, RateLimiter,
    clock::{Clock, DefaultClock},
    state::keyed::DefaultKeyedStateStore,
};

pub type TokenLimiter = RateLimiter<String, DefaultKeyedStateStore<String>, DefaultClock>;

pub struct RateLimitStatus {
    pub is_allowed: bool,
    pub retry_after: Option<u64>,
    pub limit: u32,
    pub remaining: u32,
    pub reset_after: u64,
}

#[derive(Clone)]
pub struct RateLimiterStore {
    inner: Arc<DashMap<String, Arc<TokenLimiter>>>,
    pub request_per_second: u32,
}

impl RateLimiterStore {
    pub fn new(request_per_second: u32) -> Self {
        Self {
            inner: Arc::new(DashMap::new()),
            request_per_second,
        }
    }

    pub fn check(&self, token: String, extend: bool) -> RateLimitStatus {
        let limit = if extend {
            self.request_per_second * 5
        } else {
            self.request_per_second
        };
        let limiter = self.inner.entry(token.to_string()).or_insert_with(|| {
            let quota = Quota::per_second(NonZeroU32::new(limit).unwrap());
            Arc::new(RateLimiter::keyed(quota))
        });

        match limiter.check_key(&token) {
            Ok(_) => RateLimitStatus {
                is_allowed: true,
                retry_after: None,
                limit,
                remaining: limit - 1,
                reset_after: 1,
            },
            Err(nmd) => {
                let wait_time = nmd
                    .wait_time_from(DefaultClock::default().now())
                    .as_secs()
                    .max(1);

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
}
