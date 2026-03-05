# VantisVPN - Makefile
# https://github.com/vantisCorp/VantisVPN
# Author: VANTISVPN Team <dev@vantisvpn.com>

# ============== VARIABLES ==============
.PHONY: help setup build test lint format clean dev prod deploy install update docs
.DEFAULT_GOAL := help

# Colors for output
BLUE := \033[0;34m
GREEN := \033[0;32m
YELLOW := \033[0;33m
RED := \033[0;31m
NC := \033[0m # No Color

# ============== HELP ==============
help: ## Show this help message
	@echo "$(BLUE)VantisVPN - Makefile Commands$(NC)"
	@echo ""
	@echo "$(GREEN)Usage:$(NC)"
	@echo "  make [target]"
	@echo ""
	@echo "$(GREEN)Available Targets:$(NC)"
	@awk 'BEGIN {FS = ":.*?## "} /^[a-zA-Z_-]+:.*?## / {printf "  $(YELLOW)%-20s$(NC) %s\n", $$1, $$2}' $(MAKEFILE_LIST)
	@echo ""
	@echo "$(GREEN)Examples:$(NC)"
	@echo "  make setup        # Setup development environment"
	@echo "  make build        # Build all packages"
	@echo "  make dev          # Start development server"
	@echo "  make test         # Run all tests"
	@echo ""

# ============== SETUP ==============
setup: ## Setup development environment
	@echo "$(BLUE)🚀 Setting up VantisVPN development environment...$(NC)"
	@echo "$(GREEN)📦 Installing dependencies...$(NC)"
	npm install
	@echo "$(GREEN)🦀 Installing Rust dependencies...$(NC)"
	cargo fetch
	@echo "$(GREEN)🔒 Setting up pre-commit hooks...$(NC)"
	pre-commit install --install-hooks
	@echo "$(GREEN)🐕 Setting up Husky...$(NC)"
	npx husky install
	@echo "$(BLUE)✅ Setup complete!$(NC)"
	@echo "$(YELLOW)Run 'make dev' to start development$(NC)"

# ============== INSTALLATION ==============
install: ## Install all dependencies
	@echo "$(GREEN)📦 Installing dependencies...$(NC)"
	npm install
	cargo fetch
	@echo "$(BLUE)✅ Dependencies installed!$(NC)"

update: ## Update all dependencies
	@echo "$(GREEN)📥 Updating dependencies...$(NC)"
	npm update
	cargo update
	@echo "$(BLUE)✅ Dependencies updated!$(NC)"

# ============== BUILDING ==============
build: ## Build all packages
	@echo "$(BLUE)🏗️  Building all packages...$(NC)"
	turbo run build
	@echo "$(BLUE)✅ Build complete!$(NC)"

build:rust: ## Build Rust packages only
	@echo "$(GREEN)🦀 Building Rust packages...$(NC)"
	cargo build --release
	@echo "$(BLUE)✅ Rust build complete!$(NC)"

build:web: ## Build web packages only
	@echo "$(GREEN)🌐 Building web packages...$(NC)"
	turbo run build --filter=web
	@echo "$(BLUE)✅ Web build complete!$(NC)"

# ============== DEVELOPMENT ==============
dev: ## Start development servers
	@echo "$(BLUE)🚀 Starting development servers...$(NC)"
	turbo run dev

dev:rust: ## Start Rust development
	@echo "$(GREEN)🦀 Starting Rust development...$(NC)"
	cargo watch -x run

dev:web: ## Start web development
	@echo "$(GREEN)🌐 Starting web development...$(NC)"
	turbo run dev --filter=web

# ============== TESTING ==============
test: ## Run all tests
	@echo "$(BLUE)🧪 Running all tests...$(NC)"
	turbo run test
	@echo "$(BLUE)✅ Tests complete!$(NC)"

test:rust: ## Run Rust tests only
	@echo "$(GREEN)🦀 Running Rust tests...$(NC)"
	cargo test --all-features
	@echo "$(BLUE)✅ Rust tests complete!$(NC)"

test:watch: ## Run tests in watch mode
	@echo "$(GREEN)👀 Running tests in watch mode...$(NC)"
	turbo run test --watch

test:coverage: ## Generate test coverage report
	@echo "$(GREEN)📊 Generating coverage report...$(NC)"
	cargo tarpaulin --out Html
	@echo "$(BLUE)✅ Coverage report generated in target/tarpaulin/$(NC)"

# ============== LINTING ==============
lint: ## Run all linters
	@echo "$(BLUE)🔍 Running all linters...$(NC)"
	turbo run lint
	cargo clippy --all-targets --all-features -- -D warnings
	@echo "$(BLUE)✅ Linting complete!$(NC)"

lint:fix: ## Auto-fix linting issues
	@echo "$(GREEN)🔧 Auto-fixing linting issues...$(NC)"
	turbo run lint --fix
	cargo clippy --fix --allow-dirty --allow-staged
	@echo "$(BLUE)✅ Linting fixed!$(NC)"

# ============== FORMATTING ==============
format: ## Format all code
	@echo "$(BLUE)🎨 Formatting all code...$(NC)"
	turbo run format
	cargo fmt
	@echo "$(BLUE)✅ Formatting complete!$(NC)"

format:check: ## Check if code is formatted
	@echo "$(GREEN)🔍 Checking formatting...$(NC)"
	turbo run format --check
	cargo fmt -- --check
	@echo "$(BLUE)✅ Formatting check complete!$(NC)"

# ============== CLEANING ==============
clean: ## Clean build artifacts
	@echo "$(RED)🧹 Cleaning build artifacts...$(NC)"
	turbo run clean
	cargo clean
	rm -rf node_modules .turbo dist build .next
	@echo "$(BLUE)✅ Clean complete!$(NC)"

clean:deps: ## Clean all dependencies
	@echo "$(RED)🗑️  Cleaning all dependencies...$(NC)"
	rm -rf node_modules
	cargo clean
	@echo "$(BLUE)✅ Dependencies cleaned!$(NC)"

clean:all: ## Clean everything
	@echo "$(RED)🔥 Cleaning everything...$(NC)"
	$(MAKE) clean:deps
	rm -rf .turbo .cache target dist build .next
	@echo "$(BLUE)✅ Full clean complete!$(NC)"

# ============== DOCUMENTATION ==============
docs: ## Generate documentation
	@echo "$(GREEN)📚 Generating documentation...$(NC)"
	cargo doc --all-features --no-deps --open
	@echo "$(BLUE)✅ Documentation generated!$(NC)"

docs:serve: ## Serve documentation
	@echo "$(GREEN)🌐 Serving documentation...$(NC)"
	cargo doc --all-features --no-deps --open

# ============== SECURITY ==============
audit: ## Run security audit
	@echo "$(BLUE)🔒 Running security audit...$(NC)"
	cargo audit
	npm audit
	@echo "$(BLUE)✅ Security audit complete!$(NC)"

audit:fix: ## Auto-fix security issues
	@echo "$(GREEN)🔧 Auto-fixing security issues...$(NC)"
	cargo audit fix
	npm audit fix
	@echo "$(BLUE)✅ Security issues fixed!$(NC)"

secrets:scan: ## Scan for secrets
	@echo "$(RED)🔐 Scanning for secrets...$(NC)"
	gitleaks detect --source . --verbose --report-format json --report-path .gitleaks-report.json
	@echo "$(BLUE)✅ Secret scan complete!$(NC)"

# ============== RELEASE ==============
release:patch: ## Create patch release
	@echo "$(GREEN)🎉 Creating patch release...$(NC)"
	npx standard-version --release-as patch
	git push --follow-tags
	@echo "$(BLUE)✅ Patch release created!$(NC)"

release:minor: ## Create minor release
	@echo "$(GREEN)🎉 Creating minor release...$(NC)"
	npx standard-version --release-as minor
	git push --follow-tags
	@echo "$(BLUE)✅ Minor release created!$(NC)"

release:major: ## Create major release
	@echo "$(GREEN)🎉 Creating major release...$(NC)"
	npx standard-version --release-as major
	git push --follow-tags
	@echo "$(BLUE)✅ Major release created!$(NC)"

# ============== DEPLOYMENT ==============
deploy: ## Deploy to production
	@echo "$(BLUE)🚀 Deploying to production...$(NC)"
	turbo run build
	@echo "$(GREEN)📦 Deploying...$(NC)"
	@echo "$(BLUE)✅ Deploy complete!$(NC)"

deploy:staging: ## Deploy to staging
	@echo "$(BLUE)🚀 Deploying to staging...$(NC)"
	turbo run build
	@echo "$(GREEN)📦 Deploying to staging...$(NC)"
	@echo "$(BLUE)✅ Staging deploy complete!$(NC)"

# ============== DOCKER ==============
docker:build: ## Build Docker images
	@echo "$(GREEN)🐳 Building Docker images...$(NC)"
	docker-compose build
	@echo "$(BLUE)✅ Docker build complete!$(NC)"

docker:up: ## Start Docker containers
	@echo "$(GREEN)🐳 Starting Docker containers...$(NC)"
	docker-compose up -d
	@echo "$(BLUE)✅ Docker containers started!$(NC)"

docker:down: ## Stop Docker containers
	@echo "$(RED)🐳 Stopping Docker containers...$(NC)"
	docker-compose down
	@echo "$(BLUE)✅ Docker containers stopped!$(NC)"

docker:logs: ## Show Docker logs
	@echo "$(GREEN)📋 Showing Docker logs...$(NC)"
	docker-compose logs -f

# ============== MONITORING ==============
watch: ## Watch for changes and rebuild
	@echo "$(GREEN)👀 Watching for changes...$(NC)"
	turbo run build --watch

benchmark: ## Run benchmarks
	@echo "$(GREEN)📊 Running benchmarks...$(NC)"
	cargo bench
	@echo "$(BLUE)✅ Benchmarks complete!$(NC)"

# ============== UTILITIES ==============
tree: ## Show project tree structure
	@echo "$(GREEN)🌲 Project structure:$(NC)"
	tree -L 3 -I 'node_modules|target|.git|dist|build' --dirsfirst

check: ## Run all checks
	@echo "$(BLUE)🔍 Running all checks...$(NC)"
	$(MAKE) format:check
	$(MAKE) lint
	$(MAKE) test
	$(MAKE) audit
	@echo "$(BLUE)✅ All checks complete!$(NC)"

ci: ## Run CI pipeline
	@echo "$(BLUE)🔄 Running CI pipeline...$(NC)"
	$(MAKE) check
	$(MAKE) build
	@echo "$(BLUE)✅ CI pipeline complete!$(NC)"