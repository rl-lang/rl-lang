use crate::entry::{ConceptCategory, ConceptEntry, DescriptionEntry, DescriptionKind};

pub static CONSTANTS: ConceptEntry = ConceptEntry {
    name: "constants",
    summary: "immutable bindings declared with `CONST <type> <name> = <value>`, checked for reassignment at both compile time and runtime; the initializer can be any well-typed expression, not just a literal",
    category: ConceptCategory::Syntax,
    prerequisites: &["variables", "types"],
    descriptions: &[
        DescriptionEntry {
            kind: DescriptionKind::Syntax,
            title: Some("declaring a constant"),
            description: "declare a constant with `CONST <type> <name> = <value>` but it cannot be reassigned, convention is UPPER_CASE (but anything works)",
            examples: &[
                "CONST int    MAX_SIZE  = 100",
                "CONST float  EULER     = 2.71828",
                "CONST bool   DEBUG     = false",
                "CONST string LANG      = \"rl\"",
                "CONST char   NEWLINE   = '\\n'",
            ],
            expected_output: &[],
        },
        DescriptionEntry {
            kind: DescriptionKind::Syntax,
            title: Some("constant arrays"),
            description: "constant arrays use `CONST arr[<type>]`",
            examples: &[
                "CONST arr[int]    PRIMES = [2, 3, 5, 7, 11]",
                "CONST arr[string] DAYS   = [\"sat\", \"sun\", \"mon\"]",
            ],
            expected_output: &[],
        },
        DescriptionEntry {
            kind: DescriptionKind::Explanation,
            title: Some("the initializer isn't limited to literals"),
            description: "a `CONST`'s value can be any well-typed expression, not just a literal - it's evaluated once when the constant is declared",
            examples: &[
                "fn double(int n) -> int {\n    return n * 2\n}\n\nCONST int DOUBLED = double(21)\nprintln(DOUBLED)  // 42",
            ],
            expected_output: &["42"],
        },
        DescriptionEntry {
            kind: DescriptionKind::Pitfall,
            title: Some("reassignment is rejected at compile time and runtime"),
            description: "assigning to a `CONST` - directly, through a compound operator, or by index on a constant array - is caught both by the type checker and, as a backstop, at runtime",
            examples: &[
                "CONST int MAX = 100\n// MAX = 200   // error: cannot assign to constant 'MAX'\n// MAX += 1    // same error, since += desugars to an assignment",
            ],
            expected_output: &[],
        },
        DescriptionEntry {
            kind: DescriptionKind::Pitfall,
            title: Some("a dec can shadow a constant of the same name"),
            description: "only declaring a second `CONST` with a name already in scope is a compile error - declaring a `dec` variable with that same name doesn't trigger the check, so a plain variable can silently shadow an existing constant in the same scope",
            examples: &["CONST int MAX = 100\ndec int MAX = 5\nprintln(MAX)  // 5"],
            expected_output: &["5"],
        },
    ],
    pitfalls: &[
        "a `CONST`'s initializer isn't limited to literals - it can be any well-typed expression, evaluated once at declaration",
        "reassigning a constant (directly or via a compound operator) is rejected at both compile time and runtime",
        "only declaring a second `CONST` with an already-used name is a compile error - a `dec` variable can silently shadow a constant of the same name in the same scope",
    ],
    related: &["variables", "types", "arrays"],
    related_stdlib: &[],
    since: Some("v0.1.5"),
};
