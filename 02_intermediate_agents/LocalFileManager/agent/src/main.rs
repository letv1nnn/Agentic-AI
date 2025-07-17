#![allow(unused)]

use agent::tools::{collect_files, get_file_metadata, route_file, move_file, init_logging};
use agent::tools_invocation;
use agent::agent_core;

// Agent's Responsibilities:
//   - Orginizing file be type into subfolders.
//   - Deleting or archiving stale files.
//   - Renaming files using consistent format.
//   - Logging every action taken for auditability.

// Input example:
// "Move all old documents to Archive"
// "Find all PDFs modified in the last 7 days"
// "Show me large image files over 10MB"


#[tokio::main]
async fn main() {
    take_input().await;
}


pub async fn take_input() {
    println!("Local File Manager Agent is starting...");
    println!("Type '-help' for available commands");

    let agent = agent_core::LocalFileManagerAgent::new();
    loop {
        println!("\nEnter your command:");
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).expect("Failed to read input");

        match input.trim() {
            "-exit" | "-quit" => {
                println!("Exiting Local File Manager Agent.");
                break;
            },
            "-query" | "-q" => {
                // Prompt for query input
                let mut query = String::new();
                println!("Enter your query:");
                std::io::stdin().read_line(&mut query).expect("Failed to read query");

                if query.trim().is_empty() {
                    println!("Query cannot be empty. Please try again.");
                    continue;
                }

                println!("Processing query: {}...", query.trim());

                // Create agent input
                let agent_input = agent_core::AgentInput {
                    query: query.trim().to_string(),
                };

                // Handle the input using the agentq
                let output = agent.handle_input(agent_input).await;

                // handle the result
                match output.status {
                    agent_core::AgentOutputStatus::Success => {
                        if let Some(msg) = output.message {
                            // println!("Success: {}", msg);
                        } else {
                            // println!("Operation completed successfully");
                        }
                    },
                    agent_core::AgentOutputStatus::Failure(err) => {
                        println!("Error: {}", err);
                    },
                }
            },
            "-help" | "-h" => {
                print_help();
            }
            _ => println!("Unknwown command. Please try again."),
        }
        
    }

}

fn print_help() {
    println!("LOCAL FILE MANAGER AGENT");
    println!("This agent helps manage local files using natural language commands.");
    println!("\nAvailable commands:");
    println!("  -query, -q   - Enter a natural language query");
    println!("  -help, -h    - Show this help message");
    println!("  -exit, -quit - Exit the program");
    println!("\nExample queries:");
    println!("  \"Find all PDFs in my Documents folder\"");
    println!("  \"Move all old files to Archive\"");
    println!("  \"List all large files (>100MB) in Downloads\"");
}