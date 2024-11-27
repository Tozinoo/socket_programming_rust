// use mio::net::TcpListener;
// use mio::{Events, Poll, Token};
// use std::collections::HashMap;
// use std::sync::Arc;
// use std::thread;
// use rayon::ThreadPoolBuilder;
// use crate::client;
//
// /// Poll에 등록할 기본 Interest 설정
// pub fn listener_interest() -> mio::Interest {
//     mio::Interest::READABLE
// }
//
// /// 이벤트 처리
// pub fn handle_events(
//     poll: &mut Poll,
//     listener: Arc<TcpListener>,
//     clients: &mut HashMap<Token, (mio::net::TcpStream, Vec<u8>)>,
//     next_client_token: &mut usize,
//     events: &mut Events,
// ) -> std::io::Result<()> {
//     let mut to_remove: Vec<Token> = Vec::new();
//     let threadpool = ThreadPoolBuilder::new().num_threads(4).build().unwrap();
//
//     for event in events.iter() {
//         match event.token() {
//             Token(0) => {
//                 // Listener 이벤트 처리 (새로운 클라이언트 연결)
//                 if event.is_readable() {
//                     if let Ok((mut stream, addr)) = listener.accept() {
//                         println!("Accepted connection from {}", addr);
//                         client::register_client(poll, clients, stream, next_client_token)?;
//                         println!("clients {:?}", clients);
//                     }
//                 }
//             }
//             token => {
//                 // 클라이언트 이벤트 처리
//                 if let Some((stream, buffer)) = clients.get_mut(&token) {
//                     threadpool.spawn(move || {
//                         let readable = client::handle_read(token, stream, buffer).expect("readable error");
//                         let writable = client::handle_write(token, stream, buffer).expect("readable error");
//
//                         if event.is_readable() {
//                             if readable {
//                                 to_remove.push(token);
//                             }
//                         }
//                         if event.is_writable() {
//                             if writable {
//                                 to_remove.push(token);
//                             }
//                         }
//                     });
//                 }
//
//
//                 // if let Some((mut stream, buffer)) = clients.remove(&token) {
//                 //     let buffer_clone = buffer.clone();
//                 //     let token_clone = token;
//                 //     let readable = event.is_readable();
//                 //     let writable = event.is_writable();
//                 //     println!("readable {}", readable);
//                 //     println!("writable {}", writable);
//                 //     println!("clients {:?}", clients);
//                 //
//                 //     threadpool.spawn(move || {
//                 //         if readable {
//                 //             let mut buf = buffer_clone.clone();
//                 //             if let Ok(success) = client::handle_read(token_clone, &mut stream, &mut buf) {
//                 //                 if !success {
//                 //                     println!("Disconnecting client {:?}", token_clone);
//                 //                 }
//                 //             }
//                 //         }
//                 //         if writable {
//                 //             let mut buf = buffer_clone.clone();
//                 //             if let Ok(success) = client::handle_write(token_clone, &mut stream, &mut buf) {
//                 //                 if !success {
//                 //                     println!("Disconnecting client {:?}", token_clone);
//                 //                 }
//                 //             }
//                 //         }
//                 //     });
//                 }
//         }
//     }
//
//     // 연결 종료된 클라이언트 제거
//     for token in to_remove {
//         clients.remove(&token);
//     }
//
//     Ok(())
// }
