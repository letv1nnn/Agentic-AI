# Agentic AI Architecture

This repository contains some approaches and design patterns for building AI Systems. It serves as a notes that can be used in a real agents, patterns explores a lot of interesting topics such as Reactive programming, llm integration, deployment stages and integration of many technologies. 

## Discussed topics
- [***Connecting LLM API***](https://github.com/letv1nnn/Agentic-AI-Architecture/tree/main/connecting_llm_api) - templates for openai and ollama api connection.

- [***Core Agent Architecture***](https://github.com/letv1nnn/Agentic-AI-Architecture/tree/main/core_agent_architecture) - basic implementation of a reactive programming approach and a simple actor model using shared state, instead of isolated ownership.
    
    I've created created 4 modeules: ***"agent traits and behavior model"***, ***"task execution, tool invocation and error handling"***, ***"event-driven design patterns"*** and ***"message passing and state management"***. Each module’s implementation is described with its corresponding name. I strongly recommend to read my [***Fearless Concurrency***](https://github.com/letv1nnn/Computer-Science/tree/main/Fearless-Concurrency) research written in Rust, it describes many useful topics related to the agent architecture such as ***Reactive Programming***, ***Coroutines*** and ***Actor Model***.

- [***Tool-Using Agents***](https://github.com/letv1nnn/Agentic-AI-Architecture/tree/main/tool_using_agents) - some code that describes secure tool execution.

    It consists of three modules: ***"chaining tool-use"***, ***"safe system-level commands execution"*** and ***"secure tool functions"***. Basically, these modules contain the right design pattern to execute tools and make a pipeline execution.