use crate::entry::{ConceptCategory, ConceptEntry, DescriptionEntry, DescriptionKind};

pub static CONTROL_FLOW: ConceptEntry = ConceptEntry {
    name: "control flow",
    summary: "branching with `if`/`else if`/`else` and looping with `while`, plus `break` to exit a loop early and `continue` to skip to the next condition check",
    category: ConceptCategory::ControlFlow,
    prerequisites: &["types", "operators"],
    descriptions: &[
        DescriptionEntry {
            kind: DescriptionKind::Syntax,
            title: Some("if"),
            description: "`if` runs a block when the condition is true",
            examples: &["dec int x = 10\n\nif (x > 5) {\n    println(\"big\")\n}"],
            expected_output: &[],
        },
        DescriptionEntry {
            kind: DescriptionKind::Explanation,
            title: Some("else if and else"),
            description: "`else if` and `else` add additional branches, checked in order - the first branch whose condition is true runs and the rest are skipped",
            examples: &[
                "dec int x = 5\n\nif (x > 10) {\n    println(\"big\")\n} else if (x == 5) {\n    println(\"five\")\n} else {\n    println(\"small\")\n}",
            ],
            expected_output: &[],
        },
        DescriptionEntry {
            kind: DescriptionKind::Syntax,
            title: Some("while"),
            description: "`while` runs a block repeatedly for as long as the condition is true, checking it before every iteration - including the first",
            examples: &["dec int i = 0\n\nwhile (i < 5) {\n    println(i)\n    i += 1\n}"],
            expected_output: &[],
        },
        DescriptionEntry {
            kind: DescriptionKind::Explanation,
            title: Some("break"),
            description: "`break` exits the innermost enclosing loop immediately, skipping any remaining iterations",
            examples: &["dec int i = 0\nwhile (true) {\n    if (i == 3) { break }\n    i += 1\n}"],
            expected_output: &[],
        },
        DescriptionEntry {
            kind: DescriptionKind::Explanation,
            title: Some("continue"),
            description: "`continue` skips the rest of the current iteration and jumps back to the loop's condition check",
            examples: &[
                "dec int i = 0\nwhile (i < 5) {\n    i += 1\n    if (i == 3) { continue }\n    println(i)  // prints 1, 2, 4, 5\n}",
            ],
            expected_output: &[],
        },
        DescriptionEntry {
            kind: DescriptionKind::Pitfall,
            title: Some("while checks the condition before the first iteration, not after"),
            description: "unlike a do-while loop, `while`'s condition is checked before the body ever runs, so the loop can execute zero times - rl has no post-condition loop that guarantees at least one iteration",
            examples: &["dec int i = 10\nwhile (i < 5) {\n    println(i)  // never runs, i is already >= 5\n}"],
            expected_output: &[],
        },
        DescriptionEntry {
            kind: DescriptionKind::Pitfall,
            title: Some("continue doesn't run a per-iteration step for you"),
            description: "`continue` in a `while` loop jumps straight to the condition check, not to some implicit increment step - any per-iteration update (like `i += 1`) has to happen before the code that can `continue`, or the loop never makes progress and runs forever",
            examples: &[],
            expected_output: &[],
        },
    ],
    pitfalls: &[
        "`while`'s condition is checked before the body ever runs, so it can execute zero times - there's no post-condition loop that guarantees at least one iteration",
        "`continue` in a `while` loop jumps straight back to the condition check rather than running an implicit increment - any per-iteration update needs to happen before the code that can `continue`, or the loop never makes progress",
    ],
    related: &["for loops", "match", "operators", "logical operators"],
    related_stdlib: &[],
    since: None,
};

pub static FOR_LOOPS: ConceptEntry = ConceptEntry {
    name: "for loops",
    summary: "C-style (`for [<type> <var> = <init>, <cond>, <incr>] { }`) and range-based (`for <var> in <start>..<end>`) loops, plus iterating directly over an array literal or array variable",
    category: ConceptCategory::ControlFlow,
    prerequisites: &["control flow", "types", "arrays"],
    descriptions: &[
        DescriptionEntry {
            kind: DescriptionKind::Syntax,
            title: Some("C-style for loop"),
            description: "C-style for loop: `for [<type> <var> = <init>, <condition>, <increment>] { }`",
            examples: &["for [int i = 0, i < 5, i += 1] {\n    println(i)\n}"],
            expected_output: &[],
        },
        DescriptionEntry {
            kind: DescriptionKind::Syntax,
            title: Some("range-based for loop"),
            description: "range-based for loop iterates from start to end (exclusive): `for <var> in <start>..<end>`",
            examples: &["for i in 0..5 {\n    println(i)  // 0 1 2 3 4\n}"],
            expected_output: &[],
        },
        DescriptionEntry {
            kind: DescriptionKind::Explanation,
            title: Some("iterating an array literal"),
            description: "iterate over an inline array literal",
            examples: &["for x in [10, 20, 30] {\n    println(x)\n}"],
            expected_output: &[],
        },
        DescriptionEntry {
            kind: DescriptionKind::Explanation,
            title: Some("iterating an array variable"),
            description: "iterate over an array variable with `for <var> in <array>`",
            examples: &[
                "dec arr[string] names = [\"ali\", \"bob\", \"carl\"]\n\nfor name in names {\n    println(name)\n}",
            ],
            expected_output: &[],
        },
        DescriptionEntry {
            kind: DescriptionKind::Pitfall,
            title: Some("range end is exclusive"),
            description: "`<start>..<end>` excludes `end` - `0..5` iterates 0 through 4, not 5",
            examples: &["for i in 0..3 {\n    println(i)  // 0 1 2, not 3\n}"],
            expected_output: &[],
        },
        DescriptionEntry {
            kind: DescriptionKind::Pitfall,
            title: Some("the loop variable is a copy, not a reference"),
            description: "in a range-based or array `for` loop, the loop variable is a fresh copy of each element, the same way arrays are copied by value elsewhere - reassigning it inside the loop body doesn't write back into the source array",
            examples: &[
                "dec arr[int] nums = [1, 2, 3]\nfor n in nums {\n    n = 0  // only changes the local copy\n}\nprintln(nums)  // [1, 2, 3]",
            ],
            expected_output: &["[1, 2, 3]"],
        },
    ],
    pitfalls: &[
        "a range `<start>..<end>` excludes `end` - `0..5` iterates 0 through 4, not 5",
        "the loop variable in a range-based or array `for` loop is a fresh copy of each element, not a reference - reassigning it inside the loop body doesn't write back into the source array",
    ],
    related: &["control flow", "arrays", "operators"],
    related_stdlib: &["array"],
    since: Some("v0.1.5"),
};
