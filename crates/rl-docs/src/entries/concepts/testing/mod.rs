use crate::entry::{ConceptCategory, ConceptEntry, DescriptionEntry, DescriptionKind};

pub static TESTING: ConceptEntry = ConceptEntry {
    name: "testing",
    summary: "attribute-driven tests: `!#[test]` functions run under `rl test` (or `rlt --test` for C), with `!#[setup]`/`!#[teardown]` hooks, `cases(N)` property tests, and `std::test` skips and non-fatal assertions",
    category: ConceptCategory::Tooling,
    prerequisites: &["functions"],
    descriptions: &[
        DescriptionEntry {
            kind: DescriptionKind::Syntax,
            title: Some("marking a test"),
            description: "a plain `!#[test]` marks a unit test. Parameters compose: `group(\"name\")` for filtering and reporting, `register(\"name\")` for the named registry, `cases(N)` for property tests over N generated inputs",
            examples: &[
                "!#[test]\nfn adds_correctly() {\n    test_assert_eq(1 + 1, 2, \"math\")\n}",
                "!#[test(group(\"money\"))]\nfn withdraw_full_balance() { }",
                "!#[test(group(\"money\"), cases(100))]\nfn withdraw_never_negative(int amt: >0, int balance: >=amt) { }",
            ],
            expected_output: &[],
        },
        DescriptionEntry {
            kind: DescriptionKind::Syntax,
            title: Some("setup and teardown"),
            description: "`!#[setup]` runs before each case in the file, `!#[teardown]` runs after each case - even when the case fails. Uses: resetting shared globals (cases observe one shared Vm, so mutations leak between cases), seeding files or fixtures, and closing handles. Teardown running on failure is what makes it different from just putting cleanup at the end of each case",
            examples: &[
                "!#[setup]\nfn reset_ledger() { }",
                "!#[teardown]\nfn close_ledger() { }",
            ],
            expected_output: &[],
        },
        DescriptionEntry {
            kind: DescriptionKind::Explanation,
            title: Some("running tests"),
            description: "`rl test` discovers `!#[test]` functions, runs setup, cases (inits/finals keep their semantics), teardown, and reports pass/fail/skip with a non-zero exit on failure. `rl test --match <pattern>` filters by group/register name. Tests never run under `rl run`. `rlt --test` builds the same runner as a C binary",
            examples: &[
                "rl test tests.rl",
                "rl test tests.rl --match money",
            ],
            expected_output: &[],
        },
        DescriptionEntry {
            kind: DescriptionKind::Explanation,
            title: Some("runtime helpers"),
            description: "attributes cannot express in-body control flow or non-fatal assertions, so `std::test` covers them: `test_skip` / `test_skip_if` stop a case as skipped, `test_assert_eq` / `test_assert_ne` record pass/fail with a message, `test_assert_panics` / `test_assert_no_panic` check failure behavior. Outcomes accumulate without aborting, so one failure never hides the rest",
            examples: &[
                "test_skip_if(false, \"never skips\")",
                "get test_assert_panics from std::test\n\nget result_unwrap from std::res\ntest_assert_panics(fn() {\n    result_unwrap(err(\"boom\"))\n})",
            ],
            expected_output: &[],
        },
        DescriptionEntry {
            kind: DescriptionKind::Explanation,
            title: Some("property tests"),
            description: "`cases(N)` derives generators from parameter types (ints bounded, strings randomized, arrays recurse) narrowed by refinements (`>0` bounds the range instead of generate-then-filter). Failures shrink greedily and the runner reports the minimal failing inputs. Maps, sets, tuples, records, and chars are explicitly out of scope and fail cleanly. `cases(0)` is valid and runs zero iterations (vacuous ok) - useful for temporarily disabling a property without deleting it. Register lookup, static proving, and full C-side generation are deferred to phase 2",
            examples: &[
                "!#[test(cases(100))]\nfn withdraw_never_negative(int amt: >0, int balance: >=amt) { }",
            ],
            expected_output: &[],
        },
    ],
    pitfalls: &[
        "tests never run under `rl run` - a passing `rl run` says nothing about the test suite",
        "skips are not failures: a fully-skipped suite still exits zero",
        "property generation is deterministic (fixed seed), so failures replay bit-for-bit",
    ],
    related: &["functions", "contracts", "result", "errors"],
    related_stdlib: &["test"],
    since: Some("v2.3.0"),
};
