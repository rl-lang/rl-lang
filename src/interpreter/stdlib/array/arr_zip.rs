use crate::ast::statements::TypeAnnotation;
use crate::interpreter::{evaluator::Evaluator, values::Value};
use crate::utils::errors::{Error, Reason};
use crate::utils::span::Span;

/// Zips two arrays into an array of tuples.
///
/// `arr_zip([1, 2, 3], ["a", "b", "c"])` → `[(1, "a"), (2, "b"), (3, "c")]`
///
/// Stops at the shorter array (same behaviour as Rust's `zip`).
pub fn std_arr_zip(_: &mut Evaluator, args: Vec<Value>, span: Span) -> Result<Value, Error> {
    if args.len() != 2 {
        return Err(Error::at(
            Reason::Runtime,
            format!("arr_zip: expected 2 arguments, got {}", args.len()),
            span,
        ));
    }

    let (a, b) = match (&args[0], &args[1]) {
        (Value::Values { items: a, .. }, Value::Values { items: b, .. }) => (a, b),
        _ => {
            return Err(Error::at(
                Reason::Runtime,
                format!(
                    "arr_zip: expected two arrays, got {} and {}",
                    args[0].type_name(),
                    args[1].type_name()
                ),
                span,
            ));
        }
    };

    let items = a
        .iter()
        .zip(b.iter())
        .map(|(x, y)| Value::Tuple(vec![x.clone(), y.clone()]))
        .collect();

    Ok(Value::Values {
        items_type: TypeAnnotation::Tuple(vec![TypeAnnotation::Null, TypeAnnotation::Null]),
        items,
    })
}
