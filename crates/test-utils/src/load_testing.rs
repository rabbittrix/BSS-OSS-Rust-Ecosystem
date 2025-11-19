//! Load testing utilities

use std::time::{Duration, Instant};
use tokio::time::sleep;

/// Load test configuration
#[derive(Debug, Clone)]
pub struct LoadTestConfig {
    pub concurrent_users: usize,
    pub requests_per_user: usize,
    pub ramp_up_duration: Duration,
    pub test_duration: Duration,
}

impl Default for LoadTestConfig {
    fn default() -> Self {
        Self {
            concurrent_users: 10,
            requests_per_user: 100,
            ramp_up_duration: Duration::from_secs(10),
            test_duration: Duration::from_secs(60),
        }
    }
}

/// Load test results
#[derive(Debug, Clone)]
pub struct LoadTestResults {
    pub total_requests: usize,
    pub successful_requests: usize,
    pub failed_requests: usize,
    pub average_response_time: Duration,
    pub min_response_time: Duration,
    pub max_response_time: Duration,
    pub requests_per_second: f64,
    pub error_rate: f64,
    pub test_duration: Duration,
}

impl LoadTestResults {
    pub fn new() -> Self {
        Self {
            total_requests: 0,
            successful_requests: 0,
            failed_requests: 0,
            average_response_time: Duration::ZERO,
            min_response_time: Duration::MAX,
            max_response_time: Duration::ZERO,
            requests_per_second: 0.0,
            error_rate: 0.0,
            test_duration: Duration::ZERO,
        }
    }

    pub fn record_request(&mut self, success: bool, response_time: Duration) {
        self.total_requests += 1;
        if success {
            self.successful_requests += 1;
        } else {
            self.failed_requests += 1;
        }

        if response_time < self.min_response_time {
            self.min_response_time = response_time;
        }
        if response_time > self.max_response_time {
            self.max_response_time = response_time;
        }

        // Update average (simplified calculation)
        let total_nanos = self.average_response_time.as_nanos() as f64
            * (self.total_requests - 1) as f64
            + response_time.as_nanos() as f64;
        self.average_response_time =
            Duration::from_nanos((total_nanos / self.total_requests as f64) as u64);

        if !self.test_duration.is_zero() {
            self.requests_per_second =
                self.total_requests as f64 / self.test_duration.as_secs_f64();
        }
        self.error_rate = (self.failed_requests as f64 / self.total_requests as f64) * 100.0;
    }
}

impl Default for LoadTestResults {
    fn default() -> Self {
        Self::new()
    }
}

/// Run a load test
pub async fn run_load_test<F, Fut>(config: LoadTestConfig, test_function: F) -> LoadTestResults
where
    F: Fn() -> Fut + Send + Sync + Clone + 'static,
    Fut: std::future::Future<Output = (bool, Duration)> + Send,
{
    let mut results = LoadTestResults {
        test_duration: config.test_duration,
        ..Default::default()
    };

    let start_time = Instant::now();
    let mut handles = Vec::new();

    // Ramp up users gradually
    let ramp_up_interval = config.ramp_up_duration / config.concurrent_users as u32;

    for user_id in 0..config.concurrent_users {
        let test_fn = test_function.clone();
        let handle = tokio::spawn(async move {
            // Wait for ramp-up
            sleep(ramp_up_interval * user_id as u32).await;

            let mut user_results = Vec::new();
            for _ in 0..config.requests_per_user {
                let request_start = Instant::now();
                let (success, _response_time) = test_fn().await;
                let elapsed = request_start.elapsed();
                user_results.push((success, elapsed));
            }
            user_results
        });
        handles.push(handle);
    }

    // Collect results
    for handle in handles {
        if let Ok(user_results) = handle.await {
            for (success, response_time) in user_results {
                results.record_request(success, response_time);
            }
        }
    }

    results.test_duration = start_time.elapsed();
    results
}

/// Stress test configuration
#[derive(Debug, Clone)]
pub struct StressTestConfig {
    pub initial_users: usize,
    pub max_users: usize,
    pub step_size: usize,
    pub step_duration: Duration,
    pub requests_per_user: usize,
}

impl Default for StressTestConfig {
    fn default() -> Self {
        Self {
            initial_users: 1,
            max_users: 100,
            step_size: 10,
            step_duration: Duration::from_secs(30),
            requests_per_user: 10,
        }
    }
}

/// Stress test results
#[derive(Debug, Clone)]
pub struct StressTestResults {
    pub steps: Vec<StressTestStep>,
}

#[derive(Debug, Clone)]
pub struct StressTestStep {
    pub user_count: usize,
    pub results: LoadTestResults,
}

/// Run a stress test
pub async fn run_stress_test<F, Fut>(
    config: StressTestConfig,
    test_function: F,
) -> StressTestResults
where
    F: Fn() -> Fut + Send + Sync + Clone + 'static,
    Fut: std::future::Future<Output = (bool, Duration)> + Send,
{
    let mut stress_results = StressTestResults { steps: Vec::new() };

    let mut current_users = config.initial_users;

    while current_users <= config.max_users {
        let load_config = LoadTestConfig {
            concurrent_users: current_users,
            requests_per_user: config.requests_per_user,
            ramp_up_duration: Duration::from_secs(5),
            test_duration: config.step_duration,
        };

        let step_results = run_load_test(load_config, test_function.clone()).await;

        stress_results.steps.push(StressTestStep {
            user_count: current_users,
            results: step_results,
        });

        current_users += config.step_size;
    }

    stress_results
}
