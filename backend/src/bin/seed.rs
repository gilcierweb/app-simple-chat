#[path = "../auth/mod.rs"]
mod auth;
#[path = "../db/schema.rs"]
mod schema;

use std::collections::HashMap;

use auth::password::password_hash;
use chrono::{Duration, Utc};
use diesel::prelude::*;
use diesel::{Connection, OptionalExtension, PgConnection};
use serde_json::json;
use uuid::Uuid;

use schema::{
    conversation_members, conversations, message_receipts, messages, profiles, roles, user_keys,
    user_roles, users,
};

#[derive(Debug, Clone)]
struct SeededUser {
    id: Uuid,
    email: String,
    role: String,
}

#[derive(Debug, Clone)]
struct SeedMessage {
    id: Uuid,
    conversation_id: Uuid,
    sender_email: &'static str,
    body: &'static str,
    iv: &'static str,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv().ok();

    let database_url = std::env::var("DATABASE_URL")?;
    let mut conn = PgConnection::establish(&database_url)?;

    conn.transaction::<_, diesel::result::Error, _>(|conn| {
        let role_ids = seed_roles(conn)?;
        let seeded_users = seed_users(conn)?;
        seed_profiles(conn, &seeded_users)?;
        seed_user_roles(conn, &seeded_users, &role_ids)?;
        seed_user_keys(conn, &seeded_users)?;

        let conv_ids = seed_conversations(conn, &seeded_users)?;
        seed_conversation_members(conn, &seeded_users, &conv_ids)?;
        seed_messages(conn, &seeded_users, &conv_ids)?;
        seed_message_receipts(conn, &seeded_users)?;

        Ok(())
    })?;

    println!("\nSeed completed successfully.");
    println!("\nTest users (password for all: Password123!):");
    for user in TEST_USERS {
        println!("- {} ({})", user.email, user.role);
    }

    Ok(())
}

fn seed_roles(conn: &mut PgConnection) -> QueryResult<HashMap<String, Uuid>> {
    let now = Utc::now();
    let mut role_ids = HashMap::new();

    for role_name in ["admin", "operator", "viewer"] {
        let existing = roles::table
            .filter(roles::name.eq(role_name))
            .filter(roles::resource_type.is_null())
            .filter(roles::resource_id.is_null())
            .select(roles::id)
            .first::<Uuid>(conn)
            .optional()?;

        let role_id = if let Some(id) = existing {
            id
        } else {
            let id = Uuid::new_v4();
            diesel::insert_into(roles::table)
                .values((
                    roles::id.eq(id),
                    roles::name.eq(role_name),
                    roles::resource_type.eq(None::<String>),
                    roles::resource_id.eq(None::<Uuid>),
                    roles::created_at.eq(now),
                    roles::updated_at.eq(now),
                ))
                .execute(conn)?;
            id
        };

        role_ids.insert(role_name.to_string(), role_id);
    }

    Ok(role_ids)
}

fn seed_users(conn: &mut PgConnection) -> QueryResult<Vec<SeededUser>> {
    let now = Utc::now();
    let mut seeded = Vec::with_capacity(TEST_USERS.len());

    for spec in TEST_USERS {
        let wanted_id = Uuid::parse_str(spec.id).expect("Invalid user UUID in TEST_USERS");

        let existing = users::table
            .filter(users::email.eq(spec.email))
            .select(users::id)
            .first::<Uuid>(conn)
            .optional()?;

        let user_id = if let Some(existing_id) = existing {
            existing_id
        } else {
            let hash = password_hash(spec.password.to_string());
            diesel::insert_into(users::table)
                .values((
                    users::id.eq(wanted_id),
                    users::email.eq(spec.email),
                    users::password_hash.eq(hash),
                    users::sign_in_count.eq(0),
                    users::failed_attempts.eq(0),
                    users::totp_enabled.eq(false),
                    users::confirmed_at.eq(Some(now)),
                    users::created_at.eq(now),
                    users::updated_at.eq(now),
                ))
                .execute(conn)?;
            wanted_id
        };

        seeded.push(SeededUser {
            id: user_id,
            email: spec.email.to_string(),
            role: spec.role.to_string(),
        });
    }

    Ok(seeded)
}

fn seed_profiles(conn: &mut PgConnection, users_seeded: &[SeededUser]) -> QueryResult<()> {
    for seeded in users_seeded {
        let spec = user_spec_by_email(&seeded.email);

        diesel::insert_into(profiles::table)
            .values((
                profiles::id.eq(Uuid::new_v4()),
                profiles::user_id.eq(seeded.id),
                profiles::first_name_enc.eq(Some(spec.first_name.as_bytes().to_vec())),
                profiles::last_name_enc.eq(Some(spec.last_name.as_bytes().to_vec())),
                profiles::phone_enc.eq(None::<Vec<u8>>),
                profiles::nickname.eq(Some(spec.nickname.to_string())),
                profiles::bio.eq(Some(spec.bio.to_string())),
                profiles::birthday.eq(None::<chrono::NaiveDate>),
                profiles::avatar_url.eq(None::<String>),
                profiles::status.eq(true),
                profiles::social_network.eq(json!({
                    "github": spec.github,
                    "x": spec.x,
                })),
            ))
            .on_conflict(profiles::user_id)
            .do_nothing()
            .execute(conn)?;
    }

    Ok(())
}

fn seed_user_roles(
    conn: &mut PgConnection,
    users_seeded: &[SeededUser],
    role_ids: &HashMap<String, Uuid>,
) -> QueryResult<()> {
    for seeded in users_seeded {
        if let Some(role_id) = role_ids.get(&seeded.role) {
            diesel::insert_into(user_roles::table)
                .values((
                    user_roles::user_id.eq(seeded.id),
                    user_roles::role_id.eq(*role_id),
                ))
                .on_conflict_do_nothing()
                .execute(conn)?;
        }
    }

    Ok(())
}

fn seed_user_keys(conn: &mut PgConnection, users_seeded: &[SeededUser]) -> QueryResult<()> {
    let now = Utc::now();

    for seeded in users_seeded {
        for key_type in [1, 2, 3] {
            let exists = user_keys::table
                .filter(user_keys::user_id.eq(seeded.id))
                .filter(user_keys::key_type.eq(key_type))
                .select(user_keys::id)
                .first::<Uuid>(conn)
                .optional()?;

            if exists.is_none() {
                let key_payload = format!("seed-{}-key-type-{}", seeded.email, key_type);
                diesel::insert_into(user_keys::table)
                    .values((
                        user_keys::id.eq(Uuid::new_v4()),
                        user_keys::user_id.eq(seeded.id),
                        user_keys::key_type.eq(key_type),
                        user_keys::public_key.eq(key_payload),
                        user_keys::signature.eq(None::<String>),
                        user_keys::created_at.eq(now),
                        user_keys::updated_at.eq(now),
                        user_keys::expires_at.eq(None::<chrono::DateTime<Utc>>),
                        user_keys::used_at.eq(None::<chrono::DateTime<Utc>>),
                    ))
                    .execute(conn)?;
            }
        }
    }

    Ok(())
}

fn seed_conversations(
    conn: &mut PgConnection,
    users_seeded: &[SeededUser],
) -> QueryResult<HashMap<&'static str, Uuid>> {
    let now = Utc::now();
    let id_by_email = user_map(users_seeded);

    let convs = [
        (
            "admin_bob",
            "56b1b77f-2aa6-4172-bf2d-c42ba5d0eb3e",
            1,
            "admin@chatapp.com",
            None::<Vec<u8>>,
        ),
        (
            "admin_alice",
            "7b60e804-6ca3-444c-a471-0c57b55decb5",
            1,
            "admin@chatapp.com",
            None::<Vec<u8>>,
        ),
        (
            "team_room",
            "179794b8-0679-4e51-b976-ccba427a16c7",
            2,
            "admin@chatapp.com",
            Some(b"core-team".to_vec()),
        ),
    ];

    let mut map = HashMap::new();

    for (label, conv_id_str, conv_type, owner_email, name_enc) in convs {
        let conv_id = Uuid::parse_str(conv_id_str).expect("Invalid conversation UUID");
        let owner_id = *id_by_email.get(owner_email).expect("Owner user not seeded");

        diesel::insert_into(conversations::table)
            .values((
                conversations::id.eq(conv_id),
                conversations::conversation_type.eq(conv_type),
                conversations::name_enc.eq(name_enc.clone()),
                conversations::avatar_url.eq(None::<String>),
                conversations::created_by.eq(owner_id),
                conversations::created_at.eq(now),
                conversations::updated_at.eq(now),
            ))
            .on_conflict(conversations::id)
            .do_nothing()
            .execute(conn)?;

        map.insert(label, conv_id);
    }

    Ok(map)
}

fn seed_conversation_members(
    conn: &mut PgConnection,
    users_seeded: &[SeededUser],
    conv_ids: &HashMap<&'static str, Uuid>,
) -> QueryResult<()> {
    let now = Utc::now();
    let id_by_email = user_map(users_seeded);

    let members = [
        ("admin_bob", "admin@chatapp.com", 1),
        ("admin_bob", "bob@chatapp.com", 3),
        ("admin_alice", "admin@chatapp.com", 1),
        ("admin_alice", "alice@chatapp.com", 3),
        ("team_room", "admin@chatapp.com", 1),
        ("team_room", "bob@chatapp.com", 2),
        ("team_room", "alice@chatapp.com", 3),
        ("team_room", "carol@chatapp.com", 3),
        ("team_room", "dave@chatapp.com", 3),
        ("team_room", "eve@chatapp.com", 3),
    ];

    for (room_label, email, member_role) in members {
        let conversation_id = *conv_ids
            .get(room_label)
            .expect("Conversation missing in map");
        let user_id = *id_by_email.get(email).expect("User missing in map");

        diesel::insert_into(conversation_members::table)
            .values((
                conversation_members::conversation_id.eq(conversation_id),
                conversation_members::user_id.eq(user_id),
                conversation_members::role.eq(member_role),
                conversation_members::joined_at.eq(now),
                conversation_members::last_read_at.eq(None::<chrono::DateTime<Utc>>),
                conversation_members::created_at.eq(now),
                conversation_members::updated_at.eq(now),
            ))
            .on_conflict_do_nothing()
            .execute(conn)?;
    }

    Ok(())
}

fn seed_messages(
    conn: &mut PgConnection,
    users_seeded: &[SeededUser],
    conv_ids: &HashMap<&'static str, Uuid>,
) -> QueryResult<()> {
    let now = Utc::now();
    let id_by_email = user_map(users_seeded);

    let items = [
        SeedMessage {
            id: Uuid::parse_str("1f9d9bae-40c2-425c-9190-a6b23fba4a54").expect("invalid uuid"),
            conversation_id: *conv_ids.get("admin_bob").expect("missing conv"),
            sender_email: "admin@chatapp.com",
            body: "seed: hello bob",
            iv: "Wq1gNyk0yFiiaUQM",
        },
        SeedMessage {
            id: Uuid::parse_str("e940035d-d5b5-4904-9884-c730f4a1c3b1").expect("invalid uuid"),
            conversation_id: *conv_ids.get("admin_bob").expect("missing conv"),
            sender_email: "bob@chatapp.com",
            body: "seed: hi admin",
            iv: "HETN0K7nVdroZMkY",
        },
        SeedMessage {
            id: Uuid::parse_str("69cf4fe4-b9b7-4590-b506-118cc28fb51d").expect("invalid uuid"),
            conversation_id: *conv_ids.get("team_room").expect("missing conv"),
            sender_email: "carol@chatapp.com",
            body: "seed: team sync at 10",
            iv: "lA2d4D6xN8qQ7mYu",
        },
    ];

    for (index, item) in items.iter().enumerate() {
        let sender_id = *id_by_email
            .get(item.sender_email)
            .expect("message sender missing");

        diesel::insert_into(messages::table)
            .values((
                messages::id.eq(item.id),
                messages::conversation_id.eq(item.conversation_id),
                messages::sender_id.eq(sender_id),
                messages::ciphertext.eq(item.body.as_bytes().to_vec()),
                messages::iv.eq(item.iv),
                messages::message_type.eq(1),
                messages::reply_to_id.eq(None::<Uuid>),
                messages::created_at.eq(now + Duration::seconds(index as i64)),
                messages::updated_at.eq(now + Duration::seconds(index as i64)),
                messages::deleted_at.eq(None::<chrono::DateTime<Utc>>),
            ))
            .on_conflict(messages::id)
            .do_nothing()
            .execute(conn)?;
    }

    Ok(())
}

fn seed_message_receipts(conn: &mut PgConnection, users_seeded: &[SeededUser]) -> QueryResult<()> {
    let now = Utc::now();
    let id_by_email = user_map(users_seeded);

    let items = [
        (
            Uuid::parse_str("1f9d9bae-40c2-425c-9190-a6b23fba4a54").expect("invalid uuid"),
            "bob@chatapp.com",
            1,
        ),
        (
            Uuid::parse_str("e940035d-d5b5-4904-9884-c730f4a1c3b1").expect("invalid uuid"),
            "admin@chatapp.com",
            2,
        ),
        (
            Uuid::parse_str("69cf4fe4-b9b7-4590-b506-118cc28fb51d").expect("invalid uuid"),
            "admin@chatapp.com",
            1,
        ),
    ];

    for (message_id, email, status) in items {
        let user_id = *id_by_email.get(email).expect("receipt user missing");

        diesel::insert_into(message_receipts::table)
            .values((
                message_receipts::message_id.eq(message_id),
                message_receipts::user_id.eq(user_id),
                message_receipts::status.eq(status),
                message_receipts::delivered_at.eq(Some(now)),
                message_receipts::read_at.eq(if status == 2 { Some(now) } else { None }),
                message_receipts::created_at.eq(now),
                message_receipts::updated_at.eq(now),
            ))
            .on_conflict_do_nothing()
            .execute(conn)?;
    }

    Ok(())
}

fn user_map(users_seeded: &[SeededUser]) -> HashMap<&str, Uuid> {
    users_seeded
        .iter()
        .map(|u| (u.email.as_str(), u.id))
        .collect::<HashMap<&str, Uuid>>()
}

fn user_spec_by_email(email: &str) -> &'static TestUserSpec {
    TEST_USERS
        .iter()
        .find(|u| u.email == email)
        .expect("Unknown user email in seed")
}

#[derive(Debug, Clone, Copy)]
struct TestUserSpec {
    id: &'static str,
    email: &'static str,
    password: &'static str,
    first_name: &'static str,
    last_name: &'static str,
    nickname: &'static str,
    bio: &'static str,
    github: &'static str,
    x: &'static str,
    role: &'static str,
}

const TEST_USERS: [TestUserSpec; 6] = [
    TestUserSpec {
        id: "92666b8d-ff27-4fa0-9ed1-2b43e456b39b",
        email: "admin@chatapp.com",
        password: "Password123!",
        first_name: "Admin",
        last_name: "Root",
        nickname: "admin",
        bio: "System administrator account for local testing.",
        github: "gilcierweb",
        x: "admin_chatapp",
        role: "admin",
    },
    TestUserSpec {
        id: "7607b7e1-e0cd-47f5-afde-9dbb5578c37f",
        email: "bob@chatapp.com",
        password: "Password123!",
        first_name: "Bob",
        last_name: "Builder",
        nickname: "bob",
        bio: "Operator user for integration and websocket tests.",
        github: "bob-chat",
        x: "bob_chatapp",
        role: "operator",
    },
    TestUserSpec {
        id: "91e9c417-7ff7-4e75-acb2-29d0c4cb3cdd",
        email: "alice@chatapp.com",
        password: "Password123!",
        first_name: "Alice",
        last_name: "Wong",
        nickname: "alice",
        bio: "Viewer account used in encrypted direct message scenarios.",
        github: "alice-viewer",
        x: "alice_chatapp",
        role: "viewer",
    },
    TestUserSpec {
        id: "249d3e8c-ca4d-46c4-9035-b0f385c6d1cc",
        email: "carol@chatapp.com",
        password: "Password123!",
        first_name: "Carol",
        last_name: "Meyer",
        nickname: "carol",
        bio: "Viewer account for conversation navigation regression tests.",
        github: "carol-labs",
        x: "carol_chatapp",
        role: "viewer",
    },
    TestUserSpec {
        id: "5fd5b3a5-9bc9-4ea4-95d3-2e8a83f89b7a",
        email: "dave@chatapp.com",
        password: "Password123!",
        first_name: "Dave",
        last_name: "Silva",
        nickname: "dave",
        bio: "Operator account for group conversation moderation tests.",
        github: "dave-ops",
        x: "dave_chatapp",
        role: "operator",
    },
    TestUserSpec {
        id: "39fd81de-df4a-4f6d-a6e1-b8570ca3e4a2",
        email: "eve@chatapp.com",
        password: "Password123!",
        first_name: "Eve",
        last_name: "Souza",
        nickname: "eve",
        bio: "Viewer account for end-to-end UX checks.",
        github: "eve-view",
        x: "eve_chatapp",
        role: "viewer",
    },
];
