#![allow(unused)]

use agent::tools::{collect_files, get_file_metadata, route_file, move_file, init_logging};
use agent::tools_invocation;
use agent::agent_core;

// Agent's Responsibilities:
//   - Orginizing file be type into subfolders.
//   - Deleting or archiving stale files.
//   - Renaming files using consistent format.
//   - Logging every action taken for auditability.

#[tokio::main]
async fn main() {
    println!("Local File Manager Agent is starting...");

    init_logging();
    let dir = "/home/letv1n/Rust/rust_practise";
    let files = collect_files(dir);

    for path in files.await.iter() {
        if let Some(metadata) = get_file_metadata(path) {
            if let Some(dest) = route_file(&metadata) {
                if let Err(e) = move_file(&metadata, &dest).await {
                    tracing::error!("Failed to move file {}: {}", metadata.path.display(), e);
                } 
            }
        }
    }
}
