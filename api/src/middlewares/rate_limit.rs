use std::{num::NonZeroU32, sync::Arc};

use dashmap::DashMap;
use governor::{
    clock::{Clock, DefaultClock}, state::keyed::DefaultKeyedStateStore, Quota, RateLimiter
};

pub type TokenLimiter = RateLimiter<String, DefaultKeyedStateStore<String>, DefaultClock>;

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

    pub fn check(&self, token: String, extend: bool) -> Result<(), u64> {
        let limit = if extend { self.request_per_second * 5 } else { self.request_per_second };
        let limiter = self.inner.entry(token.to_string()).or_insert_with(|| {
            let quota = Quota::per_second(NonZeroU32::new(limit).unwrap());
            Arc::new(RateLimiter::keyed(quota))
        });

        match limiter.check_key(&token) {
            Ok(_) => Ok(()),
            Err(nmd) => {
                let wait_time = nmd.wait_time_from(DefaultClock::default().now()).as_secs().max(1);
                Err(wait_time)
            }
        }
    }
}
