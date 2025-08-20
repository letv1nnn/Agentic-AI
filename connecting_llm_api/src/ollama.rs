use serde::{Deserialize, Serialize};
use reqwest::Client;
use crate::openai::ChatMessage;


#[derive(Serialize)]
struct OllamaRequest {
    model: String,
    messages: Vec<ChatMessage>,
}

#[derive(Serialize, Deserialize)]
struct OllamaResponse {
    message: ChatMessage,
}

pub async fn send_to_ollama(prompt: &str) -> Result<String, Box<dyn std::error::Error>> {
    let client = Client::new();

    let messages = vec![
        ChatMessage {
            role: "system".to_string(),
            content: "You are a code writing assistant".to_string(),
        },
        ChatMessage {
            role: "user".to_string(),
            content: prompt.to_string(),
        },
    ];

    let request = OllamaRequest {
        model: "mistral".to_string(),
        messages
    };

    let response = client
        .post("http://localhost:11434/api/chat")
        .json(&request)
        .send()
        .await?
        .json::<OllamaResponse>()
        .await?;

    Ok(response.message.content)
}