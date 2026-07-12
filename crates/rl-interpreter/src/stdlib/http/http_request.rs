use crate::{
    interpreter::{
        evaluator::Evaluator,
        stdlib::{
            common::{check_arity_range, extract_string, verr, vs},
            http::common::ureq_result_to_value,
        },
        values::Value,
    },
    utils::{errors::Error, span::Span},
};

pub fn func(_: &mut Evaluator, args: Vec<Value>, span: Span) -> Result<Value, Error> {
    check_arity_range(&args, 2, 4, "http_request", span)?;

    let method = match extract_string(args[0].clone(), "http_request") {
        Ok(s) => s,
        Err(e) => return Ok(verr!(vs!(format!("http_request: {e}")))),
    };
    let url = match extract_string(args[1].clone(), "http_request") {
        Ok(s) => s,
        Err(e) => return Ok(verr!(vs!(format!("http_request: {e}")))),
    };
    let body = match args.get(2) {
        Some(Value::String(s)) => Some(s.clone()),
        Some(other) => {
            return Ok(verr!(vs!(format!(
                "http_request: expects a string body, got {}",
                other.type_name()
            ))));
        }
        None => None,
    };

    let mut request = ureq::request(&method, &url);
    if let Some(Value::Values { items, .. }) = args.get(3) {
        for item in items {
            match item {
                Value::Tuple(pair) if pair.len() == 2 => {
                    if let (Value::String(name), Value::String(value)) = (&pair[0], &pair[1]) {
                        request = request.set(name, value);
                        continue;
                    }
                    return Ok(verr!(vs!(
                        "http_request: expects headers as an array of (string, string) tuples"
                            .to_string()
                    )));
                }
                _ => {
                    return Ok(verr!(vs!(
                        "http_request: expects headers as an array of (string, string) tuples"
                            .to_string()
                    )));
                }
            }
        }
    } else if args.get(3).is_some() {
        return Ok(verr!(vs!(
            "http_request: expects headers as an array of (string, string) tuples".to_string()
        )));
    }

    let result = match &body {
        Some(b) => request.send_string(b),
        None => request.call(),
    };
    Ok(ureq_result_to_value(&url, result))
}
