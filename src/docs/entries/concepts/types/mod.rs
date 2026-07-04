use crate::docs::entry::{ConceptEntry, DescriptionEntry};

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
        },
        DescriptionEntry {
            description: "`int` is a 64-bit integer",
            examples: &["dec int x = 100\ndec int neg = -42"],
        },
        DescriptionEntry {
            description: "`float` is a 64-bit floating point number",
            examples: &["dec float pi = 3.14\ndec float neg = -0.5"],
        },
        DescriptionEntry {
            description: "`bool` is either `true` or `false`",
            examples: &["dec bool on = true\ndec bool off = false"],
        },
        DescriptionEntry {
            description: "`string` is a string enclosed in double quotes",
            examples: &["dec string name = \"Mohamed\"\ndec string empty = \"\""],
        },
        DescriptionEntry {
            description: "`char` is a single character enclosed in single quotes",
            examples: &["dec char letter = 'a'\ndec char digit  = '9'"],
        },
        DescriptionEntry {
            description: "byte is an unsigned 8-bit integer, integer literals are bytes by default and widen to int when needed",
            examples: &[
                "dec byte b = 255",
                "dec int  x = 10   // byte literal widens to int",
            ],
        },
    ],
};
