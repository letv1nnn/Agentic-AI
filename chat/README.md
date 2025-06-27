# Rust TCP Chat Application

A simple TCP-based chat application implemented in Rust, demonstrating non-blocking I/O, multi-threading, and message passing between clients and a server.

## Features

- Non-blocking TCP server that handles multiple client connections
- Clients can send and receive messages in real-time
- Message broadcasting to all connected clients
- Graceful handling of client disconnections
- Fixed-size message buffers (32 bytes) with null-byte padding

## How It Works

1. The server binds to `127.0.0.1:6000` and listens for incoming connections
2. Clients connect to the same address and can send messages via stdin
3. Messages are broadcast to all connected clients
4. Type `:quit` to disconnect a client

## Usage

1. Start the server:
   ```bash
   cargo run server
2. Start the client:
   ```bash
   cargo run client
![image](https://github.com/user-attachments/assets/99425682-5be8-427b-9726-a21bc242d4de)

