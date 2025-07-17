use serde::{Deserialize, Serialize};
use reqwest::Client;
use serde::ser::StdError;
use std::collections::HashMap;
use std::process::Command;


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

pub async fn send_request(prompt: &str) -> Result<String, Box<dyn StdError>> {
    let client = Client::new();

    let system_prompt = r#"
    You are a Local File Manager Assistant. Your job is to:
    1. Understand file management requests
    2. Choose the appropriate action from available functions
    3. Return your response in JSON format like:
    {
        "action": "function_name",
        "parameters": {
            "param1": "value1",
            "param2": "value2"
        },
        "explanation": "Brief explanation of the action"
    }

    Available functions:
    - collect_files(base_dir: str) -> List files in directory, only files, not directories, in other cases call execute_command
    - get_file_metadata(path: str) -> Get file details or metadata or information about the file
    - route_file(file: FileMetadata) -> Determine where file should go
    - move_file(metadata: FileMetadata, target_dir: str) -> Move file
    - execute_command(command: str) -> Execute shell command
    - find_files_by_extension(base_dir: str, extension: str) -> Find files by extension
    - find_large_files(base_dir: str, min_size: u64) -> Find large files

    For file operations, always use absolute paths.
    "#;

    let messages = vec![
        ChatMessage {
            role: "system".to_string(),
            content: system_prompt.to_string(),
        },
        ChatMessage {
            role: "user".to_string(),
            content: prompt.to_string(),
        },
    ];

    let request = ChatRequest {
        model: "mistral".to_string(),
        messages,
        temperature: Some(0.3),
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

    let mut response = String::new();
    for line in body.lines() {
        if let Ok(chat_response) = serde_json::from_str::<ChatResponse>(line) {
            if let Some(message) = chat_response.message {
                response.push_str(&message.content);
            }
        }  
    }

    Ok(response)
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ActionResponse {
    pub action: String,
    pub parameters: HashMap<String, String>,
    pub explanation: Option<String>,
}

// return the formatted to rust struct
pub async fn ollama_input_config(prompt: &String) -> Result<ActionResponse, Box<dyn StdError>> {
    let full_prompt = format!("User Query: {}", prompt);
    let response = send_request(&full_prompt).await?;

    let action_response: ActionResponse = serde_json::from_str(&response)?;
    
    Ok(action_response)
}

pub async fn execute_command(command: &str) -> Result<String, Box<dyn StdError>> {
    let output = if cfg!(target_os = "windows") {
        Command::new("cmd")
            .args(["/C", command])
            .output()?
    } else {
        Command::new("sh")
            .arg("-c")
            .arg(command)
            .output()?
    };

    if output.status.success() {
        Ok(String::from_utf8(output.stdout)?)
    } else {
        Err(String::from_utf8(output.stderr)?.into())
    }
}
