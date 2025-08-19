use tokio::sync::mpsc;

// REACTIVE PROGRAMMING is the main concept here

//  defining event types and channels
#[derive(Debug)]
pub enum AgentEvent {
    InputRecieved(String),
    TaskCompleted { task_id: String, success: bool },
    SystemAlert(String),
    ExternalMessage(String),
}

pub type EventSender = mpsc::Sender<AgentEvent>;
pub type EventReciever = mpsc::Receiver<AgentEvent>;

// let (tx, rx): (EventSender, EventReciever) = mpsc::channel(100);

// Designing and event-driven agent loop
pub async fn run_agent(mut rx: EventReciever) {
    while let Some(event) = rx.recv().await {
        match event {
            AgentEvent::InputRecieved(text) => {
                println!("Agent received input: {}", text);
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
// multiple producers
/*let tx_clone = tx.clone();
tokio::spawn(async move {
    tx_clone
        .send(AgentEvent::SystemAlert("Memory usage high".into()))
        .await.unwrap()
})*/

// State machines and event-driven transitions
#[allow(unused)]
enum AgentState {
    Idle,
    WaitingForTask,
    Executing,
    Error(String),
}

pub async fn drive_behavior(mut rx: EventReciever) {
    let mut state = AgentState::Idle;
    while let Some(event) = rx.recv().await {
        match (&state, event) {
            (AgentState::Idle, AgentEvent::InputRecieved(input)) => {
                println!("Starting task for input: {}", input);
                state = AgentState::WaitingForTask;
                // Dispatch here
            },
            (AgentState::WaitingForTask, AgentEvent::TaskCompleted {success , ..}) => {
                state = if success {
                    AgentState::Idle
                } else {
                    AgentState::Error("Task failed".into())
                }
            },
            (AgentState::Error(_), AgentEvent::InputRecieved(_)) => {
                println!("Cannot except input while in error state");
            },
            _ => {}
        }
    }
}

