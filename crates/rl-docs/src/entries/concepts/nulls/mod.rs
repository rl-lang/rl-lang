use crate::entry::{ConceptCategory, ConceptEntry, DescriptionEntry, DescriptionKind};

pub static NULL: ConceptEntry = ConceptEntry {
    name: "null",
    summary: "the value representing the absence of data; also the implicit return value of a function that doesn't explicitly return anything",
    category: ConceptCategory::Types,
    prerequisites: &["types", "variables"],
    descriptions: &[
        DescriptionEntry {
            kind: DescriptionKind::Explanation,
            title: Some("what null represents"),
            description: "`null` represents the absence of a value and can be assigned to a variable of any declared type",
            examples: &["dec int x = null\nprintln(x)  // null"],
            expected_output: &["null"],
        },
        DescriptionEntry {
            kind: DescriptionKind::Explanation,
            title: Some("implicit function returns"),
            description: "a function with no `-> <type>` and no `return` statement implicitly returns `null` when it finishes",
            examples: &[
                "fn do_nothing() {\n    // does nothing\n}\n\nprintln(do_nothing())  // null",
            ],
            expected_output: &["null"],
        },
        DescriptionEntry {
            kind: DescriptionKind::Explanation,
            title: Some("checking for null"),
            description: "check whether a value is `null` with `is_null` from `std::types`",
            examples: &[
                "get is_null from std::types\n\ndec int x = null\nprintln(is_null(x))  // true",
            ],
            expected_output: &["true"],
        },
        DescriptionEntry {
            kind: DescriptionKind::Pitfall,
            title: Some("declared type doesn't rule out null"),
            description: "assigning `null` to a variable of a declared type still type-checks at compile time; nothing stops `null` from flowing into code that expects a real value of that type, so the mismatch only surfaces at runtime",
            examples: &["dec int x = null\ndec int y = x + 1  // type-checks, fails at runtime"],
            expected_output: &[],
        },
        DescriptionEntry {
            kind: DescriptionKind::Pitfall,
            title: Some("a missing return type means a silent null"),
            description: "leaving off `-> <type>` on a function makes it silently return `null` even if the body computes a value that's just never `return`ed - easy to miss if a return type was intended",
            examples: &[
                "fn add(int a, int b) {\n    a + b  // never returned\n}\n\nprintln(add(2, 3))  // null",
            ],
            expected_output: &["null"],
        },
    ],
    pitfalls: &[
        "a variable holding null still passes compile-time type checks for its declared type - the mismatch is only caught at runtime",
        "a function without an explicit `-> <type>` silently returns null instead of erroring, even if that wasn't intended",
    ],
    related: &["types", "variables", "functions"],
    related_stdlib: &["types"],
    since: Some("v0.1.5"),
};
