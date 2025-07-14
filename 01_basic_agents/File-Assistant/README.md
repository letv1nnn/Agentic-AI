# Linux File Assistant

## Description
**LLM File Assistant** is a lightweight, local-first smart file manager powered by a Large Language Model (LLM) using [Ollama](https://ollama.com), written in **Rust**.

It allows you to interact with your file system using natural language commands such as:

> "List all `.md` files in my `notes` folder."  
> "Delete all `.tmp` files in `temp/`."  
> "Summarize the contents of `meeting_notes.txt`."

The assistant uses an LLM to parse and plan actions, then safely executes them on your system using built-in Rust tools.

---

## Features & Functionality

### File Management via Natural Language
- List files in any directory:  
  → _"Show all files in `downloads/`"_
- Show largest or smallest files:  
  → _"Find the 5 largest files in `projects/`"_
- Delete selected files with confirmation:  
  → _"Remove all `.log` files from `logs/`"_

---

## Build and Run
- Cline the repo
  ```bash
  git clone https://github.com/letv1nnn/Agentic-AI.git
  cd Agentic-AI/01_basic_agents/File-Assistant/
  ```
- Build
  ```bash
  sh build.sh
  ```
- Run
  ```bash
  ./agent_main/target/release/agent_main
  ```