#!/bin/bash
# Security vulnerability scanning script

set -e

echo "🔒 Running security vulnerability scan..."

# Install cargo-audit if not already installed
if ! command -v cargo-audit &> /dev/null; then
    echo "Installing cargo-audit..."
    cargo install cargo-audit
fi

# Run security audit
echo "Scanning for security vulnerabilities..."
cargo audit

if [ $? -eq 0 ]; then
    echo "✅ No security vulnerabilities found"
    exit 0
else
    echo "⚠️  Security vulnerabilities detected. Please review and fix."
    exit 1
fi

