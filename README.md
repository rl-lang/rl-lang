<div align="center">
  <img src="assets/logo-circle.svg" width="200">
  <h1>RL</h1>
  <p>A statically-typed interpreted language written in Rust with a clean syntax, a TUI REPL, and a growing standard library.</p>
  <p>Toolchain: <code>rl</code> (core CLI) · <code>rlc</code> (compiler) · <code>rlt</code> (C transpiler) · <code>rlrepl</code> (REPL) · <code>rlsp</code> (LSP) · <code>rldocs</code> (docs) · <code>rlm</code> (toolchain manager)</p>
</div>

<!-- Static Project Info -->
[![Discord](https://img.shields.io/badge/Discord-5865F2?style=for-the-badge&logo=discord&logoColor=white)](https://discord.gg/9T9mB4VJB)
[![Rust](https://img.shields.io/badge/Made%20with-Rust-000000?style=for-the-badge&logo=rust&logoColor=white)](https://www.rust-lang.org/)
[![License](https://img.shields.io/badge/License-MIT%20OR%20Apache--2.0-blue?style=for-the-badge)](https://github.com/rl-lang/rl-lang/blob/main/LICENSE)

<!-- Website Status -->
[![Website](https://img.shields.io/website?url=https%3A%2F%2Frl-lang.github.io%2Fthe-book%2F&label=Wiki&message=online&style=for-the-badge)](https://rl-lang.github.io/the-book/)
[![Website](https://img.shields.io/website?url=https%3A%2F%2Frl-lang.github.io%2Frl-lang%2F&label=api-docs&message=online&style=for-the-badge)](https://rl-lang.github.io/rl-lang/)

<!-- Repository & Package Metrics -->
[![Last Commit](https://img.shields.io/github/last-commit/rl-lang/rl-lang?style=for-the-badge)](https://github.com/rl-lang/rl-lang/commits/main)
[![Crates.io](https://img.shields.io/crates/v/rl_cli?style=for-the-badge)](https://crates.io/crates/rl-cli)
[![Crates.io Downloads](https://img.shields.io/crates/d/rl_cli?style=for-the-badge)](https://crates.io/crates/rl-cli)
[![GitHub Repo stars](https://img.shields.io/github/stars/rl-lang/rl-lang?style=for-the-badge)](https://github.com/rl-lang/rl-lang)

<!-- CI/CD -->
[![Check CI](https://github.com/rl-lang/rl-lang/actions/workflows/check.yaml/badge.svg)](https://github.com/rl-lang/rl-lang/actions/workflows/check.yaml)
[![Release](https://github.com/rl-lang/rl-lang/actions/workflows/release.yml/badge.svg)](https://github.com/rl-lang/rl-lang/actions/workflows/release.yml)

## Quick look

```rl
get println, len from std::io
get pow, mod, factorial, fibonacci, is_prime from std::math
get PI from std::math::consts

fn collatz(int n) {
    dec int steps = 0
    while (n != 1) {
        if (mod(n, 2) == 0) {
            n = n / 2
        } else {
            n = n * 3 + 1
        }
        steps += 1
    }
    return steps
}

println(factorial(10))    // 3628800
println(fibonacci(15))    // 610
println(is_prime(97))     // true
println(collatz(27))      // 111

dec float r = 5.0
println(PI() * pow(r, 2.0))  // 78.53981633974483
```

Statements are separated by newlines. Semicolons (`;`) are optional and can be used as an alternative statement terminator.

All keywords have Arabic equivalents (e.g. `دالة` for `fn`, `لكل` for `for`, `بينما` for `while`, `أرجع` for `return`). Identifiers may freely mix Arabic and Latin characters.

The pipe operator `|>` chains function calls left-to-right: `"hello" |> to_upper()` becomes `"hello".to_upper()`.

## Installation

### Via install script (recommended)

Prebuilt binaries are published for every [release](https://github.com/rl-lang/rl-lang/releases). The install script downloads the build you pick (or the latest stable) and puts it on your PATH.

**Linux / WSL** (installs to `$HOME/.local/bin`; set `RL_INSTALL_DIR` to override):

```bash
curl -fsSL https://raw.githubusercontent.com/rl-lang/rl-lang/main/install.sh -o install.sh
bash install.sh
```

**Android (Termux)** - works the same way on aarch64 devices (install script detects Termux and downloads the Android build):

```bash
curl -fsSL https://raw.githubusercontent.com/rl-lang/rl-lang/main/install.sh -o install.sh
bash install.sh
```

Non-interactively (install the standard `rl` build of `v1.0.0`):

```bash
bash install.sh v1.0.0
```

**Windows (PowerShell)** (installs to `%LOCALAPPDATA%\rl-lang\bin` and adds it to your user PATH - restart your terminal afterwards):

```powershell
Invoke-WebRequest https://raw.githubusercontent.com/rl-lang/rl-lang/main/install.ps1 -OutFile install.ps1
.\install.ps1
```

The installer lets you pick which binaries to install: `rl`, `rlc`, `rlt`, `rlrepl`, `rlsp`, `rldocs`, `rlm`.

### From source

```bash
git clone https://github.com/rl-lang/rl-lang
cd rl-lang/crates/rl-cli
cargo build --release
# binary at target/release/rl
```

### Via cargo install

```bash
cargo install rl-cli
```

### Via releases

from [releases](https://github.com/rl-lang/rl-lang/releases) you can choose `nightly` builds or the `latest` build

## Usage

```bash
# check available commands
rl --help

# start a new project
rl new example-project

# in project root
rl dev              # run project
rl run src/main.rl  # run file directly
rl check            # type-check only

# standalone binaries
rlc compile src/main.rl  # compile to .rlc bytecode
rlc run src/main.rl      # lex+parse+check+compile+run
rlt transpile src/main.rl --compile  # transpile to C and compile
rlrepl                   # interactive REPL
rldocs --tui             # browse docs in TUI
rlsp                     # LSP server for editors

# package manager
rl pm install some-package
rl pm list

# toolchain manager
rlm install
rlm list
```

## Documentation

Full language reference and stdlib documentation is available on the [wiki](https://rl-lang.github.io/the-book/).

## Editor support

### VS Code

- Install the [rl-lang extension](https://github.com/rl-lang/vscode-rl) for syntax highlighting in `.rl` files.
- Install the [rl-lang runner extension](https://github.com/rl-lang/vscode-rl-lang) to run and check files from the editor.
- Install the [rl-lang LSP extension](https://github.com/rl-lang/vscode-rl-lsp) for diagnostics and hover.

### Tree-sitter

A Tree-sitter grammar is available at [rl-lang/tree-sitter-rl](https://github.com/rl-lang/tree-sitter-rl) for editors that support it (Neovim, Helix, Zed, etc.).

## Benchmarks

Criterion benchmarks live in `crates/rl-benches`. Run with:

```bash
cargo bench
```

## Development

```bash
cargo test --all-features   # full test suite
cargo clippy -- -D warnings # lints
cargo bench                 # criterion benchmarks
```

Feature flags:

| Flag        | State              | Description  |
| :---------: | :----------------: | :----------: |
| `vm`        | `On` by default    | Bytecode VM backend |
| `repl`      | `On` by default    | Interactive TUI REPL |
| `docs`      | `On` by default    | Documentation tooling |
| `docs-tui`  | `On` by default    | Interactive TUI mode for docs |
| `pm`        | `On` by default    | Package manager |
| `cranelift` | `Off` experimental | Cranelift JIT backend |
| `debug`     | `Off` by default   | Logging and debugging |

Per-module std feature flags (all on by default, disable for custom builds):

`std-array`, `std-audio`, `std-bitwise`, `std-c`, `std-collections`, `std-debug`, `std-fs`, `std-gui`, `std-http`, `std-io`, `std-math`, `std-net`, `std-path`, `std-process`, `std-random`, `std-result`, `std-string`, `std-terminal`, `std-time`, `std-types`

## Contributors

<!--
  Deprecation notice:
    The all contributors bot integration is deprecated for this repo
    migrated contributors showcase to contrib.rocks
-->

<a href="https://github.com/rl-lang/rl-lang/graphs">
  <img src="https://contrib.rocks/image?repo=rl-lang/rl-lang" />
</a>
<!-- made with contrib.rocks (many thanks :D) -->

## License

Licensed under either of [MIT](LICENSE-MIT.md) or [Apache 2.0](APACHE-LICENSE) at your option.
