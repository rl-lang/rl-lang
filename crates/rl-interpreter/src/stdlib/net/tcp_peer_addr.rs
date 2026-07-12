use crate::{
    evaluator::Evaluator,
    stdlib::{
        common::{extract_number, verr, vok, vs},
        net::NetHandle,
    },
    values::Value,
};

pub fn func(eval: &mut Evaluator, id: Value) -> Value {
    let id = match extract_number(id, "tcp_peer_addr") {
        Ok(a) if a as i64 >= 0 => a as i64,
        Ok(_) => {
            return verr!(vs!(
                "tcp_peer_addr: id handle cannot be negative".to_string()
            ));
        }
        Err(e) => return verr!(vs!(format!("tcp_accept: {}", e))),
    };
    let stream = match eval.net_handles.get(&id) {
        Some(NetHandle::TcpStream(stream)) => stream,
        Some(_) => {
            return verr!(vs!(format!(
                "tcp_peer_addr: handle {} is not a TCP stream",
                id
            )));
        }
        None => return verr!(vs!(format!("tcp_peer_addr: unknown handle {}", id))),
    };
    match stream.peer_addr() {
        Ok(addr) => vok!(vs!(addr.to_string())),
        Err(e) => verr!(vs!(format!("tcp_peer_addr: {}", e))),
    }
}
