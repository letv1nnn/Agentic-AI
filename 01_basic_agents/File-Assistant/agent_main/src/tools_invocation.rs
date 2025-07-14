use crate::agent_core::{Agent, AgentInput, AgentOutput, AgentStatus, ReactiveAgent};
use std::sync::Arc;
use tokio::sync::Mutex;
use std::collections::HashMap;
use crate::ollama_request_tool;

#[derive(Debug)]
pub enum ToolInvocation {
    ShellCommand(String),
    OllamaRequest(String),
}

pub struct Registers {
    registers: Arc<Mutex<HashMap<String, ToolInvocation>>>,
}

pub trait TaskExecutor {
    fn execute_tool(&mut self, tool: ToolInvocation) 
        -> impl Future<Output = TaskResult> + Send;
}

#[derive(Debug)]
pub struct TaskResult {
    pub output: Option<String>,
    pub status: TaskStatus,
}

#[derive(Debug)]
pub enum TaskStatus {
    Success,
    Failed(String),
}

impl TaskExecutor for ReactiveAgent {
    async fn execute_tool(&mut self, tool: ToolInvocation) -> TaskResult {
        match tool {
            // Process the Ollama request
            ToolInvocation::OllamaRequest(request) => {
                if let Ok(commands) = ollama_request_tool::send_request::<String>(&request).await {
                    TaskResult {
                        output: Some(commands.trim().to_string()),
                        status: TaskStatus::Success,
                    }
                } else {
                    let err_msg = String::from("Failed while processing the prompt to Ollama.");
                    TaskResult {
                        output: None,
                        status: TaskStatus::Failed(err_msg),
                    }
                }
            },
            // Execute the shell command and capture the output
            ToolInvocation::ShellCommand(command) => {
                let output = tokio::process::Command::new("sh")
                    .arg("-c")
                    .arg(&command)
                    .output()
                    .await;
            
                match output {
                    Ok(output) if output.status.success() => TaskResult {
                        output: Some(String::from_utf8_lossy(&output.stdout).into_owned()),
                        status: TaskStatus::Success,
                    },
                    Ok(output) => TaskResult {
                        output: Some(String::from_utf8_lossy(&output.stderr).into_owned()),
                        status: TaskStatus::Failed("Command failed".to_string()),
                    },
                    Err(e) => TaskResult {
                        output: None,
                        status: TaskStatus::Failed(format!("Execution failed: {}", e)),
                    },
                }
            }            
        }
    }
}

