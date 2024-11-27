mod server;
mod client;
mod socket;

use libc::sockaddr_in;
use std::net::Ipv4Addr;
use socket::errors::SocketError;
use crate::socket::socket::Socket;

fn main() -> Result<(), SocketError> {
    let ip = Ipv4Addr::new(127, 0, 0, 1);
    let port = 3001;

    let socket = Socket::new(ip, port)?;

    loop {
        // client accept
        match socket.accept() {
            Ok((client_fd, client_addr)) => {
                // client ip랑 port 출력.
                let client_ip = std::net::Ipv4Addr::from(u32::from_be(client_addr.sin_addr.s_addr));
                let client_port = u16::from_be(client_addr.sin_port);
                println!("Accepted connection from {}:{}", client_ip, client_port);

                // 클라이언트 데이터 처리
                handle_client(client_fd, client_addr, ip, port);
            }
            Err(e) => {
                eprintln!("Failed to accept connection: {:?}", e);
            }
        }
    }
}

fn handle_client(client_fd: i32, client_addr: sockaddr_in, my_ip: Ipv4Addr, my_port: u16) {
    let mut buffer = [0; 1024];

    // ip port 받아서 사용
    let client_ip = std::net::Ipv4Addr::from(u32::from_be(client_addr.sin_addr.s_addr));
    let client_port = u16::from_be(client_addr.sin_port);

    loop {
        // 데이터 수신
        let bytes_read = unsafe {
            libc::read(client_fd, buffer.as_mut_ptr() as *mut _, buffer.len())
        };

        // 연결 종료 시 -1을 보내기 때문에 연결 종료를 알 수 있음
        if bytes_read <= 0 {
            println!("Client disconnected.");
            unsafe { libc::close(client_fd) };
            break;
        }

        let received_message = match std::str::from_utf8(&buffer[..bytes_read as usize]) {
            Ok(message) => message,
            Err(_) => {
                println!("Received invalid UTF-8 data from {}:{}", client_ip, client_port);
                continue;
            }
        };

        // 서버의 정보를 포함한 에코 메시지 생성
        let response_message = format!(
            "Server {}:{} - Received: {}",
            my_ip, my_port, received_message
        );

        // 에코 메시지를 클라이언트로 전송
        // 데이터 쓰기
        // client_fd로 buffer값 write
        let bytes_written = unsafe {
            libc::write(
                client_fd,
                response_message.as_bytes().as_ptr() as *const _,
                response_message.len(),
            )
        };

        if bytes_written <= 0 {
            println!("Failed to send data to {}:{}", client_ip, client_port);
            unsafe { libc::close(client_fd) };
            break;
        }

        println!(
            "Received from {}:{}: {}",
            client_ip, client_port, received_message
        );
    }
}