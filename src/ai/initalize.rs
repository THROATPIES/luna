use ollama_rs::Ollama;

use crate::LOCAL_OLLAMA_HOST;

pub fn gather_ollama_handler() -> Ollama {
    let ollama = Ollama::new(LOCAL_OLLAMA_HOST.0, LOCAL_OLLAMA_HOST.1);

    ollama
}
