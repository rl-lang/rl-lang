use crate::entry::FnEntry;

pub static FORMAT: FnEntry = FnEntry {
    signature: "format(template, ...)",
    description: "replaces each \"{}\" in template with the corresponding argument, in order",
    example: "get std::str::format\n\nformat(\"{} is {}\", \"age\", 30)",
    expected_output: Some("\"age is 30\""),
    returns: "string",
    errors: Some(
        "Will panic at runtime (not a catchable `result[..]` err) on the following:\n\n- `template` has more \"{}\" placeholders than arguments given\n- more arguments are given than \"{}\" placeholders used",
    ),
    see_also: &["concat"],
    since: Some("v0.1.5"),
};
