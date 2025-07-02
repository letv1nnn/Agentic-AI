
use std::io::Read;
use std::collections::HashSet;
use std::net;
use std::sync::{mpsc, Arc, Mutex};
use std::thread;
use std::time::{SystemTime, UNIX_EPOCH};

// if you want to test it on different devices, you might
// need to change this IP to yours *ipconfig on Windows and
// ifconfig on Linux to see your IP addr), then check it 
// with the ping command.
// Also configure your firewall to allow other connections.
static LOCAL: &str = "127.0.0.1:6000";

struct SharedState {
    connected_users: HashSet<String>,
}

fn main() {
    let listener = Arc::new(net::TcpListener::bind(LOCAL).unwrap());
    let (tx, rx) = mpsc::channel::<(String, String)>();
    let shared_state = Arc::new(Mutex::new(
        SharedState{
            connected_users: HashSet::new(),
        }
    ));

    let state_clone = Arc::clone(&shared_state);

    thread::spawn(move || {
        loop {
            thread::sleep(std::time::Duration::from_secs(10));
            let state = state_clone.lock().unwrap();
            println!("[SERVER] Currently connected users: {:?}", state.connected_users);
        }
    });

    let listener_clone = Arc::clone(&listener);
    let tx_clone = tx.clone();
    let shared_state_clone = Arc::clone(&shared_state);

    thread::spawn(move || {
        for stream in listener_clone.incoming() {
            let mut stream = match stream {
                Ok(s) => s,
                Err(e) => {
                    eprintln!("Connection failed: {}", e);
                    continue;
                }
            };
            let tx_conn = tx_clone.clone();
            let state_for_thread = Arc::clone(&shared_state_clone);

            thread::spawn(move || {
                let user_id = format!("user_{}", 
                    SystemTime::now()
                        .duration_since(UNIX_EPOCH)
                        .unwrap()
                        .as_secs()
                );

                {
                    let mut state = state_for_thread.lock().unwrap();
                    state.connected_users.insert(user_id.clone());
                    println!("[SERVER] {} connected", user_id);
                }

                let mut buffer = [0; 1024];
                loop {
                    let bytes_read = match stream.read(&mut buffer) {
                        Ok(0) => break,
                        Ok(n) => n,
                        Err(e) => {
                            eprintln!("Read error for {}: {}", user_id, e);
                            break
                        },
                    };
                
                    let message = String::from_utf8_lossy(&buffer[..bytes_read]).to_string();

                    if tx_conn.send((user_id.clone(), message)).is_err() {
                        eprintln!("Failed to send message from {}", user_id);
                        break;
                    }
                }

                {
                    let mut state = state_for_thread.lock().unwrap();
                    state.connected_users.remove(&user_id);
                    println!("[SERVER] {} disconnected", user_id);
                }
            });
        }
    });

    for (user_id, message) in rx {
        match message.trim() {
            ":discon" => println!("User Disconnected: <user: {:?}>", user_id),
            ":users" => {
                let state = shared_state.lock().unwrap();
                println!("[SERVER] User list requested by {}: {:?}", user_id, state.connected_users);
            },
            _ => println!("Received: {:}", message.trim()),
        }
        
    }
}