use crate::{
    Vm,
    stdlib::macros::{verr, vok, vs},
    values::VmValue,
};

pub fn std_map_is_empty(_: &mut Vm, map: VmValue) -> VmValue {
    match map {
        VmValue::Map(entries) => vok!(VmValue::Bool(entries.borrow().is_empty())),
        other => verr!(vs!(format!(
            "map_is_empty: accepts only maps, found {}",
            other.type_name()
        ))),
    }
}
