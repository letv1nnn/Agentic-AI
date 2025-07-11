#!/usr/bin/en bash

echo "Running the agent..."

if [[ "$OS" == "Windows_NT" ]]; then
    ./target/release/agent_run.exe
else
    ./target/release/agent_run
fi

