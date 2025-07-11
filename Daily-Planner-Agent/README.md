# Daily Planner Agent

## Description
A lightweight terminal-based AI agent that helps plan your day based on:
- Available calendar slots
- Task priorities and deadlines
- Time preferences (e.g., "No meetings before 10am")

The agent uses a local large language model (LLM) through [Ollama](https://ollama.com) to process inputs and return a structured schedule.

---

## Core Functionality

### Observation
Accepts natural language input describing:
- Available hours
- Tasks (optional: importance, deadline)
- Personal preferences

### Reasoning
Passes the structured prompt to an LLM (e.g., `mistral`, `llama2`) running locally via Ollama to generate a daily plan.

### Action
- Outputs the schedule directly in the terminal
- Future: Option to save to a file or calendar format (e.g., `.ics`)

---

## System Modules

### Current Components
- **Ollama I/O Layer** – Sends user input to Ollama and reads streamed response
- **Planner Agent** – Handles input, manages prompts, displays output
- **CLI Interface** – Interactive command shell
- **Agent Core (WIP)** – Abstracted planning logic using trait-based architecture

### Upcoming Features
- Tool use: Integrate external functions like calendar checks or task prioritization
- Memory: Save past planning results to improve follow-up requests and suggestions
- JSON output: Optionally return structured output for UI use or export

---

## Requirements

- **Rust** (1.70+ recommended)
- **Tokio** (for async runtime)
- **Ollama** installed and running locally

> To install Ollama, follow [official instructions](https://ollama.com/download).

---

## Build & Run Instructions

1. **Clone the repo**
   ```bash
   git clone https://github.com/your-username/daily-planner-agent.git
   cd daily-planner-agent
