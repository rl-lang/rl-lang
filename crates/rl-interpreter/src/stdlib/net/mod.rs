//! `std::net` - TCP/UDP networking built directly on `std::net`.

use crate::native::Module;
use std::net::{TcpListener, TcpStream, UdpSocket};

mod common;
mod resolve;
mod tcp_accept;
mod tcp_close;
mod tcp_connect;
mod tcp_listen;
mod tcp_local_addr;
mod tcp_peer_addr;
mod tcp_read;
mod tcp_set_nonblocking;
mod tcp_set_timeout;
mod tcp_shutdown;
mod tcp_write;
mod udp_bind;
mod udp_close;
mod udp_connect;
mod udp_recv;
mod udp_recv_from;
mod udp_send;
mod udp_send_to;

pub use rl_commons::keywords::net::KEYWORDS;

/// A single native networking resource, stored behind an `int` handle.
pub enum NetHandle {
    TcpListener(TcpListener),
    TcpStream(TcpStream),
    UdpSocket(UdpSocket),
}

pub fn module() -> Module {
    Module::new("net")
        .with_function("tcp_listen", tcp_listen::func)
        .with_function("tcp_accept", tcp_accept::func)
        .with_function("tcp_connect", tcp_connect::func)
        .with_function("tcp_read", tcp_read::func)
        .with_function("tcp_write", tcp_write::func)
        .with_function("tcp_peer_addr", tcp_peer_addr::func)
        .with_function("tcp_local_addr", tcp_local_addr::func)
        .with_function("tcp_set_timeout", tcp_set_timeout::func)
        .with_function("tcp_set_nonblocking", tcp_set_nonblocking::func)
        .with_function("tcp_shutdown", tcp_shutdown::func)
        .with_function("tcp_close", tcp_close::func)
        .with_function("udp_bind", udp_bind::func)
        .with_function("udp_connect", udp_connect::func)
        .with_function("udp_send", udp_send::func)
        .with_function("udp_send_to", udp_send_to::func)
        .with_function("udp_recv", udp_recv::func)
        .with_function("udp_recv_from", udp_recv_from::func)
        .with_function("udp_close", udp_close::func)
        .with_function("resolve", resolve::func)
}
