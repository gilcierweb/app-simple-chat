-- Your SQL goes here
-- ─── E2E Keys ─────────────────────────────────────────────────────────────────
CREATE TABLE user_keys (
    id          UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    user_id     UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    -- 'identity' | 'signed_prekey' | 'one_time_prekey' - enum
    key_type    INTEGER NOT NULL DEFAULT 1,
    -- Base64-encoded X25519 public key
    public_key  TEXT NOT NULL,
    -- Signature (for signed prekeys)
    signature   TEXT,
    created_at          TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at          TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    expires_at  TIMESTAMPTZ,
    used_at     TIMESTAMPTZ
);

CREATE INDEX idx_user_keys_user_type ON user_keys (user_id, key_type);
CREATE INDEX idx_user_keys_unused_otpk ON user_keys (user_id) WHERE key_type = 3 AND used_at IS NULL;
