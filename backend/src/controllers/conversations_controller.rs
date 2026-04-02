#![allow(dead_code)]

use actix_web::{HttpResponse, delete, get, post, web};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::controllers::messages_controller::MessageResponse;
use crate::errors::{AppError, AppResult};
use crate::middleware::auth::AuthUser;
use crate::repositories::container::AppContainer;

#[derive(Debug, Serialize, Deserialize)]
pub struct ConversationResponse {
    pub id: Uuid,
    pub conversation_type: String,
    pub name: Option<String>,
    pub avatar_url: Option<String>,
    pub created_by: Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub members: Option<Vec<ConversationMemberResponse>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ConversationMemberResponse {
    pub conversation_id: Uuid,
    pub user_id: Uuid,
    pub role: i32,
}

impl From<crate::models::conversation::Conversation> for ConversationResponse {
    fn from(conv: crate::models::conversation::Conversation) -> Self {
        Self {
            id: conv.id,
            conversation_type: if conv.conversation_type == 1 {
                "direct".to_string()
            } else {
                "group".to_string()
            },
            name: None,
            avatar_url: conv.avatar_url,
            created_by: conv.created_by,
            created_at: conv.created_at,
            updated_at: conv.updated_at,
            members: None,
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct CreateConversationBody {
    pub participant_user_id: Option<Uuid>,
    pub participant_email: Option<String>,
    pub conversation_type: Option<i32>,
}

#[derive(Debug, Deserialize)]
pub struct SendMessageBody {
    pub ciphertext: Vec<u8>,
    pub iv: String,
    pub message_type: i32,
}

#[derive(Debug, Serialize)]
pub struct UserLookupResponse {
    pub user_id: Uuid,
    pub email: String,
}

/// GET /api/v1/conversations
#[get("/conversations")]
pub async fn list_conversations(
    user: AuthUser,
    container: web::Data<AppContainer>,
) -> AppResult<HttpResponse> {
    let user_id = user.claims().sub;

    let convs = container
        .conversations
        .find_by_user(user_id)
        .await
        .map_err(AppError::Database)?;

    let mut response_list = Vec::new();
    for conv in convs {
        let members = container
            .conversation_members
            .find_by_conversation(conv.id)
            .await
            .map_err(AppError::Database)?;

        let member_responses: Vec<ConversationMemberResponse> = members
            .iter()
            .map(|m| ConversationMemberResponse {
                conversation_id: m.conversation_id,
                user_id: m.user_id,
                role: m.role,
            })
            .collect();

        response_list.push(ConversationResponse {
            id: conv.id,
            conversation_type: if conv.conversation_type == 1 {
                "direct".to_string()
            } else {
                "group".to_string()
            },
            name: None,
            avatar_url: conv.avatar_url,
            created_by: conv.created_by,
            created_at: conv.created_at,
            updated_at: conv.updated_at,
            members: Some(member_responses),
        });
    }

    Ok(HttpResponse::Ok().json(response_list))
}

/// GET /api/v1/users/lookup?email=xxx
#[get("/users/lookup")]
pub async fn lookup_user_by_email(
    query: web::Query<std::collections::HashMap<String, String>>,
    container: web::Data<AppContainer>,
) -> AppResult<HttpResponse> {
    let email = query.get("email").cloned().unwrap_or_default();

    let user = container
        .users
        .find_by_email(&email)
        .await
        .map_err(AppError::Database)?
        .ok_or_else(|| AppError::NotFound(t!("users.not_found").into_owned()))?;

    Ok(HttpResponse::Ok().json(serde_json::json!({
        "user_id": user.id,
        "email": user.email,
        "conversation_type": "direct"
    })))
}

/// POST /api/v1/conversations
#[post("/conversations")]
pub async fn create_conversation(
    user: AuthUser,
    body: web::Json<CreateConversationBody>,
    container: web::Data<AppContainer>,
) -> AppResult<HttpResponse> {
    let user_id = user.claims().sub;

    let participant_id = if let Some(id) = body.participant_user_id {
        id
    } else if let Some(email) = &body.participant_email {
        let user = container
            .users
            .find_by_email(email)
            .await
            .map_err(AppError::Database)?
            .ok_or_else(|| AppError::NotFound(t!("users.not_found").into_owned()))?;
        user.id
    } else {
        return Err(AppError::BadRequest(
            t!("conversations.participant_required").into_owned(),
        ));
    };

    let conv_type = body.conversation_type.unwrap_or(1);

    // Check if direct conversation already exists between these two users
    if conv_type == 1 {
        if let Some(existing_conv) = container
            .conversations
            .find_existing_direct_conversation(user_id, participant_id)
            .await
            .map_err(AppError::Database)?
        {
            // Return existing conversation with members
            let members = container
                .conversation_members
                .find_by_conversation(existing_conv.id)
                .await
                .map_err(AppError::Database)?;

            let member_responses: Vec<ConversationMemberResponse> = members
                .iter()
                .map(|m| ConversationMemberResponse {
                    conversation_id: m.conversation_id,
                    user_id: m.user_id,
                    role: m.role,
                })
                .collect();

            let response = ConversationResponse {
                id: existing_conv.id,
                conversation_type: "direct".to_string(),
                name: None,
                avatar_url: existing_conv.avatar_url,
                created_by: existing_conv.created_by,
                created_at: existing_conv.created_at,
                updated_at: existing_conv.updated_at,
                members: Some(member_responses),
            };
            return Ok(HttpResponse::Ok().json(response));
        }
    }

    let conv = container
        .conversations
        .create(&crate::models::conversation::NewConversation {
            id: Uuid::new_v4(),
            conversation_type: conv_type,
            name_enc: None,
            avatar_url: None,
            created_by: user_id,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        })
        .await
        .map_err(AppError::Database)?;

    container
        .conversation_members
        .create(&crate::models::conversation_member::NewConversationMember {
            conversation_id: conv.id,
            user_id,
            role: 1,
            joined_at: Utc::now(),
            last_read_at: None,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        })
        .await
        .map_err(AppError::Database)?;

    container
        .conversation_members
        .create(&crate::models::conversation_member::NewConversationMember {
            conversation_id: conv.id,
            user_id: participant_id,
            role: 3,
            joined_at: Utc::now(),
            last_read_at: None,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        })
        .await
        .map_err(AppError::Database)?;

    let response = ConversationResponse {
        id: conv.id,
        conversation_type: if conv.conversation_type == 1 {
            "direct".to_string()
        } else {
            "group".to_string()
        },
        name: None,
        avatar_url: conv.avatar_url,
        created_by: conv.created_by,
        created_at: conv.created_at,
        updated_at: conv.updated_at,
        members: Some(vec![
            ConversationMemberResponse {
                conversation_id: conv.id,
                user_id,
                role: 1,
            },
            ConversationMemberResponse {
                conversation_id: conv.id,
                user_id: participant_id,
                role: 3,
            },
        ]),
    };
    Ok(HttpResponse::Created().json(response))
}

/// GET /api/v1/conversations/{conversation_id}/messages
#[get("/conversations/{conversation_id}/messages")]
pub async fn get_conversation_messages(
    user: AuthUser,
    path: web::Path<Uuid>,
    container: web::Data<AppContainer>,
) -> AppResult<HttpResponse> {
    let user_id = user.claims().sub;
    let conversation_id = path.into_inner();

    let _ = container
        .conversation_members
        .find(conversation_id, user_id)
        .await
        .map_err(AppError::Database)?
        .ok_or_else(|| AppError::Forbidden(t!("conversations.not_member").into_owned()))?;

    let msgs = container
        .messages
        .find_by_conversation(conversation_id, None, 50)
        .await
        .map_err(AppError::Database)?;

    let response: Vec<MessageResponse> = msgs.into_iter().map(MessageResponse::from).collect();

    Ok(HttpResponse::Ok().json(response))
}

/// POST /api/v1/conversations/{conversation_id}/messages
#[post("/conversations/{conversation_id}/messages")]
pub async fn send_message(
    user: AuthUser,
    path: web::Path<Uuid>,
    body: web::Json<SendMessageBody>,
    container: web::Data<AppContainer>,
) -> AppResult<HttpResponse> {
    let sender_id = user.claims().sub;
    let conversation_id = path.into_inner();

    let _ = container
        .conversation_members
        .find(conversation_id, sender_id)
        .await
        .map_err(AppError::Database)?
        .ok_or_else(|| AppError::Forbidden(t!("conversations.not_member").into_owned()))?;

    let msg = container
        .messages
        .create(
            conversation_id,
            sender_id,
            body.ciphertext.clone(),
            body.iv.clone(),
            body.message_type,
            None,
        )
        .await
        .map_err(AppError::Database)?;

    let response = MessageResponse::from(msg.clone());

    // Broadcast to conversation members via WebSocket
    tracing::info!(conv_id = %conversation_id, "Broadcasting new message to room");
    let ws_msg = crate::ws::server::WsMessage::new(
        "new_message",
        serde_json::json!({
            "conversation_id": conversation_id,
            "message_id": msg.id,
            "sender_id": msg.sender_id,
            "ciphertext": response.ciphertext, // Use Base64 from response
            "iv": response.iv,
            "message_type": response.message_type,
            "reply_to_id": response.reply_to_id,
            "created_at": response.created_at,
        }),
    );
    container
        .ws_state
        .broadcast_to_room(&conversation_id.to_string(), ws_msg);

    Ok(HttpResponse::Created().json(response))
}

/// DELETE /api/v1/conversations/{conversation_id}/messages/{message_id}
#[delete("/conversations/{conversation_id}/messages/{message_id}")]
pub async fn delete_message(
    user: AuthUser,
    path: web::Path<(Uuid, Uuid)>,
    container: web::Data<AppContainer>,
) -> AppResult<HttpResponse> {
    let user_id = user.claims().sub;
    let (conversation_id, message_id) = path.into_inner();

    let _ = container
        .conversation_members
        .find(conversation_id, user_id)
        .await
        .map_err(AppError::Database)?
        .ok_or_else(|| AppError::Forbidden(t!("conversations.not_member").into_owned()))?;

    container
        .messages
        .soft_delete(message_id)
        .await
        .map_err(AppError::Database)?;

    Ok(HttpResponse::NoContent().finish())
}
