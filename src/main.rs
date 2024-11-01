mod ai;
mod constants;
mod database;
mod message_store;
mod routes;
mod structs;
mod utils;

use actix_web::{web, App, HttpServer};
use ai::initalize::gather_ollama_handler;
use constants::*;
use database::initalize::gather_surreal_handler;
use routes::get_current_conversation::current_conversation;
use routes::get_modelinfo::model_info;
use routes::get_models::list_models;
use routes::post_clear_messages::clear_conversation;
use routes::{get_chat::start_chat, get_image_for_context::generate_image};
use std::sync::{Arc, Mutex};
use structs::{AppState, MessageStore};
use surrealdb::engine::remote::ws::Client;
use surrealdb::Surreal;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let ollama = gather_ollama_handler();
    let db: Surreal<Client> = gather_surreal_handler().await;
    let state = web::Data::new(AppState {
        ollama,
        db,
        message_store: Arc::new(Mutex::new(MessageStore::new(MAX_HISTORY_MESSAGES))),
    });

    dbg!(format!(
        "Listening on http://{}:{}/",
        ACTIX_WEB_HOST_PORT.0, ACTIX_WEB_HOST_PORT.1
    ));
    HttpServer::new(move || {
        App::new()
            .app_data(state.clone())
            .service(list_models)
            .service(start_chat)
            .service(model_info)
            .service(clear_conversation)
            .service(current_conversation)
            .service(generate_image)
    })
    .bind((ACTIX_WEB_HOST_PORT.0, ACTIX_WEB_HOST_PORT.1))?
    .run()
    .await
}
