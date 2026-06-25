use crate::docs::entry::{FnEntry, StdEntry};

pub static TYPES: StdEntry = StdEntry {
    name: "types",
    description: "functions for type checking and conversion",
    functions: FUNCTIONS,
};

static FUNCTIONS: &[&FnEntry] = &[
    &IS_BOOL, &IS_CHAR, &IS_FLOAT, &IS_INT, &IS_NULL, &IS_STRING, &TO_BIN, &TO_BOOL, &TO_CHAR,
    &TO_FLOAT, &TO_HEX, &TO_INT, &TO_OCT, &TO_STRING,
];

static IS_BOOL: FnEntry = FnEntry {
    signature: "is_bool(x)",
    description: "true if x is a bool",
    example: "get std::types::is_bool\n\nis_bool(true) // true",
};

static IS_CHAR: FnEntry = FnEntry {
    signature: "is_char(x)",
    description: "true if x is a char",
    example: "get std::types::is_char\n\nis_char('a') // true",
};

static IS_FLOAT: FnEntry = FnEntry {
    signature: "is_float(x)",
    description: "true if x is a float",
    example: "get std::types::is_float\n\nis_float(3.14) // true",
};

static IS_INT: FnEntry = FnEntry {
    signature: "is_int(x)",
    description: "true if x is an int",
    example: "get std::types::is_int\n\nis_int(42) // true",
};

static IS_NULL: FnEntry = FnEntry {
    signature: "is_null(x)",
    description: "true if x is null",
    example: "get std::types::is_null\n\nis_null(null) // true",
};

static IS_STRING: FnEntry = FnEntry {
    signature: "is_string(x)",
    description: "true if x is a string",
    example: "get std::types::is_string\n\nis_string(\"hi\") // true",
};

static TO_BIN: FnEntry = FnEntry {
    signature: "to_bin(x)",
    description: "converts int, char, or string to a binary string representation",
    example: "get std::types::to_bin\n\nto_bin(10) // \"1010\"",
};

static TO_BOOL: FnEntry = FnEntry {
    signature: "to_bool(x)",
    description: "converts int, float, string, or null to bool - 0/0.0/\"\"/null are false, everything else is true",
    example: "get std::types::to_bool\n\nto_bool(0) // false",
};

static TO_CHAR: FnEntry = FnEntry {
    signature: "to_char(x)",
    description: "converts an int (unicode codepoint) or single-character string to char",
    example: "get std::types::to_char\n\nto_char(65) // 'A'",
};

static TO_FLOAT: FnEntry = FnEntry {
    signature: "to_float(x)",
    description: "converts int, bool, or numeric string to float",
    example: "get std::types::to_float\n\nto_float(3) // 3.0",
};

static TO_HEX: FnEntry = FnEntry {
    signature: "to_hex(x)",
    description: "converts int, char, or string to a hexadecimal string representation",
    example: "get std::types::to_hex\n\nto_hex(255) // \"ff\"",
};

static TO_INT: FnEntry = FnEntry {
    signature: "to_int(x)",
    description: "converts float, bool, char, or string (including 0x hex strings) to int",
    example: "get std::types::to_int\n\nto_int(\"0xff\") // 255",
};

static TO_OCT: FnEntry = FnEntry {
    signature: "to_oct(x)",
    description: "converts int, char, or string to an octal string representation",
    example: "get std::types::to_oct\n\nto_oct(8) // \"10\"",
};

static TO_STRING: FnEntry = FnEntry {
    signature: "to_string(x)",
    description: "converts int, float, bool, or char to string",
    example: "get std::types::to_string\n\nto_string(42) // \"42\"",
};
