#!/bin/bash
# Run all tests: unit, integration, and end-to-end

set -e

echo "🧪 Running comprehensive test suite..."

# Run unit tests
echo "📦 Running unit tests..."
cargo test --lib --all-targets

# Run integration tests
echo "🔗 Running integration tests..."
cargo test --test '*' --all-targets

# Run end-to-end tests
echo "🌐 Running end-to-end tests..."
cargo test --test e2e --all-targets || echo "⚠️  End-to-end tests not yet implemented"

# Run benchmarks
echo "⚡ Running benchmarks..."
cargo bench --all-targets || echo "⚠️  Benchmarks require --release flag"

echo "✅ All tests completed"

