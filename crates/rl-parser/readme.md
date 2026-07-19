# rl-parser

> Parser for the rl-lang programming language

Part of the [rl-lang](https://github.com/rl-lang/rl-lang) workspace.

## Pipeline position

```text
source -> Lexer -> [Token] -> Parser -> [Statement] -> Checker -> Evaluator
```

`rl-parser` transforms the flat `Vec<Token>` produced by `rl-lexer` into a `Vec<Statement>` - the abstract syntax tree defined by `rl-ast`.

## Modules

| Module | Contents |
|---|---|
| `parser_logic` | The `Parser` struct and its core cursor primitives |
| `expressions` | Precedence-climbing expression parser (`equality`, `comparsion`, `term`, `factor`, `unary`, `postfix`, `primary`, `logical`, `struct_literal`) |
| `statements` | One sub-module per statement kind: `if`, `while`, `for`, function/variable/const/record/tag declarations, `import`, `match` |
| `utils` | Shared helpers, including type annotation parsing |

## Features

- `debug` - enables `log`-based tracing of the parsing process, optional and off by default

## Usage

```toml
[dependencies]
rl-parser = { workspace = true }
```

```rust
use rl_lexer::tokenizer::Tokenizer;
use rl_parser::parser_logic::Parser;
use rl_utils::source::SourceFile;

let source = SourceFile::new("script.rl", "dec int x = 1".to_string());
let tokens = Tokenizer::lex(source.clone())?;
let (ast, statements) = Parser::parse(tokens, source)?;
```

## License

Licensed under either of Apache License, Version 2.0 or MIT license at your option - see [LICENSE.md](../../LICENSE.md) for the full text and why both are offered.
