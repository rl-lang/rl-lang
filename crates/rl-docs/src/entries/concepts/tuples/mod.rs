use crate::entry::{ConceptCategory, ConceptEntry, DescriptionEntry, DescriptionKind};

pub static TUPLES: ConceptEntry = ConceptEntry {
    name: "tuples",
    summary: "a fixed-size, heterogeneous grouping of values declared with `dec (<type>, <type>, ...) <n> = (<items>)`, read by literal-index `t[i]` or destructured into named bindings - copied by value, not shared by reference",
    category: ConceptCategory::Syntax,
    prerequisites: &["types"],
    descriptions: &[
        DescriptionEntry {
            kind: DescriptionKind::Syntax,
            title: Some("declaring a tuple"),
            description: "a tuple holds a fixed number of values of different types, declared with `dec (<type>, <type>, ...) <n> = (<items>)`",
            examples: &[
                "dec (int, string) p = (42, \"hello\")",
                "dec (int, float, bool) t = (1, 3.14, true)",
            ],
            expected_output: &[],
        },
        DescriptionEntry {
            kind: DescriptionKind::Syntax,
            title: Some("indexing"),
            description: "access tuple elements by index with `t[i]` (zero-based)",
            examples: &[
                "dec (int, string) p = (42, \"hello\")\nprintln(p[0])  // 42\nprintln(p[1])  // hello",
            ],
            expected_output: &[],
        },
        DescriptionEntry {
            kind: DescriptionKind::Syntax,
            title: Some("destructuring"),
            description: "destructure a tuple into named bindings in one declaration",
            examples: &[
                "dec int x, string y = (10, \"world\")\nprintln(x)  // 10\nprintln(y)  // world",
                "dec int a, float b, bool c = (5, 2.5, true)",
            ],
            expected_output: &[],
        },
        DescriptionEntry {
            kind: DescriptionKind::Syntax,
            title: Some("constant tuples"),
            description: "constant tuples use CONST with the tuple type",
            examples: &["CONST (int, string) P = (0, \"origin\")"],
            expected_output: &[],
        },
        DescriptionEntry {
            kind: DescriptionKind::Explanation,
            title: Some("tuples as array elements"),
            description: "tuples can be used as array elements for homogeneous collections of structured data",
            examples: &["dec arr[(int, string)] rows = [(1, \"one\"), (2, \"two\")]"],
            expected_output: &[],
        },
        DescriptionEntry {
            kind: DescriptionKind::Pitfall,
            title: Some("index must be a literal integer"),
            description: "`t[i]` requires `i` to be a literal integer, not a variable - since each position in a tuple can hold a different type, the index has to be known at compile time so the checker can determine the result type",
            examples: &[
                "// dec (int, string) p = (42, \"hello\")\n// dec int idx = 0\n// println(p[idx])  // error: tuple index must be a literal integer",
            ],
            expected_output: &[],
        },
        DescriptionEntry {
            kind: DescriptionKind::Pitfall,
            title: Some("destructuring arity and order must match"),
            description: "a destructuring `dec` must bind exactly as many names as the tuple has elements, in the same order, with each name's declared type matching the tuple's type at that position - a mismatched count or type is a compile-time error, not a partial destructure",
            examples: &[],
            expected_output: &[],
        },
        DescriptionEntry {
            kind: DescriptionKind::Pitfall,
            title: Some("tuples are copied by value"),
            description: "like arrays, tuples are value types - assigning one tuple variable to another, or destructuring it, copies the elements rather than aliasing them; this is different from records, which share their underlying data",
            examples: &[],
            expected_output: &[],
        },
    ],
    pitfalls: &[
        "`t[i]` requires `i` to be a literal integer, not a variable - since each position can hold a different type, the index has to be known at compile time to determine the result type",
        "a destructuring `dec` must bind exactly as many names as the tuple has elements, in the same order, with each name's type matching the tuple's type at that position",
        "tuples are copied by value, like arrays, not shared by reference like records - assigning or destructuring a tuple copies its elements",
    ],
    related: &["arrays", "records", "variables", "types"],
    related_stdlib: &[],
    since: Some("v0.1.5"),
};
