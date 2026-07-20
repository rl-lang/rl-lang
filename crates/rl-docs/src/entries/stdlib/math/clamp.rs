use crate::entry::FnEntry;

pub static CLAMP: FnEntry = FnEntry {
    signature: "clamp(x, min, max)",
    description: "clamps x between min and max, returning min if x < min, max if x > max",
    example: r#"get std::math::clamp

clamp(12, 15, 20)?"#,
    expected_output: Some("15"),
    returns: "result[int] or result[float]",
    errors: Some(
        r#"Will return error on the following:

- `x`, `min`, or `max` is not an int or float
- `x`, `min`, and `max` are not all the same type (e.g. mixing int and float)"#,
    ),
    see_also: &["min", "max"],
    since: Some("v0.1.5"),
};
