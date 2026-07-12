use std::net::Shutdown;

use crate::{
    evaluator::Evaluator,
    stdlib::{
        common::{extract_number, extract_string, verr, vnl, vok, vs},
        net::NetHandle,
    },
    values::Value,
};

pub fn func(eval: &mut Evaluator, id: Value, mode: Value) -> Value {
    let id = match extract_number(id, "tcp_shutdown") {
        Ok(a) if a as i64 >= 0 => a as i64,
        Ok(_) => return verr!(vs!("tcp_shutdown: id handle cannot be negative".to_string())),
        Err(e) => return verr!(vs!(format!("tcp_shutdown: {}", e))),
    };
    let mode = match extract_string(mode, "tcp_shutdown") {
        Ok(s) => s,
        Err(e) => return verr!(vs!(format!("tcp_shutdown: {} ", e))),
    };

    let mode = match mode.as_str() {
        "read" => Shutdown::Read,
        "write" => Shutdown::Write,
        "both" => Shutdown::Both,
        other => {
            return verr!(vs!(format!(
                "tcp_shutdown: expected \"read\", \"write\", or \"both\", got \"{}\"",
                other
            )));
        }
    };
    let stream = match eval.net_handles.get(&id) {
        Some(NetHandle::TcpStream(stream)) => stream,
        Some(_) => {
            return verr!(vs!(format!(
                "tcp_shutdown: handle {} is not a TCP stream",
                id
            )));
        }
        None => return verr!(vs!(format!("tcp_shutdown: unknown handle {}", id))),
    };
    match stream.shutdown(mode) {
        Ok(()) => vok!(vnl!()),
        Err(e) => verr!(vs!(format!("tcp_shutdown(): {}", e))),
    }
}
