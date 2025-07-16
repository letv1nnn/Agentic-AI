use serde::{Deserialize, Serialize};
use reqwest::Client;
use serde::ser::StdError;

#[derive(Serialize, Deserialize, Debug)]
pub struct ChatMessage {
    pub role: String,
    pub content: String,
}

#[derive(Serialize, Debug)]
pub struct ChatRequest {
    pub model: String,
    pub messages: Vec<ChatMessage>,
    pub temperature: Option<f32>,
}

#[derive(Debug, Deserialize)]
pub struct ChatResponse {
    pub message: Option<ChatMessage>,
    pub done: bool,
}

pub async fn send_request<T>(prompt: &str) -> Result<String, Box<dyn StdError>> {
    let client = Client::new();

    let messages = vec![
        ChatMessage {
            role: "user".to_string(),
            content: prompt.to_string(),
        },
    ];

    let request = ChatRequest {
        model: "mistral".to_string(),
        messages,
        temperature: Some(0.7),
    };

    let response_result = client
        .post("http://localhost:11434/api/chat")
        .json(&request)
        .send()
        .await?;

    if !response_result.status().is_success() {
        return Err(format!("Server returned status code: {}", response_result.status()).into());
    }

    let body = response_result.text().await?;

    let mut commands = String::new();

    for line in body.lines() {
        if let Ok(chunk) = serde_json::from_str::<ChatResponse>(line) {
            if let Some(msg) = chunk.message {
                commands.push_str(&msg.content);
            }
        }
    }
    
    Ok(commands)
}
