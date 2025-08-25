# Designing the Core Agent Architecture

***Defining a Core Agent Trait***

Typically includes a function to receive input, process it, and return a response or action. It receives a task or input, process it asynchronously, and returns a structured result.
```rust
#[async_trait]
pub trait Agent {
    async fn handle_input(&mut self, input: AgentInput) -> AgentResult;
}
```

***Implementing a Reactive Agent***

Here's a basic reactive agent that echoes commands and fails on forbidden keywords. This kind of agent is useful for determenistic, fast responses to events.
```rust
if input.message.contains("shutdown") {
    return AgentResult {
        output: String::new(),
        status: AgentStatus::Error("Restricted command".into())
    };
}
AgentResult {
    output: format!("Echo: {}", input.message),
    status: AgentStatus::Error("Restricted command".into())
}
```

***Building a Planning Agent***

A planning agent performs multi-step reasoning. Instead of responding immediately, it generates a plan, possibly consults an LLM, breaks down tasks, executes them in sequence. In the example below, `plan_task()` could call and LLM or heuristic planner to break the input into smaller steps.
```rust
// possible agent structure
pub struct PlanningAgent {
    pub steps: Vec<String>,
}

// implementation of `handle_input` function
self.steps = plan_task(&input.message).await;
if self.steps.is_empty() {
    return AgentResult {
        output: "No viable plan".into(),
        status: AgentStatus::Error("Planning failed".into()),
    }
}

for step in &self.steps {
    println!("Executing step: {}", step);
}

AgentResult {
    output: format!("Executed {} steps", self.steps.len()),
    status: AgentStatus::Success,
}
```


***Event-driven design patterns***

In the event-driven system, components do not pull for updates. Instead, they react to signals pushed be producers. An event could be anything: a file update, a user command, a task completion, an error from subprocess, or a message from another agent. These events are sent through channels and processed by listeners, usually async tasks, without blocking the main thread or each other.

***Event Types*** (as mentioned, events can be anything).
```rust
pub enum AgentEvent {
    InputRecieved(String),
    TaskCompleted { task_id: String, success: bool },
    SystemAlert(String),
    ExternalMessage(String),
}
```

***Tokio channel for sending and receiving these events***
```rust
pub type EventSender = mpsc::Sender<AgentEvent>;
pub type EventReciever = mpsc::Receiver<AgentEvent>;

let (tx, rx): (EventSender, EventReciever) = mpsc::channel(100);
```

***Event-driven Agent Loop***
```rust
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
```

***Publishing Events to the Agent***
```rust
// to launch this loop concurrently:
tokio::spawn(run_agent(rx));

// multiple producers
let tx_clone = tx.clone();
tokio::spawn(async move {
    tx_clone
        .send(AgentEvent::SystemAlert("Memory usage high".into()))
        .await.unwrap()
})
```

Agent can also be extended with state machines, so they could behaive differently depending on the agent's state.
