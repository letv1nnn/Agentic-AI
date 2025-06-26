use std::io::{self, Write};
use std::net::{IpAddr, TcpStream};
use std::str::FromStr;
use std::sync::mpsc::Sender;

const MAX_PROTS: u16 = 65535;

#[allow(dead_code)]
pub struct Arguments {
    pub flag: String,
    pub ipaddr: IpAddr,
    pub threads: u16,
}

impl Arguments {
    pub fn new(args: &[String]) -> Result<Arguments, &'static str> {
        if args.len() < 2 {
            return Err("Not enough arguments!");
        } else if args.len() > 4 {
            return Err("Too many arguments!");
        }
        let f = args[1].clone();
        match IpAddr::from_str(&f) {
            Ok(ipaddr) => Ok(Arguments { flag: "".to_string(), ipaddr, threads: 4 }),
            _ => {
                if (f.contains("-h") || f.contains("-help")) && args.len() == 2 {
                    println!("Usage: -j to select how many threads you want\r\n-h or -help to show this help message.");
                    Err("help")
                } else if f.contains("-h") || f.contains("-help") {
                    Err("Too many arguments")
                } else if f.contains("-j") {
                    let ipaddr = match IpAddr::from_str(&args[3]) {
                        Ok(ip) => ip,
                        Err(_) => return Err("Incorrect behaviour!"),
                    };
                    let threads: u16 = match args[2].trim().parse() {
                        Ok(threads) => threads,
                        Err(_) => return Err("Failed to parse threads to integer!"),
                    };
                    return Ok(Arguments { threads, flag: f, ipaddr });
                } else {
                    return Err("Invalid syntax");
                }
            },
        }
    }
}

pub fn scan(tx: Sender<u16>, start_point: u16, ipaddr: IpAddr, num_threads: u16) {
    let mut port = start_point + 1;
    loop {
        match TcpStream::connect((ipaddr, port)) {
            Ok(_) => {
                print!(".");
                io::stdout().flush().unwrap();
                tx.send(port).unwrap(); 
            },
            Err(_) => {}
        }
        if (MAX_PROTS - port )<= num_threads {
            break;
        }
        port += num_threads;
    }
}