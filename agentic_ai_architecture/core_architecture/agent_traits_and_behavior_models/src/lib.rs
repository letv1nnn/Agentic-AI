
use std::future::Future;

// Core agent trait
pub trait Agent {
    fn handle_input(&mut self, input: AgentInput) -> impl Future<Output = AgentResult> + Send;
}

// Supproting types
pub struct AgentInput {
    pub message: String,
    pub contex: Option<String>,
}

pub struct AgentResult {
    pub output: String,
    pub status: AgentStatus,
}

pub enum AgentStatus {
    Success,
    Error(String),
    InProgress,
}


// REACTIVE AGENT IMPLEMENTATION
pub struct EchoAgent;

impl Agent for EchoAgent {
    async fn handle_input(&mut self, input: AgentInput) -> AgentResult {
        if input.message.contains("shutdown") {
            return AgentResult {
                output: String::new(),
                status: AgentStatus::Error("Restricted command".into()),
            };
        }
        AgentResult {
            output: format!("Echo: {}", input.message),
            status: AgentStatus::Success,
        }
    }
}

// PLANNING AGENT IMPLEMENTATION
pub struct PlanningAgent {
    pub steps: Vec<String>,
}

impl Agent for PlanningAgent {
    async fn handle_input(&mut self, input: AgentInput) -> AgentResult {
        self.steps = plan_task(&input.message).await;

        if self.steps.is_empty() {
            return AgentResult {
                output: "No viable plan".into(),
                status: AgentStatus::Error("Planning failed".into()),
            };
        }

        for step in &self.steps {
            println!("Executing step: {}", step);
        }

        AgentResult {
            output: format!("Executed {} steps", self.steps.len()),
            status: AgentStatus::Success,
        }
    
    }
}

pub async fn plan_task(_task: &String) -> Vec<String> {
    // in this example, plan_task could call an LLM or 
    // heuristic planner to break the input into smaller steps.
    Vec::new()
}


// trait for tool execution
pub trait ToolUser {
    fn use_tool(&self, name: &str, args: &[String]) -> impl Future<Output = Result<String, String>> + Send;
}


// Orchestrators
pub async fn run_agent<T>(agent: &mut T, input: AgentInput)
    where T: Agent + ToolUser + Send {
        let result = agent.handle_input(input).await;

        if let AgentStatus::Success = result.status {
            let output = agent.use_tool("notify", &[result.output.clone()]).await;
            println!("Tool execution result: {:?}", output);
        }
    }


// Modeling Role-Shifting Behavior
// strategy pattern: the agent holds a dynamic behavior type and
// swaps implementations at runtime.

pub trait RoleHandler {
    fn handle(&mut self, input: AgentInput) -> impl Future<Output = AgentResult> + Send;
}
// Wrap multiple roles inside a dynamic agent.
pub struct RoleShiftingAgent {
    current_role: Box<dyn RoleHandler + Send>,
}

impl Agent for RoleShiftingAgent {
    async fn handle_input(&mut self, input: AgentInput) -> AgentResult {
        if input.message.contains("switch") {
            self.current_role = Box::new(PlanningAgent { steps: vec![] });
        }
        self.current_role.handle(input).await;
    }
}

