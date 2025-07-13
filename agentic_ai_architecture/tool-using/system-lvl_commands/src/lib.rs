// Executing system level commands with safty and security in mind
// This module provides a safe interface for executing system commands,
// ensuring that commands are run in a controlled environment.

// designing a safe execution wrapper

use tokio::process::Command;
use std::time::Duration;
use tokio::time::timeout;
use serde_json::Value;

pub async fn safe_exec(command: &str, args: &[String]) -> Result<String, String> {
    let output = timeout(Duration::froim_secs(5), async {
        .args(args)
        .output()
        .await
    }).await;

    match output {
        Ok(Ok(output)) => {
            if output.status.success() {
                let stdout = String::from_utf8_lossy(&output.stdout).to_string();
                Ok(stdout)
            } else {
                let stderr = String::from_utf8_lossy(&output.stderr).to_string();
                Err(format!("Command failed: {}", stderr))
            }
        },
        Ok(Err(e)) => Err(format!("Failed to execute command: {}", e)),
        Err(_) => Err("Command timed out".to_string()),
    }
}

// Registrating executable tools in the agent.
use serde::{Serialize, Deserialize};

#[derive(Deserialize)]
struct LsArgs {
    path: String,
}

pub struct ListDirTool;

impl crate::Tool for ListDirTool {
    fn name(&self) -> &str {
        "list_directory"
    }

    fn description(&self) -> &str {
        "Lists the contents of a directory."
    }

    async fn execute(&self, input: crate::ToolInput) -> crate::ToolOutput {
        let args = match serde_json::from_value::<LsArgs>(input.args) {
            Ok(args) => args,
            Err(e) => return crate::ToolOutput {
                success: false,
                message: format!("Invalid arguments: {}", e),
            },
        };

        if args.path.contains("..") || args.path.contains(";") {
            return crate::ToolOutput {
                success: false,
                message: "Path traversal is not allowed.".to_string(),
            };
        }

        match safe_exec("ls", &[args.path]).await {
            Ok(output) => crate::ToolOutput {
                success: true,
                message: output,
            },
            Err(e) => crate::ToolOutput {
                success: false,
                message: e,
            },
        }
    }
}

// handling cross-platform differences
#[cfg(target_os = "linux")]
pub fn get_command_alias_linux(name: &str) -> Option<&'static str> {
    match name {
        "list" => Some("ls"),
        "disk" => Some("df"),
        "memory" => Some("free"),
        _ => None,
    }
}
