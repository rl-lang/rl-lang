use crate::interpreter::{
    evaluator::Evaluator,
    stdlib::{
        common::{extract_number, verr, vok, vs},
        http::HttpHandle,
    },
    values::Value,
};

pub fn func(eval: &mut Evaluator, id: Value) -> Value {
    let id = match extract_number(id, "http_request_body") {
        Ok(a) if a as i64 >= 0 => a as i64,
        Ok(_) => {
            return verr!(vs!(
                "http_request_body: id handle cannot be negative".to_string()
            ));
        }
        Err(e) => return verr!(vs!(format!("{}", e))),
    };
    let request = match eval.http_handles.get_mut(&id) {
        Some(HttpHandle::Request(request)) => request,
        Some(_) => {
            return verr!(vs!(format!(
                "http_request_body(): handle {} is not a request",
                id
            )));
        }
        None => return verr!(vs!(format!("http_request_body(): unknown handle {}", id))),
    };

    let mut body = String::new();
    match request.as_reader().read_to_string(&mut body) {
        Ok(_) => vok!(vs!(body)),
        Err(e) => verr!(vs!(format!("http_request_body(): {}", e))),
    }
}
