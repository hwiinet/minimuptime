use ping::*;
use std::net::IpAddr;
use colored::*;
use tokio::time::{self, Duration};

fn justping(ip: IpAddr) {
    let timeout = Duration::from_secs(1); // Set timeout to 1 second
    let ping_result = ping(ip, Some(timeout), None, None, None, None);

    match ping_result {
        Ok(_ping_result) => {
            println!("[{}] Device is online", ip.to_string().green().bold());
        }
        Err(_e) => {
            println!("[{}] Device is offline", ip.to_string().red().bold());
        }
    }
}

#[tokio::main]
async fn main() {
    let args = std::env::args().collect::<Vec<String>>();
    let ipfile: String;
    let interval = Duration::from_secs(1);

    if args.len() == 2 {
        ipfile = std::fs::read_to_string(&args[1]).expect("Failed to read file");
    } else {
        ipfile = std::fs::read_to_string("iplist.txt").expect("Failed to read iplist.txt");
    }

    let iplist: Vec<&str> = ipfile.split("\n").collect();

    for ip in iplist {
        if ip.len() > 0 {
            let ip = ip.parse().expect("Failed to parse IP address");
            justping(ip);
        }
    }
}

