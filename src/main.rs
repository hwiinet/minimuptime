use ping::*;
use std::net::IpAddr;
use colored::*;
use std::thread;
use std::time::Duration;
use std::io::Write;
use serde::Deserialize;

#[derive(Deserialize)]
struct MmuConfig {
    interval: u64,
    ipfile: String,
    timeout: i64,
}

#[derive(Deserialize)]
struct Config {
    mmu: MmuConfig,
}

fn justping(ip: IpAddr, conftimeout: u64) {
    let timeout = Duration::from_secs(conftimeout); // Set timeout to 1 second
    let ping_result = ping(ip, Some(timeout), None, None, None, None);
    let logfile = "log.txt";

    match ping_result {
        Ok(_ping_result) => {
            println!("[ONLINE][{}@{}] Device is online", ip.to_string().green().bold(), chrono::Utc::now().format("%Y-%m-%d %H:%M:%S").to_string().purple().bold());
            let mut file = std::fs::OpenOptions::new().append(true).open(logfile).expect("Failed to open log file");
            let log = format!("{} - {} - ONLINE\n", chrono::Utc::now().format("%Y-%m-%d_%H:%M:%S").to_string(), ip.to_string());
            write!(file, "{}", log).expect("Failed to write to log file");
        }
        Err(_e) => {
            println!("[ERROR!][{}@{}] Device is offline", ip.to_string().red().bold(), chrono::Utc::now().format("%Y-%m-%d %H:%M:%S").to_string().red().bold());
            let mut file = std::fs::OpenOptions::new().append(true).open(logfile).expect("Failed to open log file");
            let log = format!("{} - {} - OFFLINE\n", chrono::Utc::now().format("%Y-%m-%d_%H:%M:%S").to_string(), ip.to_string());
            file.write_all(log.as_bytes()).expect("Failed to write to log file");
        }
    }
}

#[tokio::main]
async fn main() {
    let configfile = std::fs::read_to_string("minimuptime.toml").expect("Failed to read minimuptime.toml");
    let config = toml::from_str::<Config>(&configfile).expect("Failed to parse minimuptime.toml");
    let args = std::env::args().collect::<Vec<String>>();
    let mut ipfile = std::fs::read_to_string(&config.mmu.ipfile).expect("Failed to read ipfile");
    let interval = Duration::from_secs(config.mmu.interval);
    let timeout: i64 = config.mmu.timeout;

    if args.len() == 2 {
        ipfile = std::fs::read_to_string(&args[1]).expect("Failed to read file");
        println!("[NOTICE] {} {}...", "Reading IP list from file".cyan().bold(), args[1].cyan().bold());
    } else {
        println!("[NOTICE] {}", "Reading IP list defined in config file...".cyan().bold());
    }

    let iplist: Vec<&str> = ipfile.split("\n").collect();

    loop {
        println!("[NOTICE] {}", "Pinging devices...".cyan().bold());
        for ip in &iplist {
            if ip.len() > 0 {
                let ip = ip.parse().expect("Failed to parse IP address");
                justping(ip, timeout.try_into().unwrap());
            }
        }
        thread::sleep(interval);
    }
}

