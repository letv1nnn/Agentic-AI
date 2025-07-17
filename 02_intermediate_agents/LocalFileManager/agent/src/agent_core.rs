use crate::tools::{collect_files, get_file_metadata, route_file, move_file, init_logging};
use crate::tools_invocation;
use std::path::PathBuf;
use tracing::{info, error};
use crate::ollama_integration::{self, ActionResponse};

pub enum AgentOutputStatus {
    Success,
    Failure(String),
}

pub struct AgentInput {
    pub query: String,
}

pub struct AgentOutput {
    pub status: AgentOutputStatus,
    pub message: Option<String>,
}

pub struct LocalFileManagerAgent;

impl LocalFileManagerAgent {
    pub fn new() -> Self {
        LocalFileManagerAgent
    }
    pub async fn handle_input(&self, input: AgentInput) -> AgentOutput {
        init_logging();

        match ollama_integration::ollama_input_config(&input.query).await {
            Ok(action) => {
                info!("Received action: {:?}", action);
                self.execute_action(action).await
            }
            Err(e) => {
                error!("Error processing input: {}", e);
                AgentOutput {
                    status: AgentOutputStatus::Failure(e.to_string()),
                    message: Some("Failed to process your request".to_string()),
                }
            }
        }
    }
    async fn execute_action(&self, action: ActionResponse) -> AgentOutput {
        match action.action.as_str() {
            "collect_files" => {
                if let Some(base_dir) = action.parameters.get("base_dir") {
                    let files = collect_files(base_dir).await;
                    AgentOutput {
                        status: AgentOutputStatus::Success,
                        message: Some(format!("Found {} files in {}", files.len(), base_dir)),
                    }
                } else {
                    AgentOutput {
                        status: AgentOutputStatus::Failure("Missing base_dir parameter".to_string()),
                        message: None,
                    }
                }
            }
            _ => AgentOutput {
                status: AgentOutputStatus::Failure("Unknown action".to_string()),
                message: None,
            },
        }
    }
}
