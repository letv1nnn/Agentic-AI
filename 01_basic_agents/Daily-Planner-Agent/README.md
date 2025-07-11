# Daily Planner Agent

## Description
A lieight terminal-based AI agent that helps plan your day based on:
- Available calendar slots
- Task priorities and deadlines
- Time preferences (e.g., "No meetings before 10am")

The agent uses a local large language model (LLM) through [Ollama](https://ollama.com) to process inputs and return a structured schedule.

---

### Current Components
- **Ollama I/O Layer** – Sends user input to Ollama and reads streamed response
- **Planner Agent** – Handles input, manages prompts, displays output
- **CLI Interface** – Interactive command shell
- **Agent Core (WIP)** – Abstracted planning logic using trait-based architecture

---

## Requirements

- **Rust**
- **Tokio** (for async runtime)
- **Ollama** installed and running locally

> To install Ollama, follow [official instructions](https://ollama.com/download).

---

## Build & Run Instructions

1. **Clone the repo**
   ```bash
   git clone https://github.com/letv1nnn/Agentic-AI-with-Rust.git
   cd daily-planner-agent
   ```
2. **Build**
   ```bash
   cargo build --release
   ```
3. **Run**
   ```bash
   sh ./run.sh/
   ```
4. **Clean**
   ```bash
   cargo clean
   ```
