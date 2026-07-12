use crate::{
    evaluator::Evaluator,
    stdlib::common::{verr, vok, vs},
    values::Value,
};
use rl_ast::statements::TypeAnnotation;
use std::net::ToSocketAddrs;

pub fn func(_: &mut Evaluator, host_port: String) -> Value {
    match host_port.to_socket_addrs() {
        Ok(addrs) => {
            let items: Vec<Value> = addrs.map(|a| vs!(a.ip().to_string())).collect();
            vok!(Value::Values {
                items_type: TypeAnnotation::String,
                items,
            })
        }
        Err(e) => verr!(vs!(format!("resolve(\"{}\"): {}", host_port, e))),
    }
}
