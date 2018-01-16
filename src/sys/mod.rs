#[cfg(unix)]
pub use self::unix::{pipe, Awakener, EventedFd, Events, Io, Selector, TcpListener, TcpStream,
                     UdpSocket, UnixSocket};

#[cfg(unix)]
mod unix;

#[cfg(windows)]
pub use self::windows::{Awakener, Events, Selector, TcpListener, TcpStream, UdpSocket};

#[cfg(windows)]
mod windows;
