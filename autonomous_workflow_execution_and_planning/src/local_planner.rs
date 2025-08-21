use serde::{Deserialize, Serialize};
use async_trait::async_trait;
use serde_json::{json, Value};
use tokio::sync::RwLock;
use std::{collections::HashMap, sync::Arc};
use core_agent_architecture::{agent_traits_and_behavior_model::{Agent, AgentInput, AgentResult, AgentStatus}};

// structuring the planner interface
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PlanStep {
    pub tool_name: String,
    pub args: Value,
    pub output_key: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Plan {
    pub goal: String,
    pub steps: Vec<PlanStep>,
}

#[async_trait]  
pub trait Planner: Send + Sync {
    async fn generate_plan(&self, goal: &str) -> Option<Plan>;
}

// implementing a template-based local planner
pub struct KeywordPlanner;

#[async_trait]
impl Planner for KeywordPlanner {
    async fn generate_plan(&self, goal: &str) -> Option<Plan> {
        if goal.contains("summarize log file") {
            Some(Plan {
                goal: goal.to_string(),
                steps: vec![
                    PlanStep {
                        tool_name: "read_file".into(),
                        args: json!({"path": "system.log"}),
                        output_key: "log content".into(),
                    },
                    PlanStep {
                        tool_name: "summarize".into(),
                        args: json!({"text": "${log_content}"}),
                        output_key: "summary".into(),
                    },
                ],
            })
        } else if goal.contains("analize disk usage") {
            Some(Plan {
                goal: goal.to_string(),
                steps: vec![
                    PlanStep {
                        tool_name: "check_disk".into(),
                        args: json!({}),
                        output_key: "disk_info".into(),
                    },
                    PlanStep {
                        tool_name: "summarize".into(),
                        args: json!({"text": "${disk_info}"}),
                        output_key: "disk_summary".into(),
                    },
                ],
            })
        } else {
            None
        }
    }
}

// executing a plan with substitution logic
fn resolve_args(template: &Value, outputs: &HashMap<String, Value>) -> Value {
    match template {
        Value::String(s) => {
            if let Some(var) = s.strip_prefix("${").and_then(|s| s.strip_suffix("}")) {
                return outputs.get(var).cloned().unwrap_or(Value::Null);
            }
            Value::String(s.clone())
        },
        Value::Object(map) => {
            let resolved = map.iter().map(|(k, v)| (k.clone(), resolve_args(v, outputs))).collect();
            Value::Object(resolved)
        },
        Value::Array(arr) => {
            Value::Array(arr.iter().map(|v| resolve_args(v, outputs)).collect())
        },
        other => other.clone()
    }
}

// then build a loop that executes a plan
#[derive(Debug)]
pub struct ToolInput {
    pub args: Value, // JSON object with parameters
}

#[derive(Debug)]
pub struct ToolOutput {
    pub result: Value, // JSON result
    pub success: bool,
    pub message: Option<String>,
}

#[async_trait]
pub trait Tool: Send + Sync {
    fn name(&self) -> &'static str;
    fn description(&self) -> &'static str;

    async fn execute(&self, input: ToolInput) -> ToolOutput;
}

pub type ToolRegistery = Arc<RwLock<HashMap<String, Arc<dyn Tool>>>>;


pub async fn execute_plan(plan: Plan, registry: &ToolRegistery) -> HashMap<String, Value> {
    let mut outputs = HashMap::new();

    for step in plan.steps {
        let args = resolve_args(&step.args, &outputs);
        let tools = registry.read().await;
        if let Some(tool) = tools.get(&step.tool_name) {
            let result = tool.execute(ToolInput { args }).await;
            outputs.insert(step.output_key, result.result);
        } else {
            outputs.insert(step.output_key, Value::String("Tool not found".into()));
        }
    }

    outputs
}

// using a planner in an agent loop
pub struct PlanningAgent<P: Planner> {
    planner: P,
    registry: ToolRegistery
}

#[async_trait]
impl<P: Planner> Agent for PlanningAgent<P> {
    async fn handle_input(&mut self, input: AgentInput) -> AgentResult {
        if let Some(plan) = self.planner.generate_plan(&input.message).await {
            let result = execute_plan(plan, &self.registry).await;
            AgentResult {
                output: format!("{:#?}", result),
                status: AgentStatus::Success,
            }
        } else {
            AgentResult {
                output: "Unable to plan for this goal".into(),
                status: AgentStatus::Error("Unrecognized goal".into()),
            }
        }
    }
}