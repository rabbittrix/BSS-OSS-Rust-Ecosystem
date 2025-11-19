#!/bin/bash
# Load testing script using Apache Bench or similar

set -e

BASE_URL="${BASE_URL:-http://localhost:8080}"
CONCURRENT_USERS="${CONCURRENT_USERS:-100}"
TOTAL_REQUESTS="${TOTAL_REQUESTS:-10000}"

echo "🚀 Running load test..."
echo "Base URL: $BASE_URL"
echo "Concurrent users: $CONCURRENT_USERS"
echo "Total requests: $TOTAL_REQUESTS"

# Check if ab (Apache Bench) is available
if command -v ab &> /dev/null; then
    echo "Using Apache Bench..."
    ab -n $TOTAL_REQUESTS -c $CONCURRENT_USERS -k "$BASE_URL/swagger-ui/"
elif command -v wrk &> /dev/null; then
    echo "Using wrk..."
    wrk -t12 -c$CONCURRENT_USERS -d30s "$BASE_URL/swagger-ui/"
else
    echo "⚠️  Neither 'ab' nor 'wrk' is installed. Please install one of them:"
    echo "   - Apache Bench: apt-get install apache2-utils (Linux) or brew install apache-bench (macOS)"
    echo "   - wrk: https://github.com/wg/wrk"
    exit 1
fi

echo "✅ Load test completed"

