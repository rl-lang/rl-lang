use crate::{
    Vm,
    stdlib::macros::{verr, vok, vs},
    values::{VmMapKey, VmValue},
};

pub fn std_map_remove(_: &mut Vm, map: VmValue, key: VmValue) -> VmValue {
    match map {
        VmValue::Map(entries) => match VmMapKey::from_value(&key) {
            Some(map_key) => {
                entries.borrow_mut().remove(&map_key);
                vok!(VmValue::Map(entries))
            }
            None => verr!(vs!(format!(
                "map_remove: cannot remove {} from a map",
                key.type_name()
            ))),
        },
        other => verr!(vs!(format!(
            "map_remove: accepts only maps, found {}",
            other.type_name()
        ))),
    }
}
