use crate::docs::entry::{ConceptCategory, ConceptEntry, DescriptionEntry, DescriptionKind};

pub static RECORDS: ConceptEntry = ConceptEntry {
    name: "records",
    summary: "a named, fixed-shape collection of typed fields, declared once with `record`",
    descriptions: &[
        DescriptionEntry {
            kind: DescriptionKind::Syntax,
            title: None,
            description: "declare a record with `record Name { Type field, Type field }`, using the same `Type name` order as function parameters; a trailing comma after the last field is allowed",
            examples: &["record Point {\n    int x,\n    int y,\n}"],
            expected_output: &[],
        },
        DescriptionEntry {
            kind: DescriptionKind::Syntax,
            title: Some("constructing a value"),
            description: "build an instance with `Name { field: value, ... }`; fields can be given in any order but every field must be present, exactly once, with a value matching its declared type",
            examples: &[
                "record Point {\n    int x,\n    int y,\n}\n\ndec Point p = Point { x: 1, y: 2 }\ndec Point q = Point { y: 4, x: 3 }",
            ],
            expected_output: &[],
        },
        DescriptionEntry {
            kind: DescriptionKind::Explanation,
            title: Some("reading and writing fields"),
            description: "access a field with `.field`, and write to it with `.field = value`; field assignment is an expression that evaluates to the assigned value, just like a variable assignment",
            examples: &[
                "record Point {\n    int x,\n    int y,\n}\n\ndec Point p = Point { x: 1, y: 2 }\nprintln(p.x)  // 1\np.x = 10\nprintln(p.x)  // 10",
            ],
            expected_output: &[],
        },
        DescriptionEntry {
            kind: DescriptionKind::Explanation,
            title: Some("records as types"),
            description: "a record name is a type once declared: use it for `dec` bindings, function parameters and return types, or as an array element type",
            examples: &[
                "record Point {\n    int x,\n    int y,\n}\n\nfn manhattan(Point p) -> int {\n    return p.x + p.y\n}\n\ndec arr[Point] path = [Point { x: 0, y: 0 }, Point { x: 1, y: 1 }]",
            ],
            expected_output: &[],
        },
        DescriptionEntry {
            kind: DescriptionKind::Pitfall,
            title: Some("declare before use"),
            description: "a record must be declared earlier in the file than any place that uses it as a literal, a type annotation, or a field type; the parser reads top to bottom and only recognizes `Name { ... }` as a record literal once it has already seen `record Name { ... }`, so there's no forward-referencing between records or from earlier code to a later declaration",
            examples: &[],
            expected_output: &[],
        },
        DescriptionEntry {
            kind: DescriptionKind::Pitfall,
            title: Some("records are shared, not copied"),
            description: "assigning a record value to another variable, or passing it into a function, does not copy its fields - both names refer to the same underlying data, so mutating a field through one is visible through the other; this is different from arrays, which are copied by value",
            examples: &[
                "record Point {\n    int x,\n    int y,\n}\n\ndec Point p = Point { x: 1, y: 1 }\ndec Point alias = p\nalias.x = 99\nprintln(p.x)  // 99, not 1",
            ],
            expected_output: &[],
        },
    ],
    category: ConceptCategory::Types,
    prerequisites: &["types"],
    pitfalls: &[
        "records must be declared before any use, including as a type in an earlier function's signature",
        "constructing a record requires every declared field, no more and no less; missing or extra fields are a runtime error",
        "records have reference semantics - copies alias the same fields, unlike arrays",
    ],
    related: &["types", "tags", "arrays", "functions"],
    related_stdlib: &[],
    since: Some("v0.1.5"),
};
