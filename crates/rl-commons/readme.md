# rl-commons

> Common types shared across rl-lang crates to break dependency cycles

Part of the [rl-lang](https://github.com/rl-lang/rl-lang) workspace.

## Overview

`rl-commons` exists purely to hold small, shared types that would otherwise force a dependency cycle between crates (for example, between the checker and the interpreter). It has zero dependencies of its own.

Currently it provides:

- `ModuleNames` - a tree describing the functions and submodules exposed by a stdlib/native module, used by both the checker (to validate imports) and the interpreter (to bind native functions) without either depending on the other.
- The language's reserved keyword list.

## Modules

| Module | Contents |
|---|---|
| `keywords` | The reserved keyword list for the rl language, shared by the lexer and tooling (e.g. formatting, project scaffolding) |

## Usage

```toml
[dependencies]
rl-commons = { workspace = true }
```

```rust
use rl_commons::ModuleNames;

let math = ModuleNames::new("math");
```

## License

Licensed under either of Apache License, Version 2.0 or MIT license at your option, per the [rl-lang workspace license](https://github.com/rl-lang/rl-lang/blob/main/LICENSE).
