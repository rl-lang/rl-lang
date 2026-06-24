use crate::docs::entry::{ConceptEntry, DescriptionEntry};

pub static ERROR_TYPE: ConceptEntry = ConceptEntry {
    name: "error",
    descriptions: &[
        DescriptionEntry {
            description: "error is a value that wraps any non-error value to signal failure",
            examples: &[
                "dec error e = error(404)",
                "dec error e = error(\"not found\")",
                "dec error e = error(false)",
            ],
        },
        DescriptionEntry {
            description: "check if a value is an error with `is_error` from std::types",
            examples: &[
                "get is_error from std::types\ndec error e = error(1)\nprintln(is_error(e))  // true\nprintln(is_error(42)) // false",
            ],
        },
        DescriptionEntry {
            description: "unwrap the inner value of an error with `error_unwrap` from std::types",
            examples: &[
                "get error_unwrap from std::types\ndec error e = error(404)\ndec int code = error_unwrap(e)\nprintln(code)  // 404",
            ],
        },
        DescriptionEntry {
            description: "error cannot wrap another error",
            examples: &["// error(error(1))  // runtime error: error cannot wrap another error"],
        },
        DescriptionEntry {
            description: "functions can return error to signal failure to callers",
            examples: &[
                "fn divide(int a, int b) -> error {\n    if b == 0 {\n        return error(\"division by zero\")\n    }\n    return error(a / b)\n}",
            ],
        },
    ],
};
