# Configuration

Git Listener uses a layered configuration system:

1. `config/default.toml` (default values)
2. `config/{environment}.toml` (environment overrides, selected by `GL_APP__ENVIRONMENT`)
3. Environment variables with `GL_` prefix
4. CLI arguments

## CLI Arguments

| Flag | Env Variable | Description | Default |
|------|--------------|-------------|---------|
| `-c`, `--config` | `GL_CONFIG` | Path to config directory | `config/` |
| `-d`, `--database-url` | `GL_DATABASE__URL` | PostgreSQL connection URL | *(from config)* |
| `-t`, `--discord-token` | `GL_DISCORD__TOKEN` | Discord bot token | *(required)* |
| `--app-id` | `GL_DISCORD__APPLICATION_ID` | Discord application ID | *(from config)* |
| `-w`, `--webhook-port` | `GL_WEBHOOK__BIND_PORT` | Webhook server port | `8080` |
| `-a`, `--api-port` | `GL_API__BIND_PORT` | API server port | `8081` |
| `-l`, `--log-level` | `GL_LOGGING__LEVEL` | Log level (trace/debug/info/warn/error) | `info` |

## Environment Variables

| Variable | Description | Default |
|----------|-------------|---------|
| `GL_APP__ENVIRONMENT` | Runtime environment (development/production) | `development` |
| `GL_DISCORD__TOKEN` | Discord bot token | *(required)* |
| `GL_DISCORD__APPLICATION_ID` | Discord application ID | *(required)* |
| `GL_DATABASE__URL` | PostgreSQL connection URL | `postgres://postgres:postgres@localhost:5432/git_listener` |
| `GL_DATABASE__MAX_CONNECTIONS` | Maximum database pool connections | `20` |
| `GL_WEBHOOK__BIND_ADDRESS` | Webhook server bind address | `0.0.0.0` |
| `GL_WEBHOOK__BIND_PORT` | Webhook server port | `8080` |
| `GL_WEBHOOK__MAX_BODY_SIZE` | Maximum webhook payload size (bytes) | `5242880` |
| `GL_API__ENABLED` | Enable REST API | `false` |
| `GL_API__BIND_ADDRESS` | API server bind address | `127.0.0.1` |
| `GL_API__BIND_PORT` | API server port | `8081` |
| `GL_LOGGING__LEVEL` | Log level (trace/debug/info/warn/error) | `info` |
| `GL_LOGGING__FORMAT` | Log format (text/json) | `text` |

All environment variables use `__` as a separator for nested keys (e.g., `GL_DATABASE__URL` maps to `database.url` in the config file).
