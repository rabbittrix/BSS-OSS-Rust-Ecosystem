#!/bin/bash
# Test coverage script for BSS/OSS Rust Ecosystem

set -e

echo "🧪 Running test coverage analysis..."

# Install cargo-tarpaulin if not already installed
if ! command -v cargo-tarpaulin &> /dev/null; then
    echo "Installing cargo-tarpaulin..."
    cargo install cargo-tarpaulin
fi

# Run tests with coverage
echo "Running tests with coverage..."
cargo tarpaulin --out Xml --out Html --output-dir coverage

# Generate coverage report
echo "Coverage report generated in coverage/ directory"
echo "Open coverage/tarpaulin-report.html in your browser to view the report"

# Check if coverage meets threshold (80%)
COVERAGE=$(grep -oP 'line-rate="\K[0-9.]+' coverage/cobertura.xml | head -1)
COVERAGE_PERCENT=$(echo "$COVERAGE * 100" | bc)

echo "Current coverage: ${COVERAGE_PERCENT}%"

if (( $(echo "$COVERAGE_PERCENT < 80" | bc -l) )); then
    echo "⚠️  Warning: Coverage is below 80% threshold"
    exit 1
else
    echo "✅ Coverage meets 80% threshold"
    exit 0
fi

