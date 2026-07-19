# rl-lexer

> Lexer and tokenizer for the rl-lang programming language

Part of the [rl-lang](https://github.com/rl-lang/rl-lang) workspace.

## Pipeline position

```text
source -> Lexer -> [Token] -> Parser -> [Statement] -> Checker -> Evaluator
```

`rl-lexer` is the first stage of the pipeline: it converts raw source text into a flat `Vec<Token>` that the parser consumes.

## Modules

| Module | Contents |
|---|---|
| `tokenizer` | The `Tokenizer` struct and its main scanning loop |
| `tokentypes` | The `Token` and `TokenType` definitions |
| `scanner` | Top-level scan driver called by the pipeline |
| `types` | Sub-scanners for each literal kind: string, char, number, identifier |
| `utils` | Shared cursor helpers used across the sub-scanners |

## Features

- `debug` - enables `log`-based tracing of the scanning process, optional and off by default

## Usage

```toml
[dependencies]
rl-lexer = { workspace = true }
```

```rust
use rl_lexer::tokenizer::Tokenizer;
use rl_utils::source::SourceFile;

let source = SourceFile::new("script.rl", "dec int x = 1".to_string());
let tokens = Tokenizer::lex(source)?;
```

## License

Licensed under either of Apache License, Version 2.0 or MIT license at your option - see [LICENSE.md](../../LICENSE.md) for the full text and why both are offered.
