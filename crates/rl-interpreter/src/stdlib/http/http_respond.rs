use crate::{
    evaluator::Evaluator,
    stdlib::{
        common::{check_arity_range, extract_number, extract_string, verr, vok, vs},
        http::HttpHandle,
    },
    values::Value,
};
use rl_utils::{errors::Error, span::Span};

pub fn func(eval: &mut Evaluator, args: Vec<Value>, span: Span) -> Result<Value, Error> {
    check_arity_range(&args, 3, 4, "http_post", span)?;

    let id = match extract_number(args[0].clone(), "http_respond") {
        Ok(a) if a as i64 >= 0 => a as i64,
        Ok(_) => {
            return Ok(verr!(vs!(
                "http_respond: id handle cannot be negative".to_string()
            )));
        }
        Err(e) => return Ok(verr!(vs!(format!("{}", e)))),
    };

    let status = match &args[1] {
        Value::Integer(n) if (100..=599).contains(n) => *n as u16,
        other => {
            return Ok(verr!(vs!(format!(
                "http_respond: expects a valid HTTP status int, got {}",
                other.type_name()
            ))));
        }
    };

    let body = match extract_string(args[2].clone(), "http_respond") {
        Ok(s) => s,
        Err(e) => return Ok(verr!(vs!(format!("{e}")))),
    };
    let content_type = match args.get(3) {
        Some(Value::String(s)) => Some(s.clone()),
        Some(other) => {
            return Ok(verr!(vs!(format!(
                "http_respond: expects a string content_type, got {}",
                other.type_name()
            ))));
        }
        None => None,
    };

    let request = match eval.http_handles.remove(&id) {
        Some(HttpHandle::Request(request)) => request,
        Some(other) => {
            eval.http_handles.insert(id, other);
            return Ok(verr!(vs!(format!(
                "http_respond: handle {} is not a request",
                id
            ))));
        }
        None => return Ok(verr!(vs!(format!("http_respond: unknown handle {}", id)))),
    };

    let mut response = tiny_http::Response::from_string(body).with_status_code(status);
    if let Some(ct) = content_type
        && let Ok(header) = tiny_http::Header::from_bytes(&b"Content-Type"[..], ct.as_bytes())
    {
        response = response.with_header(header);
    }

    match request.respond(response) {
        Ok(()) => Ok(vok!(Value::Null)),
        Err(e) => Ok(verr!(vs!(format!("http_respond: {}", e)))),
    }
}
