//! Test coverage utilities

use std::collections::HashMap;
use std::sync::Mutex;

/// Test coverage tracker
pub struct CoverageTracker {
    modules: Mutex<HashMap<String, ModuleCoverage>>,
}

impl CoverageTracker {
    pub fn new() -> Self {
        Self {
            modules: Mutex::new(HashMap::new()),
        }
    }

    pub fn record_function_call(&self, module: &str, function: &str) {
        let mut modules = self.modules.lock().unwrap();
        let module_cov = modules.entry(module.to_string()).or_default();
        module_cov.record_function_call(function);
    }

    pub fn get_coverage(&self) -> HashMap<String, ModuleCoverage> {
        self.modules.lock().unwrap().clone()
    }

    pub fn get_total_coverage(&self) -> f64 {
        let modules = self.modules.lock().unwrap();
        if modules.is_empty() {
            return 0.0;
        }

        let total_functions: usize = modules.values().map(|m| m.total_functions).sum();
        let called_functions: usize = modules.values().map(|m| m.called_functions.len()).sum();

        if total_functions == 0 {
            return 0.0;
        }

        (called_functions as f64 / total_functions as f64) * 100.0
    }
}

impl Default for CoverageTracker {
    fn default() -> Self {
        Self::new()
    }
}

/// Module coverage information
#[derive(Clone, Debug)]
pub struct ModuleCoverage {
    pub total_functions: usize,
    pub called_functions: Vec<String>,
}

impl ModuleCoverage {
    pub fn new() -> Self {
        Self {
            total_functions: 0,
            called_functions: Vec::new(),
        }
    }

    pub fn record_function_call(&mut self, function: &str) {
        if !self.called_functions.contains(&function.to_string()) {
            self.called_functions.push(function.to_string());
        }
    }

    pub fn coverage_percentage(&self) -> f64 {
        if self.total_functions == 0 {
            return 0.0;
        }
        (self.called_functions.len() as f64 / self.total_functions as f64) * 100.0
    }
}

impl Default for ModuleCoverage {
    fn default() -> Self {
        Self::new()
    }
}
