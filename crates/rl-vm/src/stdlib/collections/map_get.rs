use crate::{
    Vm,
    stdlib::macros::{verr, vok, vs},
    values::{VmMapKey, VmValue},
};

pub fn std_map_get(_: &mut Vm, map: VmValue, key: VmValue) -> VmValue {
    match map {
        VmValue::Map(entries) => {
            let map_key = match VmMapKey::from_value(&key) {
                Some(k) => k,
                None => {
                    return verr!(vs!(format!(
                        "map_get: cannot use {} as a map key",
                        key.type_name()
                    )));
                }
            };
            match entries.borrow().get(&map_key) {
                Some(value) => vok!(value.clone()),
                None => verr!(vs!(format!(
                    "map_get: key {} not found in map",
                    map_key.into_value()
                ))),
            }
        }
        other => verr!(vs!(format!(
            "map_get: accepts only maps, found {}",
            other.type_name()
        ))),
    }
}
