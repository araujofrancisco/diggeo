use std::collections::HashSet;
use std::fs;
use std::io::{self, BufRead};
use std::net::{IpAddr, ToSocketAddrs};
use std::process;

use atty::Stream;
use clap::Parser;
use reqwest;
use serde_json::Value;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Domain name to resolve (use this flag to perform a DNS lookup like digeo.sh)
    #[arg(long)]
    dig: Option<String>,

    /// IP addresses to lookup (if not using --dig)
    ips: Vec<String>,
}

/// Reads the configuration file from /etc/diggeo.conf and extracts the API key.
/// 
/// The file should contain a line formatted like:
///   api_key = your_api_key_value
fn read_api_key() -> Result<String, String> {
    let config_path = "/etc/diggeo.conf";
    let content = fs::read_to_string(config_path)
        .map_err(|err| format!("Failed to read config {}: {}", config_path, err))?;

    for line in content.lines() {
        // Remove any comments and trim whitespace.
        let line = line.split('#').next().unwrap_or("").trim();
        if line.is_empty() {
            continue;
        }
        // Look for lines containing a key and value separated by '='.
        if let Some((key, value)) = line.split_once('=') {
            let key = key.trim();
            let value = value.trim();
            if key == "api_key" && !value.is_empty() {
                return Ok(value.to_string());
            }
        }
    }

    Err("api_key not found in config file".to_string())
}


/// Queries the ipgeolocation API to get the country for an IP address.
/// Returns the country name (or "Unknown" if not found).
fn get_country(api_key: &str, ip: &str) -> Result<String, reqwest::Error> {
    let url = format!("https://api.ipgeolocation.io/ipgeo?apiKey={}&ip={}", api_key, ip);
    // Using reqwest's blocking API for simplicity.
    let response = reqwest::blocking::get(&url)?;
    let json: Value = response.json()?;
    let country = json["country_name"].as_str().unwrap_or("Unknown");
    Ok(country.to_string())
}

/// Resolves a domain name into its (IPv4) IP addresses.
/// This uses the standard library DNS lookup via ToSocketAddrs.
/// (Note: unlike the bash version that uses dig + grep, here we simply collect unique IPv4 addresses.)
fn resolve_domain(domain: &str) -> Result<Vec<IpAddr>, Box<dyn std::error::Error>> {
    let addrs = (domain, 80).to_socket_addrs()?;
    let mut ips = HashSet::new();
    for addr in addrs {
        // Filter for IPv4 addresses only (optional).
        if addr.ip().is_ipv4() {
            ips.insert(addr.ip());
        }
    }
    Ok(ips.into_iter().collect())
}

fn main() {
    let args = Args::parse();

    // Read API key from file
    let api_key = match read_api_key() {
        Ok(key) => key,
        Err(e) => {
            eprintln!("Error reading API key: {}", e);
            process::exit(1);
        }
    };

    let mut ips: Vec<String> = Vec::new();

    if let Some(domain) = args.dig {
        // In domain resolution mode – resolve the domain to IP addresses.
        match resolve_domain(&domain) {
            Ok(resolved) if !resolved.is_empty() => {
                for ip in resolved {
                    ips.push(ip.to_string());
                }
            }
            Ok(_) => {
                eprintln!("No IPv4 addresses found for domain: {}", domain);
                process::exit(1);
            }
            Err(e) => {
                eprintln!("Failed to resolve domain {}: {}", domain, e);
                process::exit(1);
            }
        }
    } else if !args.ips.is_empty() {
        // Use the provided IP addresses from command-line arguments.
        ips = args.ips;
    } else if !atty::is(Stream::Stdin) {
        // No arguments; perhaps the input is being piped.
        let stdin = io::stdin();
        for line in stdin.lock().lines() {
            match line {
                Ok(ip) => {
                    let trimmed = ip.trim();
                    if !trimmed.is_empty() {
                        ips.push(trimmed.to_string());
                    }
                }
                Err(e) => {
                    eprintln!("Error reading piped input: {}", e);
                    process::exit(1);
                }
            }
        }
    } else {
        eprintln!("Usage examples:");
        eprintln!("  diggeo 8.8.8.8 1.1.1.1");
        eprintln!("  cat ips.txt | diggeo");
        eprintln!("  diggeo --dig example.com");
        process::exit(1);
    }

    // Process each IP address and display the result.
    for ip in ips {
        match get_country(&api_key, &ip) {
            Ok(country) => println!("{}:{}", ip, country),
            Err(e) => eprintln!("Error fetching geolocation for {}: {}", ip, e),
        }
    }
}
