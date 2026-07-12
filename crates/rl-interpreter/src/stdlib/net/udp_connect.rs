use crate::interpreter::{
    evaluator::Evaluator,
    stdlib::{
        common::{extract_number, extract_string, verr, vnl, vok, vs},
        net::NetHandle,
    },
    values::Value,
};

pub fn func(eval: &mut Evaluator, id: Value, address: Value) -> Value {
    let id = match extract_number(id, "udp_connect") {
        Ok(a) if a as i64 >= 0 => a as i64,
        Ok(_) => return verr!(vs!("udp_connect: id handle cannot be negative".to_string())),
        Err(e) => return verr!(vs!(format!("udp_connect: {}", e))),
    };
    let addr = match extract_string(address, "udp_connect") {
        Ok(s) => s,
        Err(e) => return verr!(vs!(format!("udp_connect: {} ", e))),
    };

    let socket = match eval.net_handles.get(&id) {
        Some(NetHandle::UdpSocket(socket)) => socket,
        Some(_) => {
            return verr!(vs!(format!(
                "udp_connect: handle {} is not a UDP socket",
                id
            )));
        }
        None => return verr!(vs!(format!("udp_connect: unknown handle {}", id))),
    };
    match socket.connect(&addr) {
        Ok(()) => vok!(vnl!()),
        Err(e) => verr!(vs!(format!("udp_connect(\"{}\"): {}", addr, e))),
    }
}
