use libc::{
    bind, setsockopt, socket, sockaddr_in, SOL_SOCKET, SO_REUSEADDR, SO_KEEPALIVE, SO_LINGER,
    SO_RCVBUF, SO_SNDBUF, IPPROTO_TCP, TCP_NODELAY, linger, listen, accept, socklen_t, sockaddr
};
use std::io::{self};
use std::mem;
use std::os::raw::c_int;
use crate::SocketError;
use socket::set_nonblocking;
use crate::socket::socket;

pub fn create_socket() -> Result<c_int, SocketError> {
    let sockfd = unsafe { socket(libc::AF_INET, libc::SOCK_STREAM, 0) };

    if sockfd < 0 {
        Err(SocketError::Create(io::Error::last_os_error()))
    } else {
        // 소켓 옵션 설정
        if let Err(e) = set_distributed_socket_options(sockfd) {
            unsafe { libc::close(sockfd) }; // 실패 시 소켓 닫기
            return Err(SocketError::SetOption(e));
        }

        if let Err(e) = set_nonblocking(sockfd) {
            unsafe { libc::close(sockfd) }; // 실패 시 소켓 닫기
            return Err(SocketError::SetOption(e));
        }

        Ok(sockfd)
    }
}

pub fn bind_socket(sockfd: c_int, addr: &sockaddr_in) -> Result<(), SocketError> {
    let result = unsafe {
        bind(
            sockfd,
            addr as *const sockaddr_in as *const libc::sockaddr,
            mem::size_of::<sockaddr_in>() as u32,
        )
    };
    if result < 0 {
        Err(SocketError::Bind(io::Error::last_os_error()))
    } else {
        Ok(())
    }
}

pub fn listen_socket(sockfd: c_int, backlog: c_int) -> Result<(), SocketError> {
    let result = unsafe { listen(sockfd, backlog) };
    if result < 0 {
        Err(SocketError::Listen(io::Error::last_os_error()))
    } else {
        Ok(())
    }
}

pub fn accept_connection(sockfd: c_int) -> Result<(c_int, sockaddr_in), SocketError> {
    let mut client_addr: sockaddr_in = unsafe { mem::zeroed() };
    let mut addr_len: socklen_t = mem::size_of::<sockaddr_in>() as u32;

    // 클라이언트 연결 수락
    let client_fd = unsafe {
        accept(
            sockfd,
            &mut client_addr as *mut sockaddr_in as *mut sockaddr,
            &mut addr_len as *mut socklen_t,
        )
    };

    if client_fd < 0 {
        let err = io::Error::last_os_error();
        if err.kind() == io::ErrorKind::WouldBlock {
            return Err(SocketError::WouldBlock);
        } else {
            return Err(SocketError::Accept(err));
        }
    }
    Ok((client_fd, client_addr))
}

fn set_distributed_socket_options(sockfd: c_int) -> io::Result<()> {
    // SO_REUSEADDR
    let reuseaddr: c_int = 1;
    let result = unsafe {
        setsockopt(
            sockfd,
            SOL_SOCKET,
            SO_REUSEADDR,
            &reuseaddr as *const _ as *const libc::c_void,
            mem::size_of::<c_int>() as u32,
        )
    };
    if result < 0 {
        return Err(io::Error::last_os_error());
    }

    // SO_KEEPALIVE
    let keepalive: c_int = 1;
    let result = unsafe {
        setsockopt(
            sockfd,
            SOL_SOCKET,
            SO_KEEPALIVE,
            &keepalive as *const _ as *const libc::c_void,
            mem::size_of::<c_int>() as u32,
        )
    };
    if result < 0 {
        return Err(io::Error::last_os_error());
    }

    // TCP_NODELAY
    let nodelay: c_int = 1;
    let result = unsafe {
        setsockopt(
            sockfd,
            IPPROTO_TCP,
            TCP_NODELAY,
            &nodelay as *const _ as *const libc::c_void,
            mem::size_of::<c_int>() as u32,
        )
    };
    if result < 0 {
        return Err(io::Error::last_os_error());
    }

    // SO_RCVBUF
    let recv_buf: c_int = 128 * 1024; // 128KB
    let result = unsafe {
        setsockopt(
            sockfd,
            SOL_SOCKET,
            SO_RCVBUF,
            &recv_buf as *const _ as *const libc::c_void,
            mem::size_of::<c_int>() as u32,
        )
    };
    if result < 0 {
        return Err(io::Error::last_os_error());
    }

    // SO_SNDBUF
    let send_buf: c_int = 128 * 1024; // 128KB
    let result = unsafe {
        setsockopt(
            sockfd,
            SOL_SOCKET,
            SO_SNDBUF,
            &send_buf as *const _ as *const libc::c_void,
            mem::size_of::<c_int>() as u32,
        )
    };
    if result < 0 {
        return Err(io::Error::last_os_error());
    }

    // SO_LINGER
    let linger_option = linger {
        l_onoff: 1,             // Linger 활성화
        l_linger: 5,            // 5초 동안 대기
    };
    let result = unsafe {
        setsockopt(
            sockfd,
            SOL_SOCKET,
            SO_LINGER,
            &linger_option as *const _ as *const libc::c_void,
            mem::size_of::<linger>() as u32,
        )
    };
    if result < 0 {
        return Err(io::Error::last_os_error());
    }

    // SO_BROADCAST
    let broadcast: c_int = 1;
    let result = unsafe {
        setsockopt(
            sockfd,
            SOL_SOCKET,
            libc::SO_BROADCAST,
            &broadcast as *const _ as *const libc::c_void,
            mem::size_of::<c_int>() as u32,
        )
    };
    if result < 0 {
        return Err(io::Error::last_os_error());
    }

    Ok(())
}
