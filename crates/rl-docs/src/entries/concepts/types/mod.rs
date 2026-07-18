use crate::entry::{ConceptCategory, ConceptEntry, DescriptionEntry, DescriptionKind};

pub static TYPES: ConceptEntry = ConceptEntry {
    name: "types",
    summary: "rl's primitive types - int, float, bool, string, char, and byte - each with a mandatory declared type checked at compile time",
    category: ConceptCategory::Syntax,
    prerequisites: &[],
    descriptions: &[
        DescriptionEntry {
            kind: DescriptionKind::Syntax,
            title: Some("static typing"),
            description: "rl is statically typed - every variable has a declared type, checked at compile time",
            examples: &[
                "dec int    x = 42",
                "dec float  y = 3.14",
                "dec bool   b = true",
                "dec string s = \"hello\"",
                "dec char   c = 'a'",
            ],
            expected_output: &[],
        },
        DescriptionEntry {
            kind: DescriptionKind::Explanation,
            title: Some("int"),
            description: "`int` is a 64-bit signed integer, and the default type of any bare integer literal",
            examples: &["dec int x = 100\ndec int neg = -42\nprintln(neg)  // -42"],
            expected_output: &["-42"],
        },
        DescriptionEntry {
            kind: DescriptionKind::Explanation,
            title: Some("float"),
            description: "`float` is a 64-bit floating point number",
            examples: &["dec float pi = 3.14\nprintln(pi)  // 3.14"],
            expected_output: &["3.14"],
        },
        DescriptionEntry {
            kind: DescriptionKind::Explanation,
            title: Some("bool"),
            description: "`bool` is either `true` or `false`",
            examples: &["dec bool on = true\nprintln(on)  // true"],
            expected_output: &["true"],
        },
        DescriptionEntry {
            kind: DescriptionKind::Explanation,
            title: Some("string"),
            description: "`string` is a string enclosed in double quotes",
            examples: &["dec string name = \"Mohamed\"\nprintln(name)  // Mohamed"],
            expected_output: &["Mohamed"],
        },
        DescriptionEntry {
            kind: DescriptionKind::Explanation,
            title: Some("char"),
            description: "`char` is a single character enclosed in single quotes",
            examples: &["dec char letter = 'a'\nprintln(letter)  // 'a'"],
            expected_output: &["'a'"],
        },
        DescriptionEntry {
            kind: DescriptionKind::Explanation,
            title: Some("byte"),
            description: "`byte` is an unsigned 8-bit integer (0-255); unlike the other primitives, a bare integer literal is always typed `int`, never `byte` - get a byte value with `<literal> as byte` (see the `byte` concept for the full casting rules)",
            examples: &["dec byte b = 10 as byte\nprintln(b)  // 10"],
            expected_output: &["10"],
        },
        DescriptionEntry {
            kind: DescriptionKind::Pitfall,
            title: Some("literals don't default to byte"),
            description: "a bare integer literal is always `int`, even a small one that would fit in a byte - `dec byte b = 10` (without `as byte`) is a compile-time type mismatch, not an implicit narrowing",
            examples: &[
                "// dec byte b = 10  // error: type mismatch: expected byte, got int\ndec byte b = 10 as byte\nprintln(b)  // 10",
            ],
            expected_output: &["10"],
        },
    ],
    pitfalls: &[
        "a bare integer literal is always `int`, never `byte`, no matter how small - `dec byte b = 10` without `as byte` is a compile-time type mismatch",
    ],
    related: &["byte", "casting", "variables"],
    related_stdlib: &["types"],
    since: Some("v0.1.5"),
};
