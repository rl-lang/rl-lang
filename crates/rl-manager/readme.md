# rl-manager

Toolchain manager for the rl programming language. Installs, updates, and
removes rl-lang binaries from GitHub Releases. Ships as the `rlm` binary.

## Usage

```bash
rlm install              # interactive install (TUI picker)
rlm install latest       # install latest stable
rlm install nightly      # install nightly
rlm install v2.2.0       # install a specific version
rlm install -b rl,rlc    # install specific binaries only
rlm install --no-tui     # CLI-only mode, no TUI
rlm update               # update rlm itself to the latest version
rlm uninstall            # remove installed rl-lang binaries
rlm list                 # list installed rl-lang binaries
```

## Variants

Release artifacts come in variants (`rl`, `rl_vm`, `rl_debug`, ... with
`_no_docs` / `_no_repl` trims). Pick them with `-b`:

```bash
rlm install --variant rl,rl_vm latest
```

## Modules

| Module | Contents |
|---|---|
| `install` | Download, verify, and install release binaries |
| `platform` | OS/arch detection (`Platform`, `Arch`) |
| `variants` | Build variant table (`Variant`, `all_variants`) |
| `version` | Version parsing and comparison (`Version`) |
| `tui` | Interactive install picker (`tui` feature) |
| `error` | `RlmError` / `Result` |

## Features

- `tui` (default) - interactive ratatui install picker

## License

Licensed under either of Apache License, Version 2.0 or MIT license at your option - see [LICENSE.md](../../LICENSE.md) for the full text.
