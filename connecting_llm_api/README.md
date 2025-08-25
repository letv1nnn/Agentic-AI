# Connecting to LLM APIs (OpenAI, Ollama)

***Definition of the request structure for both OpenAI and Ollama APIs***
```rust
#[derive(Serialize, Deserialize)]
pub struct ChatMessage {
    pub role: String,
    pub content: String,
}

#[derive(Serialize)]
struct ChatRequest {
    model: String,
    messages: Vec<ChatMessage>,
    temperature: f32,
}

#[derive(Deserialize)]
struct Choice {
    message: ChatMessage
}

#[derive(Deserialize)]
struct ChatResponse {
    choices: Vec<Choice>
}
```