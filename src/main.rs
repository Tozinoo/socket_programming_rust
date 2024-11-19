use std::net::*;

const LOCAL_ADDR: &str = "127.0.0.1:8081";

fn main() {
    let bind = TcpListener::bind(LOCAL_ADDR).expect("bind error");
    let accept = TcpListener::accept(&bind).expect("accpet error");

    println!("## Server Running ##");

    for stream in bind.incoming() {
        if stream.is_ok() {
            println!("클라이언트 연결")
        }
    }
}
