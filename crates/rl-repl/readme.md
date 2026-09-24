# rl-repl

> Interactive REPL for the rl-lang programming language

Part of the [rl-lang](https://github.com/rl-lang/rl-lang) workspace. A TUI REPL built on `ratatui` and `crossterm`, wired up behind the `repl` feature of `rl-cli`.

## Layout

```text
|-- ✦ rl ------------------------ ● live --|
|  ❯ dec int x = 10                        |
|  ❯ x + 1                                 |
|  11                                       |
|--------------------------------------------|
|-- input ---- Tab complete · Shift+↑↓ ... --|
|  ❯ _                                      |
|--------------------------------------------|
```

A dark, Tokyo-Night-adjacent palette is centralized in `theme.rs` - see that module for the full color list.

## Key bindings

| Key | Action |
|---|---|
| `Enter` | Submit / continue multiline |
| `Ctrl+C` | Exit |
| `Esc` | Cancel multiline input |
| `↑` / `↓` | History navigation |
| `Shift+↑/↓` | Scroll output |
| `Ctrl+←/→` | Word jump |
| `Home` / `End` | Line start / end |
| `Tab` | Complete word at cursor / cycle candidates |

## Meta-commands

`:`-prefixed commands typed at the prompt (see `:help` in the REPL):

| Command | Action |
|---|---|
| `:help` | Print all available commands |
| `:stdlib` | List all stdlib modules |
| `:stdlib <mod>` | List all functions in a stdlib module |
| `:save <file>` | Save submitted lines to a file |
| `:load <file>` | Print a file's contents into the output |
| `:attach <file>` | Lex, parse, and evaluate a file into the env |
| `:detach <file>` | Remove a file from the attached list |
| `:clear` | Clear the output buffer |
| `:reset` | Reset the evaluator to a fresh environment |
| `:exit` | Exit the REPL |

## Backends

The REPL UI never touches an execution engine directly - it drives a
[`ReplBackend`](src/backend.rs), backed by the bytecode VM:

| Backend | Execution engine | Feature |
|---|---|---|
| `VmBackend` | bytecode VM (`rl-vm`) with persistent global state across inputs | `vm` |

`VmBackend` keeps a single `Vm` + `Resolver` alive for the whole session, so
declarations, functions, and `get x from std::io` imports survive between
submitted inputs. Each input is resolved against the persistent resolver,
compiled with a `Compiler` seeded at the current global slot count, and run
via `run_and_return` so a trailing expression's value is rendered.

## Modules

| Module | Contents |
|---|---|
| `backend` | [`ReplBackend`] trait plus the `VmBackend` implementation |
| `logic_loop` | Main event loop driving the REPL |
| `command_handler` | Handles REPL meta-commands |
| `completion` | Tab-completion candidate generation (`:`-commands, keywords, stdlib paths, bound names) |
| `depth_checker` | Detects unterminated input to trigger multiline continuation |
| `input_eval` | Lexes/parses submitted input, then delegates evaluation to the backend |
| `lines_types` | Types backing the scrollable output/history buffers |
| `output_render` | Renders evaluation results and errors to the output area |
| `syntax_highlighting` | Live syntax highlighting of the input bar |
| `theme` | Centralized color palette used by every widget |
| `utils` | Shared REPL helpers |

## Features

- `vm` (default) - bytecode VM backend (`rl-vm` + `rl-resolver`)

The crate requires the `vm` feature.

## Dependencies

Builds on `rl-ast`, `rl-docs`, `rl-parser`, `rl-lexer`, `rl-utils`, `crossterm`,
and `ratatui`, plus `rl-vm` (`vm`) and `rl-resolver` (`vm`).

## Usage

```toml
[dependencies]
rl-repl = { workspace = true }
```

```rust
use rl_repl::start_vm_repl;

start_vm_repl();
```

## License

Licensed under either of Apache License, Version 2.0 or MIT license at your option - see [LICENSE.md](../../LICENSE.md) for the full text and why both are offered.
