use crate::entry::{FnEntry, StdEntry};

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

pub static NET: StdEntry = StdEntry {
    name: "net",
    description: "TCP and UDP networking built on std::net; blocking, single-threaded, handle-based",
    functions: FUNCTIONS,
    since: None,
    unstable: false,
};

static FUNCTIONS: &[&FnEntry] = &[
    &tcp_listen::TCP_LISTEN,
    &tcp_accept::TCP_ACCEPT,
    &tcp_connect::TCP_CONNECT,
    &tcp_read::TCP_READ,
    &tcp_write::TCP_WRITE,
    &tcp_peer_addr::TCP_PEER_ADDR,
    &tcp_local_addr::TCP_LOCAL_ADDR,
    &tcp_set_timeout::TCP_SET_TIMEOUT,
    &tcp_set_nonblocking::TCP_SET_NONBLOCKING,
    &tcp_shutdown::TCP_SHUTDOWN,
    &tcp_close::TCP_CLOSE,
    &udp_bind::UDP_BIND,
    &udp_connect::UDP_CONNECT,
    &udp_send::UDP_SEND,
    &udp_send_to::UDP_SEND_TO,
    &udp_recv::UDP_RECV,
    &udp_recv_from::UDP_RECV_FROM,
    &udp_close::UDP_CLOSE,
    &resolve::RESOLVE,
];
