.PHONY: help build test clean lint fmt docs docker-build docker-up docker-down install-deps

# Default target
help:
	@echo "VANTISVPN - Makefile"
	@echo ""
	@echo "Available targets:"
	@echo "  make build          - Build the core library"
	@echo "  make test           - Run all tests"
	@echo "  make clean          - Clean build artifacts"
	@echo "  make lint           - Run linter (clippy)"
	@echo "  make fmt            - Format code"
	@echo "  make docs           - Generate documentation"
	@echo "  make docker-build   - Build Docker images"
	@echo "  make docker-up      - Start Docker containers"
	@echo "  make docker-down    - Stop Docker containers"
	@echo "  make install-deps   - Install development dependencies"
	@echo ""

# Build the core library
build:
	@echo "Building VANTISVPN Core..."
	cargo build --manifest-path src/core/Cargo.toml --release
	@echo "Build complete!"

# Build with debug symbols
build-debug:
	@echo "Building VANTISVPN Core (debug)..."
	cargo build --manifest-path src/core/Cargo.toml
	@echo "Build complete!"

# Run all tests
test:
	@echo "Running tests..."
	cargo test --manifest-path src/core/Cargo.toml --verbose
	@echo "Tests complete!"

# Run tests with coverage
test-coverage:
	@echo "Running tests with coverage..."
	cargo tarpaulin --manifest-path src/core/Cargo.toml --out Html --output-dir ./coverage
	@echo "Coverage report generated in ./coverage/"

# Clean build artifacts
clean:
	@echo "Cleaning build artifacts..."
	cargo clean --manifest-path src/core/Cargo.toml
	rm -rf target/
	rm -rf coverage/
	@echo "Clean complete!"

# Run linter
lint:
	@echo "Running clippy..."
	cargo clippy --manifest-path src/core/Cargo.toml --all-targets --all-features -- -D warnings
	@echo "Linting complete!"

# Format code
fmt:
	@echo "Formatting code..."
	cargo fmt --all
	@echo "Formatting complete!"

# Check formatting
fmt-check:
	@echo "Checking formatting..."
	cargo fmt --all -- --check
	@echo "Formatting check complete!"

# Generate documentation
docs:
	@echo "Generating documentation..."
	cargo doc --manifest-path src/core/Cargo.toml --no-deps --open
	@echo "Documentation generated!"

# Build Docker images
docker-build:
	@echo "Building Docker images..."
	docker-compose build
	@echo "Docker images built!"

# Start Docker containers
docker-up:
	@echo "Starting Docker containers..."
	docker-compose up -d
	@echo "Containers started!"
	@echo "Grafana: http://localhost:3000"
	@echo "Prometheus: http://localhost:9090"
	@echo "Kibana: http://localhost:5601"
	@echo "Consul: http://localhost:8500"

# Stop Docker containers
docker-down:
	@echo "Stopping Docker containers..."
	docker-compose down
	@echo "Containers stopped!"

# View Docker logs
docker-logs:
	docker-compose logs -f

# Install development dependencies
install-deps:
	@echo "Installing development dependencies..."
	cargo install cargo-tarpaulin
	cargo install cargo-audit
	cargo install cargo-outdated
	@echo "Dependencies installed!"

# Run security audit
audit:
	@echo "Running security audit..."
	cargo audit
	@echo "Audit complete!"

# Check for outdated dependencies
outdated:
	@echo "Checking for outdated dependencies..."
	cargo outdated
	@echo "Check complete!"

# Run all checks (CI pipeline locally)
ci: fmt-check lint test audit
	@echo "All CI checks passed!"

# Release build with all checks
release: fmt-check lint test build
	@echo "Release build complete!"

# Development setup
setup: install-deps
	@echo "Setting up development environment..."
	@echo "Development environment ready!"