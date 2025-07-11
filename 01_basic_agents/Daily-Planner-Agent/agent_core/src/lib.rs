use ollama_config::send_request;
use chrono::Local;

pub trait Agent {
    fn handle_input(&mut self, input: AgentInput) -> impl std::future::Future<Output = AgentResult> + Send;   
}

pub struct AgentInput {
    pub message: String,
    pub context: Option<String>,
}

pub struct AgentResult {
    pub response: String,
    pub status: AgentStatus,
}

pub enum AgentStatus {
    Success,
    Error(String),
}

pub struct DailyPlannerAgent;

impl DailyPlannerAgent {
    pub async fn new() -> Self {
        DailyPlannerAgent
    }

    pub async fn run(&mut self) {
        println!("+---------------------------------------+");
        println!("|  <=====> DAILY PLANNED AGENT <=====>  |");
        println!("+---------------------------------------+");

        println!("Available commands\n  \"-help\"\n  \"-info\"\n  \"-plnr\"\n  \"-exit\"");

        loop {
            println!("Command:");
            let mut command = String::new();
            std::io::stdin().read_line(&mut command).expect("Failed to read line!");
            match command.trim() {
                "-plnr" | "-p" => {
                    self.planner().await;
                },
                "-help" | "-h" => {
                    println!("Available commands\n  \"-help, -h\" ->> commands info\n  \"-info, -i\" ->> info about the agent\n  \"-plnr, -p\" ->> daily planner agent\n  \"-exit, -quit, -q\" ->> shut down the agent");
                },
                "-info" | "-i" => {
                    println!("Agent that helps plan a day given some input constraints (calendar slots, tasks, time preferences).\nIt based on the current time, so it is not mandatory to specify it.");
                },
                "-exit" | "-quit" | "-q" => {
                    println!("Have a good one!");
                    break;
                },
                _ => {
                    println!("Unknown command.");
                },
            };
        }
    }

    pub async fn planner(&self) {
        let mut prompt = String::new();
        println!("Enter your planning request (tasks, constraints, time preferences):");
        std::io::stdin().read_line(&mut prompt).expect("Failed to read line!");

        println!("It might take some time...");

        let input = AgentInput {
            message: prompt.trim().to_string(),
            context: None,
        };

        let mut agent = DailyPlannerAgent;
        let result = agent.handle_input(input).await;

        match result.status {
            AgentStatus::Success => println!("Plan:\n{}", result.response),
            AgentStatus::Error(err) => eprintln!("❌ Error: {}", err),
        }
    }
}

impl Agent for DailyPlannerAgent {
    async fn handle_input(&mut self, input: AgentInput) -> AgentResult {
        let current_time = Local::now();

        let prompt = format!(
            "You are a daily planner assistant. Rely on the current time: {:?}, or based on the time that user specified. Based on the following request, generate a detailed plan as a table, table should be readable:\n{}\n",
            current_time, input.message
        );

        match send_request(&prompt).await {
            Ok(response) => AgentResult {
                response,
                status: AgentStatus::Success,
            },
            Err(err) => AgentResult {
                response: "".into(),
                status: AgentStatus::Error(err.to_string()),
            },
        }
    }
}
