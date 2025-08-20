use async_trait::async_trait;



// defining a core agent trait
#[async_trait]
pub trait Agent {
    async fn handle_input(&mut self, input: AgentInput) -> AgentResult;
}

pub struct AgentInput {
    pub message: String,
    pub context: Option<String>,
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

// implementing a reactive agent
pub struct EchoAgent;

#[async_trait]
impl Agent for EchoAgent {
    async fn handle_input(&mut self, input: AgentInput) -> AgentResult {
        if input.message.contains("shutdown") {
            return AgentResult {
                output: String::new(),
                status: AgentStatus::Error("Restricted command".into())
            };
        }
        AgentResult {
            output: format!("Echo: {}", input.message),
            status: AgentStatus::Error("Restricted command".into())
        }
    }
}

// building a planning based agent
pub struct PlanningAgent {
    pub steps: Vec<String>,
}

#[async_trait]
impl Agent for PlanningAgent {
    async fn handle_input(&mut self, _input: AgentInput) -> AgentResult {
        // self.steps = plan_task(&input.message).await;
        if self.steps.is_empty() {
            return AgentResult {
                output: "No viable plan".into(),
                status: AgentStatus::Error("Planning failed".into()),
            }
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

// extending behavior with composition
#[async_trait]
pub trait ToolUser {
    async fn use_tool(&self, name: &str, args: &[String]) -> Result<String, String>;
}

pub async fn run_agent<T>(agent: &mut T, input: AgentInput)
where 
    T: Agent + Send + ToolUser {
        let result = agent.handle_input(input).await;

        if let AgentStatus::Success = result.status {
            let output = agent.use_tool("notify", &[result.output.clone()]).await;
            println!("Tool execution result: {:?}", output);
        }
    }


// modeling role-shifting behavior
#[async_trait]
pub trait RoleHandler {
    async fn handle(&mut self, input: AgentInput) -> AgentResult;
}

pub struct RoleShiftingAgent {
    current_role: Box<dyn RoleHandler + Send>,
}

#[async_trait]
impl Agent for RoleShiftingAgent {
    async fn handle_input(&mut self, input: AgentInput) -> AgentResult {
        if input.message.contains("switch") {
            // self.current_role = Box::new(PlanningAgent {steps: vec![]});
        }
        self.current_role.handle(input).await
    }
}