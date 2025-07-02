
use std::io::{self, Write};
use std::net::TcpStream;

// if you want to test it on different devices, you might
// need to change this IP to yours *ipconfig on Windows and
// ifconfig on Linux to see your IP addr), then check it 
// with the ping command. 
// Also configure your firewall to allow other connections.
static LOCAL: &str = "127.0.0.1:6000";

fn main() {
    let mut stream = TcpStream::connect(LOCAL).unwrap();
    println!("Connection with the server esteblished.");
    loop {
        println!("Message:");
        let mut msg = String::new();
        io::stdin().read_line(&mut msg).expect("Failed to read line!");
        msg = msg.trim().to_string();
        if msg == ":discon" {
            stream.write_all(msg.as_bytes()).unwrap();
            println!("Disconnecting from server...");
            break;
        }
        stream.write_all(msg.as_bytes()).unwrap();
    }
}
