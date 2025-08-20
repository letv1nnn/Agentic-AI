// agents must handle LLM timouts, token exhaustion, or malformed
// responses gracefully. Use timeouts in the HTTP client and log any failures
// with detailed context. Always validate model outputs, especially if they are
// used to construct shell commands, API requests, or file operations. When
// using local models, monitor CPU and memory consumptions to avoid
// system-level contemtion, especially in multi-agent environments.

pub mod openai;
pub mod ollama;

// For example, use a timeout:
/*
let response = client
    .post("http://localhost:11434/api/chat")
    .timeout(std::time::Duration::from_secs(15))
    .json(&request)
    .send()
    .await?;
*/