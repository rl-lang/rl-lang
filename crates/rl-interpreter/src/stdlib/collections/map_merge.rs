use crate::{
    evaluator::Evaluator,
    stdlib::common::{verr, vok, vs},
    values::Value,
};

pub fn std_map_merge(_: &mut Evaluator, map1: Value, map2: Value) -> Value {
    match (map1, map2) {
        (
            Value::Map {
                key_type,
                value_type,
                entries,
            },
            Value::Map {
                key_type: key_type2,
                value_type: value_type2,
                entries: entries2,
            },
        ) => {
            if !Evaluator::types_compatible(&key_type2, &key_type)
                || !Evaluator::types_compatible(&value_type2, &value_type)
            {
                return verr!(vs!(format!(
                    "map_merge: type mismatch: expects map[{:?}, {:?}], found map[{:?}, {:?}]",
                    key_type, value_type, key_type2, value_type2
                )));
            }
            for (k, v) in entries2.borrow().iter() {
                entries.borrow_mut().insert(k.clone(), v.clone());
            }
            vok!(Value::Map {
                key_type,
                value_type,
                entries,
            })
        }
        (other, _) => verr!(vs!(format!(
            "map_merge: accepts only maps, found {}",
            other.type_name()
        ))),
    }
}
