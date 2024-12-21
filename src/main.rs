use ping::ping;
use std::net::IpAddr;
use colored::*;
use std::time::Duration;

fn main() {
    // collect args and parse them
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        println!("Usage: {} <ip>", args[0]);
        return;
    }
    let ip = args[1].parse::<IpAddr>().unwrap(); // Grab IP from args
    
    // set required options for ping()
    let timeout = Duration::from_secs(1); // Set timeout to 1 second
    let ping_result = ping(ip, Some(timeout), None, None, None, None);

    match ping_result {
        Ok(_ping_result) => {
            println!("{}", "Device is online".green().bold());
        }
        Err(e) => {
            println!("{}", e.to_string().red().bold());
        }
    }
}

