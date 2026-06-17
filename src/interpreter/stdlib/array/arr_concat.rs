use crate::{
    interpreter::{evaluator::Evaluator, values::Value},
    utils::errors::Error,
};

pub fn std_arr_concat(_: &mut Evaluator, array1: Value, array2: Value) -> Result<Value, Error> {
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
            if it_1 != it_2 {
                return Err(Error::init(
                    format!(
                        "type mismatch: array type {:?}, cannot concat {:?}",
                        it_1, it_2
                    ),
                    None,
                    None,
                ));
            }
            let mut v = i1;
            v.extend(i2);
            Ok(Value::Values {
                items_type: it_1,
                items: v,
            })
        }
        _ => Err(Error::init(
            "arr_concat() accepts only arrays".to_string(),
            None,
            None,
        )),
    }
}
