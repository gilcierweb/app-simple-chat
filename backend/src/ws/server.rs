use actix_web::{web, HttpRequest, HttpResponse};

#[derive(Clone)]
pub struct WsState;

impl WsState {
    pub fn new() -> Self {
        Self
    }
}

pub async fn ws_handler(
    _req: HttpRequest,
    _stream: web::Payload,
    _state: web::Data<WsState>,
) -> Result<HttpResponse, actix_web::Error> {
    // Simplified WebSocket handler - just return OK for now
    Ok(HttpResponse::Ok().body("WebSocket endpoint"))
}
