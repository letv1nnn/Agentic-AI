use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use serde_json::Value;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ToolStep {
    pub tool_name: String,
    pub args: Value,
    pub output_key: String, // used to store the result of the tool call
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ToolPlan {
    pub goal: String,
    pub steps: Vec<ToolStep>,
}

// Executing the tool plan
use std::sync::Arc;
use tokio::sync::RwLock;
use crate::{ToolRegistry, ToolInput, ToolOutput};

pub async fn execute_tool_plan(
    tool_plan: ToolPlan,
    tool_registry: Arc<RwLock<ToolRegistry>>,
) -> Result<HashMap<String, Value>, String> {
    let mut results = HashMap::new();

    for step in tool_plan.steps {
        let tool_name = step.tool_name;
        let args = step.args;

        // Retrieve the tool from the registry
        let tool_registry = tool_registry.read().await;
        let tool = match tool_registry.get_tool(&tool_name) {
            Some(tool) => tool,
            None => return Err(format!("Tool '{}' not found", tool_name)),
        };

        // Prepare the input for the tool
        let input = ToolInput {
            args: args.clone(),
            output_key: step.output_key.clone(),
        };

        // Execute the tool
        match tool.execute(input).await {
            Ok(output) => {
                results.insert(step.output_key, output.result);
            }
            Err(e) => return Err(format!("Error executing tool '{}': {}", tool_name, e)),
        }
    }

    Ok(results)
}


