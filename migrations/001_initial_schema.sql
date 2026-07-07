CREATE EXTENSION IF NOT EXISTS "pgcrypto";

CREATE TABLE guilds (
    id BIGINT PRIMARY KEY,
    name TEXT NOT NULL,
    owner_id BIGINT NOT NULL,
    preferred_locale TEXT NOT NULL DEFAULT 'en-US',
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE repositories (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    guild_id BIGINT NOT NULL REFERENCES guilds(id) ON DELETE CASCADE,
    provider TEXT NOT NULL,
    provider_repo_id TEXT,
    owner_name TEXT NOT NULL,
    repo_name TEXT NOT NULL,
    full_name TEXT NOT NULL,
    webhook_secret_hash TEXT NOT NULL,
    destination_channel_id BIGINT NOT NULL,
    is_active BOOLEAN NOT NULL DEFAULT TRUE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE(guild_id, full_name)
);

CREATE INDEX idx_repositories_guild_id ON repositories(guild_id);
CREATE INDEX idx_repositories_full_name ON repositories(full_name);

CREATE TABLE webhook_endpoints (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    repository_id UUID NOT NULL REFERENCES repositories(id) ON DELETE CASCADE,
    endpoint_token TEXT NOT NULL UNIQUE,
    is_active BOOLEAN NOT NULL DEFAULT TRUE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_webhook_endpoints_token ON webhook_endpoints(endpoint_token);

CREATE TABLE event_configs (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    repository_id UUID NOT NULL REFERENCES repositories(id) ON DELETE CASCADE,
    event_type TEXT NOT NULL,
    is_enabled BOOLEAN NOT NULL DEFAULT TRUE,
    UNIQUE(repository_id, event_type)
);

CREATE INDEX idx_event_configs_repo ON event_configs(repository_id);

CREATE TABLE filters (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    repository_id UUID NOT NULL REFERENCES repositories(id) ON DELETE CASCADE,
    filter_type TEXT NOT NULL,
    filter_value TEXT NOT NULL,
    filter_action TEXT NOT NULL CHECK (filter_action IN ('include', 'exclude')),
    is_active BOOLEAN NOT NULL DEFAULT TRUE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE(repository_id, filter_type, filter_value)
);

CREATE INDEX idx_filters_repo ON filters(repository_id);

CREATE TABLE templates (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    repository_id UUID NOT NULL REFERENCES repositories(id) ON DELETE CASCADE,
    event_type TEXT NOT NULL,
    template_content TEXT NOT NULL,
    is_active BOOLEAN NOT NULL DEFAULT TRUE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE(repository_id, event_type)
);

CREATE TABLE embed_configs (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    repository_id UUID NOT NULL REFERENCES repositories(id) ON DELETE CASCADE,
    color INTEGER,
    username TEXT,
    avatar_url TEXT,
    use_thread BOOLEAN NOT NULL DEFAULT FALSE,
    thread_name TEXT,
    use_forum BOOLEAN NOT NULL DEFAULT FALSE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE(repository_id)
);

CREATE TABLE mention_configs (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    repository_id UUID NOT NULL REFERENCES repositories(id) ON DELETE CASCADE,
    mention_type TEXT NOT NULL CHECK (mention_type IN ('role', 'user', 'here', 'everyone')),
    mention_id BIGINT,
    event_type TEXT,
    is_active BOOLEAN NOT NULL DEFAULT TRUE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_mention_configs_repo ON mention_configs(repository_id);

CREATE TABLE audit_logs (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    guild_id BIGINT NOT NULL REFERENCES guilds(id) ON DELETE CASCADE,
    actor_id BIGINT NOT NULL,
    action TEXT NOT NULL,
    details JSONB,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_audit_logs_guild ON audit_logs(guild_id);
CREATE INDEX idx_audit_logs_created ON audit_logs(created_at);
