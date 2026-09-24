//! `std::time` - Unix timestamp functions and time formatting.
//!
//! All timestamps are Unix seconds as `i64`.
//! `format_time` uses a strftime-like pattern with tokens: `%Y`, `%y`, `%m`,
//! `%B`, `%b`, `%d`, `%A`, `%a`, `%w`, `%j`, `%U`, `%W`, `%V`, `%H`, `%I`,
//! `%M`, `%S`, `%p`, `%P`, `%z`, `%Z`. `time_parts` returns
//! `[year, month, day, hour, minute, second]` as an `arr[int]`. `time_add`
//! and `time_diff` are trivial arithmetic helpers - a proper time type is
//! planned. Ported once from the former per-runtime `stdlib/time/*.rs` copies.

use rl_std_macros::native_fn;
use std::sync::OnceLock;
use std::time::{Instant, SystemTime, UNIX_EPOCH};

// ---- now ------------------------------------------------------------------

#[native_fn(module = "time")]
pub fn time_now() -> i64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs() as i64
}

#[native_fn(module = "time")]
pub fn time_now_ms() -> i64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_millis() as i64
}

// ---- arithmetic helpers ---------------------------------------------------

// yes... useless... for now
// should add timestamp or time type later
#[native_fn(module = "time")]
pub fn time_add(ts: i64, seconds: i64) -> i64 {
    ts + seconds
}

#[native_fn(module = "time")]
pub fn time_diff(a: i64, b: i64) -> i64 {
    a - b
}

// ---- Unix timestamp -> parts ----------------------------------------------

/// Decomposes a Unix timestamp into `(year, month, day, hour, minute, second)`
/// using the Gregorian calendar algorithm (proleptic calendar, UTC only, no
/// DST).
fn unix_to_parts(timestamp: i64) -> (i32, u32, u32, u32, u32, u32) {
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

/// Day-of-week: 0=Sunday .. 6=Saturday (Zeller / Tomohiko Sakamoto).
fn day_of_week(year: i32, month: u32, day: u32) -> u32 {
    static T: [i32; 12] = [0, 3, 2, 5, 0, 3, 5, 1, 4, 6, 2, 4];
    let y = year - if month < 3 { 1 } else { 0 };
    ((y + y / 4 - y / 100 + y / 400 + T[(month - 1) as usize] + day as i32).rem_euclid(7))
        as u32
}

/// Day-of-year (1..=366).
fn day_of_year(year: i32, month: u32, day: u32) -> u32 {
    let is_leap = (year % 4 == 0 && year % 100 != 0) || year % 400 == 0;
    let days = [0, 31, 59, 90, 120, 151, 181, 212, 243, 273, 304, 334];
    let base = days[(month - 1) as usize] + day as i32;
    (if is_leap && month > 2 { base + 1 } else { base }) as u32
}

/// ISO week number of year (Monday as first day, 01..53).
fn iso_week_number(year: i32, month: u32, day: u32) -> u32 {
    let doy = day_of_year(year, month, day) as i32;
    let dow = day_of_week(year, month, day) as i32;
    let week = (doy - dow + 10) / 7;
    if week < 1 {
        // ISO week belongs to previous year; compute that year's 52 or 53.
        let prev_y = year - 1;
        let prev_doy = day_of_year(prev_y, 12, 31);
        let prev_dow = day_of_week(prev_y, 12, 31) as i32;
        let w = (prev_doy as i32 - prev_dow + 10) / 7;
        if w >= 1 { w as u32 } else { 52 }
    } else if week > 52 {
        // Could be week 53 of this year if Dec 31 is Thu+.
        let dec31_dow = day_of_week(year, 12, 31) as i32;
        if dec31_dow >= 1 { 53 } else { 52 }
    } else {
        week as u32
    }
}

const WEEKDAY_NAMES: [&str; 7] = [
    "Sunday", "Monday", "Tuesday", "Wednesday", "Thursday", "Friday", "Saturday",
];
const WEEKDAY_SHORT: [&str; 7] = ["Sun", "Mon", "Tue", "Wed", "Thu", "Fri", "Sat"];
const MONTH_NAMES: [&str; 12] = [
    "January", "February", "March", "April", "May", "June",
    "July", "August", "September", "October", "November", "December",
];
const MONTH_SHORT: [&str; 12] = [
    "Jan", "Feb", "Mar", "Apr", "May", "Jun",
    "Jul", "Aug", "Sep", "Oct", "Nov", "Dec",
];

/// Simple string substitution on strftime-like tokens.  Supported:
///
/// - `%Y`  4-digit year            `2026`
/// - `%y`  2-digit year            `26`
/// - `%m`  month (01-12)           `07`
/// - `%B`  full month name         `July`
/// - `%b`  abbreviated month       `Jul`
/// - `%d`  day of month (01-31)    `17`
/// - `%A`  full weekday name       `Thursday`
/// - `%a`  abbreviated weekday     `Thu`
/// - `%w`  weekday number (0-6)    `4`
/// - `%j`  day of year (001-366)   `198`
/// - `%U`  week number (Sun start, 00-53) `28`
/// - `%W`  week number (Mon start, 00-53) `28`
/// - `%V`  ISO week number (01-53) `28`
/// - `%H`  hour 24h (00-23)        `16`
/// - `%I`  hour 12h (01-12)        `04`
/// - `%M`  minute (00-59)          `32`
/// - `%S`  second (00-59)          `29`
/// - `%p`  AM/PM                   `PM`
/// - `%P`  am/pm                   `pm`
/// - `%z`  UTC offset              `+0000`
/// - `%Z`  timezone name           `UTC`
fn apply_pattern(
    pattern: &str,
    year: i32,
    month: u32,
    day: u32,
    hour: u32,
    minute: u32,
    second: u32,
) -> String {
    let dow = day_of_week(year, month, day) as usize;
    let doy = day_of_year(year, month, day);
    let iso_w = iso_week_number(year, month, day);
    let hour_12 = match hour % 12 {
        0 => 12,
        h => h,
    };
    let ampm = if hour < 12 { "AM" } else { "PM" };
    let ampm_lower = if hour < 12 { "am" } else { "pm" };

    // Order matters: %Y before %y, %H before %H, etc.
    pattern
        .replace("%Y", &format!("{:04}", year))
        .replace("%y", &format!("{:02}", (year % 100).abs()))
        .replace("%m", &format!("{:02}", month))
        .replace("%B", MONTH_NAMES[(month - 1) as usize])
        .replace("%b", MONTH_SHORT[(month - 1) as usize])
        .replace("%d", &format!("{:02}", day))
        .replace("%A", WEEKDAY_NAMES[dow])
        .replace("%a", WEEKDAY_SHORT[dow])
        .replace("%w", &dow.to_string())
        .replace("%j", &format!("{:03}", doy))
        .replace("%V", &format!("{:02}", iso_w))
        .replace("%U", &format!("{:02}", (doy as i32 - dow as i32 + 7) / 7))
        .replace("%W", &format!("{:02}", (doy as i32 - (dow as i32 + 6) % 7 + 7) / 7))
        .replace("%H", &format!("{:02}", hour))
        .replace("%I", &format!("{:02}", hour_12))
        .replace("%M", &format!("{:02}", minute))
        .replace("%S", &format!("{:02}", second))
        .replace("%p", ampm)
        .replace("%P", ampm_lower)
        .replace("%z", "+0000")
        .replace("%Z", "UTC")
}

// ---- formatting (language `result[string]`) -------------------------------

#[native_fn(module = "time")]
pub fn format_time(timestamp: i64, pattern: String) -> Result<String, String> {
    if timestamp < 0 {
        return Err("timestamp is negative".to_string());
    }
    let (year, month, day, hour, minute, second) = unix_to_parts(timestamp);
    Ok(apply_pattern(
        &pattern, year, month, day, hour, minute, second,
    ))
}

#[native_fn(module = "time")]
pub fn format_date_str(timestamp: i64) -> Result<String, String> {
    if timestamp < 0 {
        return Err("timestamp is negative".to_string());
    }
    let (year, month, day, hour, minute, second) = unix_to_parts(timestamp);
    Ok(apply_pattern(
        "%Y-%m-%d", year, month, day, hour, minute, second,
    ))
}

#[native_fn(module = "time")]
pub fn format_time_str(timestamp: i64) -> Result<String, String> {
    if timestamp < 0 {
        return Err("timestamp is negative".to_string());
    }
    let (year, month, day, hour, minute, second) = unix_to_parts(timestamp);
    Ok(apply_pattern(
        "%H:%M:%S", year, month, day, hour, minute, second,
    ))
}

// ---- parts (language `result[array[int]]`) --------------------------------

#[native_fn(module = "time")]
pub fn time_parts(timestamp: i64) -> Result<Vec<i64>, String> {
    if timestamp < 0 {
        return Err("timestamp is negative".to_string());
    }
    let (year, month, day, hour, minute, second) = unix_to_parts(timestamp);
    Ok(vec![
        year as i64,
        month as i64,
        day as i64,
        hour as i64,
        minute as i64,
        second as i64,
    ])
}

// ---- monotonic clock ------------------------------------------------------

static MONO_START: OnceLock<Instant> = OnceLock::new();

#[native_fn(module = "time")]
pub fn monotonic_now() -> i64 {
    let start = MONO_START.get_or_init(Instant::now);
    start.elapsed().as_nanos() as i64
}

rl_std_core::native_module!("time";
    funcs: [
        time_now, time_now_ms,
        time_add, time_diff,
        format_time, format_date_str, format_time_str,
        time_parts,
        monotonic_now,
    ],
);
