#![allow(dead_code)]

use actix::prelude::*;
use actix_web::{HttpRequest, HttpResponse, web};
use actix_web_actors::ws;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use uuid::Uuid;

use crate::AppState;
use crate::errors::{AppError, AppResult};

/// Shared state for WebSocket connections
pub type WsConnections = Arc<Mutex<HashMap<String, ConnectionInfo>>>;
pub type WsPresence = Arc<Mutex<HashMap<Uuid, PresenceInfo>>>;

#[derive(Clone, Debug)]
pub struct ConnectionInfo {
    pub profile_id: Uuid,
    pub username: String,
    pub room: Option<String>,
    pub addr: Recipient<WsMessage>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PresenceInfo {
    pub active_connections: usize,
    pub last_seen_at: Option<DateTime<Utc>>,
}

/// WebSocket state container
#[derive(Clone)]
pub struct WsState {
    pub connections: WsConnections,
    pub presence: WsPresence,
}

impl WsState {
    pub fn new() -> Self {
        Self {
            connections: Arc::new(Mutex::new(HashMap::new())),
            presence: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub fn add_connection(&self, conn_id: String, info: ConnectionInfo) {
        let should_broadcast_online = {
            let mut conns = self.connections.lock().unwrap();
            let is_new_connection = conns.insert(conn_id, info.clone()).is_none();
            drop(conns);

            if !is_new_connection {
                false
            } else {
                let mut presence = self.presence.lock().unwrap();
                let entry = presence.entry(info.profile_id).or_insert(PresenceInfo {
                    active_connections: 0,
                    last_seen_at: None,
                });
                entry.active_connections += 1;
                entry.last_seen_at = None;
                entry.active_connections == 1
            }
        };

        if should_broadcast_online {
            self.broadcast_presence_change(info.profile_id, true, None);
        }
    }

    pub fn remove_connection(&self, conn_id: &str) {
        let removed_profile_id = {
            let mut conns = self.connections.lock().unwrap();
            conns.remove(conn_id).map(|info| info.profile_id)
        };

        let Some(profile_id) = removed_profile_id else {
            return;
        };

        let last_seen_at = {
            let mut presence = self.presence.lock().unwrap();
            let Some(entry) = presence.get_mut(&profile_id) else {
                return;
            };

            if entry.active_connections > 0 {
                entry.active_connections -= 1;
            }

            if entry.active_connections == 0 {
                let seen_at = Utc::now();
                entry.last_seen_at = Some(seen_at);
                Some(seen_at)
            } else {
                None
            }
        };

        if let Some(seen_at) = last_seen_at {
            self.broadcast_presence_change(profile_id, false, Some(seen_at));
        }
    }

    pub fn broadcast_to_room(&self, room: &str, message: WsMessage) {
        tracing::info!(
            room = room,
            msg_type = message.msg_type,
            "Broadcasting to room"
        );
        let conns = self.connections.lock().unwrap();
        tracing::info!("Active connections: {}", conns.len());

        let ws_msg = WsMessage::new(
            "new_message",
            serde_json::json!({
                "type": "new_message",
                "conversation_id": room,
                "message_id": message.payload.get("message_id").and_then(|v| v.as_str()).unwrap_or_default(),
                "sender_id": message.payload.get("sender_id").and_then(|v| v.as_str()).unwrap_or_default(),
                "ciphertext": message.payload.get("ciphertext").and_then(|v| v.as_str()).unwrap_or_default(),
                "iv": message.payload.get("iv").and_then(|v| v.as_str()).unwrap_or_default(),
                "message_type": message.payload.get("message_type").and_then(|v| v.as_str()).unwrap_or_default(),
                "reply_to_id": message.payload.get("reply_to_id").and_then(|v| v.as_str()),
                "created_at": message.payload.get("created_at").and_then(|v| v.as_str()).unwrap_or_default(),
            }),
        );
        for (_, info) in conns.iter() {
            if info.room.as_ref() == Some(&room.to_string()) {
                tracing::info!("Sending to connection in room: {}", room);
                let _ = info.addr.try_send(ws_msg.clone());
            }
        }
    }

    pub fn broadcast_to_room_except(
        &self,
        room: &str,
        excluded_profile_id: Uuid,
        message: WsMessage,
    ) {
        let conns = self.connections.lock().unwrap();
        for (_, info) in conns.iter() {
            if info.room.as_ref() == Some(&room.to_string())
                && info.profile_id != excluded_profile_id
            {
                let _ = info.addr.try_send(message.clone());
            }
        }
    }

    pub fn send_to_user(&self, profile_id: Uuid, message: WsMessage) {
        eprintln!("[WS] send_to_user called for profile_id: {}", profile_id);
        tracing::debug!("Trying to send message to profile_id: {}", profile_id);
        let conns = self.connections.lock().unwrap();
        eprintln!("[WS] Total connections: {}", conns.len());
        for (conn_id, info) in conns.iter() {
            eprintln!(
                "[WS] Checking conn_id={}, profile_id={}",
                conn_id, info.profile_id
            );
            if info.profile_id == profile_id {
                eprintln!("[WS] FOUND! Sending to conn_id={}", conn_id);
                let _ = info.addr.try_send(message.clone());
            }
        }
    }

    pub fn get_presence(&self, profile_id: Uuid) -> PresenceInfo {
        let presence = self.presence.lock().unwrap();
        presence.get(&profile_id).cloned().unwrap_or(PresenceInfo {
            active_connections: 0,
            last_seen_at: None,
        })
    }

    pub fn is_user_online(&self, profile_id: Uuid) -> bool {
        self.get_presence(profile_id).active_connections > 0
    }

    fn broadcast_presence_change(
        &self,
        profile_id: Uuid,
        is_online: bool,
        last_seen_at: Option<DateTime<Utc>>,
    ) {
        let message = WsMessage::new(
            "presence_changed",
            serde_json::json!({
                "profile_id": profile_id,
                "is_online": is_online,
                "last_seen_at": last_seen_at,
            }),
        );
        self.broadcast_to_all(message);
    }

    fn broadcast_to_all(&self, message: WsMessage) {
        let conns = self.connections.lock().unwrap();
        for (_, info) in conns.iter() {
            let _ = info.addr.try_send(message.clone());
        }
    }
}

impl Default for WsState {
    fn default() -> Self {
        Self::new()
    }
}

/// WebSocket message types
#[derive(Message, Clone, Debug, Serialize, Deserialize)]
#[rtype(result = "()")]
pub struct WsMessage {
    pub msg_type: String,
    pub payload: serde_json::Value,
    pub sender: Option<String>,
    pub timestamp: i64,
}

impl WsMessage {
    pub fn new(msg_type: &str, payload: serde_json::Value) -> Self {
        Self {
            msg_type: msg_type.to_string(),
            payload,
            sender: None,
            timestamp: chrono::Utc::now().timestamp(),
        }
    }

    pub fn to_json_string(&self) -> String {
        serde_json::json!({
            "type": self.msg_type,
            "conversation_id": self.payload.get("conversation_id").cloned(),
            "message_id": self.payload.get("message_id").cloned(),
            "sender_id": self.payload.get("sender_id").cloned(),
            "ciphertext": self.payload.get("ciphertext").cloned(),
            "iv": self.payload.get("iv").cloned(),
            "message_type": self.payload.get("message_type").cloned(),
            "reply_to_id": self.payload.get("reply_to_id").cloned(),
            "created_at": self.payload.get("created_at").cloned(),
        })
        .to_string()
    }

    pub fn chat(content: &str, sender: &str) -> Self {
        let payload = serde_json::json!({
            "content": content,
            "sender": sender,
        });
        let mut msg = Self::new("chat", payload);
        msg.sender = Some(sender.to_string());
        msg
    }

    pub fn live_status(stream_id: &str, is_live: bool, viewer_count: i32) -> Self {
        let payload = serde_json::json!({
            "stream_id": stream_id,
            "is_live": is_live,
            "viewer_count": viewer_count,
        });
        Self::new("live_status", payload)
    }

    pub fn notification(title: &str, message: &str) -> Self {
        let payload = serde_json::json!({
            "title": title,
            "message": message,
        });
        Self::new("notification", payload)
    }
}

/// WebSocket actor for handling connections
pub struct WebSocketActor {
    pub conn_id: String,
    pub profile_id: Option<Uuid>,
    pub username: Option<String>,
    pub room: Option<String>,
    pub ws_state: web::Data<WsState>,
    pub secret: String,
}

impl Actor for WebSocketActor {
    type Context = ws::WebsocketContext<Self>;

    fn started(&mut self, _ctx: &mut Self::Context) {
        // Do not add connection to ws_state or broadcast presence until authenticated!
        tracing::info!("WebSocket connected (unauthenticated): {}", self.conn_id);
    }

    fn stopped(&mut self, _ctx: &mut Self::Context) {
        // Only remove if it was authenticated and added to state
        if self.profile_id.is_some() {
            self.ws_state.remove_connection(&self.conn_id);
        }
        tracing::info!("WebSocket disconnected: {}", self.conn_id);
    }
}

/// Handle messages from other actors
impl Handler<WsMessage> for WebSocketActor {
    type Result = ();

    fn handle(&mut self, msg: WsMessage, ctx: &mut Self::Context) {
        let text = serde_json::to_string(&msg).unwrap_or_default();
        ctx.text(text);
    }
}

/// Handle WebSocket messages
impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for WebSocketActor {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        match msg {
            Ok(ws::Message::Ping(msg)) => ctx.pong(&msg),
            Ok(ws::Message::Text(text)) => self.handle_message(&text, ctx),
            Ok(ws::Message::Binary(_)) => {}
            Ok(ws::Message::Close(reason)) => {
                ctx.close(reason);
                ctx.stop();
            }
            _ => {}
        }
    }
}

impl WebSocketActor {
    fn handle_message(&mut self, text: &str, ctx: &mut ws::WebsocketContext<Self>) {
        match serde_json::from_str::<ClientMessage>(text) {
            Ok(msg) => match msg.action.as_str() {
                "auth" => {
                    if let Some(token) = msg.data.get("token").and_then(|v| v.as_str()) {
                        match crate::middleware::auth::verify_token(token, &self.secret) {
                            Ok(claims) => {
                                self.profile_id = Some(claims.profile_id);
                                self.username = Some(claims.email.clone());

                                let conn_info = ConnectionInfo {
                                    profile_id: claims.profile_id,
                                    username: claims.email,
                                    room: self.room.clone(),
                                    addr: ctx.address().recipient(),
                                };
                                self.ws_state.add_connection(self.conn_id.clone(), conn_info);

                                let response = WsMessage::new(
                                    "authenticated",
                                    serde_json::json!({"status": "success"}),
                                );
                                ctx.text(serde_json::to_string(&response).unwrap_or_default());
                                tracing::info!(
                                    "WebSocket authenticated: {} (profile: {})",
                                    self.conn_id, claims.profile_id
                                );
                            }
                            Err(e) => {
                                tracing::warn!("WebSocket auth failed: {:?}", e);
                                ctx.close(Some(ws::CloseReason {
                                    code: ws::CloseCode::Policy,
                                    description: Some("Invalid token".to_string()),
                                }));
                                ctx.stop();
                            }
                        }
                    }
                }
                _ => {
                    // Enforce authentication for all other actions
                    let (Some(profile_id), Some(username)) = (self.profile_id, &self.username) else {
                        tracing::warn!("Unauthenticated action attempt: {}", msg.action);
                        return;
                    };

                    match msg.action.as_str() {
                        "join_room" => {
                            if let Some(room) = msg.data.get("room").and_then(|v| v.as_str()) {
                                self.room = Some(room.to_string());
                                let conn_info = ConnectionInfo {
                                    profile_id,
                                    username: username.clone(),
                                    room: self.room.clone(),
                                    addr: ctx.address().recipient(),
                                };
                                self.ws_state.add_connection(self.conn_id.clone(), conn_info);

                                let response = WsMessage::new(
                                    "joined_room",
                                    serde_json::json!({
                                        "room": room,
                                        "user": username,
                                    }),
                                );
                                ctx.text(serde_json::to_string(&response).unwrap_or_default());
                            }
                        }
                        "leave_room" => {
                            self.room = None;
                            let conn_info = ConnectionInfo {
                                profile_id,
                                username: username.clone(),
                                room: None,
                                addr: ctx.address().recipient(),
                            };
                            self.ws_state.add_connection(self.conn_id.clone(), conn_info);
                        }
                        "chat" => {
                            if let (Some(room), Some(content)) =
                                (&self.room, msg.data.get("content").and_then(|v| v.as_str()))
                            {
                                let chat_msg = WsMessage::chat(content, username);
                                self.ws_state.broadcast_to_room(room, chat_msg);
                            }
                        }
                        "typing" => {
                            if let Some(room) = &self.room {
                                let typing_msg = WsMessage::new(
                                    "typing",
                                    serde_json::json!({
                                        "user": username,
                                        "room": room,
                                    }),
                                );
                                self.ws_state.broadcast_to_room_except(room, profile_id, typing_msg);
                            }
                        }
                        "stop_typing" => {
                            if let Some(room) = &self.room {
                                let stop_typing_msg = WsMessage::new(
                                    "stop_typing",
                                    serde_json::json!({
                                        "user": username,
                                        "room": room,
                                    }),
                                );
                                self.ws_state.broadcast_to_room_except(room, profile_id, stop_typing_msg);
                            }
                        }
                        "ping" => {
                            let pong = WsMessage::new(
                                "pong",
                                serde_json::json!({
                                    "timestamp": chrono::Utc::now().timestamp(),
                                }),
                            );
                            ctx.text(serde_json::to_string(&pong).unwrap_or_default());
                        }
                        _ => {
                            tracing::warn!("Unknown WebSocket action: {}", msg.action);
                        }
                    }
                }
            },
            Err(e) => {
                tracing::error!("Failed to parse WebSocket message: {} - {}", text, e);
            }
        }
    }
}

/// Client message structure
#[derive(Debug, Deserialize)]
struct ClientMessage {
    action: String,
    data: serde_json::Value,
}

/// WebSocket upgrade handler
pub async fn ws_handler(
    req: HttpRequest,
    stream: web::Payload,
    ws_state: web::Data<WsState>,
) -> AppResult<HttpResponse> {
    let conn_id = Uuid::new_v4().to_string();

    tracing::info!("WebSocket upgrade request: {}", conn_id);

    let state = req.app_data::<web::Data<AppState>>();
    let secret = state
        .map(|s| s.config.jwt_secret.clone())
        .unwrap_or_default();

    let ws_actor = WebSocketActor {
        conn_id,
        profile_id: None,
        username: None,
        room: None,
        ws_state,
        secret,
    };

    ws::start(ws_actor, &req, stream)
        .map_err(|e| AppError::Internal(t!("ws.error", error = e).into_owned()))
}

/// Response for WebSocket token endpoint
#[derive(Debug, Serialize)]
pub struct WebSocketTokenResponse {
    pub token: String,
    pub ws_url: String,
}
