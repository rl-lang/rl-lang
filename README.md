<div align="center">
<img src="logo.svg" width="200">

# RL - a simple interpreted language written in Rust

RL is a statically-typed interpreted language with a clean syntax, a TUI REPL, and a growing standard library. It runs `.rl` source files or interactively via a terminal REPL.

</div>

## Quick look

```rl
get println, len from std::display
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
git clone https://github.com/MohamedGonem/rl-lang
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

Full language reference and stdlib documentation is available on the [wiki](https://github.com/MohamedGonem/rl-lang/wiki).

## Editor support

### VS Code

Install the [rl-lang extension](https://github.com/MohamedGonem/vscode-rl) for syntax highlighting in `.rl` files.

### Tree-sitter

A Tree-sitter grammar is available at [MohamedGonem/tree-sitter-rl](https://github.com/MohamedGonem/tree-sitter-rl) for editors that support it (Neovim, Helix, Zed, etc.).

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

- [FlawlessDeveloper](https://github.com/FlawlessDeveloper) — native Rust function binding system (`IntoNativeFn`, `FromValue`, `IntoValue`), module system, and Ariadne-based error reporting

## License

Licensed under either of [MIT](MIT-LICENSE) or [Apache 2.0](APACHE-LICENSE) at your option.
