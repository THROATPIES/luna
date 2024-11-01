use actix_web::{web, HttpResponse, Responder};
use ollama_rs::generation::{
    chat::{request::ChatMessageRequest, ChatMessage},
    completion::request::GenerationRequest,
    options::GenerationOptions,
};
use serde_json::json;

use crate::{
    structs::{AppState, ChatQuery, GeneratedRecord, ModelResponse},
    CONTEXT, REPEAT_PENALTY, TARGET_MODEL, TEMPERATURE,
};

#[actix_web::get("/chat")]
pub async fn start_chat(
    state: web::Data<AppState>,
    query: web::Query<ChatQuery>,
) -> impl Responder {
    let prompt = query.prompt.clone();
    let generate_image = query.generate_image.unwrap_or(false);
    let mut message_store = state.message_store.lock().unwrap();

    // Add user message to the store
    let user_message = ChatMessage::user(prompt.clone());
    message_store.add_message(user_message.clone());

    // Get recent messages for the API request
    let messages: Vec<ChatMessage> = message_store.get_recent_messages();
    let options = GenerationOptions::default()
        .num_ctx(CONTEXT)
        .repeat_penalty(REPEAT_PENALTY)
        .temperature(TEMPERATURE);
    let builder = ChatMessageRequest::new(TARGET_MODEL.to_string(), messages).options(options);
    let res = state.ollama.send_chat_messages(builder).await;

    match res {
        Ok(response) => {
            // Add assistant message to the store
            let assistant_message =
                ChatMessage::assistant(response.message.clone().unwrap().content.clone());
            let schema_fill = ModelResponse {
                prompt: prompt.clone(),
                model: response.model.clone(),
                created_at: response.created_at.clone(),
                response: response.message.clone().unwrap().content.clone(),
                done: None,
                context: None,
                total_duration: None,
                prompt_eval_count: None,
                prompt_eval_duration: None,
                eval_count: None,
                eval_duration: None,
                keep_alive: Some(1),
            };

            // Generate image only if the generate_image flag is true
            if generate_image {
                if let Ok(image_prompt) = state
                    .ollama
                    .generate(GenerationRequest::new(
                        "trollek/qwen2-diffusion-prompter:latest".to_string(),
                        response.message.clone().unwrap().content,
                    ))
                    .await
                {
                    let image_response = &image_prompt.response;
                    println!("Creating Image Prompt: {}", image_response);

                    let image_request_url = format!(
                        "http://127.0.0.1:8080/generate-image?positive={}&negative={}&steps={}",
                        image_response, "none", 20
                    );
                    // Generate image
                    if let Ok(image_generate) =
                        reqwest::Client::new().get(image_request_url).send().await
                    {
                        println!("Image generation response: {:?}", image_generate);
                    }
                }
            }

            // Create record in database
            let created: Option<GeneratedRecord> =
                state.db.create("chat").content(schema_fill).await.unwrap();
            dbg!("/chat: created", created);
            message_store.add_message(assistant_message);
            HttpResponse::Ok().json(response)
        }
        Err(e) => {
            dbg!("/chat: err", &e);
            HttpResponse::InternalServerError().json(json!({ "error": e.to_string() }))
        }
    }
}
