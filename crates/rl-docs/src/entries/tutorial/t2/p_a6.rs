use crate::docs::entry::{ConceptCategory, ConceptEntry, DescriptionEntry, DescriptionKind};

pub static ADV_CSV_QUERY: ConceptEntry = ConceptEntry {
    name: "6. querying CSV data",
    summary: "querying CSV data",
    category: ConceptCategory::Tooling,
    prerequisites: &[],
    descriptions: &[
        DescriptionEntry {
            kind: DescriptionKind::Explanation,
            title: None,
            description: "raw rows are just arrays of strings. to query them usefully you need helper functions that know which column index means what. define column constants so you never use magic numbers",
            examples: &[
                "// csv.rl\nCONST int COL_ID         = 0\nCONST int COL_STATUS     = 1\nCONST int COL_CREATED_AT = 2\nCONST int COL_TEXT       = 3\n\n// now instead of row[1] you write row[COL_STATUS]\n// readable and safe if columns ever change",
            ],
            expected_output: &[],
        },
        DescriptionEntry {
            kind: DescriptionKind::Explanation,
            title: None,
            description: "arr_filter with a lambda is how you query rows. the lambda receives a row and returns true if it matches. this is the pattern you will use for every filtered view",
            examples: &[
                "get arr_filter from std::array\n\n// get all pending tasks\ndec arr[arr[string]] pending = arr_filter(rows, fn(arr[string] row) -> bool {\n    return row[COL_STATUS] == \"pending\"\n})\n\n// get all done tasks\ndec arr[arr[string]] done = arr_filter(rows, fn(arr[string] row) -> bool {\n    return row[COL_STATUS] == \"done\"\n})",
            ],
            expected_output: &[],
        },
        DescriptionEntry {
            kind: DescriptionKind::Explanation,
            title: None,
            description: "arr_find lets you locate a single row by id. it returns the first matching row or null if nothing matches - always check with is_null before using the result",
            examples: &[
                "get arr_find  from std::array\nget is_null   from std::types\n\nfn csv_find_by_id(arr[arr[string]] rows, string id) -> arr[string] {\n    return arr_find(rows, fn(arr[string] row) -> bool {\n        return row[COL_ID] == id\n    })\n}\n\ndec arr[string] task = csv_find_by_id(rows, \"2\")\nif (is_null(task)) {\n    println(\"not found\")\n} else {\n    println(task[COL_TEXT])\n}",
            ],
            expected_output: &[],
        },
        DescriptionEntry {
            kind: DescriptionKind::Explanation,
            title: None,
            description: "exercise: add these query functions to csv.rl:\n  csv_filter_by(rows, col, value) -> arr[arr[string]]\n  csv_find_by_id(rows, id)        -> arr[string]\n\nthen test them in main.rl against a hardcoded set of rows",
            examples: &[
                "// csv.rl\nget arr_filter, arr_find from std::array\n\nfn csv_filter_by(arr[arr[string]] rows, int col, string value) -> arr[arr[string]] {\n    return arr_filter(rows, fn(arr[string] row) -> bool {\n        return row[col] == value\n    })\n}\n\nfn csv_find_by_id(arr[arr[string]] rows, string id) -> arr[string] {\n    return arr_find(rows, fn(arr[string] row) -> bool {\n        return row[COL_ID] == id\n    })\n}\n\n// main.rl test\ndec arr[arr[string]] rows = [\n    [\"1\", \"pending\", \"1750000000\", \"buy milk\"],\n    [\"2\", \"done\",    \"1750000100\", \"write code\"],\n    [\"3\", \"pending\", \"1750000200\", \"fix bug\"],\n]\n\ndec arr[arr[string]] pending = csv_filter_by(rows, COL_STATUS, \"pending\")\nprintln(len(pending)) // 2\n\ndec arr[string] task = csv_find_by_id(rows, \"2\")\nprintln(task[COL_TEXT]) // write code",
            ],
            expected_output: &[],
        },
    ],
    pitfalls: &[],
    related: &[],
    related_stdlib: &[],
    since: None,
};
