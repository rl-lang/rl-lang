use crate::interpreter::{
    evaluator::Evaluator,
    stdlib::{
        common::{extract_number, verr, vok, vs},
        http::HttpHandle,
    },
    values::Value,
};
pub fn func(eval: &mut Evaluator, id: Value) -> Value {
    let id = match extract_number(id, "http_request_method") {
        Ok(a) if a as i64 >= 0 => a as i64,
        Ok(_) => {
            return verr!(vs!(
                "http_request_method: id handle cannot be negative".to_string()
            ));
        }
        Err(e) => return verr!(vs!(format!("{}", e))),
    };

    match eval.http_handles.get(&id) {
        Some(HttpHandle::Request(request)) => vok!(vs!(request.method().to_string())),
        Some(_) => verr!(vs!(format!(
            "http_request_method: handle {} is not a request",
            id
        ))),
        None => verr!(vs!(format!("http_request_method: unknown handle {}", id))),
    }
}
