# rl-interpreter

> Tree-walking interpreter for the rl-lang programming language

Part of the [rl-lang](https://github.com/rl-lang/rl-lang) workspace.

## Pipeline

```text
source text
  |- Tokenizer::lex()               -> Vec<Token>
  |- Parser::parse()                -> Vec<Statement>
  |- Resolver::resolve()            -> Vec<Statement>  (names -> depth/slot)
  |- Evaluator::evaluate_program()  -> ()
```

## Modules

| Module | Contents |
|---|---|
| `evaluator` | The core `Evaluator` struct and expression evaluation |
| `values` | The `Value` enum representing all runtime values |
| `native` | `Module`, `NativeFn`, and the trait system for binding Rust functions into the language |
| `scopes` | Environment stack: push/pop/insert/assign/get |
| `stdlib` | All built-in `std::*` modules: `array`, `bitwise`, `debug`, `fs`, `http`, `io`, `math`, `net`, `path`, `process`, `random`, `result`, `rl`, `string`, `terminal`, `time`, `types` |
| `utils` | Binary/unary operator dispatch and statement evaluation |
| `evaluator_types` | Addressing and index-assignment evaluation helpers |

## Dependencies

Builds on `rl-commons`, `rl-checker`, `rl-ast`, `rl-lexer`, `rl-parser`, `rl-resolver`, `rl-utils`, plus `crossterm` (terminal stdlib), `tiny_http` and `ureq` (net/http stdlib).

## Usage

```toml
[dependencies]
rl-interpreter = { workspace = true }
```

```rust
use rl_interpreter::evaluator::Evaluator;
use rl_lexer::tokenizer::Tokenizer;
use rl_parser::parser_logic::Parser;
use rl_utils::source::SourceFile;

let source = SourceFile::new("script.rl", "get println from std::io\nprintln(\"hi\")".to_string());
let tokens = Tokenizer::lex(source.clone())?;
let (ast, statements) = Parser::parse(tokens, source.clone())?;

let mut evaluator = Evaluator::default().with_stdlib().with_source_file(source);
let statements = evaluator.resolver.resolve_program(ast, statements);
evaluator.evaluate_program(&statements)?;
```

## Features

- `debug` - enables `log`-based tracing, optional and off by default

## License

Licensed under either of Apache License, Version 2.0 or MIT license at your option, per the [rl-lang workspace license](https://github.com/rl-lang/rl-lang/blob/main/LICENSE).
