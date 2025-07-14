# File Assistant

## Description
**LLM File Assistant** is a lightweight, local-first smart file manager powered by a Large Language Model (LLM) using [Ollama](https://ollama.com), written in **Rust** with **Axum**.

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

### File Content Analysis
- Summarize `.txt` or `.md` files using an LLM:  
  → _"Summarize `article.txt` for me"_

### LLM Integration
- Uses [Ollama](https://ollama.com) to run local LLMs like Mistral or LLaMA 3
- Prompts are structured to return actionable commands in JSON
- Example:
  ```json
  { "action": "ListFiles", "path": "./notes" }