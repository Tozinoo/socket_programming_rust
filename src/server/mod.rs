pub mod listener;
pub mod events;

use std::net::IpAddr;

pub fn start_server(server_ip: IpAddr, server_port: u16) -> std::io::Result<()> {
    listener::run(server_ip, server_port)
}