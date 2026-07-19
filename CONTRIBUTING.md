# Contributing to rl-lang

Thanks for your interest in contributing!

## Getting started

```bash
git clone https://github.com/MohamedGonem/rl-lang
cd rl-lang
cargo build
```

## Before submitting a PR

```bash
cargo test --all-features   # make sure all tests pass
cargo clippy -- -D warnings # no lint warnings
```

- PRs must be up to date with the `dev` branch - always branch off `dev` and rebase before submitting

## What to work on

Check the [issues](https://github.com/MohamedGonem/rl-lang/issues) page for open bugs and feature requests, or the [roadmap](https://github.com/MohamedGonem/rl-lang/wiki/Roadmap) for planned work.

## Versioning & releases

rl-lang follows SemVer (`vMAJOR.MINOR.PATCH`, with `-alpha`/`-beta`/`-rc` pre-releases). See [VERSIONING.md](VERSIONING.md) for the full breakdown of when to use each, and how the release/Discord-announcement workflow classifies a pushed tag automatically. Contributors don't need to cut releases themselves, but PR descriptions that change public behavior should note whether the change is a breaking (major), additive (minor), or fix-only (patch) change so maintainers tag it correctly.

## Guidelines

- Keep PRs focused - one fix or feature per PR
- Add tests for new behavior where possible
- Follow the existing code style
- Update docs if you change language behavior or add stdlib functions

## Adding a stdlib function

Registering a new `std::<module>::<function>` now touches five files across three crates -- don't stop at the implementation, or the function will run but won't type-check, autocomplete, or show up in `rl docs`.

1. **Implementation** -- add `crates/rl-interpreter/src/stdlib/<module>/<function>.rs` (one file per function, following the existing modules like `math/`, `string/`, `bitwise/`).
2. **Register the function** -- wire it up in `crates/rl-interpreter/src/stdlib/<module>/mod.rs` with `.with_function("name", <function>::std_<function>)`.
3. **Register the name for the checker** -- add `"name"` to that module's list in `crates/rl-commons/src/keywords.rs`. This is what powers `std::<module>::<function>` resolution, single-name shorthand resolution, and "did you mean?" suggestions in `rl-checker` -- skip it and the checker will report the function as undefined even though it runs fine.
4. **Doc entry** -- add `crates/rl-docs/src/entries/stdlib/<module>/<function>.rs` describing the function (signature, description, example).
5. **Register the doc entry** -- add it to that module's array in `crates/rl-docs/src/entries/stdlib/<module>/mod.rs` so it shows up in `rl docs` and the LSP hover.

If you're adding a brand-new module (not just a new function in an existing one), you'll also need to register the module itself in `crates/rl-interpreter/src/stdlib/mod.rs`, `crates/rl-commons/src/lib.rs` (`stdlib_names()`), and `crates/rl-docs/src/entries/mod.rs` (`stdlib_entries()`).

The bytecode VM (`rl-vm`) has its own, much smaller stdlib (`crates/rl-vm/src/stdlib/`) that only covers `io` so far -- you generally don't need to touch it unless you're specifically porting a function to the VM backend.

## Questions

Open an issue or reach out via GitHub.
