use crate::tools_invocation::{TaskExecutor, ToolInvocation, TaskStatus};
use std::future::Future;
use std::fmt::Debug;

pub trait Agent {
    fn handle_input(&mut self, input: AgentInput)
        -> impl Future<Output = AgentOutput> + Send;
}

pub struct AgentInput {
    pub content: String,
}

#[derive(Debug)]
pub struct AgentOutput {
    pub response: String,
    pub status: AgentStatus,
}

#[derive(Debug)]
pub enum AgentStatus {
    Success,
    Failed(String),
}

pub struct ReactiveAgent {
    name: String,
    description: String,
}

impl ReactiveAgent {
    pub fn new(name: String, description: String) -> Self {
        ReactiveAgent { name, description }
    }
}

impl Debug for ReactiveAgent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ReactiveAgent:\n\tname: {}\n\tdescription: {}", self.name, self.description)
    }
}

impl Agent for ReactiveAgent {
    async fn handle_input(&mut self, input: AgentInput) -> AgentOutput {
        let prompt = format!("You are a LINUX File Assistant. Tt must be a pipeline of bash command separated with comma, without any non-bash text! User Prompt: {}", input.content);
        
        let llm_tool = ToolInvocation::OllamaRequest(prompt);
        let exec_tool_one = self.execute_tool(llm_tool).await;
        
        // println!("Result from the ollama: {:#?}", exec_tool_one);            
        if let TaskStatus::Success = exec_tool_one.status {
            let commands = exec_tool_one.output.clone();
            println!("Commands to be executed: {:?}", commands);
        } else {
            let err_msg = exec_tool_one.status;
            return AgentOutput {
                response: format!("{} processed input: {}", self.name, input.content),
                status: AgentStatus::Failed(format!("Failed to process input: {:?}", err_msg)),
            };
        }

        let shell_command = ToolInvocation::ShellCommand(exec_tool_one.output.unwrap_or_default());
        let exec_tool_two = self.execute_tool(shell_command).await;
        if let TaskStatus::Success = exec_tool_two.status {
            println!("Shell command executed successfully.");
            shell_output(exec_tool_two.output);
        } else {
            let err_msg = exec_tool_two.status;
            return AgentOutput {
                response: format!("{} processed input: {}", self.name, input.content),
                status: AgentStatus::Failed(format!("Failed to execute shell command: {:?}", err_msg)),
            };
        }

        AgentOutput {
            response: format!("{} processed input: {}", self.name, input.content),
            status: AgentStatus::Success,
        }
    }
}


fn shell_output(output: Option<String>) {
    println!("Output:");
    match output {
        Some(output) => {
            if output.is_empty() {
                println!("No output from the shell command.");
            }
            
            for line in output.lines() {
                println!("{}", line.trim());
            }
            
        },
        None => println!("No output from the shell command."),
    };
}