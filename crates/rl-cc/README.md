# rl-cc

C transpiler for the rl programming language. Converts `.rl` source files to C99, which can then be compiled with `gcc`, `clang`, or any C compiler.

## Usage

```bash
rlt file.rl                    # output file.c
rlt file.rl --compile          # output file.c + compile to binary
rlt file.rl -o out             # output out.c
rlt file.rl --compile --opt 2  # compile the C with -O2
rlt file.rl --runtime          # also emit rl_runtime.h + rl_runtime.c next to the output
rlt file.rl --compile -lm      # extra flags are forwarded to cc
rlt file.rl --compile --test # build the test-driver binary instead (runs !#[test] cases, exits non-zero on failure)
```

## Supported Features

- **Types**: int, float, string, bool, char, byte, null, array, map, set, tuple, record/tag (struct/enum), result
- **Declarations**: dec, const, functions, impl methods on records
- **Control flow**: if/else, while, for, foreach, forrange, loop, break, continue, match
- **Expressions**: arithmetic, boolean (and/or/!), comparison, cast, propagate (?), lambda, closures
- **Imports**: multi-file support via `get X from path::module` - imported code is resolved at resolve time and inlined into the generated C output
- **FFI**: `std::c` module - compile C source at runtime (`compile`), load shared libraries (`load`), call C functions dynamically (`call` with libffi), symbol lookup (`has_symbol`), cleanup (`close`, `clear_cache`). Compile with `-DRL_USE_LIBFFI -lffi -ldl`
- **Networking**: `std::net` module - TCP (`tcp_listen`, `tcp_accept`, `tcp_connect`, `tcp_read`, `tcp_write`, `tcp_peer_addr`, `tcp_local_addr`, `tcp_set_timeout`, `tcp_set_nonblocking`, `tcp_shutdown`, `tcp_close`), UDP (`udp_bind`, `udp_connect`, `udp_send`, `udp_send_to`, `udp_recv`, `udp_recv_from`, `udp_close`), DNS (`resolve`). Uses POSIX sockets with a handle table.
- **Stdlib**: 370+ mapped functions across io, math, string, array, collections, time, random, terminal, fs, process, path, types, debug, result
- **Runtime**: `rl_result` tagged union, `rl_value` for map/set storage, `rl_string`, `rl_array`, `rl_map`, `rl_set`, `rl_closure`, per-program record/tuple/enum print functions

## Architecture

```
rl source -> rl-lexer -> rl-parser -> rl-resolver -> rl-checker -> rl-cc codegen -> .c file
```

The generated C code includes `rl_runtime.h` and `rl_runtime.c` (the C runtime), which provides all the type representations, print functions, and stdlib implementations.

## Files

- `src/lib.rs` - public API: `transpile()`, `TranspileConfig`, `TranspileResult`
- `src/codegen/` - C code generation (statements, expressions, scope, operators)
- `src/types.rs` - rl-to-C type mapping
- `src/name_mangle.rs` - identifier escaping for C reserved words
- `src/writer.rs` - CWriter output buffer
- `runtime/rl_runtime.h` - C runtime header (types, declarations)
- `runtime/rl_runtime.c` - C runtime implementation
