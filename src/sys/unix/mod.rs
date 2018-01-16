#[cfg(any(target_os = "linux", target_os = "android"))]
mod epoll;

#[cfg(any(target_os = "linux", target_os = "android"))]
pub use self::epoll::{Events, Selector};

#[cfg(any(target_os = "macos", target_os = "ios", target_os = "freebsd",
            target_os = "dragonfly", target_os = "netbsd"))]
mod kqueue;

#[cfg(any(target_os = "macos", target_os = "ios", target_os = "freebsd",
            target_os = "dragonfly", target_os = "netbsd"))]
pub use self::kqueue::{Events, Selector};

mod awakener;
mod eventedfd;
mod io;
mod net;
mod socket;
mod tcp;
mod udp;
mod uds;

pub use self::awakener::Awakener;
pub use self::eventedfd::EventedFd;
pub use self::io::Io;
pub use self::socket::Socket;
pub use self::tcp::{TcpListener, TcpStream};
pub use self::udp::UdpSocket;
pub use self::uds::UnixSocket;

pub fn pipe() -> ::io::Result<(Io, Io)> {
    use nix::fcntl::{O_CLOEXEC, O_NONBLOCK};
    use nix::unistd::pipe2;

    let (rd, wr) = try!(pipe2(O_NONBLOCK | O_CLOEXEC).map_err(from_nix_error));

    Ok((Io::from_raw_fd(rd), Io::from_raw_fd(wr)))
}

pub fn from_nix_error(err: ::nix::Error) -> ::io::Error {
    ::io::Error::from_raw_os_error(err.errno() as i32)
}

mod nix {
    pub use nix::{c_int, Error};
    pub use nix::errno::{EAGAIN, EINPROGRESS};
    pub use nix::fcntl::{fcntl, FcntlArg, O_NONBLOCK};
    pub use nix::sys::socket::{bind, connect, getpeername, getsockname, getsockopt, ip_mreq,
                               linger, listen, recvfrom, recvmsg, sendmsg, sendto, setsockopt,
                               shutdown, socket, sockopt, AddressFamily, CmsgSpace,
                               ControlMessage, InetAddr, Ipv4Addr, Ipv6Addr, Shutdown, SockAddr,
                               SockLevel, SockType, accept4, ipv6_mreq, MSG_DONTWAIT,
                               SOCK_CLOEXEC, SOCK_NONBLOCK};
    pub use nix::sys::time::TimeVal;
    pub use nix::sys::uio::IoVec;
    pub use nix::unistd::{dup, read, write};
}
