mod server;
mod client;

use std::env::args;
use std::net::IpAddr;

fn main() -> std::io::Result<()> {
    let args: Vec<String> = args().collect();
    if args.len() != 3 {
        eprintln!("Usage: {} <IP> <PORT>", args[0]);
        std::process::exit(1);
    }

    let server_ip: IpAddr = args[1].parse().expect("Invalid IP address format");
    let server_port: u16 = args[2].parse().expect("Invalid Port format");

    server::start_server(server_ip, server_port)
}
