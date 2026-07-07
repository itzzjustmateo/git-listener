use std::sync::Arc;
use std::time::Duration;

use dashmap::DashMap;
use std::num::NonZeroU32;

use governor::clock::DefaultClock;
use governor::state::{InMemoryState, NotKeyed};
use governor::{Quota, RateLimiter as GovRateLimiter};

use crate::config::WebhookConfig;

pub struct RateLimiterRegistry {
    limiters: DashMap<String, Arc<GovRateLimiter<NotKeyed, InMemoryState, DefaultClock>>>,
    quota: Quota,
}

impl RateLimiterRegistry {
    pub fn new(config: &WebhookConfig) -> Self {
        let burst = NonZeroU32::new(config.rate_limit_burst).expect("burst must be non-zero");
        let quota = Quota::with_period(Duration::from_secs(config.rate_limit_period_seconds))
            .expect("valid rate limit period")
            .allow_burst(burst);

        Self {
            limiters: DashMap::new(),
            quota,
        }
    }

    pub fn get_or_create(&self, key: &str) -> Arc<GovRateLimiter<NotKeyed, InMemoryState, DefaultClock>> {
        self.limiters
            .entry(key.to_owned())
            .or_insert_with(|| Arc::new(GovRateLimiter::direct(self.quota)))
            .value()
            .clone()
    }

    pub fn check_rate_limit(&self, key: &str) -> bool {
        let limiter = self.get_or_create(key);
        limiter.check().is_ok()
    }
}
