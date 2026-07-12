use crate::entry::{ConceptCategory, ConceptEntry, DescriptionEntry, DescriptionKind};

pub static CONTROL_FLOW: ConceptEntry = ConceptEntry {
    name: "control flow",
    descriptions: &[
        DescriptionEntry {
            description: "`if` runs a block when the condition is true",
            examples: &["dec int x = 10\n\nif (x > 5) {\n    println(\"big\")\n}"],
            kind: DescriptionKind::Explanation,
            title: None,
            expected_output: &[],
        },
        DescriptionEntry {
            description: "`else if` and `else` add additional branches",
            examples: &[
                "dec int x = 5\n\nif (x > 10) {\n    println(\"big\")\n} else if (x == 5) {\n    println(\"five\")\n} else {\n    println(\"small\")\n}",
            ],
            kind: DescriptionKind::Explanation,
            title: None,
            expected_output: &[],
        },
        DescriptionEntry {
            description: "`while` loops as long as the condition is true",
            examples: &["dec int i = 0\n\nwhile (i < 5) {\n    println(i)\n    i += 1\n}"],
            kind: DescriptionKind::Explanation,
            title: None,
            expected_output: &[],
        },
        DescriptionEntry {
            description: "`break` exits a loop early, `continue` skips to the next iteration",
            examples: &[
                "dec int i = 0\nwhile (true) {\n    if (i == 3) { break }\n    i += 1\n}",
                "dec int i = 0\nwhile (i < 5) {\n    i += 1\n    if (i == 3) { continue }\n    println(i)  // prints 1, 2, 4, 5\n}",
            ],
            kind: DescriptionKind::Explanation,
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

pub static FOR_LOOPS: ConceptEntry = ConceptEntry {
    name: "for loops",
    descriptions: &[
        DescriptionEntry {
            description: "C-style for loop: `for [<type> <var> = <init>, <condition>, <increment>] { }`",
            examples: &["for [int i = 0, i < 5, i += 1] {\n    println(i)\n}"],
            kind: DescriptionKind::Explanation,
            title: None,
            expected_output: &[],
        },
        DescriptionEntry {
            description: "range-based for loop iterates from start to end (exclusive): `for <var> in <start>..<end>`",
            examples: &["for i in 0..5 {\n    println(i)  // 0 1 2 3 4\n}"],
            kind: DescriptionKind::Explanation,
            title: None,
            expected_output: &[],
        },
        DescriptionEntry {
            description: "iterate over an inline array literal",
            examples: &["for x in [10, 20, 30] {\n    println(x)\n}"],
            kind: DescriptionKind::Explanation,
            title: None,
            expected_output: &[],
        },
        DescriptionEntry {
            description: "iterate over an array variable with `for <var> in <array>`",
            examples: &[
                "dec arr[string] names = [\"ali\", \"bob\", \"carl\"]\n\nfor name in names {\n    println(name)\n}",
            ],
            kind: DescriptionKind::Explanation,
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
