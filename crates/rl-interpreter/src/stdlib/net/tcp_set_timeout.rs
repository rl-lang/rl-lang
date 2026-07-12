use std::time::Duration;

use crate::{
    evaluator::Evaluator,
    stdlib::{
        common::{extract_number, verr, vnl, vok, vs},
        net::NetHandle,
    },
    values::Value,
};

pub fn func(eval: &mut Evaluator, id: Value, millis: Value) -> Value {
    let id = match extract_number(id, "tcp_set_timeout") {
        Ok(a) if a as i64 >= 0 => a as i64,
        Ok(_) => return verr!(vs!("tcp_accept: id handle cannot be negative".to_string())),
        Err(e) => return verr!(vs!(format!("tcp_set_timeout: {}", e))),
    };
    let millis = match extract_number(millis, "tcp_set_timeout") {
        Ok(a) => a,
        Err(e) => return verr!(vs!(format!("tcp_set_timeout: {}", e))),
    };

    let stream = match eval.net_handles.get(&id) {
        Some(NetHandle::TcpStream(stream)) => stream,
        Some(_) => {
            return verr!(vs!(format!(
                "tcp_set_timeout: handle {} is not a TCP stream",
                id
            )));
        }
        None => return verr!(vs!(format!("tcp_set_timeout: unknown handle {}", id))),
    };
    let duration = if millis == 0 {
        None
    } else {
        Some(Duration::from_millis(millis))
    };
    let result = stream
        .set_read_timeout(duration)
        .and_then(|_| stream.set_write_timeout(duration));
    match result {
        Ok(()) => vok!(vnl!()),
        Err(e) => verr!(vs!(format!("tcp_set_timeout: {}", e))),
    }
}
