use crate::entry::{ConceptCategory, ConceptEntry, DescriptionEntry, DescriptionKind};

pub static TUPLES: ConceptEntry = ConceptEntry {
    name: "tuples",
    descriptions: &[
        DescriptionEntry {
            description: "a tuple holds a fixed number of values of different types",
            examples: &[
                "dec (int, string) p = (42, \"hello\")",
                "dec (int, float, bool) t = (1, 3.14, true)",
            ],
            kind: DescriptionKind::Explanation,
            title: None,
            expected_output: &[],
        },
        DescriptionEntry {
            description: "access tuple elements by index with `t[i]` (zero-based)",
            examples: &[
                "dec (int, string) p = (42, \"hello\")\nprintln(p[0])  // 42\nprintln(p[1])  // hello",
            ],
            kind: DescriptionKind::Explanation,
            title: None,
            expected_output: &[],
        },
        DescriptionEntry {
            description: "destructure a tuple into named bindings in one declaration",
            examples: &[
                "dec int x, string y = (10, \"world\")\nprintln(x)  // 10\nprintln(y)  // world",
                "dec int a, float b, bool c = (5, 2.5, true)",
            ],
            kind: DescriptionKind::Explanation,
            title: None,
            expected_output: &[],
        },
        DescriptionEntry {
            description: "constant tuples use CONST with the tuple type",
            examples: &["CONST (int, string) P = (0, \"origin\")"],
            kind: DescriptionKind::Explanation,
            title: None,
            expected_output: &[],
        },
        DescriptionEntry {
            description: "tuples can be used as array elements for homogeneous collections of structured data",
            examples: &["dec arr[(int, string)] rows = [(1, \"one\"), (2, \"two\")]"],
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
