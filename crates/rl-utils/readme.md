# rl-utils

> Shared utility functions used across the rl-lang toolchain

Part of the [rl-lang](https://github.com/rl-lang/rl-lang) workspace. `rl-utils` sits underneath every other crate in the pipeline - it has no dependency on `rl-ast`, `rl-lexer`, or any other language crate, so it's a safe place for types that need to be shared everywhere without creating dependency cycles.

## Overview

This crate provides the small set of primitives that every stage of the pipeline (lexer, parser, resolver, checker, evaluator, VM) needs in common:

- Carrying source text and its file name through the pipeline
- Pointing at an exact location in that source for error messages
- A single `Error` type with categorized reasons, used everywhere errors are raised
- Fuzzy "did you mean?" suggestions for unresolved names

## Modules

| Module | Contents |
|---|---|
| `errors` | `Error` type and `Reason` categories used by every crate to report failures |
| `source` | `SourceFile` - carries the source text and file name through the pipeline |
| `span` | `Span` - a `(start, end)` byte range used to point diagnostics at exact source locations |
| `suggest` | Fuzzy name matching used to power "did you mean `foo`?" hints |

## Features

- `debug` - enables `log`-based tracing, optional and off by default

## Usage

```toml
[dependencies]
rl-utils = { workspace = true }
```

```rust
use rl_utils::source::SourceFile;
use rl_utils::span::Span;

let source = SourceFile::new("script.rl", "dec int x = 1".to_string());
```

## License

Licensed under either of Apache License, Version 2.0 or MIT license at your option - see [LICENSE.md](../../LICENSE.md) for the full text and why both are offered.
