use std::net::Ipv4Addr;
use std::os::raw::c_int;
use std::io;
use libc::{fcntl, F_GETFL, F_SETFL, O_NONBLOCK, sockaddr_in, AF_INET, sa_family_t};
use crate::socket::utils::{create_socket, bind_socket, listen_socket, accept_connection};
use crate::socket::errors::SocketError;

/// 기본 소켓 구조체
pub struct Socket {
    sockfd: c_int,
    addr: sockaddr_in,
}

impl Socket {
    pub fn new(ip: Ipv4Addr, port: u16) -> Result<Self, SocketError> {
        // 소켓 생성
        let sockfd = create_socket()?;

        #[cfg(target_os = "macos")]
        {
            let addr = sockaddr_in {
                sin_len:  std::mem::size_of::<sockaddr_in>() as u8,
                sin_family: AF_INET as sa_family_t,
                sin_port: port.to_be(),
                sin_addr: libc::in_addr {
                    s_addr: u32::from(ip).to_be(),
                },
                sin_zero: [0; 8],
            };
            //소켓 바인딩
            bind_socket(sockfd, &addr)?;
            // 소켓 리스닝
            listen_socket(sockfd, 128)?;

            println!("Socket listening on {}:{}", ip, port);

            Ok(Self { sockfd, addr })
        }


        #[cfg(target_os = "linux")]
        {
            let addr = sockaddr_in {
                // mac에서는 sin_len이 필요했는데, wls에서는 왜..?
                // linux 계열에서는 sin_len이 필요 없다고함.'
                // mac에서는 BSD 계열이라 필요함
                // sin_len:  std::mem::size_of::<sockaddr_in>() as u8,
                sin_family: AF_INET as sa_family_t,
                sin_port: port.to_be(),
                sin_addr: libc::in_addr {
                    s_addr: u32::from(ip).to_be(),
                },
                sin_zero: [0; 8],
            };
            //소켓 바인딩
            bind_socket(sockfd, &addr)?;
            // 소켓 리스닝
            listen_socket(sockfd, 128)?;

            set_nonblocking(sockfd)?;

            println!("Socket listening on {}:{}", ip, port);

            Ok(Self { sockfd, addr })
        }



    }

    // 소켓 accept
    pub fn accept(&self) -> Result<(c_int, sockaddr_in), SocketError> {
        match accept_connection(self.sockfd) {
            Ok(connection) => Ok(connection),
            Err(SocketError::WouldBlock) => {
                // 논블로킹 소켓에서 연결이 없으면 바로 반환
                Err(SocketError::WouldBlock)
            }
            Err(e) => Err(e),
        }
    }

    /// 소켓 파일 디스크립터 가져오기
    pub fn fd(&self) -> c_int {
        self.sockfd
    }

    /// 소켓 종료
    pub fn close(&self) -> io::Result<()> {
        unsafe {
            if libc::close(self.sockfd) == 0 {
                Ok(())
            } else {
                Err(io::Error::last_os_error())
            }
        }
    }
}

pub fn set_nonblocking(sockfd: c_int) -> Result<(), io::Error> {
    unsafe {
        // 기존 플래그 가져오기
        let flags = fcntl(sockfd, F_GETFL);
        if flags < 0 {
            return Err(io::Error::last_os_error());
        }

        // NONBLOCK 플래그 추가
        if fcntl(sockfd, F_SETFL, flags | O_NONBLOCK) < 0 {
            return Err(io::Error::last_os_error());
        }
    }
    Ok(())
}

#[cfg(any(unix, target_os = "wasi"))]
use std::os::unix::io::{AsRawFd, RawFd};

#[cfg(any(unix, target_os = "wasi"))]
impl AsRawFd for Socket {
    fn as_raw_fd(&self) -> RawFd {
        self.sockfd
    }
}