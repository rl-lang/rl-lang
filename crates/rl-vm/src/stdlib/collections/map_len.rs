use crate::{
    Vm,
    stdlib::macros::{verr, vok, vs},
    values::VmValue,
};

pub fn std_map_len(_: &mut Vm, map: VmValue) -> VmValue {
    match map {
        VmValue::Map(entries) => vok!(VmValue::Int(entries.borrow().len() as i64)),
        other => verr!(vs!(format!(
            "map_len: accepts only maps, found {}",
            other.type_name()
        ))),
    }
}
