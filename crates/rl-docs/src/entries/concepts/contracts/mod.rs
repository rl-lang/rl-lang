use crate::entry::{ConceptCategory, ConceptEntry, DescriptionEntry, DescriptionKind};

pub static CONTRACTS: ConceptEntry = ConceptEntry {
    name: "contracts",
    summary: "runtime-checked contracts: parameter refinements (`int amt: >0`), `requires` preconditions, and `ensures` postconditions over `ret` - violations panic for bare returns and return `Err` for `result` returns",
    category: ConceptCategory::Functions,
    prerequisites: &["functions", "result"],
    descriptions: &[
        DescriptionEntry {
            kind: DescriptionKind::Syntax,
            title: Some("parameter refinements"),
            description: "a `: <op> <operand>` predicate after a parameter constrains it at entry. The operand is a literal or another parameter's name",
            examples: &[
                "fn withdraw(int amt: >0, int balance: >=amt) -> result[int] {\n    return ok(balance - amt)\n}",
            ],
            expected_output: &[],
        },
        DescriptionEntry {
            kind: DescriptionKind::Syntax,
            title: Some("requires and ensures"),
            description: "`requires` lists preconditions as `condition [, \"message\"]` pairs; `ensures` lists postconditions where `ret` means the return value (the inner value for `result[T]`). `ensures` is skipped when the function itself returns `Err`",
            examples: &[
                "fn withdraw(int amt: >0, int balance: >=amt) -> result[int]\n    requires amt > 0, \"amt must be positive\"\n    ensures ret <= balance\n{\n    if (amt > balance) { return err(0) }\n    return ok(balance - amt)\n}",
            ],
            expected_output: &[],
        },
        DescriptionEntry {
            kind: DescriptionKind::Explanation,
            title: Some("runtime checking, no solver"),
            description: "contracts lower to ordinary guards before resolution, so every backend enforces identically. Unannotated (`Null`) returns take the bare abort form: a violation aborts the program instead of returning `Err`, because without a declared kind the resolver cannot know whether `Err` is a legal value there. The checker still infers the return type from the body as usual, so `fn f(int x: >0) { return ok(x) }` infers `result[int]` - only the violation path differs. Annotate `-> result[T]` whenever callers must receive violations as `Err` values they can handle with `?`. Static proving (phase 2) will build on these same annotations",
            examples: &[],
            expected_output: &[],
        },
        DescriptionEntry {
            kind: DescriptionKind::Explanation,
            title: Some("contracts as properties"),
            description: "the `cases(N)` generator reads the same annotations as its input spec, so one annotation serves checking and generation: refined `int` parameters generate in-bounds values instead of generate-then-filter",
            examples: &[
                "!#[test(cases(100))]\nfn withdraw_never_negative(int amt: >0, int balance: >=amt) { }",
            ],
            expected_output: &[],
        },
    ],
    pitfalls: &[
        "`ret` is reserved inside `ensures` - do not name a parameter `ret` on a contracted function",
        "lambdas inside contract conditions are not supported",
        "a bare `return;` under `ensures` checks against null",
    ],
    related: &["functions", "testing", "result", "errors"],
    related_stdlib: &["test"],
    since: Some("v2.3.0"),
};
