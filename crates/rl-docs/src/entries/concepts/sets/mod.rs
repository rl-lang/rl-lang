use crate::entry::{ConceptCategory, ConceptEntry, DescriptionEntry, DescriptionKind};

pub static SETS: ConceptEntry = ConceptEntry {
    name: "sets",
    summary: "",
    category: ConceptCategory::Syntax,
    prerequisites: &["arrays"],
    descriptions: &[
        DescriptionEntry {
            kind: DescriptionKind::Explanation,
            title: None,
            description: "declare a mutable set with `dec set[<item type>] <n> = {<item>, ...}` - the type in brackets is the shared type of every item",
            examples: &[
                "dec set[int] scores = {95, 82, 71}",
                "dec set[string] names = {\"alice\", \"bob\"}",
                "dec set[bool] flags = {true, false}",
            ],
            expected_output: &[],
        },
        DescriptionEntry {
            kind: DescriptionKind::Explanation,
            title: None,
            description: "declare a constant set the same way with `const set[<item type>] <NAME> = {...}`",
            examples: &["const set[int] LUCKY_NUMBERS = {3, 7, 21}"],
            expected_output: &[],
        },
        DescriptionEntry {
            kind: DescriptionKind::Explanation,
            title: None,
            description: "read an item by its position with `set[index]`, exactly like an array - the first item is index 0",
            examples: &["dec set[int] scores = {95, 82, 71}\nprintln(scores[0])  // 95"],
            expected_output: &[],
        },
        DescriptionEntry {
            kind: DescriptionKind::Explanation,
            title: None,
            description: "sets are typed like arrays: every item must share one type, checked against the declared item type",
            examples: &[
                "dec set[int] scores = {95, 82}\n// dec set[int] bad = {95, \"not a number\"}  // error: type mismatch",
            ],
            expected_output: &[],
        },
        DescriptionEntry {
            kind: DescriptionKind::Pitfall,
            title: None,
            description: "unlike a mathematical set, duplicate items are not removed and are not rejected - `{1, 1, 1}` is a valid 3-item set with no uniqueness check performed at declaration or runtime",
            examples: &["dec set[int] xs = {1, 1, 1}  // length is 3, not 1"],
            expected_output: &[],
        },
        DescriptionEntry {
            kind: DescriptionKind::Pitfall,
            title: None,
            description: "`set[index] = value` is rejected - sets do not support index assignment at all, unlike arrays and maps",
            examples: &[
                "dec set[int] scores = {95, 82}\n// scores[0] = 100  // error: sets does not support index assigning",
            ],
            expected_output: &[],
        },
        DescriptionEntry {
            kind: DescriptionKind::Pitfall,
            title: None,
            description: "a set literal `{...}` is only recognized in a `dec set[T]`/`const set[T]` declaration's initializer - writing `{1, 2, 3}` anywhere else (a plain assignment, a function argument, nested inside an array or map literal) is parsed as a map literal instead and fails with `expected ':' after map key`",
            examples: &[
                "dec set[int] xs = {1, 2, 3}  // ok - dedicated declaration syntax\n// xs = {4, 5, 6}  // error: parsed as a map literal, not a set",
            ],
            expected_output: &[],
        },
        DescriptionEntry {
            kind: DescriptionKind::Note,
            title: None,
            description: "like map keys, set items are restricted to hashable types",
            examples: &[],
            expected_output: &[],
        },
        DescriptionEntry {
            kind: DescriptionKind::Note,
            title: None,
            description: "`std::collections` provides `set_add`, `set_remove`, `set_contains`, `set_len`, `set_is_empty`, and `set_to_array` - but there's still no iteration syntax for sets (no `for item in set`), so looping over items means going through `set_to_array` first",
            examples: &[],
            expected_output: &[],
        },
    ],
    pitfalls: &[
        "sets don't enforce uniqueness - think of `set[T]` today as sugar for a fixed, non-index-assignable array, not a mathematical set",
        "a set literal only parses correctly as the initializer of a `dec set[T]`/`const set[T]` declaration - it is not yet a general-purpose expression",
    ],
    related: &["arrays", "maps", "types"],
    related_stdlib: &["collections"],
    since: Some("v0.1.5"),
};
