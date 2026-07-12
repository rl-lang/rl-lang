use crate::entry::{ConceptCategory, ConceptEntry, DescriptionEntry, DescriptionKind};

pub static ERROR_TYPE: ConceptEntry = ConceptEntry {
    name: "result",
    descriptions: &[
        DescriptionEntry {
            description: "`result[T]` is a type that holds either `ok(value)` on success or `err(value)` on failure",
            examples: &[
                "dec result[int]    r = ok(42)",
                "dec result[string] e = err(\"not found\")",
                "dec result[int]    r = err(404)",
            ],
            kind: DescriptionKind::Explanation,
            title: None,
            expected_output: &[],
        },
        DescriptionEntry {
            description: "`ok(value)` and `err(value)` are the two constructors; both wrap any non-result value",
            examples: &[
                "dec result[int] r = ok(100)",
                "dec result[int] r = err(0)",
                "dec result[string] r = ok(\"hello\")",
            ],
            kind: DescriptionKind::Explanation,
            title: None,
            expected_output: &[],
        },
        DescriptionEntry {
            description: "check which variant you have with `is_ok` and `is_err` from `std::res`",
            examples: &[
                "get is_ok, is_err from std::res\ndec result[int] r = ok(42)\nprintln(is_ok(r))   // true\nprintln(is_err(r))  // false",
            ],
            kind: DescriptionKind::Explanation,
            title: None,
            expected_output: &[],
        },
        DescriptionEntry {
            description: "extract the inner value with `result_unwrap` (ok) or `result_unwrap_err` (err) from `std::res`",
            examples: &[
                "get result_unwrap from std::res\ndec result[int] r = ok(10)\nprintln(result_unwrap(r))  // 10",
                "get result_unwrap_err from std::res\ndec result[int] r = err(\"oops\")\nprintln(result_unwrap_err(r))  // oops",
            ],
            kind: DescriptionKind::Explanation,
            title: None,
            expected_output: &[],
        },
        DescriptionEntry {
            description: "`result_unwrap_or(r, default)` returns the ok value or a fallback if err",
            examples: &[
                "get result_unwrap_or from std::res\ndec result[int] r = err(\"fail\")\nprintln(result_unwrap_or(r, 0))  // 0",
            ],
            kind: DescriptionKind::Explanation,
            title: None,
            expected_output: &[],
        },
        DescriptionEntry {
            description: "functions that may fail should return `result[T]` and the caller checks with `is_ok` / `is_err`",
            examples: &[
                "fn divide(int a, int b) -> result[int] {\n    if b == 0 {\n        return err(\"division by zero\")\n    }\n    return ok(a / b)\n}\n\ndec result[int] r = divide(10, 2)\nif is_ok(r) {\n    println(result_unwrap(r))  // 5\n}",
            ],
            kind: DescriptionKind::Explanation,
            title: None,
            expected_output: &[],
        },
        DescriptionEntry {
            description: "`result_map` transforms the ok value; `result_map_err` transforms the err value - both pass the other variant through unchanged",
            examples: &[
                "get result_map from std::res\ndec result[int] r = ok(5)\ndec result[int] r2 = result_map(r, fn(int x) -> int { return x * 2 })\nprintln(result_unwrap(r2))  // 10",
            ],
            kind: DescriptionKind::Explanation,
            title: None,
            expected_output: &[],
        },
        DescriptionEntry {
            description: "`error[T]` is a separate wrapper type from `result[T]` - construct it with `error(value)`, check with `is_error` from `std::res`, and unwrap with `error_unwrap`",
            examples: &[
                "get is_error, error_unwrap from std::res\ndec error e = error(\"disk full\")\nprintln(is_error(e))          // true\nprintln(error_unwrap(e))      // \"disk full\"",
            ],
            kind: DescriptionKind::Explanation,
            title: None,
            expected_output: &[],
        },
        DescriptionEntry {
            description: "an `error` cannot wrap another `error` - this raises a runtime error",
            examples: &["dec error e = error(error(\"oops\"))  // runtime error"],
            kind: DescriptionKind::Pitfall,
            title: None,
            expected_output: &[],
        },
    ],
    summary: "",
    category: ConceptCategory::Syntax,
    prerequisites: &[],
    pitfalls: &[],
    related: &[],
    related_stdlib: &[],
    since: None,
};
