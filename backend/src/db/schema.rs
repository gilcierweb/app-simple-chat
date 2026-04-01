// @generated automatically by Diesel CLI.

diesel::table! {
    audit_logs (id) {
        id -> Uuid,
        user_id -> Nullable<Uuid>,
        #[max_length = 100]
        action -> Varchar,
        #[max_length = 100]
        resource_type -> Nullable<Varchar>,
        resource_id -> Nullable<Uuid>,
        #[max_length = 45]
        ip_address -> Nullable<Varchar>,
        user_agent -> Nullable<Text>,
        metadata -> Nullable<Jsonb>,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::table! {
    conversation_members (conversation_id, user_id) {
        conversation_id -> Uuid,
        user_id -> Uuid,
        role -> Int4,
        joined_at -> Timestamptz,
        last_read_at -> Nullable<Timestamptz>,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::table! {
    conversations (id) {
        id -> Uuid,
        conversation_type -> Int4,
        name_enc -> Nullable<Bytea>,
        avatar_url -> Nullable<Varchar>,
        created_by -> Uuid,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::table! {
    message_receipts (message_id, user_id) {
        message_id -> Uuid,
        user_id -> Uuid,
        status -> Int4,
        delivered_at -> Nullable<Timestamptz>,
        read_at -> Nullable<Timestamptz>,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::table! {
    messages (id) {
        id -> Uuid,
        conversation_id -> Uuid,
        sender_id -> Uuid,
        ciphertext -> Bytea,
        iv -> Varchar,
        message_type -> Int4,
        reply_to_id -> Nullable<Uuid>,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
        deleted_at -> Nullable<Timestamptz>,
    }
}

diesel::table! {
    profiles (id) {
        id -> Uuid,
        user_id -> Uuid,
        first_name_enc -> Nullable<Bytea>,
        last_name_enc -> Nullable<Bytea>,
        phone_enc -> Nullable<Bytea>,
        nickname -> Nullable<Varchar>,
        bio -> Nullable<Text>,
        birthday -> Nullable<Date>,
        avatar_url -> Nullable<Varchar>,
        status -> Bool,
        social_network -> Jsonb,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::table! {
    refresh_tokens (id) {
        id -> Uuid,
        user_id -> Uuid,
        #[max_length = 255]
        token_hash -> Varchar,
        device_info -> Nullable<Text>,
        #[max_length = 45]
        ip_address -> Nullable<Varchar>,
        expires_at -> Timestamptz,
        revoked_at -> Nullable<Timestamptz>,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::table! {
    roles (id) {
        id -> Uuid,
        name -> Varchar,
        resource_type -> Nullable<Varchar>,
        resource_id -> Nullable<Uuid>,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::table! {
    user_keys (id) {
        id -> Uuid,
        user_id -> Uuid,
        key_type -> Int4,
        public_key -> Text,
        signature -> Nullable<Text>,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
        expires_at -> Nullable<Timestamptz>,
        used_at -> Nullable<Timestamptz>,
    }
}

diesel::table! {
    user_roles (user_id, role_id) {
        user_id -> Uuid,
        role_id -> Uuid,
    }
}

diesel::table! {
    users (id) {
        id -> Uuid,
        email -> Varchar,
        password_hash -> Varchar,
        reset_password_token -> Nullable<Varchar>,
        reset_password_sent_at -> Nullable<Timestamptz>,
        remember_created_at -> Nullable<Timestamptz>,
        sign_in_count -> Int4,
        current_sign_in_at -> Nullable<Timestamptz>,
        last_sign_in_at -> Nullable<Timestamptz>,
        current_sign_in_ip -> Nullable<Varchar>,
        last_sign_in_ip -> Nullable<Varchar>,
        confirmation_token -> Nullable<Varchar>,
        confirmed_at -> Nullable<Timestamptz>,
        confirmation_sent_at -> Nullable<Timestamptz>,
        unconfirmed_email -> Nullable<Varchar>,
        failed_attempts -> Int4,
        unlock_token -> Nullable<Varchar>,
        locked_at -> Nullable<Timestamptz>,
        totp_secret -> Nullable<Varchar>,
        totp_enabled -> Bool,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::joinable!(audit_logs -> users (user_id));
diesel::joinable!(conversation_members -> conversations (conversation_id));
diesel::joinable!(conversation_members -> users (user_id));
diesel::joinable!(conversations -> users (created_by));
diesel::joinable!(message_receipts -> messages (message_id));
diesel::joinable!(message_receipts -> users (user_id));
diesel::joinable!(messages -> conversations (conversation_id));
diesel::joinable!(messages -> users (sender_id));
diesel::joinable!(profiles -> users (user_id));
diesel::joinable!(refresh_tokens -> users (user_id));
diesel::joinable!(user_keys -> users (user_id));
diesel::joinable!(user_roles -> roles (role_id));
diesel::joinable!(user_roles -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    audit_logs,
    conversation_members,
    conversations,
    message_receipts,
    messages,
    profiles,
    refresh_tokens,
    roles,
    user_keys,
    user_roles,
    users,
);
