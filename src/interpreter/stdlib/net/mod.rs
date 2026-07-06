//! `std::net` - TCP/UDP networking built directly on `std::net`.

use crate::interpreter::native::Module;
use std::net::{TcpListener, TcpStream, UdpSocket};

mod common;
mod resolve;
mod tcp_accept;
mod tcp_close;
mod tcp_connect;
mod tcp_listen;
pub const KEYWORDS: &[&str] = &[
    "tcp_listen",
    "tcp_accept",
    "tcp_connect",
    "tcp_read",
    "tcp_write",
    "tcp_peer_addr",
    "tcp_local_addr",
    "tcp_set_timeout",
    "tcp_set_nonblocking",
    "tcp_shutdown",
    "tcp_close",
    "udp_bind",
    "udp_connect",
    "udp_send",
    "udp_send_to",
    "udp_recv",
    "udp_recv_from",
    "udp_close",
    "resolve",
];

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
        .with_function("tcp_local_addr", tcp_local_addr::func)
        .with_function("tcp_close", tcp_close::func)
        .with_function("resolve", resolve::func)
}
