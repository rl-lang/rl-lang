use crate::{
    ast::statements::TypeAnnotation,
    interpreter::{
        evaluator::Evaluator,
        stdlib::{
            common::{verr, vi, vok, vs},
            time::format_time::unix_to_parts,
        },
        values::Value,
    },
};

pub fn time_parts(_: &mut Evaluator, timestamp: i64) -> Value {
    if timestamp < 0 {
        return verr!(vs!("timestamp is negative".to_string()));
    }

    let (year, month, day, hour, minute, second) = unix_to_parts(timestamp);
    vok!(Value::Values {
        items_type: TypeAnnotation::Int,
        items: vec![
            vi!(year as i64),
            vi!(month as i64),
            vi!(day as i64),
            vi!(hour as i64),
            vi!(minute as i64),
            vi!(second as i64),
        ],
    })
}
