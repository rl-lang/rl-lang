use crate::entry::{ConceptCategory, ConceptEntry, DescriptionEntry, DescriptionKind};

pub static COMMENTS: ConceptEntry = ConceptEntry {
    name: "comments",
    summary: "`//` line comments for notes ignored by the compiler, and `///` doc comments that feed rl's generated documentation site",
    category: ConceptCategory::Syntax,
    prerequisites: &[],
    descriptions: &[
        DescriptionEntry {
            kind: DescriptionKind::Syntax,
            title: Some("line comments"),
            description: "single-line comments start with `//` - everything after it to the end of the line is ignored",
            examples: &[
                "// this is a comment\ndec int x = 10  // inline comment\nprintln(x)  // 10",
            ],
            expected_output: &["10"],
        },
        DescriptionEntry {
            kind: DescriptionKind::Explanation,
            title: Some("doc comments"),
            description: "`///` (triple slash) marks a doc comment instead of a plain line comment; placed directly above a `fn`, `record`, `tag`, `const`, or `dec` declaration, it becomes that item's documentation and is picked up by `rl docs --generate` to build a documentation site",
            examples: &[
                "/// adds two ints together\nfn add(int a, int b) -> int {\n    return a + b\n}",
            ],
            expected_output: &[],
        },
        DescriptionEntry {
            kind: DescriptionKind::Pitfall,
            title: Some("no block comments"),
            description: "there's no `/* ... */` block-comment syntax - only `//` line comments exist; writing `/* like this */` doesn't lex as a comment at all, so `/` and `*` are parsed as the division and multiplication operators instead and it's a syntax error",
            examples: &[],
            expected_output: &[],
        },
        DescriptionEntry {
            kind: DescriptionKind::Pitfall,
            title: Some("doc comments can't trail code"),
            description: "`///` doc comments always attach as documentation for the declaration that follows them, even if placed at the end of an existing line of code - unlike `//`, they can't be used as a trailing/inline comment",
            examples: &[
                "dec int x = 10  /// attaches to the next declaration, not to x\nfn helper() {}",
            ],
            expected_output: &[],
        },
    ],
    pitfalls: &[
        "there's no `/* ... */` block-comment syntax - only `//` line comments exist; writing `/* ... */` produces a syntax error since `/` and `*` are parsed as operators instead",
        "`///` doc comments always attach as documentation for the following declaration, even when placed at the end of an existing line - they can't be used as trailing/inline comments the way `//` can",
        "plain `//` comments are ignored by doc generation - only `///` comments show up in a generated documentation site",
    ],
    related: &["functions", "tooling"],
    related_stdlib: &[],
    since: Some("v0.1.5"),
};
