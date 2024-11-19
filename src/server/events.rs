use mio::net::TcpListener;
use mio::{Events, Poll, Token};
use std::collections::HashMap;

use crate::client;

/// Poll에 등록할 기본 Interest 설정
pub fn listener_interest() -> mio::Interest {
    mio::Interest::READABLE
}

/// 이벤트 처리
pub fn handle_events(
    poll: &mut Poll,
    listener: &mut TcpListener,
    clients: &mut HashMap<Token, (mio::net::TcpStream, Vec<u8>)>,
    next_client_token: &mut usize,
    events: &mut Events,
) -> std::io::Result<()> {
    let mut to_remove = Vec::new();

    for event in events.iter() {
        match event.token() {
            Token(0) => {
                // Listener 이벤트 처리 (새로운 클라이언트 연결)
                if event.is_readable() {
                    if let Ok((stream, addr)) = listener.accept() {
                        println!("Accepted connection from {}", addr);
                        client::register_client(poll, clients, stream, next_client_token)?;
                    }
                }
            }
            token => {
                // 클라이언트 이벤트 처리
                if let Some((stream, buffer)) = clients.get_mut(&token) {
                    if event.is_readable() {
                        if !client::handle_read(token, stream, buffer)? {
                            to_remove.push(token);
                        }
                    }
                    if event.is_writable() {
                        if !client::handle_write(token, stream, buffer)? {
                            to_remove.push(token);
                        }
                    }
                }
            }
        }
    }

    // 연결 종료된 클라이언트 제거
    for token in to_remove {
        clients.remove(&token);
    }

    Ok(())
}
