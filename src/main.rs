use anyhow::Result;
use clap::Parser;
use std::net::Ipv4Addr;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Show only TCP ports
    #[arg(short, long)]
    tcp: bool,
    /// Show only UDP ports
    #[arg(short, long)]
    udp: bool,
}

fn main() -> Result<()> {
    let args = Args::parse();
    let show_tcp = !args.udp;
    let show_udp = !args.tcp;

    if show_tcp {
        if let Ok(content) = std::fs::read_to_string("/proc/net/tcp") {
            println!("Proto Local Address:Port  Remote Address:Port  State");
            for line in content.lines().skip(1) {
                let parts: Vec<&str> = line.split_whitespace().collect();
                if parts.len() < 4 {
                    continue;
                }
                let local = decode_addr(parts[1]);
                let state = parts[3];
                if state == "0A" {
                    println!("tcp   {}             0.0.0.0:*            LISTEN", local);
                }
            }
        }
    }

    if show_udp {
        if let Ok(content) = std::fs::read_to_string("/proc/net/udp") {
            println!("\nProto Local Address:Port  Remote Address:Port  State");
            for line in content.lines().skip(1) {
                let parts: Vec<&str> = line.split_whitespace().collect();
                if parts.len() < 4 {
                    continue;
                }
                let local = decode_addr(parts[1]);
                println!("udp   {}             0.0.0.0:*            LISTEN", local);
            }
        }
    }
    Ok(())
}

fn decode_addr(addr_port: &str) -> String {
    let parts: Vec<&str> = addr_port.split(':').collect();
    if parts.len() != 2 {
        return addr_port.to_string();
    }
    let hex_ip = u32::from_str_radix(parts[0], 16).unwrap_or(0);
    let port = u16::from_str_radix(parts[1], 16).unwrap_or(0);
    let ip = Ipv4Addr::from(hex_ip.swap_bytes());
    format!("{}:{}", ip, port)
}
