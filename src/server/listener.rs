use std::thread;
use mio::net::TcpListener;
use mio::{Events, Poll, Token};
use std::collections::HashMap;
use std::io::{Read, Write};
use std::net::{IpAddr, SocketAddr};

use crate::server::events;

const LISTENER: Token = Token(0);

pub fn run(server_ip: IpAddr, server_port: u16) -> std::io::Result<()> {
    let server_address = SocketAddr::new(server_ip, server_port);

    let mut listener = TcpListener::bind(server_address)?;
    // let mut poll = Poll::new()?;
    // 
    // poll.registry()
    //     .register(&mut listener, LISTENER, events::listener_interest())?;

    // let mut events = Events::with_capacity(128);
    // 
    // let mut clients: HashMap<Token, (mio::net::TcpStream, Vec<u8>)> = HashMap::new();
    // let mut next_client_token: usize = 1;

    println!("Server listening on {}", server_address);


    loop {
        if let Ok((mut stream, addr)) = listener.accept() {
            println!("Accepted connection from {}", addr);
            thread::spawn(move || {
                let mut buffer = [0;1024];
                loop {
                    match stream.read(&mut buffer) {
                        Ok(0) => {
                            println!("Client at {} disconnected", addr);
                            break;
                        }
                        Ok(n) => {
                            println!("Received from {}: {}", addr, String::from_utf8_lossy(&buffer[..n]));
                            if stream.write_all(&buffer[..n]).is_err() {
                                println!("Failed {} ", addr);
                                break;
                            }
                        }
                        Err(e) => {
                            println!("Error: {}", e);
                            break;
                        }
                    }
                }
            });
            
        }
        // poll.poll(&mut events, None)?;
        // 
        // events::handle_events(
        //     &mut poll,
        //     &mut listener,
        //     &mut clients,
        //     &mut next_client_token,
        //     &mut events,
        // )?;
    }
}
