#!/bin/bash
# VantisVPN - DevContainer Post-Create Script
# This script runs after the DevContainer is created

set -e

echo "🚀 Setting up VantisVPN Development Environment..."

# ============== Install Node.js Dependencies ==============
echo "📦 Installing Node.js dependencies..."
npm install

# ============== Install Rust Tools ==============
echo "🦀 Installing Rust development tools..."
rustup component add rustfmt clippy rust-src
cargo install cargo-edit cargo-watch cargo-outdated cargo-audit

# ============== Install Additional Tools ==============
echo "🛠️ Installing additional development tools..."
npm install -g @commitlint/cli @commitlint/config-conventional
npm install -g standard-version
npm install -g prettier eslint

# ============== Setup Pre-commit ==============
echo "🔒 Setting up pre-commit hooks..."
pip3 install pre-commit
pre-commit install --hook-type pre-commit
pre-commit install --hook-type commit-msg

# ============== Initialize Git Secrets Baseline ==============
echo "🔐 Initializing secrets baseline..."
if [ ! -f .secrets.baseline ]; then
    detect-secrets scan > .secrets.baseline || true
fi

# ============== Setup Husky ==============
echo "🐕 Setting up Husky git hooks..."
npx husky install

# ============== Create Environment Template ==============
echo "⚙️ Creating environment template..."
if [ ! -f .env.example ]; then
    cat > .env.example << 'EOF'
# VantisVPN Environment Variables
# Copy this file to .env.local and fill in your values

# Application
NODE_ENV=development
APP_ENV=development
APP_DEBUG=true
APP_PORT=3000
API_PORT=8080

# GitHub
GITHUB_TOKEN=your_github_token_here
GITHUB_REPO=vantisCorp/VantisVPN

# Discord
DISCORD_WEBHOOK_URL=your_discord_webhook_here
DISCORD_BOT_TOKEN=your_discord_bot_token_here

# Slack
SLACK_WEBHOOK_URL=your_slack_webhook_here

# Sentry (Error Tracking)
SENTRY_DSN=your_sentry_dsn_here
SENTRY_ENVIRONMENT=development

# Analytics
PLAUSIBLE_DOMAIN=vantisvpn.com
PLAUSIBLE_URL=https://plausible.io

# Social Media
DISCORD_SERVER_URL=https://discord.gg/A5MzwsRj7D
INSTAGRAM_URL=https://instagram.com/vantisvpn
FACEBOOK_URL=https://facebook.com/vantisvpn
TWITTER_URL=https://twitter.com/vantisvpn
REDDIT_URL=https://reddit.com/r/vantisvpn
LINKEDIN_URL=https://linkedin.com/company/vantisvpn
PATREON_URL=https://patreon.com/vantisvpn
BUYMEACOFFEE_URL=https://buymeacoffee.com/vantisvpn
PAYPAL_URL=https://paypal.me/vantisvpn
KICKSTARTER_URL=https://kickstarter.com/vantisvpn

# Monitoring
ENABLE_METRICS=true
ENABLE_TELEMETRY=true
EOF
fi

# ============== Setup Complete ==============
echo "✅ Development environment setup complete!"
echo ""
echo "📋 Next steps:"
echo "   1. Copy .env.example to .env.local"
echo "   2. Fill in your environment variables"
echo "   3. Run 'make setup' to build the project"
echo "   4. Run 'make dev' to start development"
echo ""
echo "🎯 Happy coding!"