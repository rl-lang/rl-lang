use crate::{
    Vm,
    stdlib::macros::{verr, vok, vs},
    values::{VmMapKey, VmValue},
};

pub fn std_map_contains(_: &mut Vm, map: VmValue, key: VmValue) -> VmValue {
    match map {
        VmValue::Map(entries) => match VmMapKey::from_value(&key) {
            Some(map_key) => vok!(VmValue::Bool(entries.borrow().contains_key(&map_key))),
            None => vok!(VmValue::Bool(false)),
        },
        other => verr!(vs!(format!(
            "map_contains: accepts only maps, found {}",
            other.type_name()
        ))),
    }
}
