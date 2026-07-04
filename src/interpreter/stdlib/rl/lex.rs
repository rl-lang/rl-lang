use std::rc::Rc;

use crate::ast::statements::TypeAnnotation;
use crate::interpreter::stdlib::common::vi;
use crate::interpreter::{
    evaluator::Evaluator,
    stdlib::common::{extract_string, verr, vok, vs},
    values::Value,
};
use crate::lexer::tokenizer::Tokenizer;
use crate::utils::source::SourceFile;

pub fn func(_: &mut Evaluator, value: Value) -> Value {
    let code = match extract_string(value, "lex") {
        Ok(s) => s,
        Err(e) => return verr!(vs!(e)),
    };

    let source = SourceFile::new("<lex>", code);

    let tokens = match Tokenizer::lex(source) {
        Ok(t) => t,
        Err(e) => return verr!(vs!(e.message().to_string())),
    };

    let items: Vec<Value> = tokens
        .into_iter()
        .map(|t| {
            let kind = format!("{:?}", t.token);
            let kind = kind.split('(').next().unwrap_or(&kind).to_string();
            Value::Tuple(vec![vs!(kind), vs!(t.lexeme), vi!(t.line as i64)])
        })
        .collect();

    let items_type = TypeAnnotation::Tuple(Rc::new(vec![
        TypeAnnotation::String,
        TypeAnnotation::String,
        TypeAnnotation::Int,
    ]));

    vok!(Value::Values { items_type, items })
}
