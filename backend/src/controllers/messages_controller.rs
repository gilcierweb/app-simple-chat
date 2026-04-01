#![allow(dead_code)]

use actix_web::{web, HttpResponse};
use base64::{engine::general_purpose::STANDARD as B64, Engine};
use uuid::Uuid;

use crate::errors::AppError;
use crate::middleware::auth::AuthUser;
use crate::repositories::container::AppContainer;

#[derive(Debug, serde::Deserialize)]
pub struct ListMessagesQuery {
    pub before: Option<Uuid>,
    pub limit: Option<i64>,
}

#[derive(Debug, serde::Deserialize)]
pub struct SendMessageRequest {
    pub ciphertext: String,
    pub iv: String,
    pub message_type: i32,
    pub reply_to_id: Option<Uuid>,
}

#[derive(Debug, serde::Serialize)]
pub struct MessageResponse {
    pub id: Uuid,
    pub conversation_id: Uuid,
    pub sender_id: Uuid,
    pub ciphertext: String,
    pub iv: String,
    pub message_type: String,
    pub reply_to_id: Option<Uuid>,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

impl From<crate::models::message::Message> for MessageResponse {
    fn from(msg: crate::models::message::Message) -> Self {
        Self {
            id: msg.id,
            conversation_id: msg.conversation_id,
            sender_id: msg.sender_id,
            ciphertext: B64.encode(&msg.ciphertext),
            iv: msg.iv,
            message_type: match msg.message_type {
                1 => "text".to_string(),
                2 => "image".to_string(),
                3 => "file".to_string(),
                4 => "audio".to_string(),
                _ => "text".to_string(),
            },
            reply_to_id: msg.reply_to_id,
            created_at: msg.created_at,
        }
    }
}

/// GET /messages/:conversation_id?before=UUID&limit=50
#[actix_web::get("/messages/{conversation_id}")]
pub async fn list(
    user: AuthUser,
    path: web::Path<Uuid>,
    query: web::Query<ListMessagesQuery>,
    container: web::Data<AppContainer>,
) -> Result<HttpResponse, AppError> {
    let user_id = user.claims().sub;
    let conv_id = path.into_inner();
    let limit = query.limit.unwrap_or(50).min(100);

    let _ = container.conversation_members.find(conv_id, user_id)
        .await
        .map_err(AppError::Database)?
        .ok_or_else(|| AppError::Forbidden("Not a member of this conversation".to_string()))?;

    let msgs = container.messages.find_by_conversation(conv_id, query.before, limit)
        .await
        .map_err(AppError::Database)?;

    let response: Vec<MessageResponse> = msgs.into_iter().map(MessageResponse::from).collect();

    Ok(HttpResponse::Ok().json(response))
}

/// POST /messages/:conversation_id
#[actix_web::post("/messages/{conversation_id}")]
pub async fn send(
    user: AuthUser,
    path: web::Path<Uuid>,
    body: web::Json<SendMessageRequest>,
    container: web::Data<AppContainer>,
) -> Result<HttpResponse, AppError> {
    let user_id = user.claims().sub;
    let conv_id = path.into_inner();

    let _ = container.conversation_members.find(conv_id, user_id)
        .await
        .map_err(AppError::Database)?
        .ok_or_else(|| AppError::Forbidden("Not a member of this conversation".to_string()))?;

    let ct_bytes = B64.decode(&body.ciphertext)
        .map_err(|_| AppError::BadRequest("Invalid base64 ciphertext".into()))?;

    let msg = container.messages.create(
        conv_id,
        user_id,
        ct_bytes,
        body.iv.clone(),
        body.message_type,
        body.reply_to_id,
    ).await.map_err(AppError::Database)?;

    let response = MessageResponse::from(msg.clone());

    // Broadcast to conversation members via WebSocket
    tracing::info!(conv_id = %conv_id, "Broadcasting new message to room");
    let ws_msg = crate::ws::server::WsMessage::new(
        "new_message",
        serde_json::json!({
            "conversation_id": conv_id,
            "message_id": msg.id,
            "sender_id": msg.sender_id,
            "ciphertext": body.ciphertext,
            "iv": body.iv,
            "message_type": "text",
            "reply_to_id": body.reply_to_id,
            "created_at": msg.created_at,
        }),
    );
    container.ws_state.broadcast_to_room(&conv_id.to_string(), ws_msg);

    Ok(HttpResponse::Created().json(response))
}

/// DELETE /messages/:conversation_id/:id
#[actix_web::delete("/messages/{conversation_id}/{message_id}")]
pub async fn delete(
    user: AuthUser,
    path: web::Path<(Uuid, Uuid)>,
    container: web::Data<AppContainer>,
) -> Result<HttpResponse, AppError> {
    let user_id = user.claims().sub;
    let (conv_id, msg_id) = path.into_inner();

    let msg = container.messages.find_by_id(msg_id)
        .await
        .map_err(AppError::Database)?
        .ok_or_else(|| AppError::NotFound("Message not found".to_string()))?;

    if msg.sender_id != user_id {
        return Err(AppError::Forbidden("Cannot delete another user's message".to_string()));
    }

    container.messages.soft_delete(msg_id)
        .await
        .map_err(AppError::Database)?;

    Ok(HttpResponse::Ok().json(serde_json::json!({"message": "Message deleted"})))
}

/// POST /messages/:conversation_id/:id/receipts
#[actix_web::post("/messages/{conversation_id}/{message_id}/receipts")]
pub async fn update_receipt(
    user: AuthUser,
    path: web::Path<(Uuid, Uuid)>,
    body: web::Json<serde_json::Value>,
    container: web::Data<AppContainer>,
) -> Result<HttpResponse, AppError> {
    let _user_id = user.claims().sub;
    let (_conv_id, msg_id) = path.into_inner();
    let status_str = body
        .get("status")
        .and_then(|v| v.as_str())
        .unwrap_or("delivered");
    
    let status = match status_str {
        "delivered" => 1,
        "read" => 2,
        _ => 1,
    };

    container.messages.update_receipt(msg_id, _user_id, status)
        .await
        .map_err(AppError::Database)?;

    Ok(HttpResponse::Ok().json(serde_json::json!({"status": status_str})))
}