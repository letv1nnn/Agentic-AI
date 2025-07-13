use serde_json::Value;

// defining the tool interface
#[derive(Debug)]
pub struct ToolInput {
    pub args: Value; // JSON object with parameters.
}

#[derive(Debug)]
pub struct ToolOutput {
    pub result: Value, // Json result
    pub success: bool,
    pub message: Option<String>,
}

pub trait Tool: Send + Sync {
    fn name(&self) -> &'static str;
    fn description(&self) -> &'static str;

    async fn execute(&self, input: ToolInput) -> ToolOutput;
}

// implementing a tool function
pub struct LengthTool;

impl Tool for LengthTool {
    fn name(&self) -> &'static str {
        "length"
    }
    fn description(&self) -> &'static str {
        "Calculates the length of a given string"
    }

    async fn execute(&self, input: ToolInput) -> ToolOutput {
        let arg = input.args.get("text");

        match arg {
            Some(Value::String(text)) => ToolOutput {
                result: Value::Number(text.len().into()),
                success: true,
                message: None,
            },
            _ => ToolOutput {
                result: Value::Null,
                success: false,
                message: Some("Missing or invalid 'text' argument".into()),
            }
        }

    }
}

// creating a tool registry
// using hashmaps to invoke tools by name.

use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

pub type ToolRegistry = Arc<RwLock<HashMap<String, Arc<dyn Tool>>>>;

// create and populate the registry.
pub async fn registry_config() {
    let mut registry = HashMap::new();
    registry.insert("length".to_string(), Arc::new(LengthTool));
    let registry = Arc::new(RwLock::new(registry));
}

// register a new tool
pub async fn register_tool(registry: &ToolRegistry, tool: Arc<dyn Tool>) {
    let mut tools = registry.write().await;
    tools.insert(tool.name().to_string(), tool);
}

