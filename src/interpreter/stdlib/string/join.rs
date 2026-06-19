use crate::{
    interpreter::{evaluator::Evaluator, values::Value},
    utils::{errors::Error, span::Span},
};

pub fn std_join(
    eval: &mut Evaluator,
    strings_array: Value,
    delim: String,
    span: Span,
) -> Result<Value, Error> {
    match strings_array {
        Value::Values { items: array, .. } => {
            let mut strings: Vec<String> = vec![];
            for v in array {
                match v {
                    Value::Integer(i) => strings.push(format!("{}", i)),
                    Value::Float(f) => strings.push(format!("{}", f)),
                    Value::Bool(b) => strings.push(format!("{}", b)),
                    Value::String(s) => strings.push(s),
                    Value::Char(c) => strings.push(c.to_string()),
                    Value::Null => strings.push("null".to_string()),
                    Value::Function { .. } => {
                        return Err(eval.err(
                            "functions/lambdas/enclosures are not supported via join()".to_string(),
                            span,
                        ));
                    }
                    _ => {}
                }
            }
            Ok(Value::String(strings.join(&delim)))
        }
        _ => Err(eval.err(
            "join() expects an array as first argument".to_string(),
            span,
        )),
    }
}
