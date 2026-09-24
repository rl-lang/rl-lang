# rl-lang VM

## Overview

The `rl`-lang VM is a stack-based bytecode interpreter. It compiles the resolved AST to bytecode, then executes it in a tight dispatch loop.

## Compiler

### Entry Point

```rust
// crates/rl-vm/src/compiler.rs:180
pub fn compile(ast: &Ast, statements: &[Statement], stdlib: Module) -> Result<Chunk, VmError>
```

Single-pass AST-to-bytecode compiler. Walks the resolved AST and emits a `Chunk` (bytecode + constants + source spans).

### Compiler Struct

```rust
pub struct Compiler<'a> {
    ast: &'a Ast,
    chunk: Chunk,
    next_slot: u16,
    scope_bases: Vec<u16>,
    stdlib: Module,
    loop_stack: Vec<LoopCtx>,
    source: Option<SourceFile>,
}
```

### Entry Point Detection

- If the program has a `!#[entry]` attribute or a `main` function, it compiles declarations/imports first, then emits orchestration calls (tests -> inits -> entry -> finals)
- If no entry point, every top-level statement is compiled sequentially (script mode)

### Variable Resolution

The compiler uses a flat slot model. `resolve(depth, slot)` converts a resolver-assigned (depth, slot) pair into a flat slot index:
- If depth is 0 relative to current scope -> `GetLocal`/`SetLocal`
- Otherwise -> `GetGlobal`/`SetGlobal`

### Jump Patching

```rust
fn emit_jump(&mut self, op: OpCode) -> usize {
    self.chunk.write_op(op);
    self.chunk.write_u16(0xFFFF);  // placeholder
    self.chunk.code.len() - 2      // return position for patching
}

fn patch_jump(&mut self, offset: usize) {
    let jump = self.chunk.code.len() - offset - 2;
    self.chunk.code[offset..offset+2].copy_from_slice(&(jump as u16).to_le_bytes());
}
```

## VM

### Entry Point

```rust
// crates/rl-vm/src/vm_logic.rs:410
pub fn run(&mut self, chunk: &Chunk) -> Result<(), VmError>
```

Creates an initial `CallFrame` for the top-level chunk and executes.

### VM Struct

```rust
pub struct Vm {
    stack: Vec<VmValue>,
    globals: Vec<VmValue>,
    locals: Vec<VmValue>,
    scope_starts: Vec<usize>,
    current_ip: usize,
    current_chunk: *const Chunk,
    source: Option<SourceFile>,
    impl_methods: HashMap<String, Rc<VmFunction>>,
    stdlib_methods: HashMap<String, VmNative>,
    user_methods: HashMap<String, Rc<VmFunction>>,
    // Handle tables for C, audio, GUI, net, HTTP, I/O
    rng: Xoshiro256,
    output_buffer: Option<String>,
}
```

## Opcodes (47 total)

### Stack Operations
| Opcode | Description |
|--------|-------------|
| `Const` | Push constant from pool |
| `Pop` | Discard top |
| `Return` | Return from call frame |

### Arithmetic
| Opcode | Description |
|--------|-------------|
| `Add`, `Sub`, `Mul`, `Div` | Binary arithmetic |
| `Negate` | Unary minus |

### Comparisons
| Opcode | Description |
|--------|-------------|
| `Eq`, `NotEq` | Equality |
| `Less`, `LessEq`, `Greater`, `GreaterEq` | Ordering |

### Logic
| Opcode | Description |
|--------|-------------|
| `Not` | Logical not |

### Variables
| Opcode | Description |
|--------|-------------|
| `GetLocal`, `SetLocal` | Local variable access |
| `GetGlobal`, `SetGlobal` | Global variable access |
| `DefineLocal` | Define from top-of-stack |

### Scope
| Opcode | Description |
|--------|-------------|
| `PushScope`, `PopScope` | Enter/exit scope |

### Control Flow
| Opcode | Description |
|--------|-------------|
| `Jump` | Unconditional forward jump |
| `JumpIfFalse` | Conditional jump |
| `Loop` | Backward jump |

### Functions
| Opcode | Description |
|--------|-------------|
| `Call` | Call function with N args |
| `BuildClosure` | Create closure |

### Collections
| Opcode | Description |
|--------|-------------|
| `BuildArr`, `BuildTuple`, `BuildSet`, `BuildMap` | Construct collections |
| `Index`, `ArrSet` | Index access/assignment |
| `ArrLen` | Array length |

### Records
| Opcode | Description |
|--------|-------------|
| `BuildRecord` | Construct record |
| `FieldGet`, `FieldSet` | Field access/assignment |

### Result Type
| Opcode | Description |
|--------|-------------|
| `Ok`, `Err`, `Error` | Wrap values |
| `Propagate` | `?` operator |

### Methods
| Opcode | Description |
|--------|-------------|
| `RegisterMethod` | Register impl method |
| `RegisterStdlibMethod` | Register stdlib as method |
| `RegisterUserMethod` | Register user fn as method |
| `LookupAssoc` | Look up associated function |
| `LookupMethod` | Resolve instance method |

### Other
| Opcode | Description |
|--------|-------------|
| `Cast` | Numeric type cast |

## Chunk

```rust
pub struct Chunk {
    pub code: Vec<u8>,           // bytecode instructions
    pub constants: Vec<VmValue>, // constant pool
    pub spans: Vec<Span>,        // per-byte source spans
}
```

Each instruction is 1 byte (opcode) + 0-2 bytes (u16 little-endian operand).

## Call Frames

```rust
struct CallFrame<'a> {
    source: FrameSource<'a>,
    ip: usize,
    scope_base: usize,
    frame_base: usize,
}
```

All call frames share the same `locals` vector. Each frame's locals start at `scope_starts[frame.scope_base]`.

## Dispatch Loop

The dispatch loop in `run_frames` maintains these variables in local scope for performance:
- `cur_chunk: *const Chunk` - raw pointer to current chunk
- `ip: usize` - instruction pointer
- `scope_base: usize` - current frame's scope base index
- `frame_base: usize` - cached locals offset

The loop reads an opcode, advances ip, and matches on it. Most operations are simple: pop operands, compute, push result.

## Stack Layout

```
Function call:  ... [callee] [arg_0] [arg_1] ... [arg_N-1]

Closure:        locals[base..base+captured] = upvalues
                locals[base+captured..] = parameters
```

## Performance Characteristics

- **Dispatch**: 1-byte opcodes, no operand decoding overhead for simple ops
- **Locals**: Direct Vec indexing, no hash lookups
- **Constants**: Direct Vec indexing
- **Calls**: Stack frame management, no heap allocation per call
- **Bounds checking**: `pop_unchecked()` for hot-path arithmetic (unsafe)
