use actix_web::{web, HttpResponse, Responder};
use serde_json::json;

use crate::structs::AppState;

#[actix_web::get("/models/list")]
pub async fn list_models(state: web::Data<AppState>) -> impl Responder {
    match state.ollama.list_local_models().await {
        Ok(models) => {
            let resp = "Models Requested Ok()";
            dbg!("/models: resp", resp);
            HttpResponse::Ok().json(models)
        }
        Err(e) => {
            dbg!("/models: err", &e);
            HttpResponse::InternalServerError().json(json!({ "error": e.to_string() }))
        }
    }
}
