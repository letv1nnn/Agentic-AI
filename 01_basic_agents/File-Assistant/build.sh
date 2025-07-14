#!/usr/bin/env bash
# Script for compiling and running the File Assistant agent

cd agent_main || { echo "Failed to change directory to agent_main"; exit 1; }

cargo build --release
if [ $? -ne 0 ]; then
    echo "Build failed. Please check the errors above."
    exit 1
fi