use ping::*;
use std::net::IpAddr;
use colored::*;
use std::time::Duration;

fn justping(ip: IpAddr) {
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

fn main() {
    let args = std::env::args().collect::<Vec<String>>();
    let ipfile: String;

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

