use tokio::process::Command;
use std::time::Duration;
use tokio::time::timeout;

// designing a safe execution wrapper
pub async fn safe_exec(command: &str, args: &[String]) -> Result<String, String> {
    let output = timeout(Duration::from_secs(5), async {
        Command::new(command)
            .args(args)
            .output()
            .await
    }).await;
    match output {
        Ok(Ok(output)) if output.status.success() => {
            let stdout = String::from_utf8_lossy(&output.stdout).to_string();
            Ok(stdout.trim().to_string())
        }
        Ok(Ok(output)) => {
            let stderr = String::from_utf8_lossy(&output.stderr).to_string();
            Err(format!("Command failed: {}", stderr.trim()))
        }
        Ok(Err(e)) => Err(format!("Execution error: {}", e)),
        Err(_) => Err("Command timed out".into())
    }
}

// auditing and logging every invocation
/*
tracing::info!(
    command = command,
    args = ?args,
    result = result.as_ref().map(|s| s.chars().count()).unwrap_or(0),
    status = %result.is_ok(),
    "System command executed"
);
*/

// handling cross-platform differences
#[cfg(target_os = "linux")]
pub fn get_command_alias(name: &str) -> Option<&'static str> {
    match name {
        "list" => Some("ls"),
        "disk" => Some("df"),
        _ => None,
    }
}