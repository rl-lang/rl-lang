// rl_runtime.h - C runtime library for programs transpiled from RL by rl-cc.
//
// Every RL value maps onto one of the C types below (`rl_string`, `rl_array`,
// `rl_map`, `rl_set`, `rl_closure`, or the `rl_result` tagged union used for
// error propagation with `?`). Generated code `#include`s this header and
// links `rl_runtime.c`.
#ifndef RL_RUNTIME_H
#define RL_RUNTIME_H

#include <stdint.h>
#include <stdbool.h>
#include <stddef.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <math.h>
#ifndef M_PI
#define M_PI 3.14159265358979323846
#endif
#ifndef M_E
#define M_E 2.71828182845904523536
#endif
#include <unistd.h>
#include <sys/stat.h>
#include <time.h>
#include <ctype.h>
#include <dlfcn.h>
#include <dirent.h>

// ---- string type ----
// RL `string`: a borrowed byte buffer with an explicit length plus a
// reference count (`rc`) for shared ownership. Not NUL-terminated;
// always pair `data` with `len` (use `%.*s` with printf).

typedef struct {
    const char *data;
    uint64_t len;
    int32_t rc;
} rl_string;

// Wrap a C string literal (no copy; the caller keeps owning the bytes).
rl_string rl_str_literal(const char *s, uint64_t len);
// Length of the string in bytes.
uint64_t rl_str_len(rl_string s);
// Allocate a new string holding `a` followed by `b`.
rl_string rl_str_concat(rl_string a, rl_string b);
// Byte-wise equality comparison.
bool rl_str_eq(rl_string a, rl_string b);

// ---- forward declarations ----
// Full definitions live further below; these let the tagged unions
// reference the collection types before they are defined.

// Growable, type-erased vector. `data` points at `len` live elements of
// `elem_size` bytes (`cap` allocated); `type_tag` records the RL element
// type for printing and checked unwrapping.
typedef struct { void *data; uint64_t len; uint64_t cap; int32_t elem_size; int32_t type_tag; } rl_array;
typedef struct rl_closure rl_closure;
typedef struct rl_map rl_map;
typedef struct rl_set rl_set;

// ---- value type (tagged union for map/set storage) ----
// `rl_value` is the dynamically-typed box stored inside maps and sets,
// so collections can hold mixed RL values. Prefer `rl_result` at
// function boundaries (it carries the ok/err state); use `rl_value`
// only for elements already inside a collection.

enum rl_value_tag {
    RL_VTAG_NULL = 0,
    RL_VTAG_I64,
    RL_VTAG_F64,
    RL_VTAG_BOOL,
    RL_VTAG_CHAR,
    RL_VTAG_STR,
    RL_VTAG_ARR,
    RL_VTAG_MAP,
    RL_VTAG_SET,
    RL_VTAG_CLOSURE,
};

typedef struct rl_value {
    enum rl_value_tag tag;
    union {
        int64_t i64;
        double f64;
        bool boolean;
        rl_string str;
        rl_array arr;
        rl_map *map;
        rl_set *set;
        rl_closure *closure;
    } data;
} rl_value;

// One map slot: an owned copy of the string key plus its boxed value.
typedef struct { char *key; rl_value value; } rl_map_entry;
// Open-addressed hash map from strings to `rl_value`, grown geometrically.
struct rl_map { rl_map_entry *entries; uint64_t len; uint64_t cap; };
// Hash set of `rl_value`s backed by a flat buffer.
struct rl_set { rl_value *data; uint64_t len; uint64_t cap; };

// ---- result type (tagged union) ----
// `rl_result` is the universal function-return type of generated code:
// `is_ok` distinguishes success from an RL error, `tag` says which
// `data` union member is live, and `err_code` carries the numeric code
// for `err` values. The `?` operator checks `is_ok` and early-returns
// the whole `rl_result` on failure.

enum rl_type_tag {
    RL_TAG_NULL = 0,
    RL_TAG_I64,
    RL_TAG_F64,
    RL_TAG_BOOL,
    RL_TAG_CHAR,
    RL_TAG_STR,
    RL_TAG_ARR,
    RL_TAG_MAP,
    RL_TAG_SET,
    RL_TAG_CLOSURE,
};

typedef struct {
    bool is_ok;
    enum rl_type_tag tag;
    union {
        int64_t i64;
        double f64;
        bool boolean;
        rl_string str;
        rl_array arr;
        rl_map map;
        rl_set set;
        rl_closure *closure;
    } data;
    int32_t err_code;
} rl_result;

// Wrap a null in a successful result (used for nullable variables).
rl_result rl_ok_null(void);
// Wrap each RL value type in a successful result.
rl_result rl_ok_i64(int64_t v);
rl_result rl_ok_f64(double v);
rl_result rl_ok_bool(bool v);
rl_result rl_ok_str(rl_string v);
rl_result rl_ok_arr(rl_array v);
rl_result rl_ok_map(rl_map v);
rl_result rl_ok_set(rl_set v);
// Build a failed result carrying a message (`err "msg"`).
rl_result rl_err_msg(rl_string msg);
// Build a failed result carrying a numeric code plus a message.
rl_result rl_err_code(int64_t code, rl_string msg);
// Build a failed result carrying a numeric code.
rl_result rl_err(int64_t v);
// Alias of `rl_err` kept for older generated code.
rl_result rl_error(int64_t v);

// Concatenate `argc` string results (used for `+` on strings).
rl_string rl_str_concat_variadic(rl_result *args, uint64_t argc);
// Interpolate `args` into a `"...{0}..."` template string.
rl_result rl_str_format(rl_string tmpl, rl_result *args, uint64_t argc);

// ---- closure type ----
// RL lambdas compile to a static C function plus a captured environment.
// The generated struct holds the function pointer and the captures array.

// Closure function pointer type:
//   self  = pointer to the closure itself (for accessing captures)
//   args  = array of rl_result arguments
//   argc  = number of arguments
typedef rl_result (*rl_closure_fn)(rl_closure *self, rl_result *args, uint64_t argc);

struct rl_closure {
    rl_closure_fn fn;
    rl_result *captures;
    uint64_t capture_count;
};

// Build a closure value from a generated function and its captures.
static inline rl_closure rl_closure_new(rl_closure_fn fn, rl_result *captures, uint64_t capture_count) {
    rl_closure c = { .fn = fn, .captures = captures, .capture_count = capture_count };
    return c;
}

// Invoke a closure with `argc` already-wrapped arguments.
static inline rl_result rl_closure_call(rl_closure c, rl_result *args, uint64_t argc) {
    return c.fn(&c, args, argc);
}

// Box a closure into a successful result (heap-allocates the box).
static inline rl_result rl_ok_closure(rl_closure v) {
    rl_result r = { .is_ok = true, .tag = RL_TAG_CLOSURE, .err_code = 0 };
    rl_closure *heap = (rl_closure *)malloc(sizeof(rl_closure));
    *heap = v;
    r.data.closure = heap;
    return r;
}

// Identity helpers used as `_Generic` fallbacks (see `rl_ok` below).
static inline rl_result _rl_identity_result(rl_result v) { return v; }
static inline rl_result _rl_ok_null(void) { return rl_ok_null(); }

// Unwrap helpers - extract the inner C value from a successful
// `rl_result`. Callers must have checked `is_ok` (or `?`) first;
// these do no error checking.
static inline int64_t rl_unwrap_i64(rl_result v) { return v.data.i64; }
static inline double rl_unwrap_f64(rl_result v) { return v.data.f64; }
static inline bool rl_unwrap_bool(rl_result v) { return v.data.boolean; }
static inline rl_string rl_unwrap_str(rl_result v) { return v.data.str; }
static inline rl_array rl_unwrap_arr(rl_result v) { return v.data.arr; }
static inline rl_map rl_unwrap_map(rl_result v) { return v.data.map; }
static inline rl_set rl_unwrap_set(rl_result v) { return v.data.set; }

// Generic unwrap: passes plain C values through unchanged, unwraps the
// `i64` payload out of an `rl_result`. Used by generated code that may
// hold either form.
#define rl_unwrap(x) _Generic((x), \
    rl_result: _rl_unwrap_auto, \
    int64_t:  _rl_id_i64, \
    double:   _rl_id_f64, \
    bool:     _rl_id_bool, \
    rl_string: _rl_id_str, \
    rl_array:  _rl_id_arr, \
    rl_map:    _rl_id_map, \
    rl_set:    _rl_id_set \
)(x)

// Identity functions backing `rl_unwrap` for already-plain values.
static inline int64_t _rl_id_i64(int64_t v) { return v; }
static inline double _rl_id_f64(double v) { return v; }
static inline bool _rl_id_bool(bool v) { return v; }
static inline rl_string _rl_id_str(rl_string v) { return v; }
static inline rl_array _rl_id_arr(rl_array v) { return v; }
static inline rl_map _rl_id_map(rl_map v) { return v; }
static inline rl_set _rl_id_set(rl_set v) { return v; }

static inline int64_t _rl_unwrap_auto(rl_result v) { return v.data.i64; }

// Generic wrap: convert any plain C value into a successful `rl_result`
// (narrower ints/floats widen; `rl_result` and `void*` pass through as
// identity/null). Emitted by generated code at value boundaries.
#define rl_ok(x) _Generic((x), \
    int64_t:  rl_ok_i64, \
    uint64_t: _rl_ok_u64, \
    int32_t:  _rl_ok_i32, \
    uint32_t: _rl_ok_u32, \
    int16_t:  _rl_ok_i16, \
    uint16_t: _rl_ok_u16, \
    int8_t:   _rl_ok_i8, \
    uint8_t:  _rl_ok_u8, \
    char:     _rl_ok_char, \
    double:   rl_ok_f64, \
    float:    _rl_ok_f32, \
    bool:     rl_ok_bool, \
    rl_string: rl_ok_str, \
    rl_array:  rl_ok_arr, \
    rl_map:    rl_ok_map, \
    rl_set:    rl_ok_set, \
    rl_result: _rl_identity_result, \
    void*:    _rl_ok_null \
)(x)

// Widening wrappers backing `rl_ok` for non-`int64_t`/`double` scalars.
static inline rl_result _rl_ok_u64(uint64_t v) { return rl_ok_i64((int64_t)v); }
static inline rl_result _rl_ok_i32(int32_t v) { return rl_ok_i64(v); }
static inline rl_result _rl_ok_u32(uint32_t v) { return rl_ok_i64(v); }
static inline rl_result _rl_ok_i16(int16_t v) { return rl_ok_i64(v); }
static inline rl_result _rl_ok_u16(uint16_t v) { return rl_ok_i64(v); }
static inline rl_result _rl_ok_i8(int8_t v) { return rl_ok_i64(v); }
static inline rl_result _rl_ok_u8(uint8_t v) { return rl_ok_i64(v); }
static inline rl_result _rl_ok_f32(float v) { return rl_ok_f64(v); }
static inline rl_result _rl_ok_char(char v) {
    rl_result r = { .is_ok = true, .tag = RL_TAG_CHAR, .err_code = 0 };
    r.data.i64 = (int64_t)(unsigned char)v;
    return r;
}

// ---- array type ----

// Copy `count` elements of `elem_size` bytes into a new array.
rl_array rl_arr_from_vals(const void *vals, uint64_t count, int32_t elem_size);
// Allocate an empty array for elements of `elem_size` bytes.
rl_array rl_arr_new(int32_t elem_size);

// ---- map type ----

// Allocate an empty map.
rl_map rl_map_new(void);
// Insert or overwrite `key` (a copy of the key string is kept).
void rl_map_set(rl_map *m, const char *key, rl_value val);
// Look up `key`; returns a null-valued `rl_value` when absent.
rl_value rl_map_get(rl_map m, const char *key);
// True when `key` is present.
bool rl_map_contains(rl_map m, const char *key);
// Number of entries.
uint64_t rl_map_len(rl_map m);
// Delete `key` if present (no-op otherwise).
void rl_map_remove(rl_map *m, const char *key);

// ---- set type ----

// Allocate an empty set.
rl_set rl_set_new(void);
// Insert `val` unless an equal value is already present.
void rl_set_add(rl_set *s, rl_value val);
// True when an equal value is present.
bool rl_set_contains(rl_set s, rl_value val);
// Number of elements.
uint64_t rl_set_len(rl_set s);
// Delete the first element equal to `val` (no-op when absent).
void rl_set_remove(rl_set *s, rl_value val);

// ---- print functions ----
// `rl_print_*` writes a value with no trailing newline; `rl_println_*`
// appends `\n`. Generated code normally goes through the `rl_print` /
// `rl_println` generics below instead of calling these directly.

void rl_print_int64(int64_t v);
void rl_print_float64(double v);
void rl_print_bool(bool v);
void rl_print_char(char v);
void rl_print_str(rl_string v);
void rl_print_ptr(void *v);
void rl_print_null(void);

void rl_println_int64(int64_t v);
void rl_println_float64(double v);
void rl_println_bool(bool v);
void rl_println_char(char v);
void rl_println_str(rl_string v);
void rl_println_ptr(void *v);
void rl_println_null(void);

// Print a result's payload (errors print as `err(...)`); used by the
// script-mode `?` failure path and by `print` on errorable values.
void rl_print_result(rl_result v);
void rl_println_result(rl_result v);
// Print a result's payload without type decoration.
void rl_print_raw(rl_result v);
void rl_println_raw(rl_result v);
// Print collections in RL literal syntax (`[1, 2]`, `{k: v}`, `set{...}`).
void rl_print_rl_array(rl_array v);
void rl_println_rl_array(rl_array v);
void rl_print_rl_map(rl_map v);
void rl_println_rl_map(rl_map v);
void rl_print_rl_set(rl_set v);
void rl_println_rl_set(rl_set v);
// Closures print as an opaque `<closure>` placeholder.
void rl_print_closure(rl_closure v);
void rl_println_closure(rl_closure v);

// Generic print: dispatches on the static C type of the argument,
// falling back to a `<ptr:...>` placeholder for unknown pointers.
#define rl_print(x) _Generic((x), \
    int64_t:  rl_print_int64, \
    uint64_t: rl_print_int64, \
    int32_t:  rl_print_int64, \
    uint32_t: rl_print_int64, \
    int16_t:  rl_print_int64, \
    uint16_t: rl_print_int64, \
    int8_t:   rl_print_int64, \
    uint8_t:  rl_print_int64, \
    double:   rl_print_float64, \
    float:    rl_print_float64, \
    bool:     rl_print_bool, \
    char:     rl_print_char, \
    rl_string: rl_print_str, \
    rl_result: rl_print_result, \
    rl_array:  rl_print_rl_array, \
    rl_map:   rl_print_rl_map, \
    rl_set:   rl_print_rl_set, \
    rl_closure: rl_print_closure, \
    default:  rl_print_ptr \
)(x)

// Generic println: same dispatch as `rl_print` plus a trailing newline.
#define rl_println(x) _Generic((x), \
    int64_t:  rl_println_int64, \
    uint64_t: rl_println_int64, \
    int32_t:  rl_println_int64, \
    uint32_t: rl_println_int64, \
    int16_t:  rl_println_int64, \
    uint16_t: rl_println_int64, \
    int8_t:   rl_println_int64, \
    uint8_t:  rl_println_int64, \
    double:   rl_println_float64, \
    float:    rl_println_float64, \
    bool:     rl_println_bool, \
    char:     rl_println_char, \
    rl_string: rl_println_str, \
    rl_result: rl_println_result, \
    rl_array:  rl_println_rl_array, \
    rl_map:   rl_println_rl_map, \
    rl_set:   rl_println_rl_set, \
    rl_closure: rl_println_closure, \
    default:  rl_println_ptr \
)(x)

// The RL `never` type (e.g. the result of `panic`): uninhabited, so the
// placeholder function behind it aborts instead of returning.
typedef void rl_never;
rl_never rl_never_fn(void);
#define rl_never() rl_never_fn()

// ---- math ----
// Integer math helpers backing `std::math` for transpiled programs.
int64_t rl_math_factorial(int64_t n);
int64_t rl_math_gcd(int64_t a, int64_t b);
int64_t rl_math_lcm(int64_t a, int64_t b);
bool rl_math_is_prime(int64_t n);
int64_t rl_math_fibonacci(int64_t n);

// ---- time ----
// Wall-clock time in milliseconds since the Unix epoch.
int64_t rl_time_now_ms(void);

// ---- fs ----
// Create a single directory; the result is an error when it fails.
rl_result rl_fs_mkdir(rl_string path);

// ---- string ----
// Heap-allocated string utilities backing the RL `string` methods.
// Functions returning `rl_string` allocate a fresh buffer the caller owns;
// out-of-range indexes clamp instead of trapping.
// ASCII case conversion.
rl_string rl_str_to_upper(rl_string s);
rl_string rl_str_to_lower(rl_string s);
// Strip whitespace on both sides, or on one side only.
rl_string rl_str_trim(rl_string s);
rl_string rl_str_trim_start(rl_string s);
rl_string rl_str_trim_end(rl_string s);
// Substring predicates.
bool rl_str_contains(rl_string haystack, rl_string needle);
bool rl_str_starts_with(rl_string s, rl_string prefix);
bool rl_str_ends_with(rl_string s, rl_string suffix);
// Replace every occurrence of `from` with `to`.
rl_string rl_str_replace(rl_string s, rl_string from, rl_string to);
// Repeat `s` `count` times (empty string for `count <= 0`).
rl_string rl_str_repeat(rl_string s, int64_t count);
// Byte offset of the first `needle` hit, or -1 when absent.
int64_t rl_str_index_of(rl_string haystack, rl_string needle);
// Number of non-overlapping `needle` occurrences.
int64_t rl_str_count(rl_string haystack, rl_string needle);
// Pad with `c` up to `width` bytes on the left / right.
rl_string rl_str_pad_left(rl_string s, int64_t width, char c);
rl_string rl_str_pad_right(rl_string s, int64_t width, char c);
// Byte-range slice `[start, end)` with clamping.
rl_string rl_str_slice(rl_string s, int64_t start, int64_t end);
// Byte-reversed copy.
rl_string rl_str_reverse(rl_string s);
// Raw bytes and one-char strings for each byte.
rl_array rl_str_bytes(rl_string s);
rl_array rl_str_chars(rl_string s);
// Byte at `index` (0 when out of range).
char rl_str_char_at(rl_string s, int64_t index);
// Join an array of strings with `delim` between elements.
rl_string rl_str_join(rl_array arr, rl_string delim);
// Split on `delim` into an array of strings.
rl_array rl_str_split(rl_string s, rl_string delim);

// ---- debug ----
// Abort with message (RL `panic`); aborts as unreachable / unimplemented.
void rl_panic(rl_string msg);
void rl_unreachable(void);
void rl_todo(void);
// Abort reporting a failed `assert_eq` (got `a`, wanted `b`).
void rl_assert_fail(rl_string label, int64_t a, int64_t b);
// Abort reporting a failed `assert` with a custom message.
void rl_assert_fail_msg(rl_string label, rl_string msg);
// RL type name for a numeric type tag (for `type_of`).
rl_string rl_type_of(int64_t type_tag);
// Print a value to stderr and return it unchanged (RL `dbg`).
int64_t rl_dbg_int64(int64_t v);
double rl_dbg_float64(double v);
bool rl_dbg_bool(bool v);
rl_string rl_dbg_str(rl_string v);

// ---- path ----
// Pure path parsing (no filesystem access except the `is_*` checks).
rl_string rl_path_extension(rl_string path);
rl_string rl_path_filename(rl_string path);
rl_string rl_path_parent(rl_string path);
rl_string rl_path_stem(rl_string path);
// Drop the last component; join/push append one (push mutates in spirit,
// both return a fresh string).
rl_string rl_path_pop(rl_string path);
rl_string rl_path_join(rl_string path, rl_string target);
rl_string rl_path_push(rl_string path, rl_string target);
rl_string rl_path_set_extension(rl_string path, rl_string ext);
// Filesystem checks: true when the path exists and is a dir / file.
bool rl_path_is_dir(rl_string path);
bool rl_path_is_file(rl_string path);

// ---- fs ----
// File metadata and operations; sizes are bytes, times are Unix seconds,
// and -1 signals a failure that generated code turns into an RL error.
// Size of the file at `path`.
int64_t rl_fs_file_size(rl_string path);
// Last-modified time of the file at `path`.
int64_t rl_fs_file_modified(rl_string path);
// Copy `src` to `dst`; 0 on success, -1 on failure.
int64_t rl_fs_copy_file(rl_string src, rl_string dst);
// Create `path` plus missing parents; 0 on success, -1 on failure.
int64_t rl_fs_mkdir_all(rl_string path);
// Delete the directory tree at `path`; 0 on success, -1 on failure.
int64_t rl_fs_rmdir_all(rl_string path);
// Names (not full paths) of entries in the directory at `path`.
rl_array rl_fs_list_dir(rl_string path);
// Rename to `new_name` in the same directory; returns the new full path.
rl_string rl_fs_rename_file(rl_string path, rl_string new_name);

// ---- process ----
// Current working directory of the process.
rl_string rl_process_cwd(void);
// Change directory; 0 on success, -1 on failure.
int64_t rl_process_set_cwd(rl_string path);
// Run `cmd` through the shell and capture stdout (variants return the
// exit code or one array element per output line instead).
rl_string rl_process_exec(rl_string cmd);
int64_t rl_process_exec_code(rl_string cmd);
rl_array rl_process_exec_lines(rl_string cmd);
// Same, but with `env` assignments (e.g. `"A=1 B=2"`) prepended.
rl_string rl_process_with_exec(rl_string env, rl_string cmd);
int64_t rl_process_with_exec_code(rl_string env, rl_string cmd);
rl_array rl_process_with_exec_lines(rl_string env, rl_string cmd);
// Command-line arguments (excluding argv[0]) as an array of strings.
rl_array rl_process_args(void);
// Snapshot argv at startup; generated `main` calls this first.
void rl_store_args(int argc, char **argv);

// ---- time ----
// Format a Unix timestamp with a strftime-style `pattern`.
rl_string rl_time_format_time(int64_t timestamp, rl_string pattern);
// Format as `YYYY-MM-DD` / `HH:MM:SS` in local time.
rl_string rl_time_format_date_str(int64_t timestamp);
rl_string rl_time_format_time_str(int64_t timestamp);
// Split into `[year, month, day, hour, min, sec]` components.
rl_array rl_time_parts(int64_t timestamp);

// ---- io ----
// Read the whole file as one string / one array element per line;
// the result is an error when the file cannot be read.
rl_result rl_io_read_file(rl_string path);
rl_result rl_io_read_lines(rl_string path);
// Overwrite / append `content`; the result is an error on failure.
rl_result rl_io_write_file(rl_string path, rl_string content);
rl_result rl_io_append_file(rl_string path, rl_string content);
// Read one whitespace-separated token / int / float from stdin.
rl_string rl_io_read(void);
int64_t rl_io_read_int(void);
double rl_io_read_float(void);
// Delete the file at `path`; 0 on success, -1 on failure.
int64_t rl_io_delete_file(rl_string path);
// Write to stderr without / with a trailing newline.
void rl_io_eprint(rl_string msg);
void rl_io_eprintln(rl_string msg);

// ---- types ----
// Format an int as decimal / binary (`0b...`) / hex (`0x...`) / octal.
rl_string rl_types_to_string(int64_t v);
rl_string rl_types_to_bin(int64_t v);
rl_string rl_types_to_hex(int64_t v);
rl_string rl_types_to_oct(int64_t v);

// ---- random ----
// Unseeded pseudo-random values from the C library RNG.
// Full-range non-negative int / float in [0, 1).
int64_t rl_rand_int(void);
double rl_rand_float(void);
// Fair coin flip / flip that is true with probability `weight`.
bool rl_rand_bool(void);
bool rl_rand_bool_weighted(double weight);
// Random printable ASCII char / byte in [0, 255].
char rl_rand_char(void);
int64_t rl_rand_byte(void);
// Int in [min, max] / float in [min, max).
int64_t rl_rand_int_range(int64_t min, int64_t max);
double rl_rand_float_range(double min, double max);
// Die roll in [1, sides] / int in [0, stop) / stepped range value.
int64_t rl_rand_dice(int64_t sides);
int64_t rl_rand_range(int64_t stop);
int64_t rl_rand_range_step(int64_t start, int64_t stop, int64_t step);
// Random alphanumeric string of `count` chars.
rl_string rl_rand_string(int64_t count);

// ---- collections (rl_result-returning wrappers) ----
// Same operations as the raw `rl_set_*` / `rl_map_*` above, but wrapped
// so failures and missing keys surface as RL errors instead of traps.
// Add / remove / membership test, each reporting success as a result.
rl_result rl_set_add_s(rl_set *s, rl_value value);
rl_result rl_set_remove_s(rl_set *s, rl_value value);
rl_result rl_set_contains_s(rl_set s, rl_value value);
// Copy all elements into a fresh array.
rl_array rl_set_to_array(rl_set s);
// Membership test / removal returning a result; lookup returning the
// value or an error when the key is missing.
rl_result rl_map_contains_s(rl_map m, rl_string key);
rl_result rl_map_remove_s(rl_map m, rl_string key);
rl_result rl_map_get_s(rl_map m, rl_string key);
// Fresh arrays holding copies of all keys / all values.
rl_array rl_map_keys_s(rl_map m);
rl_array rl_map_values_s(rl_map m);
// New map holding `b` layered over `a` (`b` wins on conflicts).
rl_map rl_map_merge_s(rl_map a, rl_map b);
// Array of single-entry maps, one per key.
rl_array rl_map_to_array_s(rl_map m);

// ---- array (generic) ----
// Elementwise array utilities backing the RL `arr_*` functions.
// Mutating ops grow the buffer in place; out-of-range access returns
// an RL error instead of trapping. Sorts are ascending numeric.
// Append `v` / drop and return the last element.
rl_result rl_arr_push(rl_array a, int64_t v);
rl_result rl_arr_pop(rl_array a);
// Insert `v` at `idx` / drop the element at `idx`.
rl_result rl_arr_insert(rl_array a, int64_t idx, int64_t v);
rl_result rl_arr_remove(rl_array a, int64_t idx);
// Reversed / concatenated copies (inputs unchanged).
rl_array rl_arr_reverse(rl_array a);
rl_array rl_arr_concat(rl_array a, rl_array b);
// First / last element, or an error when empty.
rl_result rl_arr_first(rl_array a);
rl_result rl_arr_last(rl_array a);
// Copy with duplicates removed, keeping first-seen order.
rl_array rl_arr_unique(rl_array a);
// Copy of `[start, end)` with clamping.
rl_array rl_arr_slice(rl_array a, int64_t start, int64_t end);
// Membership test / first index (or -1) wrapped as results.
rl_result rl_arr_contains(rl_array a, int64_t v);
rl_result rl_arr_index_of(rl_array a, int64_t v);
// Fresh array of `count` copies of `v`.
rl_array rl_arr_fill(int64_t v, int64_t count);
// Stepped integer sequence, or an error for a zero step.
rl_result rl_arr_range(int64_t start, int64_t end, int64_t step);
// Sum / product / max / min, erroring on empty input.
rl_result rl_arr_sum(rl_array a);
rl_result rl_arr_product(rl_array a);
rl_result rl_arr_max(rl_array a);
rl_result rl_arr_min(rl_array a);
// Sorted copy (input unchanged).
rl_array rl_arr_sort(rl_array a);
// One-level flatten of nested arrays.
rl_array rl_arr_flatten(rl_array a);
// Pairwise tuples up to the shorter length.
rl_result rl_arr_zip(rl_array a, rl_array b);

// ---- closure-consuming array functions ----
// Higher-order array ops: each element (wrapped as `rl_result`) is fed
// to the RL lambda; a failing predicate aborts with that error.
// Keep elements where `pred` returns true.
rl_result rl_arr_filter_closure(rl_array arr, rl_closure pred);
// Apply `fn` to every element, collecting the outputs.
rl_result rl_arr_map_closure(rl_array arr, rl_closure fn);
// First element where `pred` returns true (error when none matches).
rl_result rl_arr_find_closure(rl_array arr, rl_closure pred);
// Left fold starting from `init`.
rl_result rl_arr_reduce_closure(rl_array arr, rl_closure fn, rl_result init);
// Index of the first match (error when none matches).
rl_result rl_arr_find_index_closure(rl_array arr, rl_closure pred);
// True when `pred` holds for all / for at least one element.
rl_result rl_arr_all_closure(rl_array arr, rl_closure pred);
rl_result rl_arr_any_closure(rl_array arr, rl_closure pred);
// Run `fn` for side effects; returns the input length.
rl_result rl_arr_for_each_closure(rl_array arr, rl_closure fn);
// Map, then concatenate one level of the resulting arrays.
rl_result rl_arr_flat_map_closure(rl_array arr, rl_closure fn);
// Sort using `cmp(a, b)` returning negative / zero / positive.
rl_result rl_arr_sort_by_closure(rl_array arr, rl_closure cmp);

// ---- closure-consuming result functions ----
// Apply `fn` to the payload of an ok result (passes errors through).
rl_result rl_result_map_closure(rl_result val, rl_closure fn);
// Apply `fn` to the code of an error result (passes ok values through).
rl_result rl_result_map_err_closure(rl_result val, rl_closure fn);

// ---- closure-consuming debug ----
// Run `fn` `iterations` times; returns elapsed milliseconds.
rl_result rl_bench_closure(rl_closure fn, int64_t iterations);

// ---- terminal (ANSI escape codes) ----
// Raw-terminal helpers backing `std::term`. Each writes its escape
// sequence to stdout and returns an ok result (or an error when the
// terminal does not support the operation).
// Switch to / back from the alternate screen buffer.
rl_result rl_term_enter(void);
rl_result rl_term_leave(void);
// Clear the whole screen / the current line.
rl_result rl_term_clear(void);
rl_result rl_term_clear_line(void);
// Move the cursor: absolute position, column only, row only,
// relative steps, or N lines down / up to column 0.
rl_result rl_term_move(int64_t col, int64_t row);
rl_result rl_term_move_to_col(int64_t col);
rl_result rl_term_move_to_row(int64_t row);
rl_result rl_term_move_up(int64_t n);
rl_result rl_term_move_down(int64_t n);
rl_result rl_term_move_left(int64_t n);
rl_result rl_term_move_right(int64_t n);
rl_result rl_term_next_line(int64_t n);
rl_result rl_term_prev_line(int64_t n);
// Save / restore the cursor position; hide / show the cursor.
rl_result rl_term_save_cursor(void);
rl_result rl_term_restore_cursor(void);
rl_result rl_term_hide_cursor(void);
rl_result rl_term_show_cursor(void);
// Read the terminal size into `out_cols` / `out_rows`.
rl_result rl_term_get_size(int64_t *out_cols, int64_t *out_rows);
// Request a terminal size (may be ignored by the emulator).
rl_result rl_term_set_size(int64_t cols, int64_t rows);
// Set the window title to the single char `ch`.
rl_result rl_term_set_title(int64_t ch);
// Scroll the viewport up / down by `n` lines.
rl_result rl_term_scroll_up(int64_t n);
rl_result rl_term_scroll_down(int64_t n);
// Flush pending output.
rl_result rl_term_flush(void);
// Truecolor foreground / background, or reset to the default pair.
rl_result rl_term_set_fg(int64_t r, int64_t g, int64_t b);
rl_result rl_term_set_bg(int64_t r, int64_t g, int64_t b);
rl_result rl_term_reset_color(void);
// Named-color ("red", "blue", ...) foreground / background.
rl_result rl_term_fg(rl_string name);
rl_result rl_term_bg(rl_string name);
// Text attributes; `reset_attr` clears them all.
rl_result rl_term_bold(void);
rl_result rl_term_dim(void);
rl_result rl_term_italic(void);
rl_result rl_term_underline(void);
rl_result rl_term_blink(void);
rl_result rl_term_reverse(void);
rl_result rl_term_crossed_out(void);
rl_result rl_term_reset_attr(void);
// Line wrapping on / off.
rl_result rl_term_enable_wrap(void);
rl_result rl_term_disable_wrap(void);
// Synchronized-output markers to avoid flicker during redraws.
rl_result rl_term_begin_sync(void);
rl_result rl_term_end_sync(void);
// Mouse-event reporting on / off.
rl_result rl_term_enable_mouse(void);
rl_result rl_term_disable_mouse(void);
// Print a value without moving to a new line.
void rl_term_print_inline(rl_result v);
// Read one key press as an array of key codes (blocks).
rl_array rl_term_read_key(void);
// True when input arrives within `ms` milliseconds.
bool rl_term_poll(int64_t ms);

// ---- result unwrap (with error checking) ----
// Checked unwrap used by RL `unwrap`: aborts with a message when `r`
// is an error instead of silently reading a dead union member.
int64_t rl_result_unwrap_i64(rl_result r);
double rl_result_unwrap_f64(rl_result r);
bool rl_result_unwrap_bool(rl_result r);
rl_string rl_result_unwrap_str(rl_result r);
// Checked unwrap of the i64 payload (generic fallback used by codegen).
rl_result rl_result_unwrap_auto(rl_result r);

// ---- math ----
// Absolute value / power over int and float payloads; non-numeric
// input yields an RL error.
rl_result rl_math_abs(rl_result x);
rl_result rl_math_pow(rl_result base, rl_result exp);

// ---- type checks ----
// Each returns an ok bool reporting the payload tag of `x`.
rl_result rl_is_bool(rl_result x);
rl_result rl_is_int(rl_result x);
rl_result rl_is_float(rl_result x);
rl_result rl_is_string(rl_result x);
rl_result rl_is_null(rl_result x);
rl_result rl_is_char(rl_result x);
rl_result rl_is_byte(rl_result x);
rl_result rl_is_error(rl_result x);

// ---- io ----
// Read the whole file as an array of byte values (error when unreadable).
rl_result rl_io_read_bytes(rl_string path);

// ---- types ----
// Turn an error result into a panic; wrap an int as a byte / char value.
rl_result rl_types_error_unwrap(rl_result x);
rl_result rl_types_to_byte(rl_result x);
rl_result rl_types_to_char(rl_result x);

// ---- random (extended) ----
// Array of `count` die rolls in [1, sides].
rl_result rl_rand_dices(int64_t count, int64_t sides);
// Array of `count` random bytes.
rl_result rl_rand_bytes(int64_t count);
// One uniform pick from `arr` (error when empty).
rl_result rl_rand_choice(rl_array arr);
// `count` picks with replacement / without replacement.
rl_result rl_rand_choices(rl_array arr, int64_t count);
rl_result rl_rand_sample(rl_array arr, int64_t count);
// Shuffled copy of `arr`.
rl_result rl_rand_shuffle(rl_array arr);

// ---- std::c (FFI) ----
// Minimal C interop: compile snippets with the system compiler, dlopen
// shared objects, and call their symbols. Handles are small integer ids
// into an internal table (`rl_c_handle` is the raw dlopen pointer type).
typedef void *rl_c_handle;
// Compile C `source` to a cached shared object; result holds its path.
rl_result rl_c_compile(rl_string source);
// Load a shared object; result holds its handle id.
rl_result rl_c_load(rl_string path);
// True (as a result) when the handle exports `fn_name`.
rl_result rl_c_has_symbol(int64_t handle_id, rl_string fn_name);
// Unload the handle (no-op for unknown ids).
rl_result rl_c_close(int64_t handle_id);
// Drop all cached compile artifacts.
rl_result rl_c_clear_cache(void);
// Call `fn_name` with `argc` boxed args described by `arg_types`
// ("i64", "f64", "str", ...), converting to `ret_type` on return.
rl_result rl_c_call(int64_t handle_id, rl_string fn_name, int64_t argc, void **argv, const char **arg_types, rl_string ret_type);

// ---- std::net (TCP/UDP) ----
// Blocking socket helpers backing `std::net`. Addresses look like
// `"127.0.0.1:8080"`. Each open socket is a handle id; results hold
// either the id / data or an RL error.
// Bind and listen; result holds the listener handle id.
rl_result rl_net_tcp_listen(rl_string address);
// Accept one client; result holds the connection handle id.
rl_result rl_net_tcp_accept(int64_t handle_id);
// Connect; result holds the connection handle id.
rl_result rl_net_tcp_connect(rl_string address);
// Read up to `max_bytes`; result holds the bytes as a string.
rl_result rl_net_tcp_read(int64_t handle_id, int64_t max_bytes);
// Write all of `data`; result holds the byte count.
rl_result rl_net_tcp_write(int64_t handle_id, rl_string data);
// Remote / local `"ip:port"` of the connection.
rl_result rl_net_tcp_peer_addr(int64_t handle_id);
rl_result rl_net_tcp_local_addr(int64_t handle_id);
// Read/write timeout in milliseconds (0 disables).
rl_result rl_net_tcp_set_timeout(int64_t handle_id, int64_t millis);
// Toggle non-blocking mode.
rl_result rl_net_tcp_set_nonblocking(int64_t handle_id, bool flag);
// Half-close the read side, write side, or both ("r" / "w" / "rw").
rl_result rl_net_tcp_shutdown(int64_t handle_id, rl_string mode);
// Close the socket.
rl_result rl_net_tcp_close(int64_t handle_id);
// Bind a UDP socket; result holds its handle id.
rl_result rl_net_udp_bind(rl_string address);
// Fix a default peer for `send` (does not handshake).
rl_result rl_net_udp_connect(int64_t handle_id, rl_string address);
// Send to the default peer / to an explicit address; result holds bytes sent.
rl_result rl_net_udp_send(int64_t handle_id, rl_string data);
rl_result rl_net_udp_send_to(int64_t handle_id, rl_string data, rl_string address);
// Receive one datagram / datagram plus sender address as a two-map.
rl_result rl_net_udp_recv(int64_t handle_id, int64_t max_bytes);
rl_result rl_net_udp_recv_from(int64_t handle_id, int64_t max_bytes);
// Close the socket.
rl_result rl_net_udp_close(int64_t handle_id);
// DNS lookup of `"host:port"`; result holds an array of `"ip:port"` strings.
rl_result rl_net_resolve(rl_string host_port);

// ---- std::http (server + client) ----
// Tiny blocking HTTP/1.1 server and client over plain TCP (no TLS).
// Servers are handle ids; accepted requests are request ids kept in an
// internal table until answered.
// Start listening on `addr` (`"127.0.0.1:8080"`); result holds server id.
rl_result rl_http_server_start(rl_string addr);
// Block for the next request / poll without blocking (error when none);
// result holds the request id.
rl_result rl_http_server_recv(int64_t handle_id);
rl_result rl_http_server_try_recv(int64_t handle_id);
// Stop the server and drop pending requests.
rl_result rl_http_server_stop(int64_t handle_id);
// Method ("GET", ...) / path+query / one header / full body of a request.
rl_result rl_http_request_method(int64_t handle_id);
rl_result rl_http_request_url(int64_t handle_id);
rl_result rl_http_request_header(int64_t handle_id, rl_string name);
rl_result rl_http_request_body(int64_t handle_id);
// Answer a request and close it; pass `has_content_type` 0 to omit.
rl_result rl_http_respond(int64_t handle_id, int64_t status, rl_string body, rl_string content_type, int has_content_type);
// GET / POST shorthand; result holds the response body as a string.
rl_result rl_http_get(rl_string url);
rl_result rl_http_post(rl_string url, rl_string body, rl_string content_type, int has_content_type);
// Full client request; result holds a map with `status`, `headers`,
// and `body`. Pass `has_body` / `has_headers` 0 to skip those parts.
rl_result rl_http_request(rl_string method, rl_string url, rl_string body, int has_body, rl_string headers_json, int has_headers);

#endif
