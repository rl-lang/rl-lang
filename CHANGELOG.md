# Changelog

All notable changes to the rl-lang toolchain are documented here. The format is based on [Keep a Changelog](https://keepachangelog.com/), and the project follows [Semantic Versioning](https://semver.org/) (see [VERSIONING.md](VERSIONING.md)). Full per-commit history is available on the [GitHub Releases](https://github.com/rl-lang/rl-lang/releases) page.

## [Unreleased]

### Added

- **Test framework (`std::test`, `rl test`)** - attribute-driven tests: `!#[test]` (plain or with `group("g")` / `register("r")` / `cases(N)` params), `!#[setup]` / `!#[teardown]` hooks, and a 6-function runtime API (`test_skip`, `test_skip_if`, `test_assert_eq`, `test_assert_ne`, `test_assert_panics`, `test_assert_no_panic`) with non-fatal accumulation. `rl test` discovers, filters (`--match`), and runs them with setup/case/teardown and a non-zero exit on failure; tests never run under `rl run`. `rlt --test` builds the same runner as a C binary (setjmp/longjmp abort capture; property tests report a skip there).
- **Contracts (`requires`/`ensures`/refinements)** - runtime-checked contracts: parameter refinements (`int amt: >0`, cross-parameter like `balance: >=amt`), `requires` preconditions with messages, `ensures` postconditions over `ret` (inner value for `result[T]`, skipped on `Err`). Violations return `Err` for `result` functions and abort otherwise. Desugared pre-resolution, so VM and C backends enforce identically.
- **Property tests (`cases(N)`)** - type-directed generators (bounded ints narrowed by refinements, floats, bools, strings, arrays; maps/sets/tuples/records/chars fail cleanly as out of scope), deterministic fixed-seed replay, greedy shrinking to minimal failing inputs reported by the runner.
- **Static contract proving (phase 2)** - parameter refinements and `requires` clauses with constant arguments prove at compile time: violations are errors, satisfied and unknown predicates stay silent for the runtime guards. Literals only; `ensures` stays runtime-only.

### Removed

- **Deprecated stdlib dupes** - removed `std::array::len` (use top-level `std::len`), the `std::io` file ops (`read_file`, `read_lines`, `read_bytes`, `write_file`, `append_file`, `delete_file`) and the `std::path` syscall ops (`path_exists`, `path_is_dir`, `path_is_file`, `path_canonicalize`, `path_absolute`, `path_expand_home`, `path_relative`) in favor of their `std::fs` canonical paths. Old paths now fail with `undefined function`; their checker deprecation entries, docs pages, and in-repo callers (including `transpile_demo.rl`) are migrated. `std::rl` deprecations are untouched.

## [2.2.0] - 2026-09-24

### Added

- **`std::cli`** - command-line interface helpers (VM + `rl-cc` C parity, outputs byte-identical). Hand-rolled `parse_args` over `process::args` with `--name value` / `--name=value` / `-s value` forms, spec as array of string maps (`flag` is string-encoded `"true"`/`"false"` because VM maps are homogeneous), `parse_args_or_exit` (usage + exit 2), `usage_string`, `prompt` / `prompt_password` / `prompt_confirm` / `prompt_choice`, `shell_split` / `shell_join`, rustyline-backed `read_line_editable` / `read_line_with_history`, and hand-rolled `progress_bar` / `spinner_tick` routed through the output buffer for test capture. Drops everything through the first `--` so `rl run prog.rl -- ...` scripts only see their own args.
- **`std::crypto`** - hashing (SHA-256/512, SHA-1/MD5 verify-only), HMAC, constant-time compare, CSPRNG tokens, base64/hex, UUID v4/v7, Argon2 passwords (VM + `rl-cc` C parity, vectors byte-identical; C Argon2 links libargon2 via `-DRL_USE_ARGON2`). Integer literals coerce element-wise into `array[byte]` params (range-checked); empty `[]` adopts the needed array element type.
- **`std::serialize`** - JSON/CSV/TOML/INI/YAML text interop (VM + `rl-cc` C parity, outputs byte-identical; C YAML links libyaml via `-DRL_USE_YAML`). JSON numbers without fraction/exponent become `int`, `null` becomes `null`; stringifiers need string-keyed maps. Serialized key order is sorted on both backends.
- **`core::` intrinsics** - compiler-blessed primitives for self-hosting (VM + `rl-cc` C parity, outputs identical modulo map order): `__arr_*` (new/push/get/set/remove/len), `__map_*` (new/get/set/remove/has/keys/len), `__set_*` (new/add/has/remove/len), `__str_len/get_byte/slice/concat`, `__syscall6` (Linux-only), `__abort`, `__type_of`. Abort-on-misuse semantics (missing keys, out-of-bounds, non-sets), mirroring `Index`. The name `core` is reserved; numeric casts stay with the `as` operator; CC maps are string-keyed.
- **Wildcard imports** - `get * from std::<module>` imports every public function from a stdlib module as a bare name. Works with the VM, CC transpiler, and type checker. Cannot be mixed with named imports in the same statement.
- **Aliased imports** - `get <fn> as <alias> from std::<module>` renames an imported function locally. The alias is used for all subsequent calls. Supports mixing aliased and plain imports: `get sin as sine, cos from std::math`.
- **`?` propagation constraint** - the type checker now rejects `?` outside functions that return `result[T]` or `cresult[T]`. Top-level `?` in script mode is allowed. This catches misuse at compile time instead of silently producing undefined behavior.
- **`std::process::exec_fg` / `with_exec_fg`** - foreground process execution with inherited stdin/stdout/stderr. Unlike `exec` (which pipes stdout), `exec_fg` lets interactive programs (editors, TUIs, pagers) access the terminal directly. Returns the exit code as `int`. Intended pattern: call `term_leave()` before `exec_fg`, then `term_enter()` after to restore the TUI.
- **Standalone binaries** - the rl-lang toolchain is now split into focused binaries:
  - `rl` - core CLI (run, check, new, dev, format, print, package, workflows, pm)
  - `rlc` - lean compiler and runner (compile .rl to .rlc, or run source/bytecode directly)
  - `rlt` - transpiler to C99 (with optional `--compile` to invoke cc)
  - `rlrepl` - interactive TUI REPL
  - `rlsp` - LSP server for editor integration
  - `rldocs` - documentation viewer (TUI, JSON, Markdown output)
  - `rlm` - toolchain manager (install, update, uninstall, list)
- **`rlm` (rl manager)** - standalone toolchain manager with TUI version/variant picker, `--no-tui` for CI, self-update capability, and SHA256 verification
- **`rl-pm` (package manager)** - manages project dependencies via `rl pm`: install, add, remove, list, update, cache clean. Dependencies declared in `rl.toml` under `[dependencies]`
- **Per-module std feature flags** - 20 feature flags (`std-array`, `std-audio`, `std-bitwise`, `std-c`, `std-collections`, `std-debug`, `std-fs`, `std-gui`, `std-http`, `std-io`, `std-math`, `std-net`, `std-path`, `std-process`, `std-random`, `std-result`, `std-string`, `std-terminal`, `std-time`, `std-types`) with `impls` as meta-feature enabling all. All enabled by default; per-module flags are for custom builds
- **`rl new --lib`** - generates `src/lib.rl` with `[dependencies]` section in rl.toml
- **`rl dev` dependency warning** - warns if rl.toml is missing a `[dependencies]` section
- **`scripts/build-local.sh`** - local build script with `--release`/`--nightly`/`--dev` profiles, `-j` for parallel jobs, `--clean` to wipe target, outputs to `target-bins/`
- **`handle` in function signatures** - `handle` now parses as a param type and `->` return (also inside `result[handle]`), matching any concrete handle kind in either direction, so handles flow through user functions on the VM and `rl-cc` alike
- **Undeclared return-type inference** - functions without `->` infer their return from the body (explicit `return`s win, else the trailing expression); all candidates must agree on one concrete type. A body using `?` infers `result[T]`, mirroring a `-> result[T]` annotation, and `?` is allowed inside undeclared bodies. `rl-cc` definitions consult the inferred type so trailing expressions actually return
- **`core::__result_ok_value` / `__result_err_value`** - trust-and-verify result assertion: return the ok/err payload with no static questions beyond the `result` shape, abort loudly on the wrong variant or a non-result (VM + `rl-cc` C parity; CC reuses the typed `rl_result_unwrap_*` family, aborting via `_rl_abort`)
- **`any[T, ...]` union types (option A)** - a value of any one member type with sticky declared types (no flow narrowing): member matching + subset rules in the checker, numeric `as` narrowing, `?` over all-result unions, member-wise operators/indexing/methods that fail loudly naming offenders. Parser normalizes (flatten, dedupe, trailing commas; empty/single/bare are errors). VM runs boxed; `rl-cc` stores scalars/containers/handles in `rl_value` (boxing + converting unboxers) and fails loudly for records/tags/tuples/bytes/closures/results. `match __type_of` is the general narrowing ceremony
- **`is` type tests with branch refinement** - `x is T` reads as bool (comparison precedence; containers test shape only, mirroring `__type_of`). `if`/`while` refine plain-variable bindings in the taken branch (`else` gets the union minus the tested type), dropping the refinement past any reassignment of the variable and never across function boundaries. VM lowers via a new `IsKind` opcode (kind codes + nominal names); `rl-cc` folds concrete operands statically and checks `rl_value` tags for dynamic ones. `for x in arr` accepts all-array unions with merged element types
- **`scripts/install-local.sh`** - install locally built binaries from `target-bins/` to `~/.local/bin/`, with interactive binary picker and `--force` overwrite
- **rl-docs concepts** - new documentation entries for package manager and toolchain manager
- **`rl-cli` lib target** - shared pipeline module (lex, parse, vm, cc) for binary targets
- **`std::collections` expansion** - 16 new functions for set operations, map defaults, heap, deque, and sorted insertion:
  - Set algebra: `set_union`, `set_intersection`, `set_difference`, `set_symmetric_difference`, `set_is_subset`, `set_is_superset`
  - Map defaults: `map_get_or`, `map_get_or_insert`
  - Min-heap: `heap_push`, `heap_pop`, `heap_peek`
  - Deque: `deque_push_front`, `deque_pop_front`
  - Sorted array: `bisect_left`, `bisect_right`, `sorted_insert`
- **`std::array` expansion** - 8 new functions for chunking, windowing, partitioning, and zipping:
  - `arr_chunk`, `arr_windows`, `arr_swap`, `arr_partition`, `arr_max_by`, `arr_min_by`, `arr_zip_longest`, `arr_cycle_take`
- **`std::str` expansion** - 13 new functions for prefix/suffix stripping, line splitting, indentation, and character classification:
  - `strip_prefix`, `strip_suffix`, `last_index_of`, `split_once`, `lines`, `wrap`, `indent`, `dedent`, `diff_lines`, `is_alpha`, `is_numeric`, `is_whitespace`, `unicode_category`
- **`std::types` expansion** - 18 new type-checking predicates for extended numeric types, collections, and handles:
  - Numeric: `is_uint`, `is_sbyte`, `is_bsbyte`, `is_bbyte`, `is_sint`, `is_suint`, `is_sfloat`
  - Collections: `is_array`, `is_map`, `is_set`, `is_tuple`, `is_function`
  - Handles: `is_c_handle`, `is_net_handle`, `is_http_handle`, `is_audio_handle`, `is_gui_handle`, `is_file_handle`
- **`std::bitwise` expansion** - 6 new functions with full type overloads (byte, sbyte, bbyte, bsbyte, sint, suint, int, uint):
  - `rotate_left`, `rotate_right`, `bit_set`, `bit_clear`, `bit_toggle`, `bit_is_set`
- **`std::result` expansion** - 2 new monadic functions:
  - `result_and_then`, `result_unwrap_or_else`
- **`std::fs` expansion** - 3 new functions:
  - `copy_dir` (recursive directory copy), `dir_size` (total size in bytes), `is_symlink`
- **`std::debug` expansion** - 2 new functions:
  - `warn` (colored yellow `[warn]` output to stderr), `stack_trace` (capture current call stack)
- **`std::time` expansion** - 2 new functions:
  - `monotonic_now` (monotonic clock for benchmarks)
  - `format_time` now supports 21 strftime tokens: `%Y`, `%y`, `%m`, `%B`, `%b`, `%d`, `%A`, `%a`, `%w`, `%j`, `%U`, `%W`, `%V`, `%H`, `%I`, `%M`, `%S`, `%p`, `%P`, `%z`, `%Z`
- **`std::random` expansion** - 1 new function:
  - `rand_seed` (re-seed the PRNG for deterministic output)
- **`std::gui` expansion** - 6 new functions for widget styling and window queries:
  - `gui_set_font_size` (set text size), `gui_set_color` (set foreground RGB), `gui_set_bg_color` (set background RGB), `gui_set_tooltip` (hover tooltip)
  - `gui_get_window_size` (query window dimensions), `gui_get_window_pos` (query window position)
- **`std::term` expansion** - 1 new function:
  - `term_get_cursor_pos` (get current cursor column and row)
- **rl-cc VM parity: calls and program structure** - method-call dispatch against imported stdlib functions, user functions and record impls (mirroring the VM, including `x.std::ns::f()` paths and chaining); aliased imports resolve to canonical paths; `!#[entry]`, `main` fallback, `!#[test]`, `!#[init[=n]]`/`!#[final[=n]]` orchestration with VM priority ordering; top-level variables emit as C file-scope globals; unannotated declarations infer C types from initializers and stdlib signatures; silent catch-alls became compile errors.
- **rl-cc VM parity: closures** - trailing-expression lambda bodies return the value; closures accepted as values, not just literals; full type mapping for params and captures; heap-allocated captures so closures escape safely; lambda bodies support the full statement set; factory functions tracked through calls; immediately-invoked lambdas; `result_unwrap_or_else` and `result_and_then`.
- **rl-cc stdlib coverage** - newly supported in transpiled programs, all verified identical against the VM:
  - Bitwise: `rotate_left`, `rotate_right`, `bit_set`, `bit_clear`, `bit_toggle`, `bit_is_set`
  - Path: `path_is_absolute`, `path_is_relative`, `path_starts_with`, `path_ends_with`, `path_normalize`, `path_absolute`, `path_canonicalize`, `path_expand_home`, `path_split`, `path_split_extension`, `path_components`, `path_with_file_name`, `path_relative`, `path_join_many`
  - String: `strip_prefix`, `strip_suffix`, `last_index_of`, `split_once`, `lines`, `wrap`, `indent`, `dedent`, `diff_lines`, `is_alpha`, `is_numeric`, `is_whitespace`, `unicode_category`
  - IO/debug/types/random/time/term: `read_all_stdin`, `decode_utf8`, `encode_utf8`, `warn`, `stack_trace`, `is_array`, `is_map`, `is_set`, `is_tuple`, `is_function`, `is_uint`, `is_sbyte`, `is_bsbyte`, `is_bbyte`, `is_sint`, `is_suint`, `is_sfloat`, `rand_seed`, `monotonic_now`, `term_get_cursor_pos`
  - Array: `arr_chunk`, `arr_windows`, `arr_swap`, `arr_partition`, `arr_max_by`, `arr_min_by`, `arr_zip_longest`, `arr_cycle_take`
  - Collections: `set_union`, `set_intersection`, `set_difference`, `set_symmetric_difference`, `set_is_subset`, `set_is_superset`, `map_get_or`, `map_get_or_insert`, `heap_push`, `heap_pop`, `heap_peek`, `deque_push_front`, `deque_pop_front`, `bisect_left`, `bisect_right`, `sorted_insert`
  - Process: `set_env`, `remove_env`, `env_keys`, `arch`, `num_cpus`, `parent_pid`, `process_exists`, `with_exec_fg`, `exec_with_stdin`, `with_exec_with_stdin`, `exec_with_env`, `with_exec_with_env`, `exec_with_cwd`, `with_exec_with_cwd`, `exec_with_timeout`, `with_exec_background`, `pipe`, `pipe_all`, plus `os_name`, `exec_fg`, `exec_background`, `process_running`, `wait_pid`, `term_pid`, `kill_pid`, `isatty`, `file_created`, `touch`
  - FS: file-handle API (`open`, `close`, `read_handle`, `write_handle`, `seek`, `flush`, `read_all`, `readline`) with a handle table, plus `list_dir_names`, `file_accessed`, `file_permissions`, `set_permissions`, `temp_file`, `temp_file_in`, `truncate_file`, `glob`, `walk_dir`, `symlink`, `readlink`, `hardlink`, `realpath`, `lock_file`, `unlock_file`, `path_canonicalize`, `path_absolute`, `path_expand_home`, `path_relative`, `copy_dir`, `dir_size`, `is_symlink`, `rmdir`, `move_file`, `rename_file`
  - Handles: `is_c_handle`, `is_net_handle`, `is_http_handle`, `is_file_handle` (`is_audio_handle`/`is_gui_handle` report false; no backends yet)
- **rl-cc http/net return shapes** - `http_get`/`http_post`/`http_request` and `udp_recv_from` return real tuples; socket handles use tagged ids per module; `udp_bind` returns a handle.
- **rl-cc std::audio** - all 18 audio functions work in transpiled programs via vendored miniaudio (playback, handles, volume, devices, metadata). The miniaudio implementation compiles only for programs using audio (`RL_USE_AUDIO`); other programs skip the 4MB header entirely.
- **Deprecations** - the type checker now warns on deprecated stdlib functions called by bare name, not just qualified paths (aliases and wildcards resolve to canonical paths). `std::rl` (`lex`, `eval`, `eval_isolated`, `check`, `rl_version`, `source_name`) is deprecated ahead of self-hosting, when metaprogramming moves to an RL-written library (see ADR-0001). Deprecation docs flags filled in for moved `io` handle functions and `path_relative`.

### Changed

- **`rl` stripped** - removed `compile`, `transpile`, `repl`, `lsp`, `docs` commands from main `rl` binary (moved to standalone binaries)
- **`rl-vm` optional in `rl-cli`** - the `vm` feature enables `rl-vm`, `rl` binary no longer bundles everything
- **`rl-pm` optional in `rl-cli`** - the `pm` feature enables `rl-pm`
- **`rl-repl` optional in `rl-cli`** - the `repl` feature enables `rl-repl`
- **CI matrix simplified** - 7 jobs (one per platform-arch) instead of 28, each building all binaries
- **Install scripts updated** - now install all 7 binaries instead of old variants, with interactive binary picker
- **Module declarations gated** - rl-std and rl-vm gate module registration behind per-module feature flags
- **`std::debug::warn` now outputs in yellow** - the `[warn]` prefix is ANSI-colored (yellow) on terminals that support it.

### Fixed

- **`styled_text`/`paint_bg` gated** - now properly gated behind `#[cfg(feature = "impls")]` in gui.rs
- **rl-cc record typedefs buffer to file scope** - record struct definitions and printers emitted during the header scan now buffer ahead of top-level globals (same ordering fix as tuples), so annotated record declarations and record-returning functions transpile
- **rl-cc correctness fixes** - `arr_insert` argument order was swapped; `==`/`!=` on strings now compares contents; comparisons yield RL bools printing `true`/`false`; `env` returns a bare string or null; `to_int`/`to_float`/`to_bool`/`to_string`/`to_bin`/`to_hex`/`to_oct` dispatch on payload tags with VM error messages; `format`/`concat` render `ok(...)` for wrapped arguments; `http`/`term`/`process`/`fs` functions return the shapes the VM returns (`result[...]` vs bare); terminal input uses crossterm-style key names with raw mode and coordinate validation; `term_get_size` returns an array and `term_set_title` takes a string; aborts flush stdout first; non-TTY stdout is line-buffered like Rust; array runtime is element-width generic (`push`, `contains`, `sort`, `reverse`, `zip` builds real tuples); tuple types buffer to file scope; non-ASCII identifiers hex-escape in name mangling; `print`/`println` are variadic; `std::gui::*` and `std::rl::eval`/`lex`/`check` fail at transpile time with clear errors.

### Removed

- **Old build variants** - removed `rl_no_docs`, `rl_no_repl`, `rl_debug`, `rl_vm`, `rl_lsp`, and all variant combinations from CI and install scripts

### Documentation

- **Imports concept updated** - new documentation entries for wildcard imports (`get * from std::math`), aliased imports (`get sin as sine from std::math`), and mixed alias/plain imports. Added pitfall entry for wildcard+named import mixing restriction.
- **Tooling concept updated** - reflects new binary separation (rl, rlc, rlt, rlrepl, rlsp, rldocs, rlm)
- **Package manager concept** - full documentation for `rl pm` commands and rl.toml dependencies
- **Toolchain manager concept** - full documentation for `rlm install`, `update`, `list`, `uninstall`

## [2.1.0] - 2026-09-09

### Added

- **`std::fs` reorganization** - the `fs` module is now the single home for everything disk-facing:
  - File I/O functions (`read_file`, `read_lines`, `read_bytes`, `write_file`, `append_file`, `delete_file`) moved from `std::io`.
  - Handle-based streaming I/O (`open`, `close`, `read_handle`, `write_handle`, `seek`, `flush`, `read_all`, `readline`) moved from `std::io`.
  - Path filesystem predicates (`path_exists`, `path_is_dir`, `path_is_file`, `path_canonicalize`, `path_absolute`, `path_expand_home`) moved from `std::path`.
  - `path_relative` moved from `std::path` (uses `getcwd` syscall).
  - Old paths still work but emit deprecation warnings. `std::io` is now console-only; `std::path` is now pure string manipulation only.
- **Numeric suffix sugar** - write `10_u8`, `3.14_f32`, `100_i32` etc. to create typed literals directly without `as` casts. Supported suffixes: `_u8`, `_i8`, `_u16`, `_i16`, `_i32`, `_u32`, `_f32`, `_i64`, `_u64`, `_f64`. Equivalent to `10 as byte`, `3.14 as small float`, etc.
- **Extended numeric support for stdlib functions** - math and bitwise functions now accept all 10 numeric types (`byte`, `sbyte`, `bbyte`, `bsbyte`, `int`, `uint`, `sint`, `suint`, `float`, `sfloat`):
  - Math: `abs`, `ceil`, `floor`, `round`, `clamp`, `max`, `min`, `mod`, `pow`, `log`, `sqrt`, `log2`, `log10`, `sin`, `cos`, `tan`, `atan`, `acos`, `asin`, `degrees`, `radians`, `exp`, `sign`, `atan2`, `hypot`, `lerp`, `map_range`.
  - Integer helpers: `factorial`, `fibonacci`, `gcd`, `lcm`, `is_prime`.
  - Bitwise: `bit_and`, `bit_or`, `bit_xor`, `bit_not`, `bit_shift_left`, `bit_shift_right`, `count_bits`, `leading_zeros`, `trailing_zeros`.
  - `Runtime::as_i64` now returns all integer-type variants (`byte`, `uint`, `sbyte`, etc.) but NOT floats. `Runtime::as_f64` returns all numeric variants (integer and float).
- **Warning severity system** - the type-checker now emits warnings (yellow) that don't block execution, separate from errors (red) that do. Warnings are reported via `checker.warnings` alongside `checker.errors`, and include colored output with `[Warning: ...]` labels.
- **Unused variable/function warnings** - the checker reports unused variables and functions at the end of scope. Functions with `!#[entry]`, `!#[init]`, `!#[final]`, or `!#[test]` attributes are automatically marked as used. When no `!#[entry]` exists, `main` is treated as the implicit entry point and won't warn.
- **`!#[allow(unused)]`** - suppresses unused variable/function warnings for the annotated declaration. Works on `dec`, `const`, and `fn` declarations.
- **`!#[deprecated("msg")]`** - marks variables, constants, and functions as deprecated. Using a deprecated item emits a yellow warning. Works on user-defined items and stdlib functions. `!#[deprecated]` (without message) also works.
- **`!#[allow(deprecated)]`** - suppresses deprecation warnings for the annotated declaration or scope.
- **`std::len`** - new top-level stdlib function for getting the length of strings, arrays, and tuples. `std::array::len` is now deprecated in favor of `std::len`.
- **Stdlib deprecation checking** - the checker warns when calling deprecated stdlib functions (e.g. `std::array::len`). The deprecation map is in `rl-checker/src/lib.rs`.
- **`deprecated` and `updated` fields for doc entries** - `FnEntry` now has `deprecated: Option<&str>` and `updated: Option<&str>` fields. The markdown and HTML doc renderers display deprecation notices and version metadata.
- **Faster builds** - new `nightly` Cargo profile (thin LTO, codegen-units=4) and `dev-release` profile (no LTO, codegen-units=16). Nightly and debug builds are significantly faster. `build-variants.sh` now builds variants in parallel.
- **macOS builds** - release and nightly now produce x86_64 and aarch64 macOS binaries.
- **SHA256 checksums** - release artifacts include `.sha256` files. Install scripts verify checksums before extracting.
- **Nightly changelog** - nightly releases now include a commit-based changelog since the last nightly build.
- **Skip unchanged nightlies** - nightly builds skip when the `dev` branch hasn't changed since the last build.
- **Installer improvements** - `install.sh` and `install.ps1` now support `--help`, `--prefix`/`-p`, `--force`/`-f`, `--variant`/`-v`, and `--uninstall` flags.
- **Man page and info page** - `rl.1` (groff) and `rl.info` (texinfo) are now included in the repository under `man/`. Install scripts automatically install them to `share/man/man1/` and `share/info/` when present in the release archive.
- **Package templates** - packaging templates added under `packages/` for Debian, Fedora RPM, Arch PKGBUILD, Gentoo ebuild, Nix derivation, Homebrew formula, Chocolatey, WinGet, Snap, and Flatpak. Includes `PUBLISHING.md` with per-platform submission steps.

### Changed

- **`term_set_title` accepts any string** - `std::term::term_set_title` now takes a `string` argument instead of `int`/`byte`. Any value can be set as the terminal window title, not just numeric byte values.
- **GitHub issue/PR templates** - issue templates converted from Markdown to YAML forms with structured inputs (dropdowns, required fields, syntax-highlighted code blocks). PR template cleaned up with a type checklist.
- **CLI help colors** - `rl` CLI help output is now colorized using clap's styling API: cyan headers, green literals, yellow placeholders, and red errors.
- **Arabic keyword aliases** - all 38 language keywords have Arabic equivalents (e.g. `دالة` for `fn`, `لكل` for `for`, `بينما` for `while`, `أرجع` for `return`). The lexer accepts either form; identifiers may freely mix Arabic and Latin characters (e.g. `اسم_المتغير`).
- **Pipe operator `|>`** - new infix operator that desugars `a |> f(args)` into `a.f(args)`. The left-hand side becomes the receiver of the method call. Chaining is supported: `a |> f() |> g()` becomes `a.f().g()`.
- **Optional semicolons** - statements can now optionally end with `;`. Semicolons are silently consumed by the parser, so `dec int x = 10;` and `dec int x = 10` are both valid.
- **`std::process` new functions** - `set_env`, `remove_env`, `env_keys`, `os_name`, `arch`, `num_cpus`, `parent_pid`, `process_exists`, `exec_with_stdin`, `with_exec_with_stdin`, `exec_with_env`, `with_exec_with_env`, `exec_with_cwd`, `with_exec_with_cwd`, `exec_with_timeout`, `exec_background`, `with_exec_background`, `wait_pid`, `term_pid`, `kill_pid`, `pipe`, `pipe_all`.
- **`std::path` new functions** - `path_is_absolute`, `path_is_relative`, `path_starts_with`, `path_ends_with`, `path_normalize`, `path_absolute`, `path_canonicalize`, `path_expand_home`, `path_split`, `path_split_extension`, `path_components`, `path_with_file_name`, `path_relative`, `path_join_many`.
- **`std::fs` new functions** - `touch`, `truncate_file`, `glob`, `walk_dir`, `symlink`, `readlink`, `hardlink`, `temp_file`, `temp_file_in`, `file_created`, `file_accessed`, `file_permissions`, `set_permissions`, `list_dir_names`, `realpath`, `lock_file`, `unlock_file`.
- **`std::io` handle-based I/O** - `open`, `close`, `read_handle`, `write_handle`, `seek`, `flush`, `read_all`, `readline` with proper `HandleKind::File` variant in the handle system. Also `read_all_stdin`, `decode_utf8`, `encode_utf8`, `isatty`.
- **`HandleKind::File`** - new handle variant in `rl-ast` for file I/O resources, following the same `IoStore` trait pattern as `NetStore`/`HttpStore`/etc.
- **Bytecode deserialization fix** - added missing `4 => HandleKind::Gui` and `5 => HandleKind::File` to the handle kind deserialization match.
- **`rl_result` rewritten (rl-cc)** - tagged union with `enum rl_type_tag`, type-safe constructors (`rl_ok_null`, `rl_ok_i64`, `rl_ok_f64`, etc.), and `_Generic` macro dispatch.
- **Tuple dedup fixed (rl-cc)** - dedup by full field-type layout, not arity.
- **Map/set storage (rl-cc)** - uses `rl_map *map` and `rl_set *set` pointers (not by-value) via `rl_value` tagged union.
- **Shebang support** - `.rl` files starting with `#!` are valid; the lexer strips the shebang line before tokenizing. `rl new --script <name>` creates a standalone executable `.rl` script with a shebang header pointing to the `rl` binary (detected via `current_exe` / PATH scan, falls back to `rlc`), a hello world body, and executable permissions (`0755`). Scripts can be run directly: `chmod +x hello.rl && ./hello.rl`.
- **Program attributes fixed** - `#![convert(kg=1000(g))]` now parses correctly. The lexer's `BangHash` token now properly consumes both characters, the shebang stripper no longer eats `#![...]` inner attributes, and the parser handles both `BangHash` and separate `Hash`+`Bang` token sequences.
- **`result_unwrap` type dispatch** - `result_unwrap`, `result_unwrap_err`, and `result_unwrap_or` now emit the correct type-specific unwrap function (`rl_result_unwrap_str`, `_f64`, `_bool`, `_i64`) based on the result's inner type. Previously all three hardcoded `rl_result_unwrap_i64`.
- **C transpiler (`rl-cc`)** - transpiles rl to C99. Full pipeline works end-to-end: lex, parse, resolve, type-check, C codegen, optional `cc` compilation. Supported features: types, arithmetic, booleans, comparisons, control flow (if/else, while, for, foreach, forrange, loop, break, continue), functions, casts, println/print, tuples, tuple destruction, arrays, maps, sets, records/structs, enums/tags, match, result type (ok/err/error), error propagation (`?`), impl methods, escape sequences, closures, lambda expressions, multi-file imports (resolved at resolve time, inlined into generated C), `std::c` FFI module (compile, load, call, has_symbol, close, clear_cache), `std::net` module (tcp_listen, tcp_accept, tcp_connect, tcp_read, tcp_write, tcp_peer_addr, tcp_local_addr, tcp_set_timeout, tcp_set_nonblocking, tcp_shutdown, tcp_close, udp_bind, udp_connect, udp_send, udp_send_to, udp_recv, udp_recv_from, udp_close, resolve). Runtime includes `rl_string`, `rl_result` (tagged union with type tag enum), `rl_value` (tagged union for map/set storage), `rl_array`, `rl_map`, `rl_set`, `rl_closure`, and per-program record/tuple/enum print functions. CLI: `rl transpile file.rl --compile`.
- **Escape sequences** - string and character literals now support the full set of escape sequences: `\n`, `\t`, `\r`, `\0`, `\\`, `\"`, `\'`, `\a`, `\b`, `\f`, `\v`, `\e`, plus `\xHH` hex byte escapes (1-2 hex digits, e.g. `\x41`, `\xff`) and `\uHHHH` / `\u{HHHH...}` unicode codepoint escapes (e.g. `\u0041`, `\u{1F600}`).
- **`rl print` command** - `rl print <file> --tokens` prints the token stream, `--parser` prints the parsed statement tree, `--ast` prints the type-checked/resolved tree. Uses box-drawing characters for nested output.
- **`rl run -c` flag** - execute inline rl code directly: `rl run -c 'println("hello")'`.
- **Tree print module** (`rl-tooling`) - box-drawing character tree printer for tokens, parser statements, and resolved statements. Used by `rl print`.
- **Closure support (rl-cc)** - `rl_closure` struct with `rl_closure_new`, `rl_closure_call`, `rl_ok_closure`, `RL_TAG_CLOSURE`, `rl_print_closure`, and 9 closure-consuming runtime functions (`arr_for_each`, `arr_all`, `arr_any`, `arr_find_index`, `arr_sort_by`, `arr_flat_map`, `result_map`, `result_map_err`, `bench`).
- **Null printing fix (rl-cc)** - nullable variables tracked via `nullable_vars` set; `dec int x = null` now stores `rl_result x = rl_ok_null();`. Read/write/print correctly unwrap and re-wrap. New `rl_print_raw`/`rl_println_raw` runtime functions print inner values without `ok()`/`err()` wrapper.
- **10 missing stdlib functions (rl-cc)** - `io::read_bytes`, `types::error_unwrap`, `types::to_byte`, `types::to_char`, `random::rand_dices`, `random::rand_bytes`, `random::rand_choice`, `random::rand_choices`, `random::rand_sample`, `random::rand_shuffle`.
- **Enum display (rl-cc)** - generates `rl_print_Enum_{name}()` with static string table per enum type.
- **Float precision (rl-cc)** - strtod round-trip approach for shortest representation.
- **Record/tuple print for nested types (rl-cc)** - `emit_field_print()` helper for nested records and tuples in print statements.

### Fixed

- **Parser missing closing delimiter errors** - the parser now reports errors for missing `]`, `}`, and `)` in array literals, map literals, set literals, grouped expressions, index access, function parameters, method parameters, lambda parameters, for-loop headers, for-range inline arrays, and type annotations (`arr[T]`, `map[K,V]`, `set[T]`, `result[T]`). Previously these were silently accepted.
- **Parser missing newline skipping** - added newline tolerance in function declarations (before `(`, between parameters, before `->`, before `{`), impl method declarations (same locations), lambda parameters, for-range `..` operator, `as` cast in postfix, and `get` imports. Multi-line function/method declarations and lambdas now parse correctly.
- **Clippy warnings** - resolved all clippy warnings across the workspace (collapsible `if`, redundant closures, `format!` misuse, derivable `Default` impl, `map_or` simplification).
- **Cross-platform fs/process/io** - `lock_file`, `unlock_file`, `symlink`, `set_permissions`, `file_permissions` now gate Unix-only APIs behind `#[cfg(unix)]` with `#[cfg(not(unix))]` stubs. `path_expand_home` checks `USERPROFILE` on Windows. `process_exists`, `term_pid`, `kill_pid`, `isatty` similarly gated.
- **P1: map::get aborts on missing key** - returns `err("key not found in map")` instead of aborting.
- **P2: res::unwrap no error checking** - new `rl_result_unwrap_i64/f64/bool/str` functions abort on err.
- **P3: math::abs truncates floats** - dispatches `fabs()` for `RL_TAG_F64`.
- **P4: math::pow(int,int) returns float** - new `rl_math_pow` with integer exponentiation path.
- **P5: arr::sort only handles int64** - added `rl_arr_cmp_f64` and `rl_arr_cmp_str` comparators.
- **P6: Time formatting local TZ** - `localtime()` -> `gmtime()` in all 4 formatting functions.
- **P7: File I/O error handling** - `rl_io_read_file/write_file/append_file` now return `rl_result` with `err()` on failure.
- **P8: arr::zip flat interleaving** - inline codegen with `memcpy` pairs.
- **P9: is_* stubs** - 8 new runtime functions with real tag checks.
- **P10: Dead duplicate match arms** - removed `sign`/`degrees`/`radians` from first match arm.
- **map_remove_s returns modified map** - matching VM reassignment semantics.
- **process_args** - stores argc/argv via `rl_store_args()` at main start.
- **`_GNU_SOURCE`** added at top of generated `.c` files for `M_PI`/`M_E`.
- **`-lm` required** for linking when `pow()` is used.

## [2.0.0] - 2026-08-13

The bytecode VM is now the sole execution backend; the tree-walking interpreter has been removed.

### Removed

- **Tree-walking interpreter** - the `rl-interpreter` crate and its `treewalker` feature are gone. `rl run` / `rl dev` execute through the bytecode VM only, and the `--treewalker` CLI flag no longer exists.
- **Interpreter test suite** - the `tests/interpreter` suite and the interpreter-vs-VM benchmark target have been removed.
- **Treewalker build variants** - the `rl_treewalker` / `rlp` release variants are no longer built or offered by the installers.

### Changed

- `rl run` / `rl dev` default to the bytecode VM backend.
- The VM now resolves programs directly via `rl-resolver` (previously it borrowed the interpreter's `Evaluator` as a resolver+stdlib holder).

## [1.0.0] - 2026-08-06

First stable release. The workspace is now a set of 1.0.0 crates, and the bytecode VM is a default, drop-in execution backend alongside the tree-walking interpreter. Interactive installers are available for Linux and Windows.

### Added

- **Bytecode VM stdlib parity** - `rl-vm` now mirrors every standard library module of `rl-interpreter` (`array`, `audio`, `bitwise`, `c`, `collections`, `debug`, `fs`, `gui`, `http`, `io`, `math`, `net`, `path`, `process`, `random`, `result`, `rl`, `string`, `terminal`, `time`, `types`).
- **New `uint` type** - unsigned 8-byte integer (`UInt`), plus `sbyte` and the `big` / `small` modifiers, and `u8` / `i16` array element types.
- **`std::gui` module** - windowed UI toolkit with widgets (`gui_button`, `gui_checkbox`, `gui_dropdown`, `gui_image`, `gui_label`, `gui_number_input`, `gui_progress_bar`, `gui_radio_group`, `gui_separator`, `gui_slider`, `gui_textarea`, `gui_textbox`, `gui_window`), lifecycle/event control functions (`gui_run`, `gui_on_click`, `gui_on_change`, ...), and `z`-level ordering.
- **`std::audio` module** - audio playback (`play_file`, `play_file_async`, `beep`), transport controls (`sound_pause`, `sound_resume`, `sound_seek`, `sound_stop`, `sound_wait`, `sound_set_speed`), volume (`set_master_volume`, `sound_get/set_volume`), and output-device listing/selection.
- **`std::c` module** - call C library functions from rl via `libffi` (`call`, `compile`, `load`, `close`), including a cache for pre-built libraries and `CArg` typed signatures.
- **`impl` keyword** - methods on records, resolved and checked across `rl-lexer`, `rl-parser`, `rl-ast`, `rl-resolver`, `rl-checker`, `rl-vm`, and `rl-interpreter`.
- **`handle` type** - opaque handles (u64) for external resources.
- **`loop` keyword** - loop control flow.
- **Stdlib functions** - `map_keys`, `map_values`, `map_to_array`, `map_merge`, `map_remove`, `map_contains`, `map_clear`, `map_get`, `map_len`, `map_is_empty` in `std::collections`; `std::math::consts`; `with_*` exec helpers in `std::process`.
- **REPL improvements** - tab completion, `:clear` and `:reset` commands, centralized theme with live syntax highlighting, and a `VmBackend` that keeps persistent VM state across submitted inputs.
- **Installers** - interactive `install.sh` (Linux) and `install.ps1` (Windows) scripts that download prebuilt binaries from GitHub Releases, with variant selection and version pinning; nightly builds publish all variants.
- **Documentation** - docs for the new modules and concepts (`std::c`, `std::audio`, `std::gui`, `impl`/records, arrays, casts, nulls, operators, types), plus `examples/` templates for new modules.

### Changed

- All crates bumped to **1.0.0**.
- The bytecode VM is now the default execution backend of `rl-cli` (enabled by the `vm` feature, on by default).
- Dropped automatic numeric promotion between `int` and `float`.
- `handle`s changed from `i64` to `u64`.
- `Generic` type annotation now holds a `String` for the type name.
- `std::term::read_key` returns `result[arr[string]]`.
- `char` prints without surrounding quotes.

### Fixed

- `uint` value casts in the interpreter and VM.
- Parser test spans are now derived from source instead of hardcoded.
- Checker `?` operator now validates the inner type correctly.
- `map` type indexing in `ExpressionKind::Index`.

### Performance

- Optimized the VM dispatch loop (lazy spans, unchecked operands, cached frame base).
- Bench suite now simulates release mode with corrected programs.

[2.1.0]: https://github.com/rl-lang/rl-lang/releases/tag/v2.1.0
[2.0.0]: https://github.com/rl-lang/rl-lang/releases/tag/v2.0.0
[1.0.0]: https://github.com/rl-lang/rl-lang/releases/tag/v1.0.0
