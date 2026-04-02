use diesel::{ExpressionMethods, OptionalExtension, QueryDsl, RunQueryDsl};
use uuid::Uuid;

use crate::db::schema::conversation_members;
use crate::db::schema::conversations;
use crate::models::conversation::{Conversation, NewConversation};
use crate::repositories::base::BaseRepo;
use crate::repositories::conversations_repository::IConversationRepository;

#[async_trait::async_trait]
impl IConversationRepository for BaseRepo {
    async fn create(&self, item: &NewConversation) -> diesel::QueryResult<Conversation> {
        let item = item.clone();
        self.run(move |conn| {
            diesel::insert_into(conversations::table)
                .values(&item)
                .get_result(conn)
        })
        .await
    }

    async fn find_by_user(&self, user_id: Uuid) -> diesel::QueryResult<Vec<Conversation>> {
        self.run(move |conn| {
            conversations::table
                .filter(
                    conversations::id.eq_any(
                        conversation_members::table
                            .filter(conversation_members::user_id.eq(user_id))
                            .select(conversation_members::conversation_id),
                    ),
                )
                .load::<Conversation>(conn)
        })
        .await
    }

    async fn find_existing_direct_conversation(
        &self,
        user1_id: Uuid,
        user2_id: Uuid,
    ) -> diesel::QueryResult<Option<Conversation>> {
        self.run(move |conn| {
            // Find direct conversations where both users are members
            // Filter: conversation_type = 1 (direct) and both user1 and user2 are members
            conversations::table
                .filter(conversations::conversation_type.eq(1))
                .filter(
                    conversations::id.eq_any(
                        conversation_members::table
                            .filter(conversation_members::user_id.eq(user1_id))
                            .select(conversation_members::conversation_id),
                    ),
                )
                .filter(
                    conversations::id.eq_any(
                        conversation_members::table
                            .filter(conversation_members::user_id.eq(user2_id))
                            .select(conversation_members::conversation_id),
                    ),
                )
                .first::<Conversation>(conn)
                .optional()
        })
        .await
    }
}
