mod server;
mod client;
mod socket;

use libc::sockaddr_in;
use std::net::Ipv4Addr;
use std::os::fd::AsRawFd;
use mio::{Events, Poll, Token, Interest};
use mio::unix::SourceFd;
use socket::errors::SocketError;
use crate::socket::socket::Socket;

const LISTENER: Token = Token(0);

fn main() -> Result<(), SocketError> {
    let ip = Ipv4Addr::new(127, 0, 0, 1);
    let port = 3001;

    let socket = Socket::new(ip, port)?;

    let mut poll = Poll::new()?;
    let mut events = Events::with_capacity(128);
    poll.registry().register(
        &mut SourceFd(&socket.as_raw_fd()),
        LISTENER,
        Interest::READABLE,
    )?;

    let mut clients = Vec::new();

    loop {
        match socket.accept() {
            Ok((client_fd, client_addr)) => {
                let client_ip = Ipv4Addr::from(u32::from_be(client_addr.sin_addr.s_addr));
                let client_port = u16::from_be(client_addr.sin_port);
                println!("Accepted connection from {}:{}", client_ip, client_port);

                // 클라이언트 데이터 처리 (별도 스레드로 실행)
                std::thread::spawn(move || {
                    handle_client(client_fd, client_addr, ip, port);
                });
            }
            Err(SocketError::WouldBlock) => {
                // `WouldBlock` 오류는 정상적인 논블로킹 상태이므로 무시
                std::thread::sleep(std::time::Duration::from_millis(100));
            }
            Err(e) => {
                eprintln!("Unexpected error while accepting connection: {:?}", e);
            }
        }
    }
}

fn handle_client(client_fd: i32, client_addr: sockaddr_in, my_ip: Ipv4Addr, my_port: u16) {
    let mut buffer = [0; 1024];
    let client_ip = Ipv4Addr::from(u32::from_be(client_addr.sin_addr.s_addr));
    let client_port = u16::from_be(client_addr.sin_port);

    loop {
        let bytes_read = unsafe {
            libc::read(client_fd, buffer.as_mut_ptr() as *mut _, buffer.len())
        };

        if bytes_read < 0 {
            let err = std::io::Error::last_os_error();
            if err.kind() == std::io::ErrorKind::WouldBlock {
                // 데이터가 없으면 대기
                std::thread::sleep(std::time::Duration::from_millis(100));
                continue;
            } else {
                println!("Read error: {:?}", err);
                unsafe { libc::close(client_fd) };
                break;
            }
        }

        if bytes_read == 0 {
            println!("Client disconnected.");
            unsafe { libc::close(client_fd) };
            break;
        }

        let received_message = match std::str::from_utf8(&buffer[..bytes_read as usize]) {
            Ok(message) => message,
            Err(_) => {
                println!("Invalid UTF-8 from {}:{}", client_ip, client_port);
                continue;
            }
        };

        let response_message = format!(
            "Server {}:{} - Received: {}",
            my_ip, my_port, received_message
        );

        let bytes_written = unsafe {
            libc::write(
                client_fd,
                response_message.as_bytes().as_ptr() as *const _,
                response_message.len(),
            )
        };

        if bytes_written < 0 {
            println!("Write error to {}:{}", client_ip, client_port);
            unsafe { libc::close(client_fd) };
            break;
        }

        println!("Received from {}:{}: {}", client_ip, client_port, received_message);
    }
}