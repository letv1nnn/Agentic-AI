// corestructure of the dynamic tool registry

use sedre_json::Value;
use serde::{Deserialize, Serialize};

pub struct ToolInput {
    pub args: Value,
}

pub struct ToolOutput {
    pub result: Value,
    pub success: bool,
    pub message: Option<String>,
}

pub trait Tool: Send + Sync {
    fn name(&self) -> &'static str;
    fn description(&self) -> &'static str;
    async fn execute(&self, input: ToolInput) -> ToolOutput;
}

// now we define the registry type
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

pub type ToolRegistry = Arc<RwLock<HashMap<String, Arc<dyn Tool>>>>;

// create and populate the registry
pub fn new_registry() -> ToolRegistry {
    Arc::new(RwLock::new(HashMap::new()))
}

// then define functions to register and retrieve tools
pub async fn register_tool(registry: &ToolRegistry, tool: Arc<dyn Tool>) {
    let name = tool.name().to_string();
    let mut tools = registry.write().await;
    tools.insert(name, tool);
}

// quering and using tools by name
pub async fn get_tool(registry: &ToolRegistry, name: &str, args: Value) -> Option<ToolOutput> {
    let tools = registry.read().await;
    if let Some(tool) = tools.get(name) {
        let input = ToolInput { args };
        Some(tool.execute(input).await)
    } else {
        None
    }
}