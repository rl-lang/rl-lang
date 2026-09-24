# ADR-0001: Deprecate `std::rl` ahead of self-hosting

Date: 2026-09-22
Author: Mohamed Gonem

## Status
Deprecated

## Context
`std::rl` exposes the host compiler pipeline as runtime functions: `lex`, `eval`, `eval_isolated`, `check`, `rl_version`, `source_name`. These are native hooks into the Rust implementation, not ordinary library code. That causes two problems:

1. RL will be self-hosted: the compiler itself will be written in RL. Metaprogramming facilities (lexing, evaluating, checking RL code) belong in an RL-written metalib sitting on top of that compiler, not as native functions welded to the current host implementation.
2. They are unimplementable outside the interpreter. Transpiled C binaries have no compiler pipeline inside them, so `eval`/`lex`/`check` can never work there - a permanent parity hole between backends.

## Decision
Deprecate all six `std::rl` functions. The type checker emits a deprecation warning on every use (qualified, bare, aliased, wildcard-imported, and method calls); the functions keep working. Removal happens after the self-hosted metalib exists.

## Alternatives considered
- Keep `std::rl` as-is - rejected: cements host-compiler hooks into the language surface and blocks the self-hosting story.
- Remove immediately - rejected: breaks existing programs with no migration window and no replacement library yet.

## Scope
Standard Library (surface), Type Checker (warnings), C backend (already rejects `eval`/`lex`/`check` at transpile time).

## Breaking Changes & Migrations
Non-breaking: deprecation produces warnings, not errors, suppressible with `!#[allow(deprecated)]`. There is no mechanical migration yet - callers should plan to drop `std::rl` usage once the RL-written metalib lands.

## Examples
### Before the change:
```rl
get eval from std::rl
dec r = eval("1 + 2")
```

### After the change:
```rl
get eval from std::rl
dec r = eval("1 + 2")
```
Same code, plus a checker warning: `'std::rl::eval' is deprecated: std::rl is deprecated and may be removed in a future version`.

## Consequences
- Makes the self-hosting direction explicit in the language surface.
- Keeps every existing program compiling while the metalib is unwritten.
- Locks in a future removal: the six functions must eventually be deleted, and the metalib must cover `lex`/`eval`/`check` before that happens.

## Related
- `crates/rl-checker/src/lib.rs` (`build_deprecated_stdlib_map`)
- `crates/rl-checker/src/scope/call.rs` (warning sites)
- `crates/rl-cc/src/codegen/expressions/mod.rs` (`std::rl` transpile-time errors)
