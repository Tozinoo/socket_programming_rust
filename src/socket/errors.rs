use std::io;

#[derive(Debug)]
pub enum SocketError {
    Create(io::Error),
    SetOption(io::Error),
    Bind(io::Error),
    Listen(io::Error),
    Accept(io::Error),
    WouldBlock
}

impl From<io::Error> for SocketError {
    fn from(error: io::Error) -> Self {
        match error.kind() {
            io::ErrorKind::WouldBlock => SocketError::WouldBlock,
            _ => SocketError::Create(error),
        }
    }
}