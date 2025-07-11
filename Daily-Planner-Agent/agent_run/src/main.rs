
use agent_core::DailyPlannerAgent;

#[tokio::main]
async fn main() {
    let mut agent = DailyPlannerAgent::new().await;
    agent.run().await;

}
