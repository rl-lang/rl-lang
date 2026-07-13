use std::net::TcpStream;

use crate::{
    evaluator::Evaluator,
    stdlib::{
        common::{extract_string, verr, vi, vok, vs},
        net::{NetHandle, common::insert_handle},
    },
    values::Value,
};

pub fn func(eval: &mut Evaluator, address: Value) -> Value {
    let addr = match extract_string(address, "tcp_connect") {
        Ok(s) => s,
        Err(e) => return verr!(vs!(format!("tcp_connect: {} ", e))),
    };
    match TcpStream::connect(&addr) {
        Ok(stream) => {
            let id = insert_handle(eval, NetHandle::TcpStream(stream));
            vok!(vi!(id))
        }
        Err(e) => verr!(vs!(format!("tcp_connect(\"{}\"): {}", addr, e))),
    }
}
