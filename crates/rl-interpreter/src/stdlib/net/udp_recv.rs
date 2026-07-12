use crate::{
    evaluator::Evaluator,
    stdlib::{
        common::{extract_number, verr, vok, vs},
        net::NetHandle,
    },
    values::Value,
};

pub fn func(eval: &mut Evaluator, id: Value, max_bytes: Value) -> Value {
    let id = match extract_number(id, "udp_recv") {
        Ok(a) if a as i64 >= 0 => a as i64,
        Ok(_) => return verr!(vs!("udp_recv: id handle cannot be negative".to_string())),
        Err(e) => return verr!(vs!(format!("udp_recv: {}", e))),
    };
    let max_bytes = match extract_number(max_bytes, "udp_recv") {
        Ok(a) => a as usize,
        Err(e) => return verr!(vs!(format!("udp_recv: {}", e))),
    };
    let socket = match eval.net_handles.get(&id) {
        Some(NetHandle::UdpSocket(socket)) => socket,
        Some(_) => {
            return verr!(vs!(format!("udp_recv: handle {} is not a UDP socket", id)));
        }
        None => return verr!(vs!(format!("udp_recv: unknown handle {}", id))),
    };
    let mut buf = vec![0u8; max_bytes];
    match socket.recv(&mut buf) {
        Ok(n) => {
            buf.truncate(n);
            vok!(vs!(String::from_utf8_lossy(&buf).into_owned()))
        }
        Err(e) => verr!(vs!(format!("udp_recv: {}", e))),
    }
}
