use crate::docs::entry::FnEntry;

pub static TYPE_OF: FnEntry = FnEntry {
    signature: "type_of(value)",
    description: "returns the runtime type name of `value` as a string (e.g. \"int\", \"array\"); complements the `is_*` checks in std::types by naming the type instead of testing for one",
    example: r#"
get std::debug::type_of
get std::io::println

dec string a = type_of(5)
dec string b = type_of("hi")

println(a," ", b)"#,
    expected_output: Some("int string"),
    returns: "string",
    errors: None,
    see_also: &["dbg"],
    since: Some("v0.1.5"),
};
