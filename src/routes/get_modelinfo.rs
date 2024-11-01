use actix_web::{web, HttpResponse, Responder};
use serde_json::json;

use crate::structs::AppState;

#[actix_web::get("/models/model-info/{model_name}")]
async fn model_info(state: web::Data<AppState>, path: web::Path<String>) -> impl Responder {
    let op = state.ollama.show_model_info(path.into_inner()).await;
    match op {
        Ok(model) => {
            let resp = "Model Info Requested Ok()";
            dbg!("/test: resp", resp);
            HttpResponse::Ok().json(model)
        }
        Err(e) => {
            dbg!("/test: err", &e);
            HttpResponse::InternalServerError().json(json!({ "error": e.to_string() }))
        }
    }
}
