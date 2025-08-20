use serde::{Serialize, Deserialize};
use serde_json::Value;
use std::collections::HashMap;
use crate::secure_tool_functions::{ToolRegistery, ToolInput, ToolOutput};

// representing tool chaines as plans
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct ToolStep {
    pub tool_name: String,
    pub args: Value,
    pub output_key: String, // var to store a result
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolPlan {
    pub goal: String,
    pub steps: Vec<ToolStep>
}

// executing the plan
pub async fn execute_plan(plan: ToolPlan, registry: &ToolRegistery) -> HashMap<String, ToolOutput> {
    let mut results: HashMap<String, ToolOutput> = HashMap::new();

    for step in plan.steps {
        let resolved_args = resolve_args(&step.args, &results);
        let tool = {
            let tools = registry.read().await;
            tools.get(&step.tool_name).cloned()
        };

        let output = match tool {
            Some(tool) => tool.execute(ToolInput { args: resolved_args }).await,
            None => {
                ToolOutput {
                    result: Value::Null,
                    success: false,
                    message: Some(format!("Tool not found: {}", step.tool_name))
                }
            }
        };
        results.insert(step.output_key.clone(), output);
    }

    results
}


// resolving dependencies between steps
// this function walks the JSON tree and substitutes any ${var} references with the actual value from previous tool outputs.
// placeholder function
#[allow(unused)]
fn resolve_args(template: &Value, context: &HashMap<String, ToolOutput>) -> Value {
    match template {
        Value::String(s) => {},
        Value::Object(map) => {},
        Value::Array(arr) => {},
        other => {},
    };
    Value::Null
}

