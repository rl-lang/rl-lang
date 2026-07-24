use crate::entry::{ConceptCategory, ConceptEntry, DescriptionEntry, DescriptionKind};

pub static IMPL_BLOCKS: ConceptEntry = ConceptEntry {
    name: "impl blocks",
    summary: "attaches functions to a `record` with `impl Name { fn ... }`, either as instance methods called via `value.method(args)` or associated functions called via `Name::method(args)`",
    category: ConceptCategory::Functions,
    prerequisites: &["records", "functions"],
    descriptions: &[
        DescriptionEntry {
            kind: DescriptionKind::Syntax,
            title: Some("declaring an impl block"),
            description: "attach functions to a record with `impl Name { fn ... }`; each `fn` inside is declared exactly like a normal function, and a record can have more than one `impl` block",
            examples: &[
                "get sqrt from std::math\n\nrecord Point {\n    int x,\n    int y,\n}\n\nimpl Point {\n    fn new(int x, int y) -> Point {\n        return Point { x: x, y: y }\n    }\n\n    fn magnitude(self) -> float {\n        return sqrt((self.x * self.x + self.y * self.y) as float)\n    }\n}",
            ],
            expected_output: &[],
        },
        DescriptionEntry {
            kind: DescriptionKind::Explanation,
            title: Some("instance methods take a bare `self`"),
            description: "a method whose first parameter is a bare `self` (no type prefix) is an instance method - `self` is implicitly typed as the enclosing record, and the method is called on a value with `.method(args)`, the same call syntax used for stdlib methods like `nums.arr_push(x)`",
            examples: &[
                "get sqrt from std::math\n\nrecord Point {\n    int x,\n    int y,\n}\n\nimpl Point {\n    fn magnitude(self) -> float {\n        return sqrt((self.x * self.x + self.y * self.y) as float)\n    }\n}\n\ndec Point p = Point { x: 3, y: 4 }\nprintln(p.magnitude())  // 5",
            ],
            expected_output: &["5"],
        },
        DescriptionEntry {
            kind: DescriptionKind::Explanation,
            title: Some("associated functions have no `self`"),
            description: "a method without a `self` parameter is an associated function, called on the record's name with `Name::method(args)` rather than on a value - this is the usual way to write a constructor",
            examples: &[
                "record Point {\n    int x,\n    int y,\n}\n\nimpl Point {\n    fn new(int x, int y) -> Point {\n        return Point { x: x, y: y }\n    }\n}\n\ndec Point origin = Point::new(0, 0)",
            ],
            expected_output: &[],
        },
        DescriptionEntry {
            kind: DescriptionKind::Explanation,
            title: Some("everything else is a normal function"),
            description: "aside from the optional bare `self`, a method's remaining parameters, return type, and body are parsed and behave exactly like a normal function declaration - a method can take further parameters, return any type, and call other functions or methods",
            examples: &[
                "record Point {\n    int x,\n    int y,\n}\n\nimpl Point {\n    fn new(int x, int y) -> Point {\n        return Point { x: x, y: y }\n    }\n\n    fn scaled(self, int factor) -> Point {\n        return Point::new(self.x * factor, self.y * factor)\n    }\n}\n\ndec Point p = Point::new(1, 2)\ndec Point doubled = p.scaled(2)\nprintln(doubled.x)  // 2",
            ],
            expected_output: &["2"],
        },
        DescriptionEntry {
            kind: DescriptionKind::Pitfall,
            title: Some("methods are dispatched by name, not by lexical scope"),
            description: "a method is looked up by `\"Name::method\"` at call time rather than resolved as a regular variable reference, so it doesn't occupy a slot in the surrounding scope the way a `fn` declaration does - it can't be shadowed, reassigned, or passed around as a plain `fn`-typed value the way a free function can",
            examples: &[],
            expected_output: &[],
        },
        DescriptionEntry {
            kind: DescriptionKind::Pitfall,
            title: Some("no field access checking against undeclared fields"),
            description: "a method body is only checked once it's called, and `self.field` follows the same rules as any other field access - reading or writing a field that the record doesn't declare is still a runtime error, not something caught by declaring the `impl` block itself",
            examples: &[],
            expected_output: &[],
        },
        DescriptionEntry {
            kind: DescriptionKind::Note,
            title: Some("multiple impl blocks and redeclared methods"),
            description: "a record's methods can be split across more than one `impl Name { ... }` block; if the same method name is declared twice for the same record, the later declaration replaces the earlier one",
            examples: &[],
            expected_output: &[],
        },
    ],
    pitfalls: &[
        "instance methods need a bare, untyped `self` as the first parameter - typing it explicitly (e.g. `Point self`) makes it a normal parameter instead, and the method loses `.method(args)` call syntax",
        "methods are looked up by name at the call site (`\"Name::method\"`), not resolved lexically - they can't be shadowed, stored in a `fn`-typed variable, or passed as a callback the way a free function can",
        "declaring the same method name twice for the same record silently replaces the earlier one, across one `impl` block or several",
    ],
    related: &["records", "functions", "types"],
    related_stdlib: &[],
    since: Some("v0.4.0"),
};
