use std::net::TcpListener;

use crate::{
    evaluator::Evaluator,
    stdlib::{
        common::{extract_string, verr, vi, vok, vs},
        net::{NetHandle, common::insert_handle},
    },
    values::Value,
};

pub fn func(eval: &mut Evaluator, address: Value) -> Value {
    let addr = match extract_string(address, "tcp_listen") {
        Ok(s) => s,
        Err(e) => return verr!(vs!(format!("tcp_listen: {} ", e))),
    };
    match TcpListener::bind(&addr) {
        Ok(listener) => {
            let id = insert_handle(eval, NetHandle::TcpListener(listener));
            vok!(vi!(id))
        }
        Err(e) => verr!(vs!(format!("tcp_listen(\"{}\"): {}", addr, e))),
    }
}
