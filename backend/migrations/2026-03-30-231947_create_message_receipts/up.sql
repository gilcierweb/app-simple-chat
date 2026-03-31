-- Your SQL goes here
-- -- Message Receipts 
CREATE TABLE message_receipts (
    message_id      UUID NOT NULL REFERENCES messages(id) ON DELETE CASCADE,
    user_id         UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    -- 'delivered' | 'read' - enum
    status          INTEGER NOT NULL DEFAULT 1,
    delivered_at    TIMESTAMPTZ,
    read_at         TIMESTAMPTZ,
    created_at          TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at          TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    PRIMARY KEY (message_id, user_id)
);

CREATE INDEX idx_receipts_user ON message_receipts (user_id);