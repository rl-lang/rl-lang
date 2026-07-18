use crate::entry::{ConceptCategory, ConceptEntry, DescriptionEntry, DescriptionKind};

pub static IMPORTS: ConceptEntry = ConceptEntry {
    name: "imports",
    summary: "`get` brings stdlib functions or local files into scope - a single fully-qualified stdlib path, a `from`-list for one or more functions, or a bare/`from`-qualified local filename resolved relative to the importing file",
    category: ConceptCategory::Modules,
    prerequisites: &["functions"],
    descriptions: &[
        DescriptionEntry {
            kind: DescriptionKind::Syntax,
            title: Some("importing a single stdlib function"),
            description: "import a single stdlib function with `get std::<module>::<function>`",
            examples: &["get std::math::sqrt\n\nsqrt(9.0)  // 3.0"],
            expected_output: &[],
        },
        DescriptionEntry {
            kind: DescriptionKind::Syntax,
            title: Some("importing multiple stdlib functions"),
            description: "import multiple stdlib functions with `get <fn1>, <fn2> from std::<module>`",
            examples: &["get sin, cos from std::math\n\nsin(0.0)  // 0.0\ncos(0.0)  // 1.0"],
            expected_output: &[],
        },
        DescriptionEntry {
            kind: DescriptionKind::Syntax,
            title: Some("importing a local file"),
            description: "import a local file with `get <filename>` and loads `<filename>.rl` from the same directory",
            examples: &["get utils\n// loads utils.rl"],
            expected_output: &[],
        },
        DescriptionEntry {
            kind: DescriptionKind::Syntax,
            title: Some("importing named items from a local file"),
            description: "import named items from a local file with `get <fn> from <path>::<file>`",
            examples: &["get add from math::utils\n// imports add from math/utils.rl"],
            expected_output: &[],
        },
        DescriptionEntry {
            kind: DescriptionKind::Pitfall,
            title: Some("the direct-path form only takes one function"),
            description: "`get std::<module>::<function>` names exactly one function - it can't list several the way the `from` form can; to import more than one function from the same stdlib module, switch to `get <fn1>, <fn2> from std::<module>` (which also works fine for a single function)",
            examples: &[
                "get std::io::print  // ok, one function\n// get std::io::print, println  // not valid - use the form below instead\nget print from std::io  // also valid for just one function",
            ],
            expected_output: &[],
        },
        DescriptionEntry {
            kind: DescriptionKind::Pitfall,
            title: Some("a bare local import pulls in the whole file"),
            description: "`get <filename>` brings in everything that file exposes, with no way to cherry-pick - to import only specific items from a local file, use `get <item> from <path>::<file>` instead",
            examples: &[
                "get utils           // everything utils.rl exposes\nget add from utils   // just add, from utils.rl",
            ],
            expected_output: &[],
        },
    ],
    pitfalls: &[
        "the direct-path form (`get std::<module>::<function>`) only names a single function - importing more than one from the same module needs the `from` form instead, which also works for just one",
        "a bare `get <filename>` imports everything the file exposes, with no way to select individual items - use `get <item> from <path>::<file>` to cherry-pick from a local file",
    ],
    related: &["functions", "tooling"],
    related_stdlib: &[],
    since: Some("v0.1.5"),
};
