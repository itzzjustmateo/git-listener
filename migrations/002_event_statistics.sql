CREATE TABLE event_statistics (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    repository_id UUID NOT NULL REFERENCES repositories(id) ON DELETE CASCADE,
    event_type TEXT NOT NULL,
    count BIGINT NOT NULL DEFAULT 0,
    last_event_at TIMESTAMPTZ,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE(repository_id, event_type)
);

CREATE INDEX idx_event_stats_repo ON event_statistics(repository_id);
