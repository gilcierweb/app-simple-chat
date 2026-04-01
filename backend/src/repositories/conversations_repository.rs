#![allow(dead_code)]

use async_trait::async_trait;
use diesel::QueryResult;
use uuid::Uuid;

use crate::models::conversation::{Conversation, NewConversation};

#[async_trait]
pub trait IConversationRepository: Send + Sync {
    async fn create(&self, item: &NewConversation) -> QueryResult<Conversation>;
    async fn find_by_user(&self, user_id: Uuid) -> QueryResult<Vec<Conversation>>;
    async fn find_existing_direct_conversation(&self, user1_id: Uuid, user2_id: Uuid) -> QueryResult<Option<Conversation>>;
}

use crate::models::conversation_member::{ConversationMember, NewConversationMember};

#[async_trait]
pub trait IConversationMemberRepository: Send + Sync {
    async fn create(&self, item: &NewConversationMember) -> QueryResult<ConversationMember>;
    async fn find_by_conversation(&self, conversation_id: Uuid) -> QueryResult<Vec<ConversationMember>>;
    async fn find(&self, conversation_id: Uuid, user_id: Uuid) -> QueryResult<Option<ConversationMember>>;
}

use crate::models::message::{Message, NewMessage};

#[async_trait]
pub trait IMessageRepository: Send + Sync {
    async fn create(
        &self,
        conversation_id: Uuid,
        sender_id: Uuid,
        ciphertext: Vec<u8>,
        iv: String,
        message_type: i32,
        reply_to_id: Option<Uuid>,
    ) -> QueryResult<Message>;
    async fn find_by_conversation(
        &self,
        conversation_id: Uuid,
        before: Option<Uuid>,
        limit: i64,
    ) -> QueryResult<Vec<Message>>;
    async fn find_by_id(&self, id: Uuid) -> QueryResult<Option<Message>>;
    async fn soft_delete(&self, id: Uuid) -> QueryResult<()>;
    async fn update_receipt(&self, message_id: Uuid, user_id: Uuid, status: i32) -> QueryResult<()>;
}
