# Local File Manager Assistant

## Description
A smart local file management assistant that uses natural language processing (via Ollama) to organize and manage your files through human understadable commands, basically just a queries like "Summarize the content of all .txt files in the specific directory".
NOTE: The Agent is kinda slow now, due to my laptop with 8gb of RAM, so I use mistral ollama instead of wizardlm which is much faster. However, you can configure it by changing the mistral to other models in the ollama_integration.rs file, depending on your RAM.

## Features
- Natural Language Processing: Understands commands like "Move all PDFs to Documents" or "Find large files"
- File Organization: Automatically categorizes files by type (documents, images, videos, etc.)
- Smart Archiving: Identifies and moves stale files to archive
- File Operations: Move, collect, analyze, and manage files with simple commands
- Audit Logging: All actions are logged for accountability
- Shell Command Execution: Can execute system commands when needed

## Prerequisites
***Download Ollama primarily***
```bash
curl -fsSL https://ollama.com/install.sh | sh
```
***To run Ollama***
```bash
ollama run mistral
```
***Clonning***
```bash
git clone git@github.com:letv1nnn/Agentic-AI.git
cd Agentic-AI/02_intermediate_agents/LocalFileManager
```

