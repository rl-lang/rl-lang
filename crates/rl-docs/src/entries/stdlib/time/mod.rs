use crate::entry::{FnEntry, StdEntry};

mod date_str;
mod format_time;
mod now;
mod now_ms;
mod time_add;
mod time_diff;
mod time_parts;
mod time_str;

pub static TIME: StdEntry = StdEntry {
    name: "time",
    description: "functions for getting the current time, formatting timestamps, and time arithmetic",
    functions: FUNCTIONS,
    since: Some("v0.1.5"),
    unstable: false,
};

static FUNCTIONS: &[&FnEntry] = &[
    &now::NOW,
    &now_ms::NOW_MS,
    &format_time::FORMAT_TIME,
    &date_str::DATE_STR,
    &time_str::TIME_STR,
    &time_add::TIME_ADD,
    &time_diff::TIME_DIFF,
    &time_parts::TIME_PARTS,
];
