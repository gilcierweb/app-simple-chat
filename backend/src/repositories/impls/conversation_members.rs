use async_trait::async_trait;
use diesel::{OptionalExtension, RunQueryDsl};
use uuid::Uuid;

use crate::db::schema::conversation_members;
use crate::models::conversation_member::{ConversationMember, NewConversationMember};
use crate::repositories::base::BaseRepo;
use crate::repositories::conversations_repository::IConversationMemberRepository;

#[async_trait::async_trait]
impl IConversationMemberRepository for BaseRepo {
    async fn create(
        &self,
        item: &NewConversationMember,
    ) -> diesel::QueryResult<ConversationMember> {
        let item = item.clone();
        self.run(move |conn| {
            diesel::insert_into(conversation_members::table)
                .values(&item)
                .get_result(conn)
        })
        .await
    }

    async fn find_by_conversation(
        &self,
        conversation_id: Uuid,
    ) -> diesel::QueryResult<Vec<ConversationMember>> {
        use diesel::{ExpressionMethods, QueryDsl};
        self.run(move |conn| {
            conversation_members::table
                .filter(conversation_members::conversation_id.eq(conversation_id))
                .load::<ConversationMember>(conn)
        })
        .await
    }

    async fn find(
        &self,
        conversation_id: Uuid,
        user_id: Uuid,
    ) -> diesel::QueryResult<Option<ConversationMember>> {
        use diesel::{ExpressionMethods, QueryDsl};
        self.run(move |conn| {
            conversation_members::table
                .filter(conversation_members::conversation_id.eq(conversation_id))
                .filter(conversation_members::user_id.eq(user_id))
                .first(conn)
                .optional()
        })
        .await
    }
}
