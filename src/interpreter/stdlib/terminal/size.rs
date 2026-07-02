use crate::ast::statements::TypeAnnotation;
use crate::interpreter::stdlib::common::{verr, vi, vnl, vok, vs};
use crate::interpreter::stdlib::terminal::common::extract_u16;
use crate::interpreter::{evaluator::Evaluator, values::Value};
use crossterm::{
    execute,
    terminal::{SetSize, size},
};
use std::io::stdout;

pub fn std_term_get_size(_: &mut Evaluator) -> Value {
    let (cols, rows) = match size() {
        Ok((cols, rows)) => (cols, rows),
        Err(e) => return verr!(vs!(format!("term_get_size(): {}", e))),
    };

    vok!(Value::Values {
        items_type: TypeAnnotation::Int,
        items: vec![vi!(cols as i64), vi!(rows as i64)],
    })
}

pub fn std_term_set_size(_: &mut Evaluator, cols: Value, rows: Value) -> Value {
    let cols = match extract_u16(cols, "cols") {
        Ok(v) => v,
        Err(e) => return verr!(vs!(e)),
    };
    let rows = match extract_u16(rows, "rows") {
        Ok(v) => v,
        Err(e) => return verr!(vs!(e)),
    };

    execute!(stdout(), SetSize(cols, rows))
        .map_err(|e| verr!(vs!(format!("term_set_size(): {}", e))));
    vok!(vnl!())
}
