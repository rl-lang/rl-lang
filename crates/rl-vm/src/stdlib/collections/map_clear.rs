use crate::{
    Vm,
    stdlib::macros::{verr, vok, vs},
    values::VmValue,
};

pub fn std_map_clear(_: &mut Vm, map: VmValue) -> VmValue {
    match map {
        VmValue::Map(entries) => {
            entries.borrow_mut().clear();
            vok!(VmValue::Map(entries))
        }
        other => verr!(vs!(format!(
            "map_clear: accepts only maps, found {}",
            other.type_name()
        ))),
    }
}
