//! Unix timestamp -> formatted string conversion.
//!
//! `unix_to_parts` decomposes a Unix timestamp into `(year, month, day, hour, minute, second)`
//! using the Gregorian calendar algorithm (proleptic calendar, UTC only, no DST).
//!
//! `apply_pattern` performs simple string substitution on strftime-like tokens:
//! `%Y` (4-digit year), `%m` (month), `%d` (day), `%H` (hour), `%M` (minute), `%S` (second).

use crate::{
    evaluator::Evaluator,
    stdlib::common::{verr, vok, vs},
    values::Value,
};

pub fn unix_to_parts(timestamp: i64) -> (i32, u32, u32, u32, u32, u32) {
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

    (
        year as i32,
        month as u32,
        day as u32,
        hour as u32,
        minute as u32,
        second as u32,
    )
}

fn apply_pattern(
    pattern: &str,
    year: i32,
    month: u32,
    day: u32,
    hour: u32,
    minute: u32,
    second: u32,
) -> String {
    pattern
        .replace("%Y", &format!("{:04}", year))
        .replace("%m", &format!("{:02}", month))
        .replace("%d", &format!("{:02}", day))
        .replace("%H", &format!("{:02}", hour))
        .replace("%M", &format!("{:02}", minute))
        .replace("%S", &format!("{:02}", second))
}

pub fn format_time(_: &mut Evaluator, timestamp: i64, pattern: String) -> Value {
    if timestamp < 0 {
        return verr!(vs!("timestamp is negative".to_string()));
    }
    let (year, month, day, hour, minute, second) = unix_to_parts(timestamp);
    vok!(vs!(apply_pattern(
        &pattern, year, month, day, hour, minute, second,
    )))
}

pub fn date_str(_: &mut Evaluator, timestamp: i64) -> Value {
    if timestamp < 0 {
        return verr!(vs!("timestamp is negative".to_string()));
    }
    let (year, month, day, hour, minute, second) = unix_to_parts(timestamp);
    vok!(vs!(apply_pattern(
        "%Y-%m-%d", year, month, day, hour, minute, second,
    )))
}

pub fn time_str(_: &mut Evaluator, timestamp: i64) -> Value {
    if timestamp < 0 {
        return verr!(vs!("timestamp is negative".to_string()));
    }
    let (year, month, day, hour, minute, second) = unix_to_parts(timestamp);
    vok!(vs!(apply_pattern(
        "%H:%M:%S", year, month, day, hour, minute, second,
    )))
}
