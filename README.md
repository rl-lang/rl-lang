<img src="logo.svg" width="200">

# RL — a simple interpreted language written in Rust

RL is a statically-typed interpreted language with a clean syntax, a TUI REPL, and a growing standard library. It runs `.rl` source files or interactively via a terminal REPL.

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
# run a source file
rl run <file.rl>

# start the TUI REPL
rl repl
```

## Language reference

### Types

| Type        | Example                          |
|-------------|----------------------------------|
| `int`       | `42`                             |
| `float`     | `3.14`                           |
| `bool`      | `true`, `false`                  |
| `string`    | `"hello"`                        |
| `char`      | `'x'`                            |
| `arr[T]`    | `[1, 2, 3]`                      |
| `fn`        | `fn(int x) { return x * 2 }`    |
| `null`      | implicit uninitialized value     |

### Variables and constants

```rl
// mutable
dec int    count  = 0
dec float  ratio  = 1.618
dec string name   = "rl"
dec bool   flag   = true
dec char   letter = 'r'

// immutable
CONST int    MAX   = 100
CONST string LANG  = "rl"

// arrays
dec arr[int]    nums = [1, 2, 3]
CONST arr[float] weights = [0.1, 0.4, 0.5]
```

### Operators

**Arithmetic** — `+`, `-`, `*`, `/`

**Compound assignment** (desugared at parse time)

```rl
x += 3   // x = x + 3
x -= 3   // x = x - 3
x *= 3   // x = x * 3
x /= 3   // x = x / 3
```

**Comparison** — `==`, `!=`, `<`, `>`, `<=`, `>=`

**Boolean** — `!` (negation)

### Comments

```rl
// single-line comment
```

### Imports

```rl
get println from std::display
get sin, cos from std::math
get PI from std::math::consts
get to_upper, trim from std::str
```

### Control flow

```rl
if (score >= 90) {
    println("A")
} else if (score >= 75) {
    println("B")
} else {
    println("F")
}
```

```rl
while (condition) {
    // body
}
```

**For loops — three forms:**

```rl
// C-style
for [int i = 0, i < 10, i += 1] { }

// range
for i in 0..10 { }

// iterable
for item in my_array { }
```

`break` and `continue` are supported inside all loop forms.

### Functions

```rl
fn add(int a, int b) {
    return a + b
}
```

**First-class functions and lambdas:**

```rl
dec fn double = fn(int x) { return x * 2 }
dec fn square = fn(int x) { return x * x }

dec arr[fn] transforms = [double, square]
println(transforms[0](5))  // 10
println(transforms[1](5))  // 25

fn apply(fn f, int x) {
    return f(x)
}
println(apply(double, 6))  // 12
```

## Standard library

Import with `get <names> from std::<module>`.

### `std::display`

| Function        | Description                              |
|-----------------|------------------------------------------|
| `print(args)`   | print values separated by spaces         |
| `println(args)` | same, with a trailing newline            |
| `len(arr)`      | number of elements in an array           |

### `std::io`

| Function      | Description                         |
|---------------|-------------------------------------|
| `input()`     | read a line from stdin              |
| `input(prompt)` | print prompt then read a line     |

### `std::math`

| Function          | Description                                     |
|-------------------|-------------------------------------------------|
| `sin(x)`          | sine (radians)                                  |
| `cos(x)`          | cosine (radians)                                |
| `tan(x)`          | tangent (radians)                               |
| `asin(x)`         | arc sine                                        |
| `acos(x)`         | arc cosine                                      |
| `atan(x)`         | arc tangent                                     |
| `atan2(y, x)`     | two-argument arc tangent                        |
| `pow(base, exp)`  | exponentiation                                  |
| `mod(a, b)`       | modulo                                          |
| `abs(x)`          | absolute value                                  |
| `sqrt(x)`         | square root                                     |
| `ceil(x)`         | ceiling                                         |
| `floor(x)`        | floor                                           |
| `round(x)`        | round to nearest                                |
| `clamp(x, lo, hi)`| clamp x between lo and hi                      |
| `min(a, b)`       | minimum                                         |
| `max(a, b)`       | maximum                                         |
| `log(x, base)`    | logarithm                                       |
| `log2(x)`         | base-2 logarithm                                |
| `log10(x)`        | base-10 logarithm                               |
| `exp(x)`          | eˣ                                              |
| `degrees(x)`      | radians → degrees                               |
| `radians(x)`      | degrees → radians                               |
| `hypot(a, b)`     | hypotenuse                                      |
| `gcd(a, b)`       | greatest common divisor                         |
| `lcm(a, b)`       | least common multiple                           |
| `lerp(a, b, t)`   | linear interpolation                            |
| `map_range(...)`  | remap a value from one range to another         |
| `sign(x)`         | sign of x (-1, 0, or 1)                        |
| `factorial(n)`    | n!                                              |
| `fibonacci(n)`    | nth Fibonacci number                            |
| `is_prime(n)`     | primality test                                  |

**`std::math::consts`** — `PI`, `TUA` (2π), `E`, `PHI`, `SQRT_2`, `LN_2`, `LN_10`, `INF`, `NAN`, and more. All constants are zero-argument functions: `PI()`.

### `std::str`

| Function                 | Description                         |
|--------------------------|-------------------------------------|
| `to_upper(s)`            | uppercase                           |
| `to_lower(s)`            | lowercase                           |
| `trim(s)`                | strip leading and trailing whitespace |
| `trim_start(s)`          | strip leading whitespace            |
| `trim_end(s)`            | strip trailing whitespace           |
| `concat(args...)`        | concatenate strings                 |
| `repeat(s, n)`           | repeat string n times               |
| `reverse(s)`             | reverse string                      |
| `contains(s, sub)`       | substring check                     |
| `starts_with(s, prefix)` | prefix check                        |
| `ends_with(s, suffix)`   | suffix check                        |
| `replace(s, from, to)`   | replace first occurrence            |
| `slice(s, start, end)`   | substring by index                  |
| `split(s, delim)`        | split into array                    |
| `join(arr, sep)`         | join array into string              |
| `char_at(s, i)`          | character at index                  |
| `index_of(s, sub)`       | first index of substring            |
| `count(s, sub)`          | count occurrences                   |
| `is_empty(s)`            | empty check                         |
| `pad_left(s, n, c)`      | left-pad to width                   |
| `pad_right(s, n, c)`     | right-pad to width                  |
| `bytes(s)`               | byte values as `arr[int]`           |
| `chars(s)`               | characters as `arr[char]`           |

### `std::types`

Type conversion and inspection.

| Function      | Description                    |
|---------------|--------------------------------|
| `to_int(x)`   | convert to int                 |
| `to_float(x)` | convert to float               |
| `to_bool(x)`  | convert to bool                |
| `to_char(x)`  | convert to char                |
| `to_string(x)`| convert to string              |
| `to_hex(x)`   | convert to hex string           |
| `to_bin(x)`   | convert to binary string        |
| `to_oct(x)`   | convert to octal string         |
| `is_int(x)`   | type check                     |
| `is_float(x)` | type check                     |
| `is_bool(x)`  | type check                     |
| `is_char(x)`  | type check                     |
| `is_string(x)`| type check                     |
| `is_null(x)`  | type check                     |

## Editor support

### VS Code

Install the [rl-lang extension](https://github.com/MohamedGonem/vscode-rl) for syntax highlighting in `.rl` files.

### Tree-sitter

A Tree-sitter grammar is available at [MohamedGonem/tree-sitter-rl](https://github.com/MohamedGonem/tree-sitter-rl) for editors that support it (Neovim, Helix, Zed, etc.).

Highlight queries are also bundled in this repo at `highlight/highlights.scm`.

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

Feature flags: `run`, `repl`, `repl_tui`, `repl_terminal`, `eval`, `debug`. All are on by default except `debug` and `repl_terminal`.

## Contributors

- [FlawlessDeveloper](https://github.com/FlawlessDeveloper) — native Rust function binding system (`IntoNativeFn`, `FromValue`, `IntoValue`), module system, and Ariadne-based error reporting

## License

Licensed under either of [MIT](MIT-LICENSE) or [Apache 2.0](APACHE-LICENSE) at your option.
