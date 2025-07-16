use agent::tools::{collect_files, get_file_metadata, route_file, move_file, init_logging};
use agent::tools_invocation;


pub enum AgentOutputStatus {
    Success,
    Failure(String),
}

pub struct AgentInput {
    pub query: String,
    pub files: Option<Vec<PathBuf>>,
}

pub struct AgentOutput {
    pub status: AgentOutputStatus,
    pub message: Option<String>,
}

pub struct LocalFileManagerAgent;

impl LocalFileManagerAgent {
    pub fn new() -> Self {
        LocalFileManagerAgent
    }
    pub async fn handle_input(&self, input: AgentInput) -> AgentOutput {
        init_logging();
        if let Some(files) = input.files {
            for path in files {
                if let Some(metadata) = get_file_metadata(&path) {
                    if let Some(dest) = route_file(&metadata) {
                        if let Err(e) = move_file(&metadata, &dest).await {
                            tracing::error!("Failed to move file {}: {}", metadata.path.display(), e);
                            return AgentOutput {
                                status: AgentOutputStatus::Failure(e.to_string()),
                                message: Some(format!("Failed to move file: {}", metadata.path.display())),
                            };
                        }
                    }
                }
            }
        }
        AgentOutput {
            status: AgentOutputStatus::Success,
            message: Some("Files processed successfully.".to_string()),
        }
    }
}
