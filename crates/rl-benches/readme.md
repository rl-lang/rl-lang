# rl-benches

> Benchmark suite for the rl-lang pipeline

Part of the [rl-lang](https://github.com/rl-lang/rl-lang) workspace. Not published (`publish = false`) - measures the performance of the lexer, parser, and full evaluation pipeline using [Criterion](https://github.com/bheisler/criterion.rs).

## Overview

Each file under `benches/` compiles as its own binary, so shared source snippets and pipeline-stage helpers live once in `src/lib.rs` and are imported with `use rl_benches::*;` instead of being duplicated per benchmark file.

## Benchmarks

| Target | Measures |
|---|---|
| `lexer` | Tokenizing throughput across declaration, control-flow, function, and import snippets |
| `parser` | Lexing + parsing throughput on the same snippet set |
| `pipeline` | Full lex -> parse -> resolve -> evaluate pipeline on representative programs (Fibonacci, result chains, array pipelines, tuple destructuring, closures) |

## Modules

| Module | Contents |
|---|---|
| `lib` | Shared source-code fixtures (`SRC_*` constants) and pipeline-stage helpers (`lex_only`, `lex_and_parse`, `full_pipeline`) |

## Dependencies

Builds on `rl-ast`, `rl-lexer`, `rl-parser`, `rl-interpreter`, `rl-utils`, and `criterion`.

## Usage

```bash
cargo bench -p rl-benches
```

HTML reports are written under `target/criterion/` (via the `html_reports` Criterion feature).

## License

Licensed under either of Apache License, Version 2.0 or MIT license at your option - see [LICENSE.md](../../LICENSE.md) for the full text and why both are offered.
