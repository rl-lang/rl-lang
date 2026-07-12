use crate::entry::{ConceptCategory, ConceptEntry, DescriptionEntry, DescriptionKind};

pub static TYPES: ConceptEntry = ConceptEntry {
    name: "types",
    descriptions: &[
        DescriptionEntry {
            description: "rl is statically typed every variable has a declared type",
            examples: &[
                "dec int    x = 42",
                "dec float  y = 3.14",
                "dec bool   b = true",
                "dec string s = \"hello\"",
                "dec char   c = 'a'",
            ],
            kind: DescriptionKind::Explanation,
            title: None,
            expected_output: &[],
        },
        DescriptionEntry {
            description: "`int` is a 64-bit integer",
            examples: &["dec int x = 100\ndec int neg = -42"],
            kind: DescriptionKind::Explanation,
            title: None,
            expected_output: &[],
        },
        DescriptionEntry {
            description: "`float` is a 64-bit floating point number",
            examples: &["dec float pi = 3.14\ndec float neg = -0.5"],
            kind: DescriptionKind::Explanation,
            title: None,
            expected_output: &[],
        },
        DescriptionEntry {
            description: "`bool` is either `true` or `false`",
            examples: &["dec bool on = true\ndec bool off = false"],
            kind: DescriptionKind::Explanation,
            title: None,
            expected_output: &[],
        },
        DescriptionEntry {
            description: "`string` is a string enclosed in double quotes",
            examples: &["dec string name = \"Mohamed\"\ndec string empty = \"\""],
            kind: DescriptionKind::Explanation,
            title: None,
            expected_output: &[],
        },
        DescriptionEntry {
            description: "`char` is a single character enclosed in single quotes",
            examples: &["dec char letter = 'a'\ndec char digit  = '9'"],
            kind: DescriptionKind::Explanation,
            title: None,
            expected_output: &[],
        },
        DescriptionEntry {
            description: "byte is an unsigned 8-bit integer, integer literals are bytes by default and widen to int when needed",
            examples: &[
                "dec byte b = 255",
                "dec int  x = 10   // byte literal widens to int",
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
