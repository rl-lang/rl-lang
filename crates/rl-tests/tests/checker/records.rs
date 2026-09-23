use super::common::{assert_checker_clean, assert_checker_msg};

#[test]
fn record_passes() {
    assert_checker_clean("record P { int x, int y }\ndec P p = P { x: 1, y: 2 }");
}

#[test]
fn unknown_field_errors() {
    assert_checker_msg(
        "record P { int x }\nP { y: 1 }",
        "record `P` has no field `y`",
    );
}

#[test]
fn field_type_mismatch_errors() {
    assert_checker_msg(
        r#"record P { int x }
P { x: "hi" }"#,
        "field `x` of record `P` expects Int, got String",
    );
}

#[test]
fn missing_fields_errors() {
    assert_checker_msg(
        "record P { int x, int y }\nP { x: 1 }",
        "record `P` expects 2 field(s), got 1",
    );
}

#[test]
fn field_access_non_record_errors() {
    assert_checker_msg("dec int x = 5\nx.y", "cannot access field `y` on Int");
}

#[test]
fn field_assign_unknown_field_errors() {
    assert_checker_msg(
        "record P { int x }\ndec P p = P { x: 1 }\np.y = 2",
        "record `P` has no field `y`",
    );
}

#[test]
fn struct_literal_brace_on_next_line() {
    // Allman layout: newline between the record name and `{` parses
    assert_checker_clean("record P { int x, int y }\ndec P p = P\n{\nx: 1,\ny: 2,\n}");
}
