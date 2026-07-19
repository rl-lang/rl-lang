# rl-cli

> Command-line interface for the rl-lang toolchain

Part of the [rl-lang](https://github.com/rl-lang/rl-lang) workspace. This is the crate that builds the `rl` binary - the top-level entry point users install to write and run rl-lang programs.

## Subcommands

| Subcommand | Action |
|---|---|
| `rl run <file>` | Lex, parse, and evaluate a single `.rl` file (`--vm` / `--cranelift` to pick an alternate backend) |
| `rl dev` | Read `rl.toml`, lex, parse, and evaluate the project entry point |
| `rl check <file>` | Lex, parse, and type-check a file, reporting errors without running it |
| `rl new <name>` | Scaffold a new project directory |
| `rl docs [topic]` | Print stdlib / concept / tutorial reference (Markdown, JSON, or an interactive TUI) |
| `rl workflows` | Scaffold GitHub Actions workflow files for a project |
| `rl package` | Embed a compiled program into a standalone artifact |
| `rl format` | Format a `.rl` source file |
| `rl repl` | Start the interactive TUI REPL (`repl` feature) |
| `rl lsp` | Start the LSP server over stdio (`lsp` feature) |

## Modules

| Module | Contents |
|---|---|
| `main` | `clap`-based CLI parsing and subcommand dispatch |
| `logic_loops` | Shared lex / parse / eval loop helpers used by several subcommands |

## Features

- `default` - `repl`, `run`, `eval`, `vm`, `docs-tui`
- `run`, `eval`, `vm` - enable the corresponding execution backends
- `repl` - pulls in `rl-repl`
- `lsp` - pulls in `rl-lsp` and `tokio`
- `cranelift` - pulls in `rl-cranelift`
- `docs-tui` - enables `rl-docs`'s `tui` feature
- `debug` - enables `log` / `env_logger`

## Dependencies

Depends on nearly every other crate in the workspace: `rl-utils`, `rl-lexer`, `rl-ast`, `rl-parser`, `rl-resolver`, `rl-vm`, `rl-interpreter`, `rl-checker`, `rl-docs`, `rl-tooling`, and optionally `rl-cranelift`, `rl-repl`, `rl-lsp`; plus `clap`, `toml`, and `serde`.

## Usage

```bash
cargo build --release
# binary at target/release/rl

rl run script.rl
rl new my-project
rl docs std::math
rl repl
```

## License

Licensed under either of Apache License, Version 2.0 or MIT license at your option - see [LICENSE.md](../../LICENSE.md) for the full text and why both are offered.
