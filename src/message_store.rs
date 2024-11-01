use std::collections::VecDeque;

use ollama_rs::generation::chat::ChatMessage;

use crate::structs::MessageStore;

impl MessageStore {
    pub fn new(max_size: usize) -> Self {
        MessageStore {
            messages: VecDeque::with_capacity(max_size),
            max_size,
        }
    }

    pub fn add_message(&mut self, message: ChatMessage) {
        if self.messages.len() >= self.max_size {
            self.messages.pop_front();
        }
        self.messages.push_back(message);
    }

    pub fn get_recent_messages(&self) -> Vec<ChatMessage> {
        self.messages.iter().cloned().collect()
    }

    pub fn clear(&mut self) {
        self.messages.clear();
    }
}
