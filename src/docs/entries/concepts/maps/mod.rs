use crate::docs::entry::{ConceptCategory, ConceptEntry, DescriptionEntry, DescriptionKind};

pub static MAPS: ConceptEntry = ConceptEntry {
    name: "maps",
    summary: "",
    category: ConceptCategory::Syntax,
    prerequisites: &["arrays"],
    descriptions: &[
        DescriptionEntry {
            kind: DescriptionKind::Explanation,
            title: None,
            description: "declare a mutable map with `dec map[<key type>, <value type>] <n> = {<key>: <value>, ...}`",
            examples: &[
                "dec map[string, int] scores = {\"alice\": 95, \"bob\": 82}",
                "dec map[int, string] names = {1: \"one\", 2: \"two\"}",
                "dec map[string, bool] flags = {\"dark_mode\": true}",
            ],
            expected_output: &[],
        },
        DescriptionEntry {
            kind: DescriptionKind::Explanation,
            title: None,
            description: "read a value by key with `map[key]`",
            examples: &[
                "dec map[string, int] scores = {\"alice\": 95, \"bob\": 82}\nprintln(scores[\"alice\"])  // 95",
            ],
            expected_output: &[],
        },
        DescriptionEntry {
            kind: DescriptionKind::Explanation,
            title: None,
            description: "write or update a value by key with `map[key] = value` - this both inserts new keys and overwrites existing ones",
            examples: &[
                "dec map[string, int] scores = {\"alice\": 95}\nscores[\"bob\"] = 82  // insert\nscores[\"alice\"] = 100 // overwrite",
            ],
            expected_output: &[],
        },
        DescriptionEntry {
            kind: DescriptionKind::Explanation,
            title: None,
            description: "maps are typed like arrays: every key must share one type, and every value must share one type",
            examples: &[
                "dec map[string, int] scores = {\"alice\": 95}\n// scores[\"bob\"] = \"not a number\"  // error: type mismatch",
            ],
            expected_output: &[],
        },
        DescriptionEntry {
            kind: DescriptionKind::Explanation,
            title: None,
            description: "only int, string, bool, byte, and char can be used as key types - float keys are rejected, since floating point equality is unreliable for lookup",
            examples: &[
                "// dec map[float, int] bad = {1.5: 1}  // error: float cannot be used as a map key",
            ],
            expected_output: &[],
        },
        DescriptionEntry {
            kind: DescriptionKind::Explanation,
            title: None,
            description: "map values can be any type, including arrays or other maps",
            examples: &[
                "dec map[string, array[int]] rosters = {\"team_a\": [1, 2, 3]}\ndec map[string, map[string, int]] nested = {\"round1\": {\"alice\": 10}}",
            ],
            expected_output: &[],
        },
        DescriptionEntry {
            kind: DescriptionKind::Pitfall,
            title: None,
            description: "reading a key that doesn't exist is a runtime error, not `null` - there is no `map_get`-with-default yet, so check with a lookup helper once the map stdlib module exists, or guard with a `match`/`if`",
            examples: &[],
            expected_output: &[],
        },
        DescriptionEntry {
            kind: DescriptionKind::Note,
            title: None,
            description: "there's no iteration syntax for maps yet (no `for key, value in map`), and no stdlib module (`map_get`, `map_keys`, `map_values`, `map_has`, `map_remove`) - only literal construction and single-level `map[key]` read/write are currently supported",
            examples: &[],
            expected_output: &[],
        },
    ],
    pitfalls: &[
        "assigning through more than one level of nesting, e.g. `arr[0][\"key\"] = value` where the outer container is an array of maps, is not yet supported - only a bare `map[key] = value` works",
    ],
    related: &["arrays", "types"],
    related_stdlib: &[],
    since: Some("v0.1.5"),
};
