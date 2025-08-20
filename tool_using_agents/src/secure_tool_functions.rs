use std::{collections::HashMap, sync::Arc};

use async_trait::async_trait;
use serde_json::Value;
use serde::Deserialize;
use tokio::sync::RwLock;

// defining a trait for all tool functions.
// this ensures that tools can be stored in collections, invoked with parameters, and return structured output.
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

// implementing a tool function
pub struct LengthTool;

#[async_trait]
impl Tool for LengthTool {
    fn name(&self) -> &'static str {
        "length"
    }
    fn description(&self) -> &'static str {
        "Calculates the length of a given string"
    }

    async fn execute(&self, input: ToolInput) -> ToolOutput {
        /*
        let arg = input.args.get("text");
        match arg {
            Some(Value::String(text)) => ToolOutput {
                result: Value::Number(text.len().into()),
                success: true,
                message: None
            },
            _ => ToolOutput {
                result: Value::Null,
                success: false,
                message: Some("Missing or invalid 'text' argument".into())
            },
        }
        */
        match serde_json::from_value::<TextArgs>(input.args) {
            Ok(parsed) => {
                let len = parsed.text.len();
                ToolOutput {
                    result: Value::Number(len.into()),
                    success: true,
                    message: None
                }
            }
            Err(e) => ToolOutput {
                result: Value::Null,
                success: false,
                message: Some(format!("Invalid input: {}", e))
            }
        }
    }
}

// creating a tool registery
// to invoke tools by name, use a registery that maps tool names to trait objects.
pub type ToolRegistery = Arc<RwLock<HashMap<String, Arc<dyn Tool>>>>;

// create and populate the registery
/*
let mut registery = HashMap::new();
registry.insert("length".to_string(), Arc::new(LengthTool));
let registery = Arc::new(RwLock::new(registery))
*/

// registring a new tool dynamically
pub async fn register_tool(registry: &ToolRegistery, tool: Arc<dyn Tool>) {
    let mut tools = registry.write().await;
    tools.insert(tool.name().to_string(), tool);
}

// tools invocations at runtime
pub async fn call_tool_by_name(registry: &ToolRegistery, name: &str, args: Value) -> Option<ToolOutput> {
    let tools = registry.read().await;
    if let Some(tool) = tools.get(name) {
        Some(tool.execute(ToolInput { args }).await)
    } else {
        None
    }
}

// ensuring security and validity
#[derive(Deserialize)]
struct TextArgs {
    text: String
}
// I've replaced functionality in the execute function above 

