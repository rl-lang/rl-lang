use std::io::Read;

use crate::{
    evaluator::Evaluator,
    stdlib::{
        common::{extract_number, verr, vok, vs},
        net::NetHandle,
    },
    values::Value,
};

pub fn func(eval: &mut Evaluator, id: Value, max_bytes: Value) -> Value {
    let id = match extract_number(id, "tcp_read") {
        Ok(a) if a as i64 >= 0 => a as i64,
        Ok(_) => return verr!(vs!("tcp_read: id handle cannot be negative".to_string())),
        Err(e) => return verr!(vs!(format!("tcp_accept: {}", e))),
    };
    let max_bytes = match extract_number(max_bytes, "tcp_read") {
        Ok(a) => a as usize,
        Err(e) => return verr!(vs!(format!("tcp_read: {}", e))),
    };

    let stream = match eval.net_handles.get_mut(&id) {
        Some(NetHandle::TcpStream(stream)) => stream,
        Some(_) => {
            return verr!(vs!(format!("tcp_read: handle {} is not a TCP stream", id)));
        }
        None => return verr!(vs!(format!("tcp_read: unknown handle {}", id))),
    };

    let mut buf = vec![0u8; max_bytes];
    match stream.read(&mut buf) {
        Ok(n) => {
            buf.truncate(n);
            vok!(vs!(String::from_utf8_lossy(&buf).into_owned()))
        }
        Err(e) => verr!(vs!(format!("tcp_read: {}", e))),
    }
}
