# Configuration

Git Listener uses a layered configuration system:

1. `config/default.toml` (default values)
2. `config/{environment}.toml` (environment overrides)
3. Environment variables with `GL_` prefix
4. CLI arguments

## Environment Variables

| Variable | Description | Default |
|----------|-------------|---------|
| `GL_DISCORD__TOKEN` | Discord bot token | (required) |
| `GL_DISCORD__APPLICATION_ID` | Discord application ID | (required) |
| `GL_DATABASE__URL` | PostgreSQL connection URL | `postgres://postgres:postgres@localhost:5432/git_listener` |
| `GL_WEBHOOK__BIND_PORT` | Webhook server port | `8080` |
| `GL_API__ENABLED` | Enable REST API | `false` |
| `GL_LOGGING__LEVEL` | Log level (trace/debug/info/warn/error) | `info` |
| `GL_LOGGING__FORMAT` | Log format (text/json) | `text` |
