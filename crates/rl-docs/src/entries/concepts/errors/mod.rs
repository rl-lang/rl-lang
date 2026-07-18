use crate::entry::{ConceptCategory, ConceptEntry, DescriptionEntry, DescriptionKind};

pub static ERROR_TYPE: ConceptEntry = ConceptEntry {
    name: "result",
    summary: "`result[T]` (`ok`/`err`) for recoverable failures, plus the separate, non-generic `error` wrapper - backed by the `std::res` and `std::types` stdlib modules respectively",
    category: ConceptCategory::Syntax,
    prerequisites: &["types", "functions"],
    descriptions: &[
        DescriptionEntry {
            kind: DescriptionKind::Explanation,
            title: Some("result[T]"),
            description: "`result[T]` is a type that holds either `ok(value)` on success or `err(value)` on failure",
            examples: &[
                "dec result[int]    r = ok(42)",
                "dec result[string] e = err(\"not found\")",
                "dec result[int]    r = err(404)",
            ],
            expected_output: &[],
        },
        DescriptionEntry {
            kind: DescriptionKind::Explanation,
            title: Some("constructing a result"),
            description: "`ok(value)` and `err(value)` are the two constructors",
            examples: &[
                "dec result[int] r = ok(100)",
                "dec result[int] r = err(0)",
                "dec result[string] r = ok(\"hello\")",
            ],
            expected_output: &[],
        },
        DescriptionEntry {
            kind: DescriptionKind::Explanation,
            title: Some("checking which variant"),
            description: "check which variant you have with `is_ok` and `is_err` from `std::res`",
            examples: &[
                "get is_ok, is_err from std::res\ndec result[int] r = ok(42)\nprintln(is_ok(r))   // true",
                "get is_ok, is_err from std::res\ndec result[int] r = ok(42)\nprintln(is_err(r))  // false",
            ],
            expected_output: &["true", "false"],
        },
        DescriptionEntry {
            kind: DescriptionKind::Explanation,
            title: Some("unwrapping"),
            description: "extract the inner value with `result_unwrap` (ok) or `result_unwrap_err` (err) from `std::res`",
            examples: &[
                "get result_unwrap from std::res\ndec result[int] r = ok(10)\nprintln(result_unwrap(r))  // 10",
                "get result_unwrap_err from std::res\ndec result[int] r = err(\"oops\")\nprintln(result_unwrap_err(r))  // oops",
            ],
            expected_output: &["10", "oops"],
        },
        DescriptionEntry {
            kind: DescriptionKind::Pitfall,
            title: Some("unwrapping the wrong variant panics"),
            description: "`result_unwrap` on an `err` value (or `result_unwrap_err` on an `ok` value) is a runtime error, not `null` or a silent default - check with `is_ok`/`is_err` first, or use `result_unwrap_or` for a fallback",
            examples: &[
                "// get result_unwrap from std::res\n// dec result[int] r = err(\"boom\")\n// result_unwrap(r)  // runtime error: result_unwrap: called on Err(boom)",
            ],
            expected_output: &[],
        },
        DescriptionEntry {
            kind: DescriptionKind::Explanation,
            title: Some("unwrap with a fallback"),
            description: "`result_unwrap_or(r, default)` returns the ok value or a fallback if err",
            examples: &[
                "get result_unwrap_or from std::res\ndec result[int] r = err(\"fail\")\nprintln(result_unwrap_or(r, 0))  // 0",
            ],
            expected_output: &["0"],
        },
        DescriptionEntry {
            kind: DescriptionKind::Explanation,
            title: Some("functions that can fail"),
            description: "functions that may fail should return `result[T]` and the caller checks with `is_ok` / `is_err`",
            examples: &[
                "fn divide(int a, int b) -> result[int] {\n    if b == 0 {\n        return err(\"division by zero\")\n    }\n    return ok(a / b)\n}\n\ndec result[int] r = divide(10, 2)\nif is_ok(r) {\n    println(result_unwrap(r))  // 5\n}",
            ],
            expected_output: &["5"],
        },
        DescriptionEntry {
            kind: DescriptionKind::Explanation,
            title: Some("transforming a result"),
            description: "`result_map` transforms the ok value; `result_map_err` transforms the err value - both pass the other variant through unchanged",
            examples: &[
                "get result_map from std::res\ndec result[int] r = ok(5)\ndec result[int] r2 = result_map(r, fn(int x) -> int { return x * 2 })\nprintln(result_unwrap(r2))  // 10",
            ],
            expected_output: &["10"],
        },
        DescriptionEntry {
            kind: DescriptionKind::Explanation,
            title: Some("the error type"),
            description: "`error` is a separate, non-generic wrapper type from `result[T]` - construct it with `error(value)`, check with `is_error`, and unwrap with `unwrap_error`, both from `std::types` (not `std::res`)",
            examples: &["get is_error from std::types\ndec error e = error(\"disk full\")\nprintln(is_error(e))  // true"],
            expected_output: &["true"],
        },
        DescriptionEntry {
            kind: DescriptionKind::Pitfall,
            title: Some("unwrap_error returns a result, it doesn't panic"),
            description: "unlike `result_unwrap`, `unwrap_error` never panics - it returns `ok(inner)` if the value really was an `error`, or `err(...)` if it wasn't, so its own result still needs unwrapping afterward",
            examples: &[
                "get is_error, unwrap_error from std::types\nget result_unwrap from std::res\n\ndec error e = error(\"disk full\")\ndec result[string] r = unwrap_error(e)\nprintln(result_unwrap(r))  // disk full",
            ],
            expected_output: &["disk full"],
        },
        DescriptionEntry {
            kind: DescriptionKind::Pitfall,
            title: Some("error cannot wrap another error"),
            description: "an `error` cannot wrap another `error` - this is caught by the type checker at compile time, and enforced again at runtime as a backstop",
            examples: &[
                "// dec error e = error(error(\"oops\"))  // compile error: error cannot wrap another error",
            ],
            expected_output: &[],
        },
        DescriptionEntry {
            kind: DescriptionKind::Pitfall,
            title: Some("ok/err can nest a result, error can't nest an error"),
            description: "unlike `error`, `ok`/`err` don't reject wrapping another `result` - `ok(ok(5))` is allowed and produces a nested `result[result[int]]`",
            examples: &["dec result[result[int]] nested = ok(ok(5))\nprintln(nested)  // ok(ok(5))"],
            expected_output: &["ok(ok(5))"],
        },
        DescriptionEntry {
            kind: DescriptionKind::Pitfall,
            title: Some("ok and err share the same type parameter"),
            description: "`result[T]` has a single type parameter shared by both variants - the value passed to `err(...)` needs to be the same type `T` as the value passed to `ok(...)`, not an independently-typed error the way some languages' Result types allow",
            examples: &[],
            expected_output: &[],
        },
    ],
    pitfalls: &[
        "`result_unwrap` on an `err` (or `result_unwrap_err` on an `ok`) is a runtime error, not a silent default - check the variant first or use `result_unwrap_or`",
        "`unwrap_error` (from `std::types`) never panics - it returns a `result[T]` instead, so the call itself needs unwrapping too",
        "`error` cannot wrap another `error` - caught at compile time by the checker, not just at runtime",
        "unlike `error`, `ok`/`err` don't reject nesting a `result` inside another `result`",
        "`result[T]` uses one type parameter for both variants - the value passed to `err` must be the same type as the value passed to `ok`, not an independently-typed error",
    ],
    related: &["propagation", "types", "functions"],
    related_stdlib: &["res", "types"],
    since: None,
};
