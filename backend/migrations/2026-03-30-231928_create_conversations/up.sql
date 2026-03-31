-- Your SQL goes here
-- -- Conversations
CREATE TABLE conversations (
    id                  UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    -- 'direct' | 'group' - enum
    conversation_type   INTEGER NOT NULL DEFAULT 1,
    -- Encrypted group name (null for direct)
    name_enc            BYTEA,
    avatar_url          VARCHAR,
    created_by          UUID NOT NULL REFERENCES users(id),
    created_at          TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at          TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
CREATE INDEX idx_conversations_type ON conversations (conversation_type);
CREATE INDEX idx_conversations_created_by ON conversations (created_by);
CREATE INDEX idx_conversations_type_updated ON conversations (conversation_type, updated_at DESC);
