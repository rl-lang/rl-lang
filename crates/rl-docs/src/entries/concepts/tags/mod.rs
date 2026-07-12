use crate::entry::{ConceptCategory, ConceptEntry, DescriptionEntry, DescriptionKind};

pub static TAGS: ConceptEntry = ConceptEntry {
    name: "tags",
    summary: "a named set of unit variants, declared once with `tag`, for representing one-of-a-kind states",
    descriptions: &[
        DescriptionEntry {
            kind: DescriptionKind::Syntax,
            title: None,
            description: "declare a tag with `tag Name { VariantA, VariantB, ... }`, a comma-separated list of variant names; a trailing comma after the last variant is allowed",
            examples: &["tag Color {\n    Red,\n    Green,\n    Blue,\n}"],
            expected_output: &[],
        },
        DescriptionEntry {
            kind: DescriptionKind::Syntax,
            title: Some("referencing a variant"),
            description: "refer to a variant with `Name.Variant`; tags don't carry any associated data, so a variant reference is a complete value on its own",
            examples: &[
                "tag Color {\n    Red,\n    Green,\n    Blue,\n}\n\ndec Color c = Color.Green\nprintln(c)  // Green",
            ],
            expected_output: &[],
        },
        DescriptionEntry {
            kind: DescriptionKind::Explanation,
            title: Some("comparing variants"),
            description: "variants compare with `==` and `!=`; two variants are equal only if they belong to the same tag and are the same variant",
            examples: &[
                "tag Color {\n    Red,\n    Green,\n    Blue,\n}\n\ndec Color c = Color.Red\nif c == Color.Red {\n    println(\"stop\")\n}",
            ],
            expected_output: &[],
        },
        DescriptionEntry {
            kind: DescriptionKind::Explanation,
            title: Some("tags with match"),
            description: "since a variant reference is just a value, `match` can branch on it like any other literal, with `_` as the catch-all arm",
            examples: &[
                "tag Color {\n    Red,\n    Green,\n    Blue,\n}\n\ndec Color c = Color.Green\n\nmatch c {\n    Color.Red => { println(\"stop\") }\n    Color.Green => { println(\"go\") }\n    _ => { println(\"caution\") }\n}",
            ],
            expected_output: &[],
        },
        DescriptionEntry {
            kind: DescriptionKind::Explanation,
            title: Some("tags as types"),
            description: "a tag name is a type once declared: use it for `dec` bindings, function parameters and return types, or as an array element type",
            examples: &[
                "tag Color {\n    Red,\n    Green,\n    Blue,\n}\n\nfn describe(Color c) -> string {\n    match c {\n        Color.Red => { return \"stop\" }\n        Color.Green => { return \"go\" }\n        _ => { return \"caution\" }\n    }\n}",
            ],
            expected_output: &[],
        },
        DescriptionEntry {
            kind: DescriptionKind::Pitfall,
            title: Some("declare before use"),
            description: "a tag must be declared earlier in the file than any place that uses it, including `Name.Variant` references and type annotations; the parser only recognizes `Name.Variant` as a tag access once it has already seen `tag Name { ... }` earlier in the file",
            examples: &[],
            expected_output: &[],
        },
        DescriptionEntry {
            kind: DescriptionKind::Pitfall,
            title: Some("no associated data"),
            description: "unlike enums in some other languages, a tag variant can't carry a value (no `Shape.Circle(radius)`); if a variant needs data attached, pair it with a record field instead of trying to store it on the tag itself",
            examples: &[],
            expected_output: &[],
        },
    ],
    category: ConceptCategory::Types,
    prerequisites: &["types"],
    pitfalls: &[
        "tags must be declared before any use, including in an earlier function's signature",
        "variants carry no data - they're pure labels",
        "a variant only equals another variant of the exact same tag and name",
    ],
    related: &["types", "records", "match"],
    related_stdlib: &[],
    since: Some("v0.1.5"),
};
