# Testing Guide

## Overview

This document describes the comprehensive testing strategy for the BSS/OSS Rust Ecosystem, covering unit tests, integration tests, end-to-end tests, performance benchmarks, load testing, and security auditing.

## Test Structure

### Unit Tests

Unit tests are located in each crate's `tests/` directory or inline with `#[cfg(test)]` modules.

**Location:** `crates/{crate-name}/tests/`

**Example:**

```bash
cargo test --package security
```

### Integration Tests

Integration tests verify that multiple components work together correctly.

**Location:** `tests/integration_tmf_apis.rs`

**Run:**

```bash
cargo test --test integration_tmf_apis
```

### End-to-End Tests

End-to-end tests verify complete workflows from start to finish.

**Location:** `tests/e2e_workflows.rs`

**Run:**

```bash
cargo test --test e2e_workflows
```

## Running Tests

### Run All Tests

```bash
# Run all tests
cargo test --all-targets

# Run with output
cargo test --all-targets -- --nocapture

# Run specific test
cargo test test_name
```

### Run Tests for Specific Crate

```bash
cargo test --package security
cargo test --package revenue-management
cargo test --package service-orchestrator
```

### Run Integration Tests

```bash
# Requires test database
DATABASE_URL="postgresql://bssoss:bssoss123@localhost:5432/bssoss_test" \
cargo test --test integration_tmf_apis
```

## Test Coverage

### Generate Coverage Report

```bash
# Install cargo-tarpaulin
cargo install cargo-tarpaulin

# Run coverage
cargo tarpaulin --out Xml --out Html --output-dir coverage

# View report
open coverage/tarpaulin-report.html
```

### Coverage Threshold

The project aims for **>80% code coverage**. The CI pipeline will fail if coverage drops below this threshold.

## Performance Benchmarks

### Run Benchmarks

```bash
# Run all benchmarks
cargo bench --all-targets

# Run specific benchmark
cargo bench --bench api_performance
```

### Benchmark Results

Benchmark results are stored in `target/criterion/` and can be viewed as HTML reports.

## Load Testing

### Using Test Utils

```rust
use test_utils::load_testing::{LoadTestConfig, run_load_test};

let config = LoadTestConfig {
    concurrent_users: 100,
    requests_per_user: 1000,
    ramp_up_duration: Duration::from_secs(10),
    test_duration: Duration::from_secs(60),
};

let results = run_load_test(config, || async {
    // Your test function
    (true, Duration::from_millis(100))
}).await;
```

### Using Scripts

```bash
# Set environment variables
export BASE_URL="http://localhost:8080"
export CONCURRENT_USERS=100
export TOTAL_REQUESTS=10000

# Run load test
./scripts/load-test.sh
```

## Security Auditing

### Run Security Audit

```bash
# Install cargo-audit
cargo install cargo-audit

# Run audit
cargo audit

# Or use the script
./scripts/security-audit.sh
```

### Fix Vulnerabilities

If vulnerabilities are found:

1. Review the vulnerability report
2. Update dependencies to patched versions
3. Run `cargo update`
4. Re-run `cargo audit` to verify fixes

## Code Quality

### Formatting

```bash
# Check formatting
cargo fmt --all -- --check

# Format code
cargo fmt --all
```

### Linting

```bash
# Run clippy
cargo clippy --all-targets --all-features -- -D warnings

# Fix auto-fixable issues
cargo clippy --all-targets --all-features --fix
```

## CI/CD Integration

All quality assurance checks run automatically in CI:

- **Test Suite**: Runs on every push and PR
- **Coverage**: Generates coverage reports and uploads to Codecov
- **Lint & Format**: Checks code formatting and runs clippy
- **Security Audit**: Scans for vulnerabilities
- **Benchmarks**: Runs weekly to track performance

## Test Database Setup

For integration and end-to-end tests, you need a test database:

```bash
# Create test database
createdb bssoss_test

# Or using Docker
docker run --name bss-oss-test-postgres \
  -e POSTGRES_USER=bssoss \
  -e POSTGRES_PASSWORD=bssoss123 \
  -e POSTGRES_DB=bssoss_test \
  -p 5433:5432 \
  -d postgres:15-alpine

# Set environment variable
export TEST_DATABASE_URL="postgresql://bssoss:bssoss123@localhost:5433/bssoss_test"
```

## Best Practices

1. **Write tests first** (TDD) when possible
2. **Keep tests fast** - unit tests should run in milliseconds
3. **Use descriptive test names** - `test_create_customer_with_valid_data`
4. **Test edge cases** - empty inputs, null values, boundary conditions
5. **Mock external dependencies** - databases, APIs, file systems
6. **Clean up test data** - use transactions or cleanup functions
7. **Test error paths** - verify error handling works correctly
8. **Maintain test coverage** - aim for >80% coverage

## Troubleshooting

### Tests Fail with Database Errors

- Ensure test database is running
- Check `TEST_DATABASE_URL` environment variable
- Verify database migrations are applied

### Tests Are Slow

- Use test transactions instead of real database operations
- Mock expensive operations
- Run tests in parallel: `cargo test -- --test-threads=4`

### Coverage Report Not Generated

- Ensure `cargo-tarpaulin` is installed
- Check that tests actually run (some may be skipped)
- Verify output directory permissions

## Resources

- [Rust Testing Book](https://doc.rust-lang.org/book/ch11-00-testing.html)
- [cargo-tarpaulin Documentation](https://github.com/xd009642/tarpaulin)
- [cargo-audit Documentation](https://github.com/rustsec/rustsec/tree/main/cargo-audit)
