use crate::docs::entry::{ConceptEntry, DescriptionEntry};

pub static CASTING: ConceptEntry = ConceptEntry {
    name: "casting",
    descriptions: &[
        DescriptionEntry {
            description: "`value as type` explicitly converts between numeric types: byte, int, and float",
            examples: &[
                "dec int   n = 42 as int     // byte literal -> int",
                "dec float f = 42 as float   // byte literal -> float",
                "dec byte  b = 200 as byte   // int -> byte",
                "dec int   i = 3.9 as int    // float -> int (truncates: 3)",
            ],
        },
        DescriptionEntry {
            description: "int to byte wraps on overflow (same as Rust `as u8`)",
            examples: &[
                "dec byte b = 256 as byte  // 0  (wraps)",
                "dec byte c = 300 as byte  // 44 (300 - 256)",
            ],
        },
        DescriptionEntry {
            description: "float to int truncates toward zero",
            examples: &[
                "dec int a = 3.9 as int   // 3",
                "dec int b = -2.7 as int  // -2",
            ],
        },
    ],
};
