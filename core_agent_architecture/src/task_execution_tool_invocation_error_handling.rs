use async_trait::async_trait;
use crate::agent_traits_and_behavior_model::{Agent, AgentInput, AgentResult, AgentStatus};

// representing tasks and tools
#[derive(Clone)]
pub enum TollInvocation {
    ShellCommand { command: String, args: Vec<String> },
    HttpRequest { url: String, payload: String },
    Internal {operation: String },
}

// Implementing a task executor
#[async_trait]
pub trait TaskExecutor {
    async fn execute(&self, invocation: TollInvocation) -> TaskResult;
}

pub struct TaskResult {
    pub output: Option<String>,
    pub status: TaskStatus,
}

pub enum TaskStatus {
    Success,
    Failed(String),
    Skipped,
}

// Implementation of the executor
pub struct DefaultExecutor;

#[async_trait]
impl TaskExecutor for DefaultExecutor {
    async fn execute(&self, invocation: TollInvocation) -> TaskResult {
        match invocation {
            TollInvocation::ShellCommand{ command, args } => {
                match tokio::process::Command::new(command)
                    .args(&args)
                    .output()
                    .await
                    {
                        Ok(output) if output.status.success() => TaskResult {
                            output: Some(String::from_utf8_lossy(&output.stdout).into_owned()),
                            status: TaskStatus::Success,
                        },
                        Ok(output) => TaskResult {
                            output:  Some(String::from_utf8_lossy(&output.stderr).into_owned()),
                            status: TaskStatus::Failed("Command failed".into()),
                        },
                        Err(e) => TaskResult {
                            output: None,
                            status: TaskStatus::Failed(format!("Execution failed: {}", e)),
                        },
                    }
            },
            TollInvocation::HttpRequest{url, payload} => {
                let client = reqwest::Client::new();
                match client.post(&url).body(payload).send().await {
                    Ok(resp) if resp.status().is_success() => match resp.text().await {
                        Ok(body) => TaskResult { output: Some(body), status: TaskStatus::Success },
                        Err(e) => TaskResult { output: None, status: TaskStatus::Failed(format!("Read body failed: {}", e)) },
                    },
                    Ok(resp) => TaskResult {output: None, status: TaskStatus::Failed(format!("HTTP error: {}", resp.status()))},
                    Err(e) => TaskResult { output: None, status: TaskStatus::Failed(format!("Request failed: {}", e)) }
                }
            },
            TollInvocation::Internal { operation } => {
                let result = match operation.as_str() {
                    "ping" => Some("[png".to_string()),
                    _ => None,
                };

                match result {
                    Some(value) => TaskResult { output: Some(value), status: TaskStatus::Success },
                    None => TaskResult { output: None, status: TaskStatus::Failed("Unknown internal operation".into())},
                }
            },
        }
    }
}


// Composing Execution with Agents
pub struct ToolAgent<E: TaskExecutor + Send + Sync> {
    pub executor: E,
}

#[async_trait]
impl<E> Agent for ToolAgent<E>
    where E: TaskExecutor + Send + Sync {
        async fn handle_input(&mut self, input: AgentInput) -> AgentResult {
            let invocation = TollInvocation::ShellCommand {
                command: "echo".into(),
                args: vec![input.message],
            };
            let result = self.executor.execute(invocation).await;
            
            match result.status {
                TaskStatus::Success => AgentResult{
                    output: result.output.unwrap_or_default(),
                    status: AgentStatus::Success,
                },
                TaskStatus::Failed(e) => AgentResult{
                    output: format!("Error: {}", e),
                    status: AgentStatus::Error(e),
                },
                TaskStatus::Skipped => AgentResult{
                    output: "Task skipped".into(),
                    status: AgentStatus::InProgress,
                },
            }
        }
    }


// Error handling and resilience
pub async fn retry_http(invocation: TollInvocation, executor: &impl TaskExecutor) -> TaskResult {
    let mut attempts = 0;
    loop {
        let result = executor.execute(invocation.clone()).await;

        match result.status {
            TaskStatus::Success => return result,
            TaskStatus::Failed(_) if attempts < 3 => {
                attempts += 1;
                tokio::time::sleep(std::time::Duration::from_secs(attempts)).await;
            },
            _ => return result,
        }
    }
}

