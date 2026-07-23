use crate::{
    Vm,
    stdlib::macros::{verr, vok, vs},
    values::VmValue,
};

pub fn std_map_merge(_: &mut Vm, map1: VmValue, map2: VmValue) -> VmValue {
    match (map1, map2) {
        (VmValue::Map(entries), VmValue::Map(entries2)) => {
            for (k, v) in entries2.borrow().iter() {
                entries.borrow_mut().insert(k.clone(), v.clone());
            }
            vok!(VmValue::Map(entries))
        }
        (other, _) => verr!(vs!(format!(
            "map_merge: accepts only maps, found {}",
            other.type_name()
        ))),
    }
}
