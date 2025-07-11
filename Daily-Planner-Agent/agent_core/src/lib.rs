use ollama_config::send_request;

pub trait Agent {
    fn handle_input(&mut self, input: AgentInput) -> impl Future<Output = AgentResult> + Send;   
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
                    println!("Agent that helps plan a day given some input constraints (calendar slots, tasks, time preferences).");
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

        let input = AgentInput {
            message: prompt.trim().to_string(),
            context: None,
        };

        let mut agent = DailyPlannerAgent;
        let result = agent.handle_input(input).await;

        match result.status {
            AgentStatus::Success => println!("Plan:\n{}", result.response),
            AgentStatus::Error(err) => eprintln!("Error: {}", err),
        }
    }
}

impl Agent for DailyPlannerAgent {
    async fn handle_input(&mut self, input: AgentInput) -> AgentResult {
        let prompt = format!(
            "You are a daily planner assistant. Based on the following request, generate a detailed plan. It should be a table:\n{}\n",
            input.message
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
