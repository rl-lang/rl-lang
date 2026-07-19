# rl-vm

> Bytecode virtual machine for the rl-lang programming language

Part of the [rl-lang](https://github.com/rl-lang/rl-lang) workspace. This is the experimental, alternative execution path to the tree-walking `rl-interpreter` - it compiles the AST to bytecode and runs it on a stack-based VM instead of walking the tree directly.

## Pipeline

```text
Vec<Statement> -> Compiler::compile() -> Chunk -> Vm::run() / Vm::run_and_return()
```

## Modules

| Module | Contents |
|---|---|
| `compiler` | `Compiler` - compiles a resolved AST into a `Chunk` of bytecode |
| `bytecode` | `serialize_chunk` / `deserialize_chunk` - persisting compiled chunks to disk (used by `rl-tooling`'s package/embed step) |
| `chunk` | `Chunk` and `OpCode` - the bytecode representation |
| `vm_logic` | `Vm` - the stack-based bytecode interpreter, plus `VmError` |
| `native` | `Module` and `NativeFn` - native function binding for the VM |
| `values` | `VmValue` and `VmNativeFn` - the VM's runtime value representation |
| `stdlib` | Standard library modules exposed to VM-compiled code |

## Dependencies

Builds on `rl-ast`, `rl-lexer`, and `zstd` (for compressed bytecode serialization).

## Usage

```toml
[dependencies]
rl-vm = { workspace = true }
```

```rust
use rl_vm::{Compiler, Vm};

let chunk = Compiler::new(&ast).compile(&statements)?;
let result = Vm::new().run_and_return(&chunk)?;
```

> **Note:** the VM backend is highly experimental - it is exposed behind the `rl-cli` `vm` feature and is not yet a drop-in replacement for the tree-walking interpreter.

## License

Licensed under either of Apache License, Version 2.0 or MIT license at your option - see [LICENSE.md](../../LICENSE.md) for the full text and why both are offered.
