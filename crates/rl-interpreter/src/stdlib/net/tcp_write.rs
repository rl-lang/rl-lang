use std::io::Write;

use crate::interpreter::{
    evaluator::Evaluator,
    stdlib::{
        common::{extract_number, extract_string, verr, vi, vok, vs},
        net::NetHandle,
    },
    values::Value,
};

pub fn func(eval: &mut Evaluator, id: Value, data: Value) -> Value {
    let id = match extract_number(id, "tcp_write") {
        Ok(a) if a as i64 >= 0 => a as i64,
        Ok(_) => return verr!(vs!("tcp_write: id handle cannot be negative".to_string())),
        Err(e) => return verr!(vs!(format!("tcp_write: {}", e))),
    };
    let data = match extract_string(data, "tcp_write") {
        Ok(s) => s,
        Err(e) => return verr!(vs!(format!("tcp_write: {} ", e))),
    };

    let stream = match eval.net_handles.get_mut(&id) {
        Some(NetHandle::TcpStream(stream)) => stream,
        Some(_) => {
            return verr!(vs!(format!("tcp_write: handle {} is not a TCP stream", id)));
        }
        None => return verr!(vs!(format!("tcp_write: unknown handle {}", id))),
    };

    match stream.write(data.as_bytes()) {
        Ok(n) => vok!(vi!(n as i64)),
        Err(e) => verr!(vs!(format!("tcp_write: {}", e))),
    }
}
