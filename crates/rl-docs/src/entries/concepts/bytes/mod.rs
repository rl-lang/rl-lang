use crate::entry::{ConceptCategory, ConceptEntry, DescriptionEntry, DescriptionKind};

pub static BYTES: ConceptEntry = ConceptEntry {
    name: "byte",
    summary: "byte: rl's unsigned 8-bit integer type (0-255), reached only through an explicit `as byte` cast since literals never default to it",
    category: ConceptCategory::Syntax,
    prerequisites: &["types"],
    descriptions: &[
        DescriptionEntry {
            kind: DescriptionKind::Explanation,
            title: Some("range"),
            description: "byte is an unsigned 8-bit integer, values from 0 to 255",
            examples: &["dec byte a = 10 as byte\ndec byte b = 255 as byte\nprintln(b)  // 255"],
            expected_output: &["255"],
        },
        DescriptionEntry {
            kind: DescriptionKind::Explanation,
            title: Some("literals are int by default"),
            description: "integer literals like `1`, `42`, `255` are typed `int` by default - use `<literal> as byte` to get a byte value explicitly",
            examples: &["dec byte x = 100 as byte\nprintln(x)  // 100"],
            expected_output: &["100"],
        },
        DescriptionEntry {
            kind: DescriptionKind::Explanation,
            title: Some("casting between byte, int, and float"),
            description: "use `as` to convert between byte, int, and float",
            examples: &[
                "dec byte b = 200 as byte\ndec int x = b as int\nprintln(x)  // 200, widens exactly",
                "dec int n = 1000\ndec byte w = n as byte\nprintln(w)  // 232, wraps",
                "dec float f = 3.9\ndec byte c = f as byte\nprintln(c)  // 3, truncates toward zero",
                "dec byte b2 = 255 as byte\ndec float f2 = b2 as float\nprintln(f2)  // 255",
            ],
            expected_output: &["200", "232", "3", "255"],
        },
        DescriptionEntry {
            kind: DescriptionKind::Pitfall,
            title: Some("an out-of-range literal cast to byte is a compile error, not a wrap"),
            description: "writing an out-of-range integer or float literal directly before `as byte` is rejected at compile time (`value 1000 is too large for byte`) rather than wrapping - the wrapping behavior only shows up when the value being cast is already stored in a variable rather than written as a bare literal",
            examples: &[
                "// dec byte bad = 1000 as byte  // error: value 1000 is too large for byte\ndec int n = 1000\ndec byte w = n as byte\nprintln(w)  // 232, wraps only through a variable",
            ],
            expected_output: &["232"],
        },
        DescriptionEntry {
            kind: DescriptionKind::Explanation,
            title: Some("as is the only way to narrow int or float down to byte"),
            description: "`as` is the only way to narrow int or float down to byte",
            examples: &[
                "dec int n = 42\ndec byte b = n as byte\nprintln(b)  // 42",
                "dec int n = 42\ndec float f = n as float\nprintln(f)  // 42",
            ],
            expected_output: &["42", "42"],
        },
        DescriptionEntry {
            kind: DescriptionKind::Syntax,
            title: Some("constant bytes"),
            description: "constant bytes use `CONST byte`",
            examples: &["CONST byte MAX = 255 as byte\nprintln(MAX)  // 255"],
            expected_output: &["255"],
        },
        DescriptionEntry {
            kind: DescriptionKind::Syntax,
            title: Some("arrays of bytes"),
            description: "arrays of bytes use `arr[byte]`",
            examples: &[
                "dec arr[byte] data = [0 as byte, 127 as byte, 255 as byte]\nprintln(data)  // [0, 127, 255]",
            ],
            expected_output: &["[0, 127, 255]"],
        },
        DescriptionEntry {
            kind: DescriptionKind::Note,
            title: Some("unary minus doesn't work on byte"),
            description: "byte is unsigned, so unary `-` doesn't accept it directly - cast to `int` first if a negative result is needed (see the `casting` concept)",
            examples: &[],
            expected_output: &[],
        },
    ],
    pitfalls: &[
        "integer/float literals are `int`/`float` by default, never `byte` - use `<literal> as byte` to get a byte value",
        "an out-of-range literal written directly before `as byte` (e.g. `1000 as byte`) is a compile-time error, not a wrapping conversion - wrapping only happens when casting a value that's already stored in a variable",
        "byte is unsigned, so unary `-` doesn't work on it directly - cast to `int` first (see the `casting` concept)",
    ],
    related: &["types", "casting", "operators"],
    related_stdlib: &["types"],
    since: Some("v0.1.5"),
};
