use crate::entry::{ConceptCategory, ConceptEntry, DescriptionEntry, DescriptionKind};

pub static CASTING: ConceptEntry = ConceptEntry {
    name: "casting",
    summary: "`value as type` explicitly converts between rl's three numeric types - byte, int, and float - each with its own overflow behavior",
    category: ConceptCategory::Syntax,
    prerequisites: &["types"],
    descriptions: &[
        DescriptionEntry {
            kind: DescriptionKind::Syntax,
            title: Some("syntax"),
            description: "`value as type` explicitly converts between numeric types: byte, int, and float",
            examples: &[
                "println(42 as int)      // 42",
                "println(42 as float)    // 42",
                "println(200 as byte)    // 200",
                "println(3.9 as int)     // 3",
            ],
            expected_output: &["42", "42", "200", "3"],
        },
        DescriptionEntry {
            kind: DescriptionKind::Pitfall,
            title: Some("int to byte wraps"),
            description: "int to byte wraps on overflow (same as Rust `as u8`)",
            examples: &[
                "println(256 as byte)  // 0  (wraps)",
                "println(300 as byte)  // 44 (300 - 256)",
            ],
            expected_output: &["0", "44"],
        },
        DescriptionEntry {
            kind: DescriptionKind::Explanation,
            title: Some("float to int/byte truncates toward zero"),
            description: "float to int or float to byte drops the fractional part toward zero for in-range values",
            examples: &["println(3.9 as int)   // 3", "println(-2.7 as int)  // -2"],
            expected_output: &["3", "-2"],
        },
        DescriptionEntry {
            kind: DescriptionKind::Pitfall,
            title: Some("out-of-range float casts saturate, they don't wrap"),
            description: "unlike int to byte, a float that's outside the target's range doesn't wrap when cast to int or byte - it saturates to the target type's min or max instead, and `NaN` becomes 0",
            examples: &[
                "println(1000.0 as byte)  // 255, not a wrapped value",
                "println(-5.0 as byte)    // 0",
            ],
            expected_output: &["255", "0"],
        },
        DescriptionEntry {
            kind: DescriptionKind::Pitfall,
            title: Some("byte can't be negated directly"),
            description: "byte is unsigned, so unary `-` doesn't work on it - cast to `int` first if a negative result is needed",
            examples: &["dec byte b = 5\ndec int neg = -(b as int)\nprintln(neg)  // -5"],
            expected_output: &["-5"],
        },
        DescriptionEntry {
            kind: DescriptionKind::Pitfall,
            title: Some("as is numeric-only"),
            description: "`as` only converts between byte, int, and float - there's no `as` conversion to or from bool, char, string, or null; use the `std::types` conversion functions (`to_bool`, `to_string`, `to_int`, ...) for those instead, and note that casting a `null` value is itself a compile-time error",
            examples: &["// dec string s = 5 as string  // error: invalid cast"],
            expected_output: &[],
        },
    ],
    pitfalls: &[
        "int to byte wraps on overflow (same as Rust's `as u8`)",
        "an out-of-range float cast to int or byte saturates to the target's min/max instead of wrapping, and `NaN` becomes 0",
        "byte is unsigned, so unary `-` doesn't work on it directly - cast to `int` first",
        "`as` only converts between byte, int, and float - there's no cast to/from bool, char, string, or null; use `std::types` conversion functions for those instead",
    ],
    related: &["types", "byte", "operators"],
    related_stdlib: &["types"],
    since: Some("v0.1.5"),
};
