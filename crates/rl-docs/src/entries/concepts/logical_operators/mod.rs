use crate::entry::{ConceptCategory, ConceptEntry, DescriptionEntry, DescriptionKind};

pub static LOGICAL_OPERATORS: ConceptEntry = ConceptEntry {
    name: "logical operators",
    summary: "boolean `and`, `or`, and `!` (not) for combining and negating `bool` values, with short-circuit evaluation and a shared `and`/`or` precedence level",
    category: ConceptCategory::Syntax,
    prerequisites: &["operators", "types"],
    descriptions: &[
        DescriptionEntry {
            kind: DescriptionKind::Explanation,
            title: Some("and"),
            description: "`and` evaluates to true only if both sides are true",
            examples: &[
                "println(true and true)   // true",
                "println(true and false)  // false",
            ],
            expected_output: &["true", "false"],
        },
        DescriptionEntry {
            kind: DescriptionKind::Explanation,
            title: Some("or"),
            description: "`or` evaluates to true if either side is true",
            examples: &[
                "println(false or true)   // true",
                "println(false or false)  // false",
            ],
            expected_output: &["true", "false"],
        },
        DescriptionEntry {
            kind: DescriptionKind::Explanation,
            title: Some("combining with comparisons"),
            description: "`and`/`or` combine naturally with comparisons inside conditions",
            examples: &[
                "dec int age = 20\ndec bool has_id = true\n\nif age >= 18 and has_id {\n    println(\"allowed\")\n}",
            ],
            expected_output: &["allowed"],
        },
        DescriptionEntry {
            kind: DescriptionKind::Explanation,
            title: Some("not"),
            description: "`!` (not) negates a bool and binds tighter than `and`/`or`",
            examples: &["dec bool ok = !false and true\nprintln(ok)  // true"],
            expected_output: &["true"],
        },
        DescriptionEntry {
            kind: DescriptionKind::Pitfall,
            title: Some("and/or short-circuit"),
            description: "`and` skips evaluating the right side once the left side is already `false`; `or` skips evaluating the right side once the left side is already `true` - so a right side with side effects won't run when the left side alone already determines the result",
            examples: &[
                "fn log_and_return(bool v) -> bool {\n    println(\"evaluated\")\n    return v\n}\n\nfalse and log_and_return(true)  // \"evaluated\" is never printed",
            ],
            expected_output: &[],
        },
        DescriptionEntry {
            kind: DescriptionKind::Pitfall,
            title: Some("and and or share one precedence level"),
            description: "`and` and `or` are parsed at the same precedence and evaluated left-to-right; unlike many other languages, `and` does not bind tighter than `or`, so mixing them without parentheses groups left-to-right rather than by operator",
            examples: &[
                "println(true or false and false)  // false: parses as (true or false) and false",
            ],
            expected_output: &["false"],
        },
        DescriptionEntry {
            kind: DescriptionKind::Pitfall,
            title: Some("both sides must already be bool"),
            description: "`and`/`or` require both operands to already be `bool` - there's no implicit truthiness conversion for other types, so passing something else is a compile-time type error rather than a runtime coercion",
            examples: &[
                "dec int n = 5\n// n and true  // error: expected bool on the left side of and",
            ],
            expected_output: &[],
        },
    ],
    pitfalls: &[
        "`and` short-circuits and skips the right side once the left side is `false`; `or` does the same once the left side is `true`",
        "`and` and `or` share the same precedence and evaluate left-to-right - `and` does not bind tighter than `or` like it does in many other languages",
        "both sides of `and`/`or` must already be `bool` - there's no implicit truthiness conversion, so mismatched types are caught at compile time",
    ],
    related: &["operators", "flow_control", "types"],
    related_stdlib: &["bitwise"],
    since: None,
};
