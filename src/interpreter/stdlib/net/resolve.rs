use std::net::ToSocketAddrs;

use crate::interpreter::{
    evaluator::Evaluator,
    stdlib::common::{verr, vok, vs},
    values::Value,
};

pub fn func(_: &mut Evaluator, host_port: String) -> Value {
    match host_port.to_socket_addrs() {
        Ok(addrs) => {
            let items: Vec<Value> = addrs.map(|a| vs!(a.ip().to_string())).collect();
            vok!(Value::Values {
                items_type: crate::ast::statements::TypeAnnotation::String,
                items,
            })
        }
        Err(e) => verr!(vs!(format!("resolve(\"{}\"): {}", host_port, e))),
    }
}
