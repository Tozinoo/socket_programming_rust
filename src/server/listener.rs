// use std::collections::HashMap;
// use std::thread;
// use mio::net::TcpListener;
// use mio::{Events, Interest, Poll, Token};
// use std::io::{Read, Write};
// use std::net::{IpAddr, SocketAddr};
// use std::sync::Arc;
// use rayon::ThreadPoolBuilder;
// use crate::server::events;
//
// const LISTENER: Token = Token(0);
//
// pub fn run(server_ip: IpAddr, server_port: u16) -> std::io::Result<()> {
//     let server_address = SocketAddr::new(server_ip, server_port);
//
//     let mut listener = TcpListener::bind(server_address)?;
//
//     let mut poll = Poll::new()?;
//     poll.registry()
//         .register(&mut listener, LISTENER, Interest::READABLE | Interest::WRITABLE).expect("poll error");
//
//     let mut events = Events::with_capacity(128);
//     let mut clients: HashMap<Token, (mio::net::TcpStream, Vec<u8>)> = HashMap::new();
//     let mut next_client_token: usize = 1;
//
//     let listener = Arc::new(listener);
//
//
//     println!("Server listening on {}", server_address);
//
//     loop {
//         poll.poll(&mut events, None);
//
//         events::handle_events(
//             &mut poll,
//             Arc::clone(&listener),
//             &mut clients,
//             &mut next_client_token,
//             &mut events,
//         )?;
//     }
// }

use mio::net::TcpListener;
use mio::{Poll, Token, Interest};
use std::net::SocketAddr;

pub const SERVER: Token = Token(0);

pub fn setup_listener(addr: &SocketAddr, poll: &mut Poll) -> std::io::Result<TcpListener> {
    let mut listener = TcpListener::bind(*addr)?;
    poll.registry()
        .register(&mut listener, SERVER, Interest::READABLE)?;
    println!("Listening on {}", addr);
    Ok(listener)
}

pub fn accept_clients(
    listener: &mut TcpListener,
) -> Vec<(mio::net::TcpStream, SocketAddr)> {
    let mut accepted_clients = Vec::new();
    loop {
        match listener.accept() {
            Ok((stream, addr)) => {
                println!("New client connected: {}", addr);
                accepted_clients.push((stream, addr));
            }
            Err(ref e) if e.kind() == std::io::ErrorKind::WouldBlock => {
                break; // 더 이상 수락할 연결 없음
            }
            Err(e) => {
                eprintln!("Error accepting client: {}", e);
                break;
            }
        }
    }
    accepted_clients
}
