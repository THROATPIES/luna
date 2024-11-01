use std::{
    collections::VecDeque,
    sync::{Arc, Mutex},
};

use ollama_rs::{generation::chat::ChatMessage, Ollama};
use serde::{Deserialize, Serialize};
use surrealdb::{engine::remote::ws::Client, RecordId, Surreal};

#[allow(dead_code)] // ID is used, but only for Insert to Database
#[derive(Debug, Deserialize)]
pub struct GeneratedRecord {
    id: RecordId,
}

pub struct MessageStore {
    pub messages: VecDeque<ChatMessage>,
    pub max_size: usize,
}

pub struct AppState {
    pub ollama: Ollama,
    pub db: Surreal<Client>,
    pub message_store: Arc<Mutex<MessageStore>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ModelResponse {
    pub prompt: String,
    pub model: String,
    pub created_at: String,
    pub response: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub done: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub context: Option<Vec<i32>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_duration: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prompt_eval_count: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prompt_eval_duration: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub eval_count: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub eval_duration: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub keep_alive: Option<i32>,
}

#[derive(Deserialize)]
pub struct ChatQuery {
    pub prompt: String,
    pub generate_image: Option<bool>,
    pub session: Option<bool>,
    
}

#[derive(Deserialize)]
pub struct FormattedOutput {
    pub file_ext: String,
}

#[derive(Deserialize)]
pub struct ImageQuery {
    pub positive: String,
    pub negative: String,
    pub steps: i32,
}
