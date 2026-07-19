# rl-docs

> Documentation generation tooling for the rl-lang programming language

Part of the [rl-lang](https://github.com/rl-lang/rl-lang) workspace. Powers the `rl docs` CLI command.

## Overview

`rl-docs` is an in-process documentation system: all language reference material - stdlib signatures, language concepts, and tutorials - is stored as static data compiled directly into the binary, so no external files are needed at runtime. It's organized into three categories:

- **stdlib** - `StdEntry` / `FnEntry` for every stdlib module and function
- **concepts** - `ConceptEntry` for language features (types, loops, imports, etc.)
- **tutorials** - `ConceptEntry` steps for the beginner and advanced tutorials

`std_to_markdown`, `concept_to_markdown`, and `tutorial_to_markdown` render these entries into Markdown for display in the terminal; `docs_to_json` renders them as JSON for tooling that wants structured output.

## Modules

| Module | Contents |
|---|---|
| `entries` | Static entry data for stdlib modules, concepts, and tutorials |
| `entry` | `StdEntry`, `FnEntry`, and `ConceptEntry` type definitions |
| `tui` (feature-gated) | Interactive terminal browser for the docs, built on `ratatui` / `crossterm` |

## Features

- `tui` - enables the interactive terminal doc browser (`dep:ratatui`, `dep:crossterm`, `dep:rl-lexer`, `dep:rl-utils`)

## Usage

```toml
[dependencies]
rl-docs = { workspace = true }
```

```rust
use rl_docs::{entries::stdlib_entries, std_to_markdown};

let markdown = std_to_markdown(&stdlib_entries());
```

## License

Licensed under either of Apache License, Version 2.0 or MIT license at your option - see [LICENSE.md](../../LICENSE.md) for the full text and why both are offered.
