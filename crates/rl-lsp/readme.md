# rl-lsp

> Language Server Protocol implementation for the rl-lang programming language

Part of the [rl-lang](https://github.com/rl-lang/rl-lang) workspace. Built on `tower-lsp`, wired up behind the `lsp` feature of `rl-cli` (`rl lsp`).

## Architecture

```text
editor (VS Code, Zed, Neovim, ...)
    |  JSON-RPC over stdio
|---V----------------------------|
|  Backend  (tower-lsp handler)  |
|  |-- did_open / did_change --> pipeline::run_pipeline  --> diagnostics
|  |-- hover -----------------> hover::run_hover         --> hover info
|--------------------------------|
```

Communicates with editors over stdio using JSON-RPC. Currently supports:

- **Diagnostics** - lex/parse/type-check errors reported on every file change
- **Hover** - markdown type info at the cursor position

## Modules

| Module | Contents |
|---|---|
| `backend` | `Backend` - the `tower-lsp` `LanguageServer` implementation |
| `pipeline` | Runs the lex/parse/check pipeline over a document and collects diagnostics |
| `to_diagnostic` | Converts `rl-utils::Error` into LSP `Diagnostic` values |
| `hover` | Hover provider - surfaces type info recorded by `rl-checker` |
| `goto_definition` | Go-to-definition support |
| `references` | Find-references support |
| `rename` | Rename/refactor support |
| `utils` | Shared position/span conversion helpers |

## Dependencies

Builds on `rl-lexer`, `rl-utils`, `rl-checker`, `rl-parser`, `tower-lsp`, and `tokio`.

## Usage

```toml
[dependencies]
rl-lsp = { workspace = true }
```

```rust
#[tokio::main]
async fn main() {
    rl_lsp::run_lsp().await;
}
```

## License

Licensed under either of Apache License, Version 2.0 or MIT license at your option - see [LICENSE.md](../../LICENSE.md) for the full text and why both are offered.
