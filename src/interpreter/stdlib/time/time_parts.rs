use crate::{
    ast::statements::TypeAnnotation,
    interpreter::{evaluator::Evaluator, values::Value},
    utils::{errors::Error, span::Span},
};

pub fn time_parts(eval: &mut Evaluator, timestamp: i64, span: Span) -> Result<Value, Error> {
    if timestamp < 0 {
        return Err(eval.err("timestamp is negative".to_string(), span));
    }

    let total_seconds = timestamp;
    let time_of_day = total_seconds % 86400;
    let hour = time_of_day / 3600;
    let minute = (time_of_day % 3600) / 60;
    let second = time_of_day % 60;

    let days_since_epoch = total_seconds / 86400;
    let days_since_march0 = days_since_epoch + 719468;

    let century = days_since_march0.div_euclid(146097);
    let day_in_century = days_since_march0.rem_euclid(146097);
    let year_in_century = (day_in_century - day_in_century / 1460 + day_in_century / 36524
        - day_in_century / 146096)
        / 365;
    let day_in_year =
        day_in_century - (365 * year_in_century + year_in_century / 4 - year_in_century / 100);
    let month_index = (5 * day_in_year + 2) / 153;
    let day = day_in_year - (153 * month_index + 2) / 5 + 1;
    let month = if month_index < 10 {
        month_index + 3
    } else {
        month_index - 9
    };
    let year = year_in_century + century * 400 + if month <= 2 { 1 } else { 0 };

    Ok(Value::Values {
        items_type: TypeAnnotation::Int,
        items: vec![
            Value::Integer(year),
            Value::Integer(month),
            Value::Integer(day),
            Value::Integer(hour),
            Value::Integer(minute),
            Value::Integer(second),
        ],
    })
}
