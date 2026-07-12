use crate::docs::entry::{ConceptCategory, ConceptEntry, DescriptionEntry, DescriptionKind};

pub static BYTES: ConceptEntry = ConceptEntry {
    name: "byte",
    descriptions: &[
        DescriptionEntry {
            description: "byte is an unsigned 8-bit integer, values from 0 to 255",
            examples: &["dec byte a = 10 as byte", "dec byte b = 255 as byte"],
            kind: DescriptionKind::Explanation,
            title: None,
            expected_output: &[],
        },
        DescriptionEntry {
            description: "integer literals like 1, 42, 255 are int by default explicit cast should be used",
            examples: &["dec byte x = 100 as byte  // 100 is a byte literal"],
            kind: DescriptionKind::Explanation,
            title: None,
            expected_output: &[],
        },
        DescriptionEntry {
            description: "use `as` to explicitly cast between byte, int, and float",
            examples: &[
                "dec int  x = 200 as int    // byte -> int",
                "dec byte b = 1000 as byte  // int  -> byte (wraps: 232)",
                "dec byte c = 3.9 as byte   // float -> byte (truncates: 3)",
                "dec float f = 255 as float // byte -> float (255.0)",
            ],
            kind: DescriptionKind::Explanation,
            title: None,
            expected_output: &[],
        },
        DescriptionEntry {
            description: "`as` is the only way to narrow int or float down to byte",
            examples: &[
                "dec int   n = 42\ndec byte  b = n as byte   // explicit narrow\ndec float f = n as float  // explicit widen",
            ],
            kind: DescriptionKind::Explanation,
            title: None,
            expected_output: &[],
        },
        DescriptionEntry {
            description: "constant bytes use CONST byte",
            examples: &["CONST byte MAX = 255 as byte"],
            kind: DescriptionKind::Explanation,
            title: None,
            expected_output: &[],
        },
        DescriptionEntry {
            description: "arrays of bytes use arr[byte]",
            examples: &["dec arr[byte] data = [0 as byte, 127 as byte, 255 as byte]"],
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
