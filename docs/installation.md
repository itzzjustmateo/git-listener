# Installation

## Prerequisites

- Rust 1.85+ (2024 edition)
- PostgreSQL 16+
- Discord Bot Token
- Discord Application ID

## Quick Start (Docker)

```bash
# Clone the repository
git clone https://github.com/itzzjustmateo/git-listener.git
cd git-listener

# Copy environment file
cp .env.example .env
# Edit .env with your Discord token, application ID, and database URL:
#   GL_DISCORD__TOKEN=your_bot_token
#   GL_DISCORD__APPLICATION_ID=your_application_id
#   GL_DATABASE__URL=postgres://postgres:postgres@localhost:5432/git_listener

# Start with Docker Compose
docker compose up -d
```

## Manual Setup

```bash
# Build from source
cargo build --release

# Set environment variables
export GL_DISCORD__TOKEN="your_bot_token"
export GL_DISCORD__APPLICATION_ID="your_application_id"
export GL_DATABASE__URL="postgres://user:pass@localhost:5432/git_listener"

# Run database migrations and start the bot
cargo run --release

# The bot will start and register slash commands globally
```
