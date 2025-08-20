#![allow(unused)]

use std::{collections::HashMap, sync::{Arc}};
use tokio::sync::{Mutex, RwLock};
use tokio::sync::{mpsc, oneshot};

// asynchronous message passing with channels
#[derive(Debug)]
pub enum AgentMessage {
    Command(String),
    StatusUpdate {task_id: String, status: String},
    Shutdown,
}

pub async fn running_module() {
    let (tx, mut rx) = mpsc::channel::<AgentMessage>(100);
    tokio::spawn(async move {
        while let Some(message) = rx.recv().await {
            match message {
                AgentMessage::Command(cmd) => {
                    println!("Received comand: {}", cmd);
                },
                AgentMessage::StatusUpdate { task_id, status } => {
                    println!("Task {} updated: {}", task_id, status);
                },
                AgentMessage::Shutdown => {
                    println!("Agent shutting down");
                    break;
                }
            }
        }
    });

    // from another part of the system 
    let sender = tx.clone();
    tokio::spawn(async move {
        sender
            .send(AgentMessage::Command("analyze logs".into()))
            .await
            .unwrap()
    });
}


// request/respond between tasks
// if two-way communication is required, such as requesting a result from
// another component, combine mpsc with oneshot channels.
#[derive(Debug)]
pub enum AgentRequest {
    ExecuteTask {
        command: String,
        respond_to: oneshot::Sender<Result<String, String>>
    },
    Other {}
}

pub async fn req_resp_between_tasks() {
    let (tx, mut rx) = mpsc::channel::<AgentRequest>(100);
    while let Some(req) = rx.recv().await {
        if let AgentRequest::ExecuteTask { command, respond_to } = req {
            let result = match command.as_str() {
                "ping" => Ok("pong".into()),
                _ => Err("unknown command".into())
            };
            let _ = respond_to.send(result);
        }
    }

    let (resp_tx, resp_rx) = oneshot::channel();
    tx.send(AgentRequest::ExecuteTask { command: "ping".into(), respond_to: resp_tx }).await.unwrap();
    let response = resp_rx.await.unwrap();
    println!("Received: {:?}", response);
}

// state management with Arc, Mutex and RwLock
type MemoryStore = Arc<RwLock<HashMap<String, String>>>;

struct AgentState {
    current_task: Option<String>,
    completed_tasks: Vec<String>
}

pub async fn state_management() {
    let memory: MemoryStore = Arc::new(RwLock::new(HashMap::new()));

    let mem_clone = memory.clone();
    tokio::spawn(async move {
        let mut store = mem_clone.write().await;
        store.insert("last task".into(), "summarize data".into());
    });

    let mem_clone = memory.clone();
    tokio::spawn(async move {
        let store = mem_clone.read().await;
        if let Some(task) = store.get("last_task") {
            println!("Last task was: {}", task);
        }
    });

    let _data = {
        let store = memory.read().await;
        let _ = store.get("key").cloned();
    };
}

async fn agent_main(mut rx: mpsc::Receiver<AgentMessage>, state: Arc<Mutex<AgentState>>) {
    while let Some(msg) = rx.recv().await {
        match msg {
            AgentMessage::Command(cmd) => {
                {
                    let mut s = state.lock().await;
                    s.current_task = Some(cmd.clone());
                }
                println!("Executeing task: {}", cmd);
            }
            AgentMessage::Shutdown => break,
            AgentMessage::StatusUpdate { task_id, status } if status == "done" => {
                {
                    let mut s = state.lock().await;
                    s.completed_tasks.push(task_id);
                    s.current_task = None;
                }
                println!("Task completed and state updated.");
            }
            _ => {}
        }
    }
}