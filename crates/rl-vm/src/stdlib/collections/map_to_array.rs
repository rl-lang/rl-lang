use std::rc::Rc;

use crate::{
    Vm,
    stdlib::macros::{verr, vok, vs},
    values::VmValue,
};

pub fn std_map_to_array(_: &mut Vm, map: VmValue) -> VmValue {
    match map {
        VmValue::Map(entries) => {
            let items: Vec<VmValue> = entries
                .borrow()
                .iter()
                .map(|(k, v)| VmValue::Tuple(Rc::new(vec![k.clone().into_value(), v.clone()])))
                .collect();
            vok!(VmValue::Arr(Rc::new(items)))
        }
        other => verr!(vs!(format!(
            "map_to_array: accepts only maps, found {}",
            other.type_name()
        ))),
    }
}
