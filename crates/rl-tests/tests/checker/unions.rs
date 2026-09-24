use super::common::{assert_checker_clean, assert_checker_msg};

#[test]
fn union_member_value_passes() {
    assert_checker_clean("dec any[int, string] x = 1\n");
    assert_checker_clean("dec any[int, string] x = \"hi\"\n");
}

#[test]
fn union_non_member_value_errors() {
    assert_checker_msg(
        "dec any[int, string] x = 1.5\n",
        "type mismatch",
    );
}

#[test]
fn union_never_narrows_implicitly() {
    assert_checker_msg(
        "dec any[int, string] x = 1\ndec int y = x\n",
        "type mismatch",
    );
}

#[test]
fn union_narrows_with_numeric_cast() {
    assert_checker_clean("dec any[int, string] x = 1\ndec int y = x as int\n");
}

#[test]
fn union_subset_assigns() {
    assert_checker_clean(
        "dec any[int, string] x = 1\ndec any[int, string, bool] y = x\n",
    );
}

#[test]
fn union_superset_does_not_assign() {
    assert_checker_msg(
        "dec any[int, string, bool] x = 1\ndec any[int, string] y = x\n",
        "type mismatch",
    );
}

#[test]
fn union_propagate_unwraps_all_results() {
    assert_checker_clean(
        "get read_file from std::fs\nfn grab(string p) -> any[result[string], result[int]] { read_file(p) }\ndec s = grab(\"x\")?\n",
    );
}

#[test]
fn union_propagate_rejects_mixed() {
    assert_checker_msg(
        "get read_file from std::fs\nfn grab(string p) -> any[result[string], int] { read_file(p) }\ndec s = grab(\"x\")?\n",
        "requires a result",
    );
}

#[test]
fn union_operator_names_failing_members() {
    assert_checker_msg(
        "dec any[int, string] x = 1\ndec y = x + 1\n",
        "not supported for every member",
    );
}

#[test]
fn union_index_merges_element_types() {
    assert_checker_clean(
        "dec any[arr[int], arr[string]] a = [1]\ndec z = a[0]\n",
    );
}

#[test]
fn union_index_rejects_unindexable_member() {
    assert_checker_msg(
        "dec any[arr[int], string] a = [1]\ndec z = a[0]\n",
        "for every member of the union",
    );
}

#[test]
fn union_method_untyped_callees_stay_lenient() {
    // `len` is untyped: same leniency as non-union calls, even though
    // string has no len (a runtime error there, like plain values)
    assert_checker_clean(
        "get len from std::array\ndec any[arr[int], string] a = [1]\ndec n = a.len()\n",
    );
}

#[test]
fn union_method_requires_unanimity() {
    assert_checker_msg(
        "get arr_push from std::array\ndec any[arr[int], string] a = [1]\ndec b = a.arr_push(2)\n",
        "not supported for every member",
    );
}
