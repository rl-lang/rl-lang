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
        "get len from std\ndec any[arr[int], string] a = [1]\ndec n = a.len()\n",
    );
}

#[test]
fn union_method_requires_unanimity() {
    assert_checker_msg(
        "get arr_push from std::array\ndec any[arr[int], string] a = [1]\ndec b = a.arr_push(2)\n",
        "not supported for every member",
    );
}

#[test]
fn is_returns_bool() {
    assert_checker_clean("dec any[int, string] x = 1\ndec bool b = x is int\n");
}

#[test]
fn is_refines_true_branch() {
    assert_checker_clean(
        "dec any[int, string] x = 1\nif x is int\n{\ndec int y = x + 1\n}\n",
    );
}

#[test]
fn is_refines_else_branch_by_subtraction() {
    assert_checker_clean(
        "dec any[int, string] x = 1\nif x is string\n{\n}\nelse\n{\ndec int y = x as int\n}\n",
    );
}

#[test]
fn is_refinement_drops_past_reassignment() {
    // use-then-reassign: first use refined, later uses see the union
    assert_checker_clean(
        "get println from std::io\ndec any[int, string] x = 1\nif x is int\n{\nprintln(x + 1)\nx = \"s\"\nprintln(x)\n}\n",
    );
}

#[test]
fn is_refinement_killed_by_closure() {
    // a closure in the body may observe a later reassignment
    assert_checker_msg(
        "dec any[int, string] x = 1\nif x is int\n{\ndec f = fn() { return 1 }\ndec int y = x + 1\n}\n",
        "not supported for every member",
    );
}

#[test]
fn is_refines_while_body() {
    assert_checker_clean(
        "dec any[int, string] x = 1\nwhile x is int\n{\ndec int y = x + 1\nbreak\n}\n",
    );
}

#[test]
fn is_negation_refines_single_remainder() {
    assert_checker_clean(
        "dec any[int, string] x = 1\nif !(x is string)\n{\ndec int y = x + 1\n}\n",
    );
}

#[test]
fn for_in_union_merges_elements() {
    assert_checker_clean(
        "dec any[arr[int], arr[string]] a = [1]\nfor x in a\n{\n}\n",
    );
}

#[test]
fn for_in_union_rejects_non_arrays() {
    assert_checker_msg(
        "dec any[arr[int], string] a = [1]\nfor x in a\n{\n}\n",
        "every member to be an array",
    );
}
