-- Your SQL goes here
-- -- Messages (ciphertext only - never plaintext) 
CREATE TABLE messages (
    id              UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    conversation_id UUID NOT NULL REFERENCES conversations(id) ON DELETE CASCADE,
    sender_id       UUID NOT NULL REFERENCES users(id),
    -- AES-256-GCM ciphertext (base64 decoded, stored as bytes)
    ciphertext      BYTEA NOT NULL,
    -- Base64-encoded IV
    iv              VARCHAR NOT NULL,
    -- 'text' | 'image' | 'file' | 'audio' - enum
    message_type    INTEGER NOT NULL DEFAULT 1,
    reply_to_id     UUID REFERENCES messages(id),
    created_at          TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at          TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    deleted_at      TIMESTAMPTZ
);

CREATE INDEX idx_messages_conv_created ON messages (conversation_id, created_at DESC);
CREATE INDEX idx_messages_sender ON messages (sender_id);
