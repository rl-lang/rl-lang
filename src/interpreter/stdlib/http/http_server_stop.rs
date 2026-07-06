use crate::interpreter::{
    evaluator::Evaluator,
    stdlib::{
        common::{extract_number, verr, vnl, vok, vs},
        http::HttpHandle,
    },
    values::Value,
};
pub fn func(eval: &mut Evaluator, id: Value) -> Value {
    let id = match extract_number(id, "http_server_stop") {
        Ok(a) if a as i64 >= 0 => a as i64,
        Ok(_) => {
            return verr!(vs!(
                "http_server_stop: id handle cannot be negative".to_string()
            ));
        }
        Err(e) => return verr!(vs!(format!("{}", e))),
    };
    match eval.http_handles.get(&id) {
        Some(HttpHandle::Server(_)) => {
            eval.http_handles.remove(&id);
            vok!(vnl!())
        }
        Some(_) => verr!(vs!(format!(
            "http_server_stop: handle {} is not a server",
            id
        ))),
        None => verr!(vs!(format!("http_server_stop: unknown handle {}", id))),
    }
}
