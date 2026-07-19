# rl-tooling

> Developer tooling for the rl-lang toolchain

Part of the [rl-lang](https://github.com/rl-lang/rl-lang) workspace. Backs most of the non-pipeline `rl` CLI subcommands.

## Modules

| Module | Contents |
|---|---|
| `new` | Project scaffolding for `rl new <name>` - creates `rl.toml`, `src/main.rl`, `.gitignore`, and initializes a git repository |
| `dev` | Reads and parses a project's `rl.toml` manifest |
| `format` | Token-based source formatter |
| `package` | Embeds a compiled program into a standalone artifact (`EmbeddedProgram`) and locates embedded programs at runtime |
| `generate_docs` | Generates documentation site output (Markdown / HTML) from a project |
| `workflows` | Generates CI workflow files for a new project |

## Dependencies

Builds on `rl-lexer`, plus `serde` and `toml` (manifest parsing), and `tiny_http` / `ureq` (used by generated project workflows).

## Usage

```toml
[dependencies]
rl-tooling = { workspace = true }
```

```rust
use rl_tooling::new::create_project;

create_project("my-project", /* no_git */ false);
```

## License

Licensed under either of Apache License, Version 2.0 or MIT license at your option, per the [rl-lang workspace license](https://github.com/rl-lang/rl-lang/blob/main/LICENSE).
