use std::net::UdpSocket;

use crate::interpreter::{
    evaluator::Evaluator,
    stdlib::{
        common::{extract_string, verr, vi, vok, vs},
        net::{NetHandle, common::insert_handle},
    },
    values::Value,
};

pub fn func(eval: &mut Evaluator, address: Value) -> Value {
    let addr = match extract_string(address, "udp_bind") {
        Ok(s) => s,
        Err(e) => return verr!(vs!(format!("udp_bind: {} ", e))),
    };
    match UdpSocket::bind(&addr) {
        Ok(socket) => {
            let id = insert_handle(eval, NetHandle::UdpSocket(socket));
            vok!(vi!(id))
        }
        Err(e) => verr!(vs!(format!("udp_bind(\"{}\"): {}", addr, e))),
    }
}
