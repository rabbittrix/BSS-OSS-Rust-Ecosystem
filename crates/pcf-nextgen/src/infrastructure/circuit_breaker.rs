//! Minimal circuit breaker for protecting downstream NF calls (UDR, SMF, CHF).

use std::future::Future;
use std::sync::atomic::{AtomicU32, Ordering};
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

use bss_oss_pcf::PcfError;

/// Configuration for [`CircuitBreaker`].
#[derive(Debug, Clone)]
pub struct CircuitBreakerConfig {
    pub failure_threshold: u32,
    pub open_duration: Duration,
}

impl Default for CircuitBreakerConfig {
    fn default() -> Self {
        Self {
            failure_threshold: 5,
            open_duration: Duration::from_secs(10),
        }
    }
}

/// Three-state breaker (closed → open → half-open implied by time-based reset).
pub struct CircuitBreaker {
    cfg: CircuitBreakerConfig,
    consecutive_failures: Arc<AtomicU32>,
    opened_at: Arc<Mutex<Option<Instant>>>,
}

impl CircuitBreaker {
    pub fn new(cfg: CircuitBreakerConfig) -> Self {
        Self {
            cfg,
            consecutive_failures: Arc::new(AtomicU32::new(0)),
            opened_at: Arc::new(Mutex::new(None)),
        }
    }

    fn is_open(&self) -> bool {
        let mut guard = self.opened_at.lock().expect("circuit breaker mutex poisoned");
        if let Some(t) = *guard {
            if t.elapsed() >= self.cfg.open_duration {
                *guard = None;
                return false;
            }
            return true;
        }
        false
    }

    fn trip(&self) {
        let mut guard = self.opened_at.lock().expect("circuit breaker mutex poisoned");
        *guard = Some(Instant::now());
        self.consecutive_failures.store(0, Ordering::SeqCst);
    }

    fn on_success(&self) {
        self.consecutive_failures.store(0, Ordering::SeqCst);
        let mut guard = self.opened_at.lock().expect("circuit breaker mutex poisoned");
        *guard = None;
    }

    fn on_failure(&self) {
        let n = self.consecutive_failures.fetch_add(1, Ordering::SeqCst) + 1;
        if n >= self.cfg.failure_threshold {
            self.trip();
        }
    }

    /// Execute `f` while honoring breaker state.
    pub async fn run<F, Fut, T>(&self, f: F) -> Result<T, PcfError>
    where
        F: FnOnce() -> Fut,
        Fut: Future<Output = Result<T, PcfError>>,
    {
        if self.is_open() {
            return Err(PcfError::ServiceUnavailable(
                "circuit breaker open for downstream dependency".into(),
            ));
        }

        let fut = f();
        match fut.await {
            Ok(v) => {
                self.on_success();
                Ok(v)
            }
            Err(e) => {
                self.on_failure();
                Err(e)
            }
        }
    }
}
