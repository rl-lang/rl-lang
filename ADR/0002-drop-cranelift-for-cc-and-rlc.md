# ADR-0002: Drop Cranelift in favor of rl-cc and the RL compiler

Date: 2026-09-23
Author: Mohamed Gonem

## Status
Accepted

## Context
Three native-code paths exist or are planned: `rl-cc` (C transpiler, full stdlib parity, byte-identical e2e apps), `rl-cranelift` (7/47 opcodes, int-only, unmaintained), and the planned self-hosted RL compiler (rlc). Cranelift's 7-op int-only state contributes nothing `rl-cc` doesn't already do better, and every backend multiplies the cost of the coming IR migration (each needs re-targeting). Self-hosting wants exactly two code producers: the portable one (C) and the native one (RL-written).

## Decision
Remove the `rl-cranelift` crate, its `cranelift` CLI feature and `--cranelift` flags. Native-code ambitions move to (1) `rl-cc` as the complete portable backend, and (2) the upcoming RL-written compiler (rlc) as the native backend, both consuming the shared IR from Phase B. No Cranelift API may be reintroduced without a new ADR.

## Alternatives considered
- Keep Cranelift and grow it - rejected: duplicates `rl-cc`'s job at 7/47 ops with no maintainer; growing it costs more than its eventual value under self-hosting.
- Keep it frozen but present - rejected: dead code rots, still pays IR-migration cost, signals a supported path that isn't.

## Scope
Compiler backend (removal), CLI (`--cranelift` flags and the `cranelift` feature), workspace dependencies (cranelift-* crates).

## Breaking Changes & Migrations
Anyone invoking the Cranelift path moves to `rlt` (C transpiler); observable behavior is a superset. No language change, no stdlib change. `rl run --cranelift` becomes an error suggesting `rlt --compile`.

## Examples
### Before the change:
`rl run --cranelift prog.rl` works for int-only programs.
### After the change:
The flag is rejected; `rl run prog.rl` (VM) or `rlt prog.rl --compile` (native via C) cover all programs.

## Consequences
- One fewer backend to migrate when the IR lands; `rl-cc` becomes the sole native-code story until rlc exists.
- Loses Cranelift's register allocator and asm output as a reference; the x86 backend (Phase C) must solve allocation itself (planned: stack machine first, linear scan later).
- Frees the `rlc` name from confusion with Cranelift-based compilation.

## Related
- ADR-0001 (self-hosting direction)
- `crates/rl-cranelift/src/lib.rs` (7/47 opcode coverage, removed by this decision)
