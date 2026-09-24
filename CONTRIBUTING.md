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

Check the [issues](https://github.com/MohamedGonem/rl-lang/issues) page for open bugs and feature requests, or the [roadmap](ROADMAP.md) for planned work.

## Versioning & releases

rl-lang follows SemVer (`vMAJOR.MINOR.PATCH`, with `-alpha`/`-beta`/`-rc` pre-releases). See [VERSIONING.md](VERSIONING.md) for the full breakdown of when to use each, and how the release/Discord-announcement workflow classifies a pushed tag automatically. Contributors don't need to cut releases themselves, but PR descriptions that change public behavior should note whether the change is a breaking (major), additive (minor), or fix-only (patch) change so maintainers tag it correctly.

## Guidelines

- Keep PRs focused - one fix or feature per PR
- Add tests for new behavior where possible
- Follow the existing code style
- Update docs if you change language behavior or add stdlib functions

## Adding a stdlib function

Registering a new `std::<module>::<function>` now touches four files across three crates -- don't stop at the implementation, or the function will run but won't type-check, autocomplete, or show up in `rl docs`.

1. **Implementation** -- add the function to `crates/rl-std/src/<module>.rs` (one file per module, following the existing ones like `math.rs`, `string.rs`, `bitwise.rs`), written generic over `rl_std_core::Runtime`.
2. **Register the function** -- wire it up in that module's `handles::<R>()` builder (e.g. `crates/rl-std/src/io.rs`) so the VM stdlib picks it up via `Module::from_std`.
3. **Register the name for the checker** -- add `"name"` to that module's list in `crates/rl-commons/src/keywords.rs`. This is what powers `std::<module>::<function>` resolution, single-name shorthand resolution, and "did you mean?" suggestions in `rl-checker` -- skip it and the checker will report the function as undefined even though it runs fine.
4. **Doc entry** -- add `crates/rl-docs/src/entries/stdlib/<module>/<function>.rs` describing the function (signature, description, example), then add it to that module's array in `crates/rl-docs/src/entries/stdlib/<module>/mod.rs` so it shows up in `rl docs` and the LSP hover. Set `deprecated: Some("reason")` if the function is deprecated, and `updated: Some("vX.Y.Z")` if it was changed after initial release.

If you're adding a brand-new module (not just a new function in an existing one), you'll also need to register the module itself in `crates/rl-vm/src/stdlib/mod.rs` (`root()`), `crates/rl-commons/src/lib.rs` (`stdlib_names()`), and `crates/rl-docs/src/entries/mod.rs` (`stdlib_entries()`).

## Deprecating a stdlib function

When renaming or moving a stdlib function (e.g. `std::old::foo` -> `std::new::foo`), you need to keep the old path working while warning users to switch. Three places to touch:

1. **Signature tree** (`crates/rl-std/src/lib.rs`) -- add the function name to the new module's `signatures()` (e.g. `.with_functions(&["foo"])` on the new module), and keep it in the old module too so both paths resolve.

2. **VM runtime tree** (`crates/rl-vm/src/stdlib/mod.rs`) -- register the function under its new path in `root()`, and keep it under the old path too. Both registrations use the same implementation.

3. **Deprecation map** (`crates/rl-checker/src/lib.rs`) -- add an entry to `build_deprecated_stdlib_map()`:
   ```rust
   m.insert(
       vec!["std".into(), "old".into(), "foo".into()],
       "use std::new::foo instead".into(),
   );
   ```
   The checker looks up the full path (`["std", "old", "foo"]`) and emits a yellow `Warning: 'std::old::foo' is deprecated: use std::new::foo instead` whenever it's called. Users can suppress it with `!#[allow(deprecated)]`.

After adding the entry, run `cargo test` -- existing tests that call the old path should still pass (the function still works), and the checker will now emit a deprecation warning for new code.

## Removing a deprecated stdlib function

Once callers have migrated, delete the old path completely:

1. Delete the old `#[native_fn]` impl and its entry in the module's `native_module!` `funcs` list (`crates/rl-std/src/*.rs`).
2. Delete its entry from `build_deprecated_stdlib_map()`.
3. Move or delete its docs entries under `crates/rl-docs/src/entries/stdlib/` (move them if the canonical path has no entry yet, otherwise delete) and update the three `mod.rs` registries.
4. Migrate every in-repo caller (examples, tests, docs snippets) to the canonical path, and rewrite any tests that asserted the deprecation warning.

After removal, the old path fails with `undefined function` -- verify with `rl check` on both the old and the canonical path.

## AI usage

Using AI tools to help write a contribution is fine, but you're expected to understand, test, and take responsibility for anything you submit. See [AI_POLICY.md](AI_POLICY.md) for what's and isn't okay - it covers unreviewed AI output, hallucinated APIs, and mass-generated issues/PRs specifically.

## Questions

Open an issue or reach out via GitHub.
