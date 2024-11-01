use actix_web::{web, HttpResponse, Responder};
use serde_json::json;

use crate::structs::AppState;

#[actix_web::get("/chat/clear-conversation")]
pub async fn clear_conversation(state: web::Data<AppState>) -> impl Responder {
    let mut message_store = state.message_store.lock().unwrap();
    message_store.clear();
    HttpResponse::Ok().json(json!({ "message": "Messages cleared successfully" }))
}
