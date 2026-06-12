use crate::interpreter::{evaluator::Evaluator, values::Value};

pub fn std_arr_fill(_: &mut Evaluator, value: Value, count: i64) -> Value {
    let items_type = Evaluator::infer_type(&value);
    Value::Values {
        items_type,
        items: vec![value; count as usize],
    }
}
