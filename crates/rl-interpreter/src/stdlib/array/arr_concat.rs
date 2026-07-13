use crate::{
    evaluator::Evaluator,
    stdlib::common::{verr, vok, vs},
    values::Value,
};

pub fn std_arr_concat(_: &mut Evaluator, array1: Value, array2: Value) -> Value {
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
                return verr!(vs!(format!(
                    "arr_concat: type mismatch: array type {:?}, cannot concat {:?}",
                    it_1, it_2
                )));
            }
            let mut v = i1;
            v.extend(i2);
            vok!(Value::Values {
                items_type: it_1,
                items: v
            })
        }
        _ => verr!(vs!("arr_concat: accepts only arrays".to_string())),
    }
}
