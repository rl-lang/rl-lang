use std::rc::Rc;

use crate::{
    Vm,
    stdlib::macros::{verr, vok, vs},
    values::VmValue,
};

pub fn std_arr_sort(_: &mut Vm, array: VmValue) -> VmValue {
    match array {
        VmValue::Arr(items) => {
            if items
                .iter()
                .all(|v| matches!(v, VmValue::Int(_) | VmValue::Null))
            {
                let mut items = (*items).clone();
                items.sort_by(|a, b| match (a, b) {
                    (VmValue::Int(x), VmValue::Int(y)) => x.cmp(y),
                    _ => std::cmp::Ordering::Equal,
                });
                return vok!(VmValue::Arr(Rc::new(items)));
            }

            if items.iter().all(|v| matches!(v, VmValue::Float(_))) {
                let mut items = (*items).clone();
                items.sort_by(|a, b| match (a, b) {
                    (VmValue::Float(x), VmValue::Float(y)) => {
                        x.partial_cmp(y).unwrap_or(std::cmp::Ordering::Equal)
                    }
                    _ => std::cmp::Ordering::Equal,
                });
                return vok!(VmValue::Arr(Rc::new(items)));
            }

            if items.iter().all(|v| matches!(v, VmValue::Str(_))) {
                let mut items = (*items).clone();
                items.sort_by(|a, b| match (a, b) {
                    (VmValue::Str(x), VmValue::Str(y)) => x.cmp(y),
                    _ => std::cmp::Ordering::Equal,
                });
                return vok!(VmValue::Arr(Rc::new(items)));
            }

            verr!(vs!(
                "arr_sort: accepts only int, float, or string arrays".to_string()
            ))
        }
        other => verr!(vs!(format!(
            "arr_sort: accepts only arrays, found {}",
            other.type_name()
        ))),
    }
}
