use crate::interpreter::{
    evaluator::Evaluator,
    stdlib::{
        common::{extract_number, verr, vnl, vok, vs},
        net::NetHandle,
    },
    values::Value,
};

pub fn func(eval: &mut Evaluator, id: Value) -> Value {
    let id = match extract_number(id, "udp_close") {
        Ok(a) if a as i64 >= 0 => a as i64,
        Ok(_) => return verr!(vs!("udp_close: id handle cannot be negative".to_string())),
        Err(e) => return verr!(vs!(format!("udp_close: {}", e))),
    };
    match eval.net_handles.get(&id) {
        Some(NetHandle::UdpSocket(_)) => {
            eval.net_handles.remove(&id);
            vok!(vnl!())
        }
        Some(_) => verr!(vs!(format!("udp_close: handle {} is not a UDP socket", id))),
        None => verr!(vs!(format!("udp_close: unknown handle {}", id))),
    }
}
