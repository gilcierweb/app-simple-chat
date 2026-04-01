use async_trait::async_trait;
use uuid::Uuid;
use diesel::{RunQueryDsl, OptionalExtension};
use chrono::Utc;

use crate::repositories::base::BaseRepo;
use crate::repositories::conversations_repository::IMessageRepository;
use crate::models::message::{Message, NewMessage};
use crate::db::schema::messages;

#[async_trait::async_trait]
impl IMessageRepository for BaseRepo {
    async fn create(
        &self,
        conversation_id: Uuid,
        sender_id: Uuid,
        ciphertext: Vec<u8>,
        iv: String,
        message_type: i32,
        reply_to_id: Option<Uuid>,
    ) -> diesel::QueryResult<Message> {
        let now = Utc::now();
        let new_msg = NewMessage {
            id: Uuid::new_v4(),
            conversation_id,
            sender_id,
            ciphertext,
            iv,
            message_type,
            reply_to_id,
            created_at: now,
            updated_at: now,
            deleted_at: None,
        };
        self.run(move |conn| {
            diesel::insert_into(messages::table)
                .values(&new_msg)
                .get_result(conn)
        }).await
    }

    async fn find_by_conversation(
        &self,
        conversation_id: Uuid,
        before: Option<Uuid>,
        limit: i64,
    ) -> diesel::QueryResult<Vec<Message>> {
        use diesel::{QueryDsl, ExpressionMethods, OptionalExtension};

        let before_ts = if let Some(before_id) = before {
            Some(before_id)
        } else {
            None
        };

        self.run(move |conn| {
            let mut query = messages::table
                .filter(messages::conversation_id.eq(conversation_id))
                .filter(messages::deleted_at.is_null())
                .order(messages::created_at.desc())
                .limit(limit)
                .into_boxed();

            if let Some(before_id) = before_ts {
                if let Ok(cursor_msg) = messages::table.find(before_id).first::<Message>(conn) {
                    query = query.filter(messages::created_at.lt(cursor_msg.created_at));
                }
            }

            query.load(conn)
        }).await
    }

    async fn find_by_id(&self, id: Uuid) -> diesel::QueryResult<Option<Message>> {
        use diesel::QueryDsl;
        self.run(move |conn| {
            messages::table.find(id).first(conn).optional()
        }).await
    }

    async fn soft_delete(&self, id: Uuid) -> diesel::QueryResult<()> {
        use diesel::{QueryDsl, ExpressionMethods, RunQueryDsl};
        self.run(move |conn| {
            diesel::update(messages::table.find(id))
                .set((
                    messages::ciphertext.eq(vec![]),
                    messages::deleted_at.eq(Some(Utc::now())),
                ))
                .execute(conn)
        }).await?;
        Ok(())
    }

    async fn update_receipt(&self, message_id: Uuid, user_id: Uuid, status: i32) -> diesel::QueryResult<()> {
        use diesel::{QueryDsl, ExpressionMethods, RunQueryDsl, Insertable};

        let now = Utc::now();

        self.run(move |conn| {
            diesel::insert_into(crate::db::schema::message_receipts::table)
                .values((
                    crate::db::schema::message_receipts::message_id.eq(message_id),
                    crate::db::schema::message_receipts::user_id.eq(user_id),
                    crate::db::schema::message_receipts::status.eq(status),
                    crate::db::schema::message_receipts::delivered_at.eq(Some(now)),
                    crate::db::schema::message_receipts::created_at.eq(now),
                    crate::db::schema::message_receipts::updated_at.eq(now),
                ))
                .on_conflict((
                    crate::db::schema::message_receipts::message_id,
                    crate::db::schema::message_receipts::user_id,
                ))
                .do_update()
                .set((
                    crate::db::schema::message_receipts::status.eq(status),
                    crate::db::schema::message_receipts::read_at.eq(if status == 2 { Some(now) } else { None }),
                ))
                .execute(conn)
        }).await?;
        Ok(())
    }
}