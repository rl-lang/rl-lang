use crate::docs::entry::{ConceptEntry, DescriptionEntry};

pub static BYTES: ConceptEntry = ConceptEntry {
    name: "byte",
    descriptions: &[
        DescriptionEntry {
            description: "byte is an unsigned 8-bit integer, values from 0 to 255",
            examples: &["dec byte a = 10", "dec byte b = 255"],
        },
        DescriptionEntry {
            description: "integer literals like 1, 42, 255 are bytes by default",
            examples: &["dec byte x = 100  // 100 is a byte literal"],
        },
        DescriptionEntry {
            description: "byte widens to int automatically in assignments, array elements, and arithmetic",
            examples: &[
                "dec int x = 10    // byte literal 10 widens to int",
                "dec arr[int] nums = [1, 2, 3]  // byte literals widen to int",
            ],
        },
        DescriptionEntry {
            description: "arithmetic between two bytes returns byte, mixing byte and int returns int",
            examples: &[
                "dec byte a = 1 + 2   // byte + byte = byte (3)",
                "dec int  b = 1 + 300 // byte + int  = int  (301)",
            ],
        },
        DescriptionEntry {
            description: "constant bytes use CONST byte",
            examples: &["CONST byte MAX = 255"],
        },
        DescriptionEntry {
            description: "arrays of bytes use arr[byte]",
            examples: &["dec arr[byte] data = [0, 127, 255]"],
        },
    ],
};
