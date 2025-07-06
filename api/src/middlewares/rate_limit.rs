use dashmap::DashMap;
use governor::{
    Quota, RateLimiter,
    clock::{Clock, DefaultClock},
    state::keyed::DefaultKeyedStateStore,
};
use std::{
    num::NonZeroU32,
    sync::Arc,
    time::{SystemTime, UNIX_EPOCH},
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
    pub request_per_minute: u32,
    usage: Arc<DashMap<String, (u64, u32)>>,
}

impl RateLimiterStore {
    pub fn new(request_per_minute: u32) -> Self {
        Self {
            inner: Arc::new(DashMap::new()),
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
        let limiter = self.inner.entry(token.to_string()).or_insert_with(|| {
            let quota = Quota::per_minute(NonZeroU32::new(limit).unwrap());
            Arc::new(RateLimiter::keyed(quota))
        });

        let now_secs = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        let minute = now_secs / 60;
        let mut usage_entry = self.usage.entry(token.clone()).or_insert((minute, 0));
        let usage = usage_entry.value_mut();

        if usage.0 != minute {
            *usage = (minute, 0);
        }

        match limiter.check_key(&token) {
            Ok(_) => {
                usage.1 += 1;

                RateLimitStatus {
                    is_allowed: true,
                    retry_after: None,
                    limit,
                    remaining: limit.saturating_sub(usage.1),
                    reset_after: 60 - (now_secs % 60),
                }
            }
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
