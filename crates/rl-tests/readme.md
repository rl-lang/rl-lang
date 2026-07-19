# rl-tests

> Integration test suite for the rl-lang toolchain

Part of the [rl-lang](https://github.com/rl-lang/rl-lang) workspace. This crate is not published (`publish = false`) - it exists purely to hold end-to-end tests that exercise the lexer, parser, interpreter, and VM together.

## Layout

| Directory | Covers |
|---|---|
| `tests/lexer` | Tokenizing keywords, literals, and declarations |
| `tests/parser` | Parsing declarations, control flow (`if`/`while`/`for`), functions/lambdas, `match`, imports, and postfix expressions |
| `tests/interpreter` | End-to-end evaluation: numbers, booleans, characters, arrays, tuples, functions, control flow, and every stdlib module (`arr_zip`, `bitwise`, `math`, `result`, `string`, `types`) |
| `tests/vm` | Bytecode VM execution |
| `tests/common` | Shared test fixtures and helpers |

## Dependencies

Depends (as dev-dependencies) on `rl-ast`, `rl-lexer`, `rl-parser`, `rl-interpreter`, `rl-utils`, and `rl-vm`.

## Usage

```bash
cargo test -p rl-tests
```

## License

Licensed under either of Apache License, Version 2.0 or MIT license at your option, per the [rl-lang workspace license](https://github.com/rl-lang/rl-lang/blob/main/LICENSE).
