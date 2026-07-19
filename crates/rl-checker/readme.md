# rl-checker

> Static type checker for the rl-lang programming language

Part of the [rl-lang](https://github.com/rl-lang/rl-lang) workspace. Runs after parsing, before evaluation.

## Overview

The checker walks the AST and verifies:

- Variable and constant declarations match their declared types
- Binary/unary operators receive compatible operand types
- Function calls receive the correct number and types of arguments
- `return` types match the enclosing function's declared return type
- `break` and `continue` only appear inside loops
- Array elements are all the same type

It also populates `TypeChecker::hovers` - a side-table of `(Span, markdown)` pairs used by the LSP hover provider.

### Two-pass function checking

`TypeChecker::check` does two passes over the statement list: first it pre-declares all top-level `FunctionDeclaration`s so they are visible to each other regardless of order, then it checks every statement body. This allows mutual recursion at the top level.

## Modules

| Module | Contents |
|---|---|
| `types` | Core type representation used throughout the checker |
| `structs` | `TypeChecker` struct and shared checking state |
| `operators` | Binary, unary, and index-assignment type-compatibility rules |
| `scope` | Scope-aware declaration, assignment, and call checking |
| `statements` | Per-statement and per-expression checking logic |

## Dependencies

Builds on `rl-ast`, `rl-parser`, `rl-lexer`, `rl-docs` (for stdlib signatures), `rl-commons`, and `rl-utils`.

## Usage

```toml
[dependencies]
rl-checker = { workspace = true }
```

```rust
use rl_checker::structs::TypeChecker;

let mut checker = TypeChecker::default();
checker.check(&statements);
```

## License

Licensed under either of Apache License, Version 2.0 or MIT license at your option, per the [rl-lang workspace license](https://github.com/rl-lang/rl-lang/blob/main/LICENSE).
