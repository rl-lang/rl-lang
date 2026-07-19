# rl-cranelift

> Cranelift-based JIT compiler backend for the rl-lang programming language

Part of the [rl-lang](https://github.com/rl-lang/rl-lang) workspace. An experimental, optional backend that JIT-compiles `rl-vm` bytecode chunks down to native machine code via [Cranelift](https://cranelift.dev/), instead of interpreting them.

## Overview

`rl-cranelift` takes a `Chunk` produced by `rl-vm`'s `Compiler` and translates its `OpCode` stream into Cranelift IR, then JITs and runs it, returning the result as an `i64`. It's enabled in `rl-cli` via the optional `cranelift` feature.

## Dependencies

Builds on `rl-vm`, and the `cranelift-jit`, `cranelift-module`, `cranelift-codegen`, `cranelift-frontend`, and `target-lexicon` crates from the Cranelift project.

## Usage

```toml
[dependencies]
rl-cranelift = { workspace = true }
```

```rust
use rl_cranelift::run_chunk;

let result = run_chunk(&chunk)?; // -> i64
```

> **Note:** this backend is experimental and currently supports only a subset of what the tree-walking interpreter and bytecode VM support.

## License

Licensed under either of Apache License, Version 2.0 or MIT license at your option - see [LICENSE.md](../../LICENSE.md) for the full text and why both are offered.
