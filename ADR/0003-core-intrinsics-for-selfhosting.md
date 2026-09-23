# ADR-0003: `core::` intrinsics for self-hosting

Date: 2026-09-23
Author: Mohamed Gonem

## Status
Accepted

## Context
If the stdlib is going to be written in RL, RL needs a floor to stand on. Right now there isn't one: nothing in the language can build a map from thin air, ask what type a value is, bail out, or trap into the kernel. Every one of those is a Rust native today, which means an RL-written stdlib would just be wrappers around Rust code - relabeling, not self-hosting.

At the same time, two things are already settled and should stay settled: numeric conversions belong to the `as` operator (`300 as byte` errors, `to_byte(300)` wraps - checked vs wrapping, both deliberate), and syscalls deserve a specified escape hatch for the day libc isn't there (see: `rlos` ambitions).

## Decision
One flat `core::` module of `__`-prefixed intrinsics, in three layers:

1. `core::__*` - what RL genuinely cannot say: build and poke containers, ask types, abort, raw syscall trap. Compiler-lowered, not called.
2. Native fns - what needs the kernel or C: syscalls, FFI, math, time, entropy. Unchanged, still Rust.
3. RL stdlib - everything else. Sorting, formatting, parsing, HMAC: all ordinary RL functions on top of (1) and (2).

Flat on purpose (the module is the namespace), `__` on purpose (reads as "compiler magic, think twice"), and the resolver owns the name `core` so nobody can shadow or redefine it. Missing keys and bad indexes abort - primitives don't do soft errors, matching `m[k]` rather than `map_get`. RL code that wants `result` builds it on top.

Starting set, nothing more: `__arr_new/push/get/set`, `__map_new/get/set/keys`, `__set_new/add/has`, `__abort`, `__type_of`, `__syscall6`. Missing keys, bad indexes, non-sets and unhashable values abort - primitives don't do soft errors, matching `m[k]` rather than `map_get`. Strings and the rest follow the same shape once this slice proves itself.

`__syscall6(nr, a1..a6) -> int` is specified now, not later: raw return register, `-errno` is the caller's problem, Linux-only and proud of it. VM lowers it through `libc::syscall`, CC through `syscall(2)`, x86 through the real trap (`rdi rsi rdx r10 r8 r9` - `r10`, not `rcx`, that's the syscall ABI, don't get cute). Portable code keeps using `std::fs`/`std::io`; this exists for the day there's no libc to call.

## Alternatives considered
- `__` globals without a module - rejected: magic names loose in the global namespace with nowhere to document them.
- `core::` without the prefix - rejected: `map_get` looks callable-casual; `__map_get` looks like what it is (aborts on missing keys).
- No `__syscall6`, natives only - rejected: blocks libc-free binaries with no way back; specifying it now is nearly free.
- Raw pointers instead of handles - rejected: buys C's entire memory-bug catalog with no borrow checker to pay for it. Handles plus ints already cover FFI; revisit with measurements, not vibes.

## Scope
Standard Library (new `core` tree), Resolver (`core` reservation), Type Checker (`core::*` signatures), backends (lower like native calls for now; real opcode/IR-op lowering rides Phase B).

## Breaking Changes & Migrations
Additive. Only breakage: anyone who already claimed a top-level `core` module (nobody in-tree) must rename.

## Examples
### Before the change:
```rl
# maps only come from literals or stdlib calls
dec m = {"a": 1}
```

### After the change:
```rl
get __map_new, __map_set, __map_get, __abort from core

dec m = __map_new()
__map_set(m, "a", 1)
dec int x = __map_get(m, "a")
dec int y = __map_get(m, "nope")  # aborts: key not found in map
__abort("unreachable")            # -> T, fits any storage type
```

## Consequences
- RL-written stdlib becomes possible, starting with pure logic (`collections`, string algorithms) on top of `core::`.
- `as` stays the only conversion story. No cast intrinsics, ever.
- `__syscall6` exists but is openly Linux-only; portable code pretends it isn't there.
- Until Phase B, `core::` lowers like ordinary natives with a stability promise instead of special opcodes. The promise is the feature.

## Related
- ADR-0001 (self-hosting direction), ADR-0002 (backend consolidation)
- `crates/rl-vm/src/vm_logic.rs` (`cast`, `index_get` - the semantics intrinsics mirror)
- `crates/rl-std/src/c.rs` (`CArg` - why handles suffice instead of pointers)
