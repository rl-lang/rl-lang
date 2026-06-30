use crate::{
    interpreter::{evaluator::Evaluator, values::Value},
    utils::{errors::Error, span::Span},
};

pub fn std_arr_concat(
    eval: &mut Evaluator,
    array1: Value,
    array2: Value,
    span: Span,
) -> Result<Value, Error> {
    match (array1, array2) {
        (
            Value::Values {
                items_type: it_1,
                items: i1,
            },
            Value::Values {
                items_type: it_2,
                items: i2,
            },
        ) => {
            if it_1 != it_2 
            {
                return Err(eval.err(
                    format!(
                        "type mismatch: array type {:?}, cannot concat {:?}",
                        it_1, it_2
                    ),
                    span,
                ));
            }
            let mut v = i1;
            v.extend(i2);
            Ok(Value::Values {
                items_type: target_type,
                items: v,
            })
        }
        _ => Err(eval.err("arr_concat() accepts only arrays".to_string(), span)),
    }
}
