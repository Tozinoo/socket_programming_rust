use mio::net::TcpStream;
use mio::{Interest, Poll, Token};
use std::collections::HashMap;
use std::io::{Read, Write};

/// 클라이언트 연결 등록
pub fn register_client(
    poll: &mut Poll,
    clients: &mut HashMap<Token, (TcpStream, Vec<u8>)>,
    mut stream: TcpStream,
    next_client_token: &mut usize,
) -> std::io::Result<()> {
    let client_token = Token(*next_client_token);
    *next_client_token += 1;

    poll.registry()
        .register(&mut stream, client_token, Interest::READABLE | Interest::WRITABLE)?;

    clients.insert(client_token, (stream, Vec::new()));

    Ok(())
}

/// 클라이언트 읽기 처리
pub fn handle_read(
    token: Token,
    stream: &mut TcpStream,
    buffer: &mut Vec<u8>,
) -> std::io::Result<bool> {
    let mut buf = [0; 1024];
    match stream.read(&mut buf) {
        Ok(0) => {
            println!("Client {:?} disconnected", token);
            Ok(false) // 연결 종료
        }
        Ok(n) => {
            println!(
                "Received from {:?}: {}",
                token,
                String::from_utf8_lossy(&buf[..n])
            );
            buffer.extend_from_slice(b"Server Response: Hello!\n");
            Ok(true)
        }
        Err(e) => {
            eprintln!("Error reading from {:?}: {}", token, e);
            Ok(false) // 연결 종료
        }
    }
}

/// 클라이언트 쓰기 처리
pub fn handle_write(
    token: Token,
    stream: &mut TcpStream,
    buffer: &mut Vec<u8>,
) -> std::io::Result<bool> {
    if !buffer.is_empty() {
        match stream.write(buffer) {
            Ok(n) => {
                println!("Sent {} bytes to {:?}", n, token);
                buffer.drain(..n);
                Ok(true)
            }
            Err(e) => {
                eprintln!("Error writing to {:?}: {}", token, e);
                Ok(false) // 연결 종료
            }
        }
    } else {
        Ok(true)
    }
}
