use crate::docs::entry::{ConceptEntry, DescriptionEntry};

pub static IMPORTS: ConceptEntry = ConceptEntry {
    name: "imports",
    descriptions: &[
        DescriptionEntry {
            description: "import a single stdlib function with `get std::<module>::<function>`",
            examples: &["get std::math::sqrt\n\nsqrt(9.0)  // 3.0"],
        },
        DescriptionEntry {
            description: "import multiple stdlib functions with `get <fn1>, <fn2> from std::<module>`",
            examples: &["get sin, cos from std::math\n\nsin(0.0)  // 0.0\ncos(0.0)  // 1.0"],
        },
        DescriptionEntry {
            description: "import a local file with `get <filename>` and loads `<filename>.rl` from the same directory",
            examples: &["get utils\n// loads utils.rl"],
        },
        DescriptionEntry {
            description: "import named items from a local file with `get <fn> from <path>::<file>`",
            examples: &["get add from math::utils\n// imports add from math/utils.rl"],
        },
    ],
};
