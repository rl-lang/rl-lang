use crate::interpreter::{
    evaluator::Evaluator,
    stdlib::{
        common::{extract_number, verr, vnl, vok, vs},
        net::NetHandle,
    },
    values::Value,
};

pub fn func(eval: &mut Evaluator, id: Value, flag: bool) -> Value {
    let id = match extract_number(id, "tcp_set_nonblocking") {
        Ok(a) if a as i64 >= 0 => a as i64,
        Ok(_) => {
            return verr!(vs!(
                "tcp_set_nonblocking: id handle cannot be negative".to_string()
            ));
        }
        Err(e) => return verr!(vs!(format!("tcp_set_nonblocking: {}", e))),
    };
    let stream = match eval.net_handles.get(&id) {
        Some(NetHandle::TcpStream(stream)) => stream,
        Some(_) => {
            return verr!(vs!(format!(
                "tcp_set_nonblocking: handle {} is not a TCP stream",
                id
            )));
        }
        None => {
            return verr!(vs!(format!("tcp_set_nonblocking: unknown handle {}", id)));
        }
    };
    match stream.set_nonblocking(flag) {
        Ok(()) => vok!(vnl!()),
        Err(e) => verr!(vs!(format!("tcp_set_nonblocking: {}", e))),
    }
}
