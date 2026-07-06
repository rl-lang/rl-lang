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
    check_arity_range(&args, 2, 3, "http_post", span)?;

    let url = match extract_string(args[0].clone(), "http_post") {
        Ok(s) => s,
        Err(e) => return Ok(verr!(vs!(format!("http_post: {e}")))),
    };
    let body = match extract_string(args[1].clone(), "http_post") {
        Ok(s) => s,
        Err(e) => return Ok(verr!(vs!(format!("http_post: {e}")))),
    };

    let content_type = match args.get(2) {
        Some(Value::String(s)) => s.clone(),
        Some(other) => {
            return Ok(verr!(vs!(format!(
                "http_post: expects a string content_type, got {}",
                other.type_name()
            ))));
        }
        None => "text/plain".to_string(),
    };

    let result = ureq::post(&url)
        .set("Content-Type", &content_type)
        .send_string(&body);
    Ok(ureq_result_to_value(&url, result))
}
