# Roadmap

Current and planned work for the rl-lang project.

<!--
## How to use this file

### Tree format

The project tree uses plain ASCII:
  - `|` for vertical lines
  - `+--` for branches
  - Indent with 4 spaces for sub-items under a branch

### Adding a new crate branch

  1. Add a `+-- rl-crate-name    (short description)` line under the root.
  2. Sub-items go under it indented with 4 extra spaces and `+--`.
  3. Tag each sub-item with a status: `[DONE]`, `[TODO]`, `[WIP]`, or `[BLOCKED]`.

### Adding sub-features to a crate

  - Use `[DONE]` for completed work.
  - Use `[TODO]` for planned work not yet started.
  - Use `[WIP]` for work in progress.
  - Use `[BLOCKED]` for work blocked on another task (note the dependency).
  - Use `[CLOSED]` for issues closed without the work landing (wontfix/superseded).
  - Use `[UNDECIDED]` for items awaiting a DONE-vs-CLOSED decision.

### Active Work section

  - One `### Crate Name` heading per crate with active tasks.
  - Each task is a numbered bold item with a short description.
  - Remove tasks from Active Work once they move to `[DONE]` in the tree.
  - Keep the list short: only current/priority work goes here.

### Rules

  - Tree must stay in sync with the Active Work section.
  - Don't add vague items. Each branch/task must be concrete and testable.
  - One `[WIP]` item per crate at a time. Finish it before starting another.
  - `[BLOCKED]` items must note what they are blocked on.
  - When a release ships, move `[DONE]` items out and clear Active Work.
-->

## Project Structure

```
rl-lang
|
+-- rl-utils          (error handling, source files, helpers)
+-- rl-lexer          (tokenization)
|   +-- [DONE] Arabic keyword aliases (all 38 keywords)
|   +-- [DONE] pipe operator `|>` token
|   +-- [DONE] #427 - allow more statements to have newlines
|   +-- [DONE] lexer edge case unit tests (Unicode identifiers, mixed scripts)
+-- rl-ast            (AST node types, arena)
|   +-- [CLOSED] #190 - refactor AST
|   +-- [UNDECIDED] arena allocation tests
+-- rl-parser         (source to AST)
|   +-- [DONE] optional semicolons as statement terminators
|   +-- [DONE] pipe operator `|>` desugaring (a |> f() -> a.f())
|   +-- [DONE] wildcard imports (`get * from std::ns`)
|   +-- [DONE] aliased imports (`get X as Y from std::ns`)
|   +-- [DONE] #341 - parser tests
|   +-- [UNDECIDED] parser internal unit tests (individual parse functions)
+-- rl-resolver       (name resolution, imports)
|   +-- [UNDECIDED] #342 - resolver tests
|   +-- [TODO] shadowing, scoping, import resolution edge cases
+-- rl-checker        (type checking)
|   +-- [DONE] #471 - `?` propagation constraint (reject outside result/cresult functions)
|   +-- [TODO] #348 - refinement and contracts
+-- rl-vm             (bytecode VM)
|   +-- [DONE] #345 - vm tests
|   +-- [TODO] #349 - property-based testing
|   +-- [UNDECIDED] compiler unit tests (compiler.rs, bytecode generation)
|   +-- [UNDECIDED] VM execution loop tests (vm_logic.rs)
|   +-- [UNDECIDED] native function dispatch tests
+-- rl-std-core       (core stdlib modules)
|   +-- [DONE] #425 - new math and consts functions
|   +-- [DONE] #414 - more string std functions
|   +-- [DONE] #437 - refactor term_set_title
|   +-- [DONE] extended stdlib (72 new functions across 11 modules)
|   +-- [DONE] collections expansion (16 fn: set_union/intersection/difference, heap, deque, bisect)
|   +-- [DONE] array expansion (8 fn: chunk, windows, swap, partition, max_by, min_by, zip_longest, cycle_take)
|   +-- [DONE] string expansion (13 fn: strip_prefix/suffix, lines, wrap, indent, dedent, is_alpha/numeric/whitespace)
|   +-- [DONE] types expansion (18 fn: is_uint, is_sbyte, is_array, is_map, is_gui_handle, etc.)
|   +-- [DONE] bitwise expansion (6 fn: rotate_left/right, bit_set/clear/toggle/is_set)
|   +-- [DONE] result expansion (2 fn: result_and_then, result_unwrap_or_else)
|   +-- [DONE] fs expansion (3 fn: copy_dir, dir_size, is_symlink)
|   +-- [DONE] debug expansion (2 fn: warn, stack_trace)
|   +-- [DONE] time expansion (1 fn: monotonic_now)
|   +-- [DONE] random expansion (1 fn: rand_seed)
|   +-- [DONE] gui expansion (6 fn: set_font_size, set_color, set_bg_color, set_tooltip, get_window_size, get_window_pos)
|   +-- [DONE] term expansion (1 fn: term_get_cursor_pos)
+-- rl-std-macros     (stdlib procedural macros)
+-- rl-std            (standard library)
|   +-- [DONE] std::process - exec_background, wait_pid, term_pid, kill_pid, pipe, pipe_all, set_env, remove_env, env_keys, os_name, arch, num_cpus, parent_pid, process_exists, exec_with_stdin, exec_with_env, exec_with_cwd, exec_with_timeout (+ with_* variants)
|   +-- [DONE] std::path - path_normalize, path_is_absolute, path_is_relative, path_split, path_split_extension, path_with_file_name, path_absolute, path_relative, path_canonicalize, path_starts_with, path_ends_with, path_join_many, path_expand_home, path_components
|   +-- [DONE] std::fs - touch, truncate_file, glob, walk_dir, symlink, readlink, hardlink, temp_file, temp_file_in, file_created, file_accessed, file_permissions, set_permissions, list_dir_names, realpath, lock_file, unlock_file
|   +-- [DONE] std::io - handle-based I/O (open, close, read_handle, write_handle, seek, flush, read_all, readline) with HandleKind::File, plus read_all_stdin, decode_utf8, encode_utf8, isatty
|   +-- [DONE] HandleKind::File - new handle variant in rl-ast with IoStore trait
|   +-- [DONE] extended stdlib (72 new functions across 11 modules)
|   +-- [DONE] 24 per-module feature flags (std-array, std-audio, std-bitwise, std-c, std-cli, std-collections, std-core, std-crypto, std-debug, std-fs, std-gui, std-http, std-io, std-math, std-net, std-path, std-process, std-random, std-result, std-serialize, std-string, std-terminal, std-time, std-types) with `impls` meta-feature
|   +-- [DONE] #431 - std functions aliasing (wildcard imports + aliased imports)
|   +-- [DONE] #338 - std functions tests
|   +-- [TODO] std::test - test framework (14 fn: test_case, test_run_all, test_assert_eq, test_group, test_bench)
|   +-- [DONE] std::serialize - JSON/CSV/TOML/INI/YAML interop (shipped 2.2.0)
|   +-- [DONE] std::crypto - hashing, HMAC, tokens, UUID, Argon2 (shipped 2.2.0)
|   +-- [DONE] std::cli - arg parsing, prompts, progress bars (shipped 2.2.0)
+-- rl-commons        (shared utilities)
|   +-- [TODO] keyword edge case tests (Unicode identifiers, mixed scripts, keyword-prefix identifiers)
+-- rl-cli            (CLI binary: run, check, new, dev, format, print, package, workflows, pm)
|   +-- [DONE] `rl print` command (token/parser/ast tree output)
|   +-- [DONE] `rl run -c` flag (inline code execution)
|   +-- [DONE] `rl new --lib` (generates src/lib.rl + [dependencies])
|   +-- [DONE] `rl pm` subcommand (install, add, remove, list, update, cache)
|   +-- [DONE] standalone binaries split (rl, rlc, rlt, rlrepl, rlsp, rldocs)
|   +-- [DONE] optional feature gates (vm, repl, docs, pm, cc)
|   +-- [DONE] #426 - add support for more OS
+-- rl-repl           (interactive REPL)
|   +-- [DONE] standalone rlrepl binary
+-- rl-lsp            (language server)
|   +-- [DONE] standalone rlsp binary
|   +-- [DONE] #298 - LSP module
|   +-- [DONE] hover, goto definition, rename, references tests
|   +-- [DONE] diagnostic conversion tests
+-- rl-docs           (documentation generator)
|   +-- [DONE] standalone rldocs binary
|   +-- [DONE] package_manager and toolchain_manager concept entries
|   +-- [DONE] #188 - update notes/
+-- rl-pm             (package manager)
|   +-- [DONE] cache management (~/.cache/rlpm/)
|   +-- [DONE] download and symlink logic
|   +-- [DONE] rl.toml [dependencies] parsing
|   +-- [DONE] import resolution (deps/{name}/lib.rl fallback)
|   +-- [UNDECIDED] download, cache, resolve, symlink tests
|   +-- [UNDECIDED] toml parsing edge case tests
+-- rl-manager        (toolchain manager)
|   +-- [DONE] TUI version/variant picker
|   +-- [DONE] self-update capability
|   +-- [DONE] --no-tui for CI
+-- rl-tooling        (packaging, install helpers)
|   +-- [DONE] shebang scripts (`rl new --script`)
|   +-- [DONE] tree print module (box-drawing output for tokens/statements)
|   +-- [DONE] build-variants.sh (7 binaries)
|   +-- [DONE] install.sh / install.ps1 (7 binaries, interactive picker)
+-- rl-tests          (integration tests)
|   +-- [DONE] stdlib tests (745 tests across 11 modules)
|   +-- [TODO] #333 - test units
|   +-- [TODO] #337 - test edge cases coverage
|   +-- [TODO] #344 - interpreter tests
|   +-- [TODO] error message quality tests (content, formatting, span accuracy)
|   +-- [TODO] missing stdlib function tests (arr_filter, arr_reduce, arr_find, arr_contains, arr_sort, arr_reverse)
|   +-- [TODO] full test coverage for other OSes (Windows, macOS, Android)
+-- rl-benches        (benchmarks)
+-- rl-cc             (C transpiler)
|   |
|   +-- [DONE] type mapping (int -> int64_t, string -> rl_string, etc.)
|   +-- [DONE] variable/constant/function declarations
|   +-- [DONE] control flow (if/else, while, for, foreach, forrange, loop, break, continue)
|   +-- [DONE] return with value
|   +-- [DONE] string escape sequences
|   +-- [DONE] C11 _Generic runtime for type-dispatched printing
|   +-- [DONE] reference-counted string type
|   +-- [DONE] transpile CLI command (--runtime, --compile, --opt)
|   +-- [DONE] boolean operators (and, or, !)
|   +-- [DONE] comparison operators (==, !=, <, >, <=, >=)
|   +-- [DONE] cast expressions (as)
|   +-- [DONE] byte/sbyte/big byte/big sbyte/small int/small uint/small float types
|   +-- [DONE] tuple literals and tuple destruction
|   +-- [DONE] array literals, index access, and index assignment
|   +-- [DONE] record/struct literals, field access, and field assignment
|   +-- [DONE] enum/tag literals and match
|   +-- [DONE] result type, ok/err/error literals, and ? propagation
|   +-- [DONE] rl_result tagged union, rl_value, rl_closure runtime structs
|   +-- [DONE] per-program record/tuple/enum print functions
|   +-- [DONE] map/set declarations, literals, and len
|   +-- [DONE] impl methods on records
|   +-- [DONE] foreach, forrange, and loop
|   +-- [DONE] closures / function pointers
|   +-- [DONE] null printing (nullable_vars tracking, rl_print_raw/rl_println_raw)
|   +-- [DONE] 10 missing stdlib functions (io::read_bytes, types::error_unwrap, random::*)
|   +-- [DONE] enum display (rl_print_Enum_{name} with string table)
|   +-- [DONE] 9 closure-consuming runtime functions (arr_for_each, arr_all, arr_any, etc.)
|   +-- [DONE] std::c FFI module (compile, load, call, has_symbol, close, clear_cache)
|   +-- [DONE] std::net module (tcp_listen, tcp_accept, tcp_connect, tcp_read, tcp_write, tcp_peer_addr, tcp_local_addr, tcp_set_timeout, tcp_set_nonblocking, tcp_shutdown, tcp_close, udp_bind, udp_connect, udp_send, udp_send_to, udp_recv, udp_recv_from, udp_close, resolve)
|   +-- [DONE] result_unwrap type dispatch (str, f64, bool, i64)
|   +-- [DONE] result_unwrap_or fixed (.data.ok_value removed)
|   +-- [TODO] codegen unit tests (statements, expressions, ops, scope)
|   +-- [TODO] name mangling tests
|   +-- [TODO] type mapping tests
|   +-- [TODO] runtime embedding tests
|   +-- [TODO] refactor: split codegen.rs into modules (statements, expressions, ops, scope, types)
|   +-- [TODO] refactor: extract runtime embedding into reusable template
|   +-- [TODO] refactor: unify type mapping with rl-vm type system
|   +-- [TODO] refactor: clean up name mangling (avoid collisions with C reserved words)
|   +-- [TODO] feature parity: arr_filter, arr_map, arr_reduce, arr_find, arr_contains
|   +-- [TODO] feature parity: map_keys, map_values, map_merge, map_remove, map_clear
|   +-- [TODO] feature parity: set operations (union, intersection, difference)
|   +-- [TODO] feature parity: std::io handle-based I/O
|   +-- [TODO] feature parity: std::process (exec, env, pipe)
|   +-- [TODO] feature parity: std::time functions
|   +-- [TODO] feature parity: std::math extended functions (sin, cos, pow, sqrt)
|   +-- [TODO] feature parity: closures with capture (currently missing While/Match bodies)
|   +-- [TODO] feature parity: match on enums (tagged union dispatch)
|   +-- [TODO] better: improve error messages (line/column info, helpful hints)
|   +-- [TODO] better: optimize generated C code (dead code elimination, constant folding)
|   +-- [TODO] better: add --optimize flag for release builds (-O2/-O3)
|   +-- [TODO] better: support multiple output formats (C, LLVM IR, WASM)
|   +-- [TODO] better: add rl cc command for direct compilation without transpile step
|   +-- [TODO] len() only works on strings - fails on arrays/maps/sets
|   +-- [TODO] Map literals silently drop non-string keys
|   +-- [TODO] arr_filter/arr_map/etc. fallthrough - named function args generate wrong C name
|   +-- [TODO] Lambda body compilation incomplete - missing While, Match, ForEach, Break, Continue
|   +-- [TODO] Lambda capture collection misses While, Match, ForEach bodies
|   +-- [TODO] Nested arrays coerce to int64 - ArrayLiteral hardcodes element type
|   +-- [TODO] tagged union dispatch for match on enums
|   +-- [DONE] std::http module (http_get, http_post, http_request, http_server_start/recv/try_recv/respond/stop)
|   +-- [DONE] multi-file / module transpilation
+-- (language)
|   +-- [DONE] pipe operator |>
|   +-- [DONE] #429 - type aliasing
|   +-- [DONE] #375 - more types
|   +-- [TODO] self-hosted buffers - RL-side renderer, lexer, and C-emitter over safe buffers (no raw pointers), in that order
+-- scripts/
    +-- [DONE] scripts/build-local.sh (--release/--nightly/--dev, --clean, output to target-bins/)
    +-- [DONE] scripts/bump-version.sh (patch/minor/major/explicit)
    +-- [TODO] rl-native versions of build-local.sh and bump-version.sh
```

## Dependency Status

Heavy deps that must stay but are properly feature-gated:

| Dep | Transitive | Gate | Location |
|-----|-----------|------|----------|
| eframe | 765 | std-gui | rl-std |
| tower-lsp + tokio | 217 | lsp | rl-cli |
| rodio + cpal + symphonia | 198 | std-audio | rl-std |
| ureq + tiny_http | 124 | std-http | rl-std |
| ratatui + crossterm | 106 | tui | rl-docs, rl-manager |
| zip | 66 | - | rl-manager |
| libffi + libloading | 12 | std-c | rl-std |

## Open Issues (github.com/rl-lang/rl-lang/issues)

| # | Title | Labels |
|---|-------|--------|
| 280 | related issues tracker | tracking |
| 333 | `Test Units` | tracking |
| 337 | Tests edge cases coverage | - |
| 338 | `std` functions tests | - |
| 342 | `resolver` tests | - |
| 348 | feat `Refinement and Contracts` | language |
| 349 | feat `Property-based testing` | language |

## Active Work

1. **`std::test` test framework** - user-facing test module (`test_case`, `test_run_all`, `test_assert_eq`, `test_group`, `test_bench`)
2. **Self-hosted buffers** - RL-side renderer, lexer, and C-emitter over safe buffers (no raw pointers)

## Planned (After Self-Host)

Deferred until the self-hosted toolchain lands:

- `std::threads` - OS thread spawn/join (requires VmValue Send+Sync)
- `std::sync` - mutex, channels (requires VmShared + fork_shared)
