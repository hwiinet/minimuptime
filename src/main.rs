use ping::*;
use std::net::IpAddr;
use colored::*;
use std::thread;
use std::time::Duration;

fn justping(ip: IpAddr) {
    let timeout = Duration::from_secs(1); // Set timeout to 1 second
    let ping_result = ping(ip, Some(timeout), None, None, None, None);

    match ping_result {
        Ok(_ping_result) => {
            println!("[ONLINE][{}@{}] Device is online", ip.to_string().green().bold(), chrono::Utc::now().format("%Y-%m-%d %H:%M:%S").to_string().purple().bold());
        }
        Err(_e) => {
            println!("[ERROR!][{}@{}] Device is offline", ip.to_string().red().bold(), chrono::Utc::now().format("%Y-%m-%d %H:%M:%S").to_string().red().bold());
        }
    }
}

#[tokio::main]
async fn main() {
    let args = std::env::args().collect::<Vec<String>>();
    let ipfile: String;
    let interval = Duration::from_secs(30);

    if args.len() == 2 {
        ipfile = std::fs::read_to_string(&args[1]).expect("Failed to read file");
        println!("[NOTICE] {}", "Reading IP list from file...".cyan().bold());
    } else {
        ipfile = std::fs::read_to_string("iplist.txt").expect("Failed to read iplist.txt");
        println!("[NOTICE] {}", "Reading IP list from iplist.txt...".cyan().bold());
    }

    let iplist: Vec<&str> = ipfile.split("\n").collect();

    loop {
        println!("[NOTICE] {}", "Pinging devices...".cyan().bold());
        for ip in &iplist {
            if ip.len() > 0 {
                let ip = ip.parse().expect("Failed to parse IP address");
                justping(ip);
            }
        }
        thread::sleep(interval);
    }
}

