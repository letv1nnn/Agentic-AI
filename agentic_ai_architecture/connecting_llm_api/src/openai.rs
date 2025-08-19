use serde::{Deserialize, Serialize};
use reqwest::Client;
use std::env;

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

// implementation of the function that sends the request.
// ensure you load the API key from .env file or secure secret manager.
pub async fn send_to_openai(prompt: &str) -> Result<String, Box<dyn std::error::Error>> {
    let api_key = env::var("OPENAI_API_KEY")?;
    let client = Client::new();

    let messages = vec![
        ChatMessage {
            role: "system".to_string(),
            content: "You are a helpful assistant".to_string(),
        },
        ChatMessage {
            role: "user".to_string(),
            content: prompt.to_string(),
        },
    ];

    let request_body = ChatRequest {
        model: "gpt-4".to_string(),
        messages,
        temperature: 0.7,
    };

    let response = client
        .post("https://api.openai.com/v1/chat/completions")
        .bearer_auth(api_key)
        .json(&request_body)
        .send()
        .await?
        .json::<ChatResponse>()
        .await?;

    let reply = response
        .choices
        .first()
        .map(|c| c.message.content.clone())
        .unwrap_or_else(|| "<no reply>".to_string());

    Ok(reply)
}