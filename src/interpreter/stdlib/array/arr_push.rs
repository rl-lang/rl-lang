use crate::{
    ast::statements::TypeAnnotation,
    interpreter::{evaluator::Evaluator, values::Value},
    utils::errors::Error,
};

pub fn std_arr_push(_: &mut Evaluator, array: Value, value: Value) -> Result<Value, Error> {
    match array {
        Value::Values { items_type, items } => {
            let val_type = Evaluator::infer_type(&value);
            if val_type != items_type && val_type != TypeAnnotation::Null {
                return Err(Error::init(
                    format!(
                        "type mismatch: array expects {:?}, cannot push {:?}",
                        items_type, val_type
                    ),
                    None,
                    None,
                ));
            }
            let mut v = items;
            v.push(value);
            Ok(Value::Values {
                items_type,
                items: v,
            })
        }
        _ => Err(Error::init(
            "arr_push() accepts only arrays".to_string(),
            None,
            None,
        )),
    }
}
