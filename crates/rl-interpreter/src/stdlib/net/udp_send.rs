use crate::interpreter::{
    evaluator::Evaluator,
    stdlib::{
        common::{extract_number, extract_string, verr, vi, vok, vs},
        net::NetHandle,
    },
    values::Value,
};

pub fn func(eval: &mut Evaluator, id: Value, data: Value) -> Value {
    let id = match extract_number(id, "udp_send") {
        Ok(a) if a as i64 >= 0 => a as i64,
        Ok(_) => return verr!(vs!("udp_send: id handle cannot be negative".to_string())),
        Err(e) => return verr!(vs!(format!("{}", e))),
    };
    let data = match extract_string(data, "udp_send") {
        Ok(s) => s,
        Err(e) => return verr!(vs!(format!("{}", e))),
    };
    let socket = match eval.net_handles.get(&id) {
        Some(NetHandle::UdpSocket(socket)) => socket,
        Some(_) => {
            return verr!(vs!(format!("udp_send: handle {} is not a UDP socket", id)));
        }
        None => return verr!(vs!(format!("udp_send: unknown handle {}", id))),
    };
    match socket.send(data.as_bytes()) {
        Ok(n) => vok!(vi!(n as i64)),
        Err(e) => verr!(vs!(format!("udp_send: {}", e))),
    }
}
