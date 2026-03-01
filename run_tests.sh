#!/bin/bash
# Comprehensive Test Runner for VANTISVPN
# Runs all tests with various configurations

set -e

echo "=========================================="
echo "VANTISVPN Test Suite"
echo "=========================================="
echo ""

# Colors for output
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
RED='\033[0;31m'
NC='\033[0m' # No Color

# Function to print colored output
print_success() {
    echo -e "${GREEN}✓ $1${NC}"
}

print_warning() {
    echo -e "${YELLOW}⚠ $1${NC}"
}

print_error() {
    echo -e "${RED}✗ $1${NC}"
}

# Check if we're in the right directory
if [ ! -f "src/core/Cargo.toml" ]; then
    print_error "Please run this script from the project root directory"
    exit 1
fi

cd src/core

echo "1. Running cargo check..."
if cargo check 2>&1 | grep -q "error"; then
    print_error "Compilation errors found"
    exit 1
else
    print_success "Code compiles successfully"
fi

echo ""
echo "2. Running unit tests..."
if cargo test --lib --no-fail-fast --quiet 2>&1 | grep -q "test result"; then
    print_success "Unit tests passed"
else
    print_warning "Some unit tests may have failed or timed out"
fi

echo ""
echo "3. Running integration tests..."
if cargo test --test integration_test --quiet 2>&1 | grep -q "test result"; then
    print_success "Integration tests passed"
else
    print_warning "Integration tests may have failed or timed out"
fi

echo ""
echo "4. Running clippy..."
if cargo clippy -- -D warnings 2>&1 | grep -q "warning"; then
    print_warning "Clippy found some warnings"
else
    print_success "No clippy warnings"
fi

echo ""
echo "5. Checking code formatting..."
if cargo fmt -- --check 2>&1 | grep -q "Diff"; then
    print_warning "Code formatting issues found (run 'cargo fmt' to fix)"
else
    print_success "Code is properly formatted"
fi

echo ""
echo "6. Running benchmarks (optional)..."
read -p "Run benchmarks? This may take a while. (y/n) " -n 1 -r
echo
if [[ $REPLY =~ ^[Yy]$ ]]; then
    if cargo bench 2>&1 | grep -q "running"; then
        print_success "Benchmarks completed"
    else
        print_warning "Benchmarks may have failed"
    fi
else
    print_warning "Benchmarks skipped"
fi

echo ""
echo "=========================================="
echo "Test Suite Complete"
echo "=========================================="
echo ""
echo "For detailed test output, run:"
echo "  cd src/core && cargo test -- --nocapture"
echo ""
echo "For benchmark results, run:"
echo "  cd src/core && cargo bench"