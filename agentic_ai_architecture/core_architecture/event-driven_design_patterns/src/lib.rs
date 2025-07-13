use tokio::sync::mpsc;

//  defining event type and  channels

#[derive(Debug)]
pub enum AgentEvent {
    InputRecieved(String),
    TaskCompleted { task_id: String, success: bool },
    SystemAlert(String),
    ExternalMessage(String),
}

// This channel becomes the communication bridge between producers
// (user interfaces, file watchers, external services) and consumers
// (agents, planners, executors).

pub type EventSender = mpsc::Sender<AgentEvent>;
pub type EventReciever = mpsc::Reciever<AgentEvent>;

let (tx, rx): (EventSender, EventReciever) = mpsc::channel(100);


// Designing and event-driven agent loop
pub async fn run_agent(mut rx: EventReciever) {
    while let Some(event) = rx.recv().awai {
        match event {
            AgentEvent::InputRecieved(text) => {
                println!("Agent recieved input: {}", text);
                // proccess input, mb emit task
            },
            AgentEvent::TaskCompleted {task_id, success } => {
                println!(
                    "Task {} completed with status: {}",
                    task_id,
                    if success {"success"} else {"failure"}
                );
                // trigger downstream behavior
            },
            AgentEvent::SystemAlert(message) => {
                eprintln!("System alert: {}", message);
                // possibly escalate
            }
            AgentEvent::ExternalMessage(data) => {
                println!("External message recieved: {}", data);
                // route message to proper handler
            }
        }
    }
}

// to launch this loop concurrently:
// tokio::spawn(run_agent(rx));

// State machines and event-driven transitions
enum AgentState {
    Idle,
    WaitingForTask,
    Executing,
    Error(String),
}

