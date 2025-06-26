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
    for i in 0..num_threads{
        let tx = tx.clone();
        thread::spawn(move || {
            scan(tx, i, arguments.ipaddr, num_threads);
        });
    }

    let mut out = vec![];
    drop(tx);
    for p in rx {
        out.push(p);
    }

    println!("");
    out.sort();

    for v in out {
        println!("{} is open", v);
    }
}
