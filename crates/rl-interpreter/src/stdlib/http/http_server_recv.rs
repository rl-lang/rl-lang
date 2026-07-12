use crate::{
    evaluator::Evaluator,
    stdlib::{
        common::{extract_number, verr, vi, vok, vs},
        http::{HttpHandle, common::insert_handle},
    },
    values::Value,
};
pub fn func(eval: &mut Evaluator, id: Value) -> Value {
    let id = match extract_number(id, "http_server_recv") {
        Ok(a) if a as i64 >= 0 => a as i64,
        Ok(_) => {
            return verr!(vs!(
                "http_server_recv: id handle cannot be negative".to_string()
            ));
        }
        Err(e) => return verr!(vs!(format!("{}", e))),
    };
    let server = match eval.http_handles.get(&id) {
        Some(HttpHandle::Server(server)) => server,
        Some(_) => {
            return verr!(vs!(format!(
                "http_server_recv(): handle {} is not a server",
                id
            )));
        }
        None => return verr!(vs!(format!("http_server_recv(): unknown handle {}", id))),
    };
    match server.recv() {
        Ok(request) => {
            let req_id = insert_handle(eval, HttpHandle::Request(request));
            vok!(vi!(req_id))
        }
        Err(e) => verr!(vs!(format!("http_server_recv(): {}", e))),
    }
}
