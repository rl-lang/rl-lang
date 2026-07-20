# rl-resolver

> Name and import resolution for the rl-lang programming language

Part of the [rl-lang](https://github.com/rl-lang/rl-lang) workspace. Runs between parsing and evaluation.

## Overview

The resolver walks the AST produced by `rl-parser` and rewrites unresolved name references into slot-indexed lookups, eliminating string-based name lookups at runtime:

- `depth` - how many scopes up from the current scope the variable lives
- `slot` - the index of the variable within that scope's slot array

Unresolved `Identifier` nodes become `ResolvedIdentifier { depth, slot }`, and unresolved `Assign` nodes become `ResolvedAssign { depth, slot, value }`. Function and lambda bodies are resolved in their own pushed scope. Import statements are read from disk, lexed, parsed, and resolved inline as part of this pass.

## Modules

| Module | Contents |
|---|---|
| `expressions` | Resolves identifier and assignment expressions to `(depth, slot)` pairs |
| `statements` | Resolves declarations, function/lambda bodies, and inlines `import` statements |

## Dependencies

Builds on `rl-ast`, `rl-parser`, `rl-lexer`, and `rl-utils`.

## Usage

```toml
[dependencies]
rl-resolver = { workspace = true }
```

```rust
use rl_resolver::Resolver;

let mut resolver = Resolver::new();
```

## License

Licensed under either of Apache License, Version 2.0 or MIT license at your option - see [LICENSE.md](../../LICENSE.md) for the full text and why both are offered.
