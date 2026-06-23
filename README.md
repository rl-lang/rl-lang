<div align="center">
  <img src="logo-circle.svg" width="200">
  <h1>RL</h1>
  <p>A statically-typed interpreted language written in Rust with a clean syntax, a TUI REPL, and a growing standard library.</p>
</div>

[![Discord](https://img.shields.io/badge/Discord-5865F2?style=for-the-badge&logo=discord&logoColor=white)](https://discord.gg/9T9mB4VJB)
[![Rust](https://img.shields.io/badge/Made%20with-Rust-000000?style=for-the-badge&logo=rust&logoColor=white)](https://www.rust-lang.org/)

[![Website](https://img.shields.io/website?url=https%3A%2F%2Frl-lang.github.io%2Fthe-book%2F&label=Wiki&message=online)](https://rl-lang.github.io/the-book/)
[![License](https://img.shields.io/badge/License-MIT%20OR%20Apache--2.0-blue)](https://github.com/rl-lang/rl-lang/blob/main/LICENSE)
[![Last Commit](https://img.shields.io/github/last-commit/rl-lang/rl-lang)](https://github.com/rl-lang/rl-lang/commits/main)
[![Check CI](https://github.com/rl-lang/rl-lang/actions/workflows/check.yaml/badge.svg)](https://github.com/rl-lang/rl-lang/actions/workflows/check.yaml)
[![Release](https://github.com/rl-lang/rl-lang/actions/workflows/release.yml/badge.svg)](https://github.com/rl-lang/rl-lang/actions/workflows/release.yml)
[![Crates.io](https://img.shields.io/crates/v/rl-lang)](https://crates.io/crates/rl-lang)
[![Crates.io Downloads](https://img.shields.io/crates/d/rl-lang)](https://crates.io/crates/rl-lang)
[![GitHub Repo stars](https://img.shields.io/github/stars/rl-lang/rl-lang?style=social)](https://github.com/rl-lang/rl-lang)

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

## Installation

### From source

```bash
git clone https://github.com/rl-lang/rl-lang
cd rl-lang
cargo build --release
# binary at target/release/rl
```

### Via cargo install

```bash
cargo install rl-lang
```

## Usage

```bash
rl run <file.rl>    # run a source file
rl repl             # start the TUI REPL
rl check <file.rl>  # check for errors without running
rl dev              # run project via rl.toml
rl new <name>       # create a new project
rl docs             # print language reference
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

Criterion benchmarks live in `benches/v0_1_0.rs`. Run with:

```bash
cargo bench
```

## Development

```bash
cargo test --all-features   # full test suite
cargo clippy -- -D warnings # lints
cargo bench                 # criterion benchmarks
```

Feature flags: `run`, `repl`, `repl_tui`, `eval`. All are on by default except `debug` and `lsp` (unstable but works).

## Contributors

<!-- ALL-CONTRIBUTORS-LIST:START -->
<!-- prettier-ignore-start -->
<!-- markdownlint-disable -->
<table>
  <tbody>
    <tr>
      <td align="center" valign="top" width="14.28%"><a href="https://github.com/MohamedGonem"><img src="https://avatars.githubusercontent.com/u/73464078?v=4?s=100" width="100px;" alt="Mohamed Gonem"/><br /><sub><b>Mohamed Gonem</b></sub></a><br /><a href="https://github.com/rl-lang/rl-lang/commits?author=MohamedGonem" title="Code">💻</a> <a href="https://github.com/rl-lang/rl-lang/commits?author=MohamedGonem" title="Documentation">📖</a> <a href="#ideas-MohamedGonem" title="Ideas, Planning, & Feedback">🤔</a> <a href="https://github.com/rl-lang/rl-lang/commits?author=MohamedGonem" title="Tests">⚠️</a> <a href="#example-MohamedGonem" title="Examples">💡</a> <a href="#infra-MohamedGonem" title="Infrastructure (Hosting, Build-Tools, etc)">🚇</a> <a href="#tool-MohamedGonem" title="Tools">🔧</a></td>
      <td align="center" valign="top" width="14.28%"><a href="https://yt-dl.flawlessco.de"><img src="https://avatars.githubusercontent.com/u/37778817?v=4?s=100" width="100px;" alt="Flawlesscode"/><br /><sub><b>Flawlesscode</b></sub></a><br /><a href="https://github.com/rl-lang/rl-lang/commits?author=FlawlessDeveloper" title="Code">💻</a></td>
      <td align="center" valign="top" width="14.28%"><a href="https://github.com/kill-ux"><img src="https://avatars.githubusercontent.com/u/185858933?v=4?s=100" width="100px;" alt="Mustapha Boutoub"/><br /><sub><b>Mustapha Boutoub</b></sub></a><br /><a href="https://github.com/rl-lang/rl-lang/commits?author=kill-ux" title="Code">💻</a></td>
      <td align="center" valign="top" width="14.28%"><a href="https://github.com/RubyPaws39"><img src="https://avatars.githubusercontent.com/u/280778337?v=4?s=100" width="100px;" alt="RubyPaws39"/><br /><sub><b>RubyPaws39</b></sub></a><br /><a href="https://github.com/rl-lang/rl-lang/commits?author=RubyPaws39" title="Code">💻</a></td>
      <td align="center" valign="top" width="14.28%"><a href="https://github.com/CapThunder19"><img src="https://avatars.githubusercontent.com/u/161865581?v=4?s=100" width="100px;" alt="Anirudh Patwal"/><br /><sub><b>Anirudh Patwal</b></sub></a><br /><a href="https://github.com/rl-lang/rl-lang/commits?author=CapThunder19" title="Code">💻</a></td>
    </tr>
  </tbody>
</table>

<!-- markdownlint-restore -->
<!-- prettier-ignore-end -->

<!-- ALL-CONTRIBUTORS-LIST:END -->

## License

Licensed under either of [MIT](MIT-LICENSE) or [Apache 2.0](APACHE-LICENSE) at your option.
