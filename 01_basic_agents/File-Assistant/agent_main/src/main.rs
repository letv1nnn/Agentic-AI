#![allow(unused)]

extern crate agent_main;

use agent_main::agent_core::{Agent, ReactiveAgent, AgentInput};
use agent_main::tools_invocation::{ToolInvocation, TaskExecutor};

#[tokio::main]
async fn main() {
    let description = String::from("Allows you to interact with your file system using natural language commands.");
    let mut agent = ReactiveAgent::new("File Assistant".to_string(), description);
    info();

    loop {
        match user_input().as_str() {
            "-info" => {
                println!("{:?}", agent);
                info();
            },
            "-exit" => {
                println!("Exiting the File Assistant.");
                break;
            },
            "-fas" => {
                println!("Prompt");
                let mut input = String::new();
                std::io::stdin().read_line(&mut input).expect("Failed to read the prompt!");
                println!("Proccessing... it might take some time.");
            
                let user_prompt = AgentInput { content: input };
            
                agent.handle_input(user_prompt).await;
            },
            _ => {
                println!("Unknown command. Please try again.");
            }
        }
    }
}

fn user_input() -> String {
    println!("Please enter your command:");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("Failed to read input!");
    input.trim().to_string()
}

fn info() {
    println!("Type -exit to exit the File Assistant.");
    println!("Type -fas to start a new File Assistant session.");
    println!("Type -info to see this information again.");
}