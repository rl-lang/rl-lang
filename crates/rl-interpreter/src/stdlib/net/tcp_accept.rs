use crate::{
    evaluator::Evaluator,
    stdlib::{
        common::{extract_number, verr, vi, vok, vs},
        net::{NetHandle, common::insert_handle},
    },
    values::Value,
};

pub fn func(eval: &mut Evaluator, id: Value) -> Value {
    let id = match extract_number(id, "tcp_accept") {
        Ok(a) if a as i64 >= 0 => a as i64,
        Ok(_) => return verr!(vs!("tcp_accept: id handle cannot be negative".to_string())),
        Err(e) => return verr!(vs!(format!("tcp_accept: {}", e))),
    };

    let accept_result = match eval.net_handles.get(&id) {
        Some(NetHandle::TcpListener(listener)) => listener.accept(),
        Some(_) => {
            return verr!(vs!(format!(
                "tcp_accept: handle {} is not a TCP listener",
                id
            )));
        }
        None => return verr!(vs!(format!("tcp_accept: unknown handle {}", id))),
    };

    match accept_result {
        Ok((stream, _addr)) => {
            let new_id = insert_handle(eval, NetHandle::TcpStream(stream));
            vok!(vi!(new_id))
        }
        Err(e) => verr!(vs!(format!("tcp_accept(): {}", e))),
    }
}
