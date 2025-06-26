// Port Sniffer
// flags:
// -h -> help
// -j, number of threads, ip -> how many threads
// no flag, just an IP address

extern crate port_sniffer;

use port_sniffer::Arguments;
use port_sniffer::scan;
use std::env;
use std::process;
use std::thread;
use std::sync::mpsc::channel;

fn main() {
    let args: Vec<String> = env::args().collect();
    let arguments = Arguments::new(&args).unwrap_or_else(
        |err| {
            if err.contains("help") {
                process::exit(1);
            } else {
                eprintln!("{} issue parsing arguments: {}", args[0].clone(), err);
                process::exit(1);
            }
        }
    );

    let num_threads: u16 = arguments.threads;
    let (tx, rx) = channel();
    let mut handles = vec![];
    for i in 0..num_threads {
        let tx = tx.clone();
        let handle = thread::spawn(move || {
            scan(tx, i, arguments.ipaddr, num_threads);
        });
        handles.push(handle);
    }

    drop(tx);

    for handle in handles {
        handle.join().unwrap();
    }

    let mut out: Vec<u16> = rx.iter().collect();
    out.sort();
    println!("\nOpen ports:");
    for port in out {
        println!("{}", port);
    }
}
