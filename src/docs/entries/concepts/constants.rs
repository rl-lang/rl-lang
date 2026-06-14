use crate::docs::entry::{ConceptEntry, DescriptionEntry};

pub static CONSTANTS: ConceptEntry = ConceptEntry {
    name: "constants",
    descriptions: &[
        DescriptionEntry {
            description: "declare a constant with `CONST <type> <name> = <value>` but it cannot be reassigned, convention is UPPER_CASE (but anything works)",
            examples: &[
                "CONST int    MAX_SIZE  = 100",
                "CONST float  EULER     = 2.71828",
                "CONST bool   DEBUG     = false",
                "CONST string LANG      = \"rl\"",
                "CONST char   NEWLINE   = '\\n'",
            ],
        },
        DescriptionEntry {
            description: "constant arrays use `CONST arr[<type>]`",
            examples: &[
                "CONST arr[int]    PRIMES = [2, 3, 5, 7, 11]",
                "CONST arr[string] DAYS   = [\"sat\", \"sun\", \"mon\"]",
            ],
        },
    ],
};
