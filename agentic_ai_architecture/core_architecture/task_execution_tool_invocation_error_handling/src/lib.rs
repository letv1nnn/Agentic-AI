use agent_beh_mod::Agent;

// Representing Tasks and Tools
// Shared type that encapsulates a tool execution request.
pub enum TollInvocation {
    ShellCommand { command: String, args: Vec<String> },
    HttpRequest { url: String, payload: String },
    Internal {operation: String },
}

// Implementing a task executor
pub trait TaskExecutor {
    fn execute(&self, invocation: TollInvocation) -> impl Future<Output = TaskResult>;
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
                        }
                        Ok(output) => TaskResult {
                            output:  Some(String::from_utf8_lossy(&output.stderr).into_owned()),
                            status: TaskStatus::Failed("Command failed".into()),
                        }
                        Err(e) => TaskResult {
                            output: None,
                            status: TaskStatus::Failed(format!("Execution failed: {}", e)),
                        },
                    }
            },
            TollInvocation::HttpRequest{url, payload} => {
                    
            },
            TollInvocation::Internal { operation } => {

            },
        }

    }
}


// Composing Execution with Agents

pub struct ToolAgent<E: TaskExecutor + Send + Sync> {
    pub executor: E,
}

impl<E> Agent for ToolAgent<E>
    where E: ToolAgent + Send + Sync {
        async fn handle(&mut self, input: AgentInput) -> AgentResult {
            let invocation = TollInvocation::ShellCommand {
                command: "echo".into(),
                args: vec![input.message],
            }
            let result = self.executor.execute(invocation).await;
            
            match result.status {
                TaskStatus::Success => AgentResult{
                    output: result.output.unwrap_or_deafult(),
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
pub async fn retry_http(invocation: TollInvocation, executor: &iml TaskExecutor) {
    let mut attempts = 0;

    loop {
        let result = executor.execute(invocation.clone()).await;

        match result.status {
            TaskStatus::Success => return result,
            TaskStatus::Failed(_) => if attempts < 3 => {
                attempts += 1;
                tokio::time::sleep(std::time::Duration::from_secs(attempts)).await;
            },
            _ => return result,
        }
    }
}