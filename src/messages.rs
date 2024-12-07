pub struct ChatRequest {
    pub copilot_thread_id: String,
    pub messages: Vec<ChatMessage>,
}

pub struct ChatMessage {
    pub role: String,
    pub content: String,
}
