-- Your SQL goes here
-- -- Profiles 
CREATE TABLE profiles (
    id              UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    user_id         UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,

    -- Sensitive fields stored encrypted with pgcrypto
    first_name_enc  BYTEA,
    last_name_enc   BYTEA,
    phone_enc       BYTEA,

    nickname        VARCHAR,
    bio             TEXT,
    birthday        DATE,
    avatar_url      VARCHAR,
    status          BOOLEAN NOT NULL DEFAULT TRUE,
    social_network  JSONB NOT NULL DEFAULT '{}',

    created_at      TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at      TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE UNIQUE INDEX idx_profiles_user_id ON profiles (user_id);
