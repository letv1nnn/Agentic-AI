use crate::tools::{collect_files, get_file_metadata, route_file, move_file, init_logging};
use crate::tools_invocation;
use std::path::PathBuf;
use tracing::{info, error};
use crate::ollama_integration::{self, ActionResponse, execute_command};

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
                if action.action.is_empty() {
                    return AgentOutput {
                        status: AgentOutputStatus::Failure("No action specified".to_string()),
                        message: Some("Please provide a valid action".to_string()),
                    };
                }
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
                    println!("Collected files:");
                    for file in &files {
                        info!("  {}", file.file_name().expect("REASON").to_string_lossy());
                    }
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
            "get_file_metadata" => {
                if let Some(path) = action.parameters.get("path") {
                    match get_file_metadata(&PathBuf::from(path)).await {
                        Some(metadata) => AgentOutput {
                            status: AgentOutputStatus::Success,
                            message: Some(format!("File metadata: {:?}", metadata)),
                        },
                        None => AgentOutput {
                            status: AgentOutputStatus::Failure("File not found".to_string()),
                            message: None,
                        },
                    }
                } else {
                    AgentOutput {
                        status: AgentOutputStatus::Failure("Missing path parameter".to_string()),
                        message: None,
                    }
                }
            }
            "route_file" => {
                if let Some(file_path) = action.parameters.get("file_path") {
                    match get_file_metadata(&PathBuf::from(file_path)).await {
                        Some(metadata) => {
                            let target_dir = route_file(&metadata).await.unwrap_or_else(|| {
                                error!("No routing rule found for file: {}", file_path);
                                PathBuf::from("Uncategorized")
                            });
                            match move_file(&metadata, &target_dir).await {
                                Ok(_) => AgentOutput {
                                    status: AgentOutputStatus::Success,
                                    message: Some(format!("File routed to {}", target_dir.display())),
                                },
                                Err(e) => AgentOutput {
                                    status: AgentOutputStatus::Failure(e.to_string()),
                                    message: None,
                                },
                            }
                        }
                        None => AgentOutput {
                            status: AgentOutputStatus::Failure("File not found".to_string()),
                            message: None,
                        },
                    }
                } else {
                    AgentOutput {
                        status: AgentOutputStatus::Failure("Missing file_path parameter".to_string()),
                        message: None,
                    }
                }
            }
            "move_file" => {
                if let Some(path) = action.parameters.get("path") {
                    if let Some(target_dir) = action.parameters.get("target_dir") {
                        match get_file_metadata(&PathBuf::from(path)).await {
                            Some(metadata) => {
                                match move_file(&metadata, &PathBuf::from(target_dir)).await {
                                    Ok(_) => AgentOutput {
                                        status: AgentOutputStatus::Success,
                                        message: Some(format!("File moved to {}", target_dir)),
                                    },
                                    Err(e) => AgentOutput {
                                        status: AgentOutputStatus::Failure(e.to_string()),
                                        message: None,
                                    },
                                }
                            }
                            None => AgentOutput {
                                status: AgentOutputStatus::Failure("File not found".to_string()),
                                message: None,
                            },
                        }
                    } else {
                        AgentOutput {
                            status: AgentOutputStatus::Failure("Missing target_dir parameter".to_string()),
                            message: None,
                        }
                    }
                } else {
                    AgentOutput {
                        status: AgentOutputStatus::Failure("Missing path parameter".to_string()),
                        message: None,
                    }
                }
            }
            "find_files_by_extension" => {
                if let Some(base_dir) = action.parameters.get("base_dir") {
                    if let Some(extension) = action.parameters.get("extension") {
                        let files = tools::find_files_by_extension(base_dir, extension).await;
                        AgentOutput {
                            status: AgentOutputStatus::Success,
                            message: Some(format!("Found {} files with extension '{}'", files.len(), extension)),
                        }
                    } else {
                        AgentOutput {
                            status: AgentOutputStatus::Failure("Missing extension parameter".to_string()),
                            message: None,
                        }
                    }
                } else {
                    AgentOutput {
                        status: AgentOutputStatus::Failure("Missing base_dir parameter".to_string()),
                        message: None,
                    }
                }
            }
            "find_large_files" => {
                if let Some(base_dir) = action.parameters.get("base_dir") {
                    if let Some(min_size_str) = action.parameters.get("min_size") {
                        if let Ok(min_size) = min_size_str.parse::<u64>() {
                            let large_files = tools::find_large_files(base_dir, min_size).await;
                            AgentOutput {
                                status: AgentOutputStatus::Success,
                                message: Some(format!("Found {} files larger than {} MB", large_files.len(), min_size)),
                            }
                        } else {
                            AgentOutput {
                                status: AgentOutputStatus::Failure("Invalid min_size parameter".to_string()),
                                message: None,
                            }
                        }
                    } else {
                        AgentOutput {
                            status: AgentOutputStatus::Failure("Missing min_size parameter".to_string()),
                            message: None,
                        }
                    }
                } else {
                    AgentOutput {
                        status: AgentOutputStatus::Failure("Missing base_dir parameter".to_string()),
                        message: None,
                    }
                }
            }
            "execute_command" => {
                if let Some(command) = action.parameters.get("command") {
                    match execute_command(command).await {
                        Ok(output) => AgentOutput {
                            status: AgentOutputStatus::Success,
                            message: Some(output),
                        },
                        Err(e) => AgentOutput {
                            status: AgentOutputStatus::Failure(e.to_string()),
                            message: None,
                        },
                    }
                } else {
                    AgentOutput {
                        status: AgentOutputStatus::Failure("Missing command parameter".to_string()),
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
