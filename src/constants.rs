
pub const TARGET_MODEL: &str = "llama3.2:latest";

pub const LOCAL_OLLAMA_HOST: (&str, u16) = ("http://127.0.0.1", 11434);
pub const _HOSTED_OLLAMA_HOST: &str = "http://10.0.0.236";
pub const ACTIX_WEB_HOST_PORT: (&str, u16) = ("127.0.0.1", 8080);

pub const SURREAL_DB_HOST: &str = "127.0.0.1:8000";
pub const SURREAL_CREDENTIALS: (&str, &str) = ("throatpies", "yumshot");
pub const SURREAL_NAMESPACE: &str = "yumlabs";
pub const SURREAL_DATABASE: &str = "large_language_model_data";

pub const MAX_HISTORY_MESSAGES: usize = 10;

pub const CONTEXT: u32 = 4096;
pub const REPEAT_PENALTY: f32 = 1.4;
pub const TEMPERATURE: f32 = 0.84;

pub const IMAGE_WIDTH: u32 = 512;
pub const IMAGE_HEIGHT: u32 = 512;
pub const SD_MODEL_CHECKPOINT: &str = "Degenerate_deliberateV2";
pub const CFG_SCALE: f32 = 7.5;

pub const _START_COMMAND: &str =
    "surreal start --user throatpies --pass yumshot rocksdb:yumlabs.db";
