# rl-ast

> Abstract syntax tree types for the rl-lang programming language

Part of the [rl-lang](https://github.com/rl-lang/rl-lang) workspace.

## Pipeline position

```text
source -> Lexer -> [Token] -> Parser -> [Statement] -> Checker -> Evaluator
```

`rl-ast` defines the node types produced by the parser and consumed by every later stage (resolver, checker, interpreter, VM compiler).

## Modules

| Module | Contents |
|---|---|
| `nodes` | `Expression` and `ExpressionKind` - the expression AST |
| `statements` | `Statement`, `StatementKind`, `TypeAnnotation`, `Param` |
| `arena` | `Arena` / `Id` - arena allocation backing the AST nodes |

## Dependencies

Builds on `rl-lexer` (for token/span types shared with AST nodes) and `rl-utils`.

## Usage

```toml
[dependencies]
rl-ast = { workspace = true }
```

```rust
use rl_ast::nodes::{Expression, ExpressionKind};
use rl_ast::statements::{Statement, StatementKind};
```

## License

Licensed under either of Apache License, Version 2.0 or MIT license at your option, per the [rl-lang workspace license](https://github.com/rl-lang/rl-lang/blob/main/LICENSE).
