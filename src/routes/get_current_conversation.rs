use crate::structs::AppState;
use actix_web::{web, HttpResponse, Responder};

#[actix_web::get("/chat/current-conversation")]
async fn current_conversation(state: web::Data<AppState>) -> impl Responder {
    let message_store = state.message_store.lock().unwrap();
    let messages = message_store.messages.clone();
    HttpResponse::Ok().json(messages)
}
