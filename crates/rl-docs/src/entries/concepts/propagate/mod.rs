use crate::entry::{ConceptCategory, ConceptEntry, DescriptionEntry, DescriptionKind};

pub static PROPAGATE: ConceptEntry = ConceptEntry {
    name: "propagation",
    summary: "the postfix `?` operator that unwraps a `result[T]` - `ok(v)` evaluates to `v`, while `err(e)` returns `err(e)` immediately from the enclosing function, skipping manual `is_ok`/`is_err` checks",
    descriptions: &[
        DescriptionEntry {
            kind: DescriptionKind::Explanation,
            title: Some("unwrapping with ?"),
            description: "`expr?` unwraps a `result[T]`: on `ok(v)` it evaluates to `v`, on `err(e)` it immediately returns `err(e)` from the enclosing function",
            examples: &[
                "fn parse_positive(int n) -> result[int] {\n    if n < 0 {\n        return err(\"negative\")\n    }\n    return ok(n)\n}\n\nfn double_positive(int n) -> result[int] {\n    dec int v = parse_positive(n)?  // returns err early if parse_positive fails\n    return ok(v * 2)\n}",
            ],
            expected_output: &[],
        },
        DescriptionEntry {
            kind: DescriptionKind::Explanation,
            title: Some("avoiding manual checks"),
            description: "`?` avoids manual `is_ok`/`is_err` checks for chains of fallible calls",
            examples: &[
                "get is_ok, result_unwrap from std::res\n\nfn without_propagate(int n) -> result[int] {\n    dec result[int] r = parse_positive(n)\n    if is_ok(r) {\n        return ok(result_unwrap(r) * 2)\n    }\n    return r\n}\n\nfn with_propagate(int n) -> result[int] {\n    return ok(parse_positive(n)? * 2)\n}",
            ],
            expected_output: &[],
        },
        DescriptionEntry {
            kind: DescriptionKind::Pitfall,
            title: Some("? on a non-result value is a no-op"),
            description: "`?` on a non-`result` value is a no-op - it only short-circuits on `err`",
            examples: &["dec int x = 5?  // no-op, x = 5"],
            expected_output: &[],
        },
        DescriptionEntry {
            kind: DescriptionKind::Pitfall,
            title: Some("the enclosing function needs a compatible return type"),
            description: "the enclosing function should be declared to return `result[T]`, since `?` may return an `err` value directly out of it",
            examples: &[],
            expected_output: &[],
        },
    ],
    category: ConceptCategory::ErrorHandling,
    prerequisites: &["result"],
    pitfalls: &[
        "`?` on a non-`result` value is a no-op - it only short-circuits on `err`",
        "the enclosing function should be declared to return `result[T]`, since `?` may return an `err` value directly out of it",
    ],
    related: &["result"],
    related_stdlib: &["result"],
    since: Some("v0.1.5"),
};
