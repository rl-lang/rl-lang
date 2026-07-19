use crate::entry::{ConceptCategory, ConceptEntry, DescriptionEntry, DescriptionKind};

pub static ADV_CSV_MUTATION: ConceptEntry = ConceptEntry {
    name: "7. mutating CSV data",
    summary: "mutating CSV data",
    category: ConceptCategory::Tooling,
    prerequisites: &[],
    descriptions: &[
        DescriptionEntry {
            kind: DescriptionKind::Explanation,
            title: None,
            description: "arrays in rl are values - when you filter or map you get a new array, the original is unchanged. mutation means building a new array with the change applied. this is the same pattern arr_push uses",
            examples: &[
                "get arr_map from std::array\n\n// update a field in one row, return a new rows array\nfn csv_update_field(arr[arr[string]] rows, string id, int col, string value) -> arr[arr[string]] {\n    return arr_map(rows, fn(arr[string] row) -> arr[string] {\n        if (row[COL_ID] != id) { return row }\n        // build updated row\n        dec arr[string] updated = row\n        updated[col] = value\n        return updated\n    })?\n}",
            ],
            expected_output: &[],
        },
        DescriptionEntry {
            kind: DescriptionKind::Explanation,
            title: None,
            description: "deleting a row means filtering it out. arr_filter with the opposite condition gives you every row except the one you want gone",
            examples: &[
                "get arr_filter from std::array\n\nfn csv_remove_by_id(arr[arr[string]] rows, string id) -> arr[arr[string]] {\n    return arr_filter(rows, fn(arr[string] row) -> bool {\n        return row[COL_ID] != id\n    })?\n}",
            ],
            expected_output: &[],
        },
        DescriptionEntry {
            kind: DescriptionKind::Explanation,
            title: None,
            description: "adding a row means generating a new ID first. the simplest approach: find the current max ID and add 1. arr_reduce works well here",
            examples: &[
                "get arr_reduce        from std::array\nget to_int, to_string from std::types\n\nfn csv_next_id(arr[arr[string]] rows) -> string {\n    if (len(rows) == 0) { return \"1\" }\n    dec int max_id = arr_reduce(\n        rows,\n        fn(int acc, arr[string] row) -> int {\n            dec int id = to_int(row[COL_ID])?\n            if (id > acc) { return id }\n            return acc\n        },\n        0\n    )?\n    return to_string(max_id + 1)?\n}",
            ],
            expected_output: &[],
        },
        DescriptionEntry {
            kind: DescriptionKind::Explanation,
            title: None,
            description: "adding a row is now easy: generate the id with csv_next_id, build the full row, and arr_push it on",
            examples: &[
                "get arr_push from std::array\n\nfn csv_add_row(arr[arr[string]] rows, arr[string] fields) -> arr[arr[string]] {\n    return arr_push(rows, fields)?\n}",
            ],
            expected_output: &[],
        },
        DescriptionEntry {
            kind: DescriptionKind::Explanation,
            title: None,
            description: "exercise: add these mutation functions to csv.rl:\n  csv_add_row(rows, fields)           -> arr[arr[string]]\n  csv_remove_by_id(rows, id)          -> arr[arr[string]]\n  csv_update_field(rows, id, col, val)-> arr[arr[string]]\n  csv_next_id(rows)                   -> string",
            examples: &[
                "// test in main.rl\ndec arr[arr[string]] rows = []\n\n// add\nrows = csv_add_row(rows, [csv_next_id(rows), \"pending\", \"1750000000\", \"buy milk\"])\nrows = csv_add_row(rows, [csv_next_id(rows), \"pending\", \"1750000100\", \"write code\"])\nprintln(len(rows)) // 2\n\n// update\nrows = csv_update_field(rows, \"1\", COL_STATUS, \"done\")\nprintln(csv_find_by_id(rows, \"1\")[COL_STATUS]) // done\n\n// remove\nrows = csv_remove_by_id(rows, \"1\")\nprintln(len(rows)) // 1",
            ],
            expected_output: &[],
        },
    ],
    pitfalls: &[],
    related: &[],
    related_stdlib: &[],
    since: None,
};
