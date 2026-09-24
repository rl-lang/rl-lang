use crate::entry::{FnEntry, StdEntry};

mod date_str;
mod format_time;
mod monotonic_now;
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
    &date_str::DATE_STR,
    &format_time::FORMAT_TIME,
    &monotonic_now::MONOTONIC_NOW,
    &now::NOW,
    &now_ms::NOW_MS,
    &time_add::TIME_ADD,
    &time_diff::TIME_DIFF,
    &time_parts::TIME_PARTS,
    &time_str::TIME_STR,
];
