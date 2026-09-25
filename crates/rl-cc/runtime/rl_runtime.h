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
#include <setjmp.h>
#ifndef M_PI
#define M_PI 3.14159265358979323846
#endif
#ifndef M_E
#define M_E 2.71828182845904523536
#endif
#include <unistd.h>
#include <sys/stat.h>
#include <sys/wait.h>
#include <sys/file.h>
#include <fcntl.h>
#include <fnmatch.h>
#include <glob.h>
#include <termios.h>
#include <errno.h>
#include <time.h>
#include <sys/random.h>
#include <ctype.h>
#include <dlfcn.h>
#include <dirent.h>
#include <limits.h>

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

// One interpolation argument: the value plus whether it was already a
// plain value (`bare`) or a wrapped result (renders `ok(...)`/`err(...)`).
typedef struct {
    rl_result v;
    bool bare;
} rl_fmt_arg;

// Concatenate `argc` arguments (used by `concat`).
rl_string rl_str_concat_variadic(rl_fmt_arg *args, uint64_t argc);
// Interpolate `args` into a `"...{}..."` template string; arity
// mismatches abort like a VM error.
rl_string rl_str_format(rl_string tmpl, rl_fmt_arg *args, uint64_t argc);

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

// Build a closure value, heap-copying the captures so the closure
// outlives its definition site (returned or stored closures).
rl_closure rl_closure_new_heap(rl_closure_fn fn, rl_result *captures, uint64_t capture_count);
// Invoke a boxed closure value: unboxes, aborts loudly when the value
// is not a closure (e.g. calling a result that holds no closure).
rl_result rl_closure_call_checked(rl_result callee, rl_result *args, uint64_t argc);

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

// Flushes stdio and aborts; the loud failure behind checked unwraps.
void _rl_abort(void);

// ---- test framework (`std::test`, `rlt --test`) ----------------------------
// Abort capture for `test_assert_panics` and per-test drivers: generated
// code pushes a frame with `setjmp` directly (never hidden in a helper -
// the frame must outlive the call), and `_rl_abort` longjmps to the
// innermost frame instead of dying. Code 1 is a failure, 2 is a skip
// (`_rl_skip`); anything else is a hard abort when no frame is pushed.
#define RL_MAX_ABORT_FRAMES 64
extern jmp_buf rl_abort_frames[RL_MAX_ABORT_FRAMES];
extern int rl_abort_depth;
void _rl_unwind(int code);
void _rl_skip(void);

// Per-test-runner outcome accumulation. The generated `--test` driver
// snapshots these around each case for verdicts; asserts append here.
typedef struct {
    uint64_t passed;
    uint64_t failed;
    char **failures;
    size_t failures_len;
    size_t failures_cap;
    char **skipped;
    size_t skipped_len;
    size_t skipped_cap;
    const char *current;
} rl_test_state_t;
extern rl_test_state_t rl_test_state;
void rl_test_record(int ok, const char *msg);
int rl_result_equal(rl_result a, rl_result b);
// Deterministic property generation (fixed seed, replayable failures).
void rl_test_rng_reset(void);
uint64_t rl_test_rand_below(uint64_t n);
int64_t rl_test_rand_range(int64_t lo, int64_t hi);
double rl_test_rand_f64(void);
rl_string rl_test_rand_string(void);
int64_t rl_test_rand_range_ne(int64_t lo, int64_t hi, int64_t neq);
rl_string rl_test_rand_string_ne(const char *s, uint64_t len);
// Runs one property case: `n` generated iterations through `gen`/
// `invoke`, greedy shrinking, one summary failure. `args` is the
// shared boxed-argument buffer of length `nargs`.
void rl_test_run_property(
    const char *name,
    void (*gen)(void),
    rl_result (*invoke)(void),
    rl_result *args,
    size_t nargs,
    uint64_t n
);
rl_result rl_test_skip(rl_string reason);
rl_result rl_test_skip_if(bool cond, rl_string reason);
rl_result rl_test_assert_eq(rl_result a, rl_result b, rl_string msg);
rl_result rl_test_assert_ne(rl_result a, rl_result b, rl_string msg);
rl_result rl_test_assert_panics(rl_closure f);
rl_result rl_test_assert_no_panic(rl_closure f);
// Test registry for `test_run_registered` and `rlt --test` drivers.
// `fn` is a zero-argument test/setup/teardown function; only zero-arg
// functions are registered (parameterized ones cannot be invoked).
typedef void (*rl_test_fn_t)(void);
void rl_test_register(const char *kind, const char *name, const char *group, const char *reg, rl_test_fn_t fn);
int64_t rl_test_run_registered(rl_string name);

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

// Generic box: convert any plain C value into a dynamically-typed
// `rl_value` for intrinsics taking heterogeneous values (`__arr_push`,
// `__map_set`, set ops). Niche widths widen like `rl_ok`.
#define rl_box(x) _Generic((x), \
    int64_t:  _rl_box_i64, \
    uint64_t: _rl_box_u64, \
    int32_t:  _rl_box_i32, \
    uint32_t: _rl_box_u32, \
    double:   _rl_box_f64, \
    float:    _rl_box_f32, \
    bool:     _rl_box_bool, \
    rl_string: _rl_box_str, \
    rl_array:  _rl_box_arr, \
    rl_map:    _rl_box_map, \
    rl_set:    _rl_box_set, \
    rl_value: _rl_box_value \
)(x)

static inline rl_value _rl_box_i64(int64_t v) {
    rl_value r; r.tag = RL_VTAG_I64; r.data.i64 = v; return r;
}
static inline rl_value _rl_box_u64(uint64_t v) {
    rl_value r; r.tag = RL_VTAG_I64; r.data.i64 = (int64_t)v; return r;
}
static inline rl_value _rl_box_i32(int32_t v) {
    rl_value r; r.tag = RL_VTAG_I64; r.data.i64 = v; return r;
}
static inline rl_value _rl_box_u32(uint32_t v) {
    rl_value r; r.tag = RL_VTAG_I64; r.data.i64 = v; return r;
}
static inline rl_value _rl_box_int(int v) {
    rl_value r; r.tag = RL_VTAG_I64; r.data.i64 = v; return r;
}
static inline rl_value _rl_box_f64(double v) {
    rl_value r; r.tag = RL_VTAG_F64; r.data.f64 = v; return r;
}
static inline rl_value _rl_box_f32(float v) {
    rl_value r; r.tag = RL_VTAG_F64; r.data.f64 = v; return r;
}
static inline rl_value _rl_box_bool(bool v) {
    rl_value r; r.tag = RL_VTAG_BOOL; r.data.boolean = v; return r;
}
static inline rl_value _rl_box_str(rl_string v) {
    rl_value r; r.tag = RL_VTAG_STR; r.data.str = v; return r;
}
static inline rl_value _rl_box_arr(rl_array v) {
    rl_value r; r.tag = RL_VTAG_ARR; r.data.arr = v; return r;
}
static inline rl_value _rl_box_map(rl_map v) {
    rl_value r; r.tag = RL_VTAG_MAP;
    rl_map *mp = malloc(sizeof(rl_map));
    *mp = v;
    r.data.map = mp;
    return r;
}
static inline rl_value _rl_box_set(rl_set v) {
    rl_value r; r.tag = RL_VTAG_SET;
    rl_set *sp = malloc(sizeof(rl_set));
    *sp = v;
    r.data.set = sp;
    return r;
}
static inline rl_value _rl_box_value(rl_value v) { return v; }

// Checked unboxers for declaration-driven `core::` gets: abort on a
// tag mismatch like the checked `rl_result_unwrap_*` family.
static inline int64_t rl_unbox_i64(rl_value v) {
    if (v.tag != RL_VTAG_I64) {
        fprintf(stderr, "error: type mismatch unwrapping value\n");
        _rl_abort();
    }
    return v.data.i64;
}
static inline double rl_unbox_f64(rl_value v) {
    if (v.tag != RL_VTAG_F64) {
        fprintf(stderr, "error: type mismatch unwrapping value\n");
        _rl_abort();
    }
    return v.data.f64;
}
static inline bool rl_unbox_bool(rl_value v) {
    if (v.tag != RL_VTAG_BOOL) {
        fprintf(stderr, "error: type mismatch unwrapping value\n");
        _rl_abort();
    }
    return v.data.boolean;
}
static inline rl_string rl_unbox_str(rl_value v) {
    if (v.tag != RL_VTAG_STR) {
        fprintf(stderr, "error: type mismatch unwrapping value\n");
        _rl_abort();
    }
    return v.data.str;
}
// Numeric-converting unboxers for `as` casts out of dynamic (`any`)
// storage, mirroring the VM's `as` (any numeric converts, the rest
// abort). Narrower C widths cast again at the use site.
static inline int64_t rl_unbox_num_i64(rl_value v) {
    if (v.tag == RL_VTAG_I64) return v.data.i64;
    if (v.tag == RL_VTAG_F64) return (int64_t)v.data.f64;
    fprintf(stderr, "error: invalid cast to int\n");
    _rl_abort();
}
static inline double rl_unbox_num_f64(rl_value v) {
    if (v.tag == RL_VTAG_F64) return v.data.f64;
    if (v.tag == RL_VTAG_I64) return (double)v.data.i64;
    fprintf(stderr, "error: invalid cast to float\n");
    _rl_abort();
}
// Null dynamic value (unions accept `null`; there is no null literal
// with a storage type, so declarations emit this directly).
static inline rl_value rl_value_null(void) {
    rl_value r; r.tag = RL_VTAG_NULL; return r;
}
// Tag test for `is` over dynamic (`any`) storage: single evaluation,
// no temporaries at the use site.
static inline bool rl_value_has_tag(rl_value v, int tag) {
    return v.tag == tag;
}
static inline rl_array rl_unbox_arr(rl_value v) {
    if (v.tag != RL_VTAG_ARR) {
        fprintf(stderr, "error: type mismatch unwrapping value\n");
        _rl_abort();
    }
    return v.data.arr;
}
static inline rl_map rl_unbox_map(rl_value v) {
    if (v.tag != RL_VTAG_MAP) {
        fprintf(stderr, "error: type mismatch unwrapping value\n");
        _rl_abort();
    }
    return *v.data.map;
}
static inline rl_set rl_unbox_set(rl_value v) {
    if (v.tag != RL_VTAG_SET) {
        fprintf(stderr, "error: type mismatch unwrapping value\n");
        _rl_abort();
    }
    return *v.data.set;
}

// Generic wrap: convert any plain C value into a successful `rl_result`
// (narrower ints/floats widen; `rl_result` and `void*` pass through as
// identity/null). Emitted by generated code at value boundaries.
static inline rl_result rl_value_to_result(rl_value v) {
    switch (v.tag) {
        case RL_VTAG_NULL: return rl_ok_null();
        case RL_VTAG_I64: return rl_ok_i64(v.data.i64);
        case RL_VTAG_F64: return rl_ok_f64(v.data.f64);
        case RL_VTAG_BOOL: return rl_ok_bool(v.data.boolean);
        case RL_VTAG_STR: return rl_ok_str(v.data.str);
        case RL_VTAG_ARR: return rl_ok_arr(v.data.arr);
        case RL_VTAG_MAP: return rl_ok_map(*v.data.map);
        case RL_VTAG_SET: return rl_ok_set(*v.data.set);
        case RL_VTAG_CLOSURE: return rl_ok_closure(*v.data.closure);
        default: return rl_ok_null();
    }
}

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
    rl_closure: rl_ok_closure, \
    rl_result: _rl_identity_result, \
    rl_value: rl_value_to_result, \
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
// Copy with an explicit payload tag (literals record their kind).
rl_array rl_arr_from_vals_tag(const void *vals, uint64_t count, int32_t elem_size, int32_t tag);
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
// Print a dynamically-typed boxed value by its tag.
void rl_print_rl_value(rl_value v);
void rl_println_rl_value(rl_value v);
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
    rl_value: rl_print_rl_value, \
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
    rl_value: rl_println_rl_value, \
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
// Rotate `a` left by `shift` bits (64-bit, shift mod 64).
rl_result rl_bitwise_rotate_left(int64_t a, int64_t shift);
// Rotate `a` right by `shift` bits (64-bit, shift mod 64).
rl_result rl_bitwise_rotate_right(int64_t a, int64_t shift);
// Copy of `a` with bit `n` set.
rl_result rl_bitwise_bit_set(int64_t a, int64_t n);
// Copy of `a` with bit `n` cleared.
rl_result rl_bitwise_bit_clear(int64_t a, int64_t n);
// Copy of `a` with bit `n` flipped.
rl_result rl_bitwise_bit_toggle(int64_t a, int64_t n);
// True when bit `n` of `a` is set.
rl_result rl_bitwise_bit_is_set(int64_t a, int64_t n);

// ---- time ----
// Wall-clock time in milliseconds since the Unix epoch.
int64_t rl_time_now_ms(void);
// Monotonic nanos since first call (RL monotonic_now).
int64_t rl_time_monotonic_now(void);

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
// Join an array of any element type with `delim` by tag.
rl_string rl_str_join_t(rl_array arr, rl_string delim, int32_t tag);
// Split on `delim` into an array of strings.
rl_array rl_str_split(rl_string s, rl_string delim);
// Remainder after `prefix`, or an error when missing.
rl_result rl_str_strip_prefix(rl_string s, rl_string prefix);
// Remainder without `suffix`, or an error when missing.
rl_result rl_str_strip_suffix(rl_string s, rl_string suffix);
// Char index of the last `needle` hit, or -1 when absent.
int64_t rl_str_last_index_of(rl_string s, rl_string needle);
// Split on the first `sep` into [before, after], or an error.
rl_result rl_str_split_once(rl_string s, rl_string sep);
// One element per line (handles \n and \r\n).
rl_array rl_str_lines(rl_string s);
// Word-wrap to `width` bytes (copy when `width <= 0`).
rl_string rl_str_wrap(rl_string s, int64_t width);
// Prefix every line with `prefix`.
rl_string rl_str_indent(rl_string s, rl_string prefix);
// Remove the common leading indent.
rl_string rl_str_dedent(rl_string s);
// Line diff as [(text, -1|0|1)], always ok.
rl_result rl_str_diff_lines(rl_string a, rl_string b);
// True when non-empty and all chars are alphabetic.
bool rl_str_is_alpha(rl_string s);
// True when non-empty and all chars are ASCII digits.
bool rl_str_is_numeric(rl_string s);
// True when non-empty and all chars are whitespace.
bool rl_str_is_whitespace(rl_string s);
// Two-letter category of the first char, or an error when empty.
rl_result rl_str_unicode_category(rl_string s);

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
// Print a yellow warning to stderr (RL warn).
void rl_debug_warn(rl_string msg);
// Capture a backtrace (RL stack_trace); bare string, empty when unsupported.
rl_string rl_debug_stack_trace(void);

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
// True when `path` starts with `/`.
bool rl_path_is_absolute(rl_string path);
// True when `path` does not start with `/`.
bool rl_path_is_relative(rl_string path);
// True when `path` starts with `base` at a component boundary.
bool rl_path_starts_with(rl_string path, rl_string base);
// True when `path` ends with `child` at a component boundary.
bool rl_path_ends_with(rl_string path, rl_string child);
// Lexically clean `.` and duplicate separators (keeps `..`).
rl_string rl_path_normalize(rl_string path);
// Absolute form via cwd when relative, or an error.
rl_result rl_path_absolute(rl_string path);
// Canonical form via realpath, or an error.
rl_result rl_path_canonicalize(rl_string path);
// Replace a leading `~` with $HOME (copy when unset).
rl_string rl_path_expand_home(rl_string path);
// [parent, file] pair for `path`.
rl_array rl_path_split(rl_string path);
// [stem, extension-with-dot] pair for `path`.
rl_array rl_path_split_extension(rl_string path);
// One element per path component (`/` first when absolute).
rl_array rl_path_components(rl_string path);
// Replace the file name with `name`.
rl_string rl_path_with_file_name(rl_string path, rl_string name);
// Relative route from `from` to `to`, or an error.
rl_result rl_path_relative(rl_string from, rl_string to);
// Join all `parts` (absolute parts reset the base).
rl_string rl_path_join_many(rl_array parts);

// ---- fs ----
// File metadata and operations; sizes are bytes, times are Unix seconds.
// Size of the file at `path`, or an error.
rl_result rl_fs_file_size(rl_string path);
// Last-modified time of the file at `path`, or an error.
rl_result rl_fs_file_modified(rl_string path);
// Creation time of the file at `path`, or an error.
rl_result rl_fs_file_created(rl_string path);
// Create an empty file (or update its timestamps), or an error.
rl_result rl_fs_touch(rl_string path);
// Copy `src` to `dst`; ok with 0 on success, or an error.
rl_result rl_fs_copy_file(rl_string src, rl_string dst);
// Create `path` plus missing parents; ok null on success, or an error.
rl_result rl_fs_mkdir_all(rl_string path);
// Remove the directory at `path`; ok null on success, or an error.
rl_result rl_fs_rmdir(rl_string path);
// Move `src` to `dst`; ok null on success, or an error.
rl_result rl_fs_move_file(rl_string src, rl_string dst);
// Delete the directory tree at `path`; ok null on success, or an error.
rl_result rl_fs_rmdir_all(rl_string path);
// Full paths of entries in the directory at `path`, or an error.
rl_result rl_fs_list_dir(rl_string path);
// File names of entries in the directory at `path`, or an error.
rl_result rl_fs_list_dir_names(rl_string path);
// Last-accessed time of the file at `path`, or an error.
rl_result rl_fs_file_accessed(rl_string path);
// Raw mode bits of the file at `path`, or an error.
rl_result rl_fs_file_permissions(rl_string path);
// Set permission bits on `path`; ok null on success, or an error.
rl_result rl_fs_set_permissions(rl_string path, int64_t mode);
// Fresh temp file path; ok with the path, or an error.
rl_result rl_fs_temp_file(void);
// Fresh temp file path inside `dir`; ok with the path, or an error.
rl_result rl_fs_temp_file_in(rl_string dir);
// Truncate `path` to `len` bytes; ok null on success, or an error.
rl_result rl_fs_truncate_file(rl_string path, int64_t len);
// Paths matching `pattern` (`*`, `**`, `?`, `[...]`); ok with the list.
rl_result rl_fs_glob(rl_string pattern);
// All paths under `path` depth-first; ok with the list, or an error.
rl_result rl_fs_walk_dir(rl_string path);
// Create a symlink from `src` to `dst`; ok null on success, or an error.
rl_result rl_fs_symlink(rl_string src, rl_string dst);
// Target of the symlink at `path`, or an error.
rl_result rl_fs_readlink(rl_string path);
// Create a hard link from `src` to `dst`; ok null on success, or an error.
rl_result rl_fs_hardlink(rl_string src, rl_string dst);
// Canonical path of `path`; ok with the path, or an error.
rl_result rl_fs_realpath(rl_string path);
// Advisory exclusive lock on `path`; ok null on success, or an error.
rl_result rl_fs_lock_file(rl_string path);
// Release the advisory lock on `path`; ok null on success, or an error.
rl_result rl_fs_unlock_file(rl_string path);
// Copy the directory tree at `src` to `dst`; ok null, or an error.
rl_result rl_fs_copy_dir(rl_string src, rl_string dst);
// Total byte size under `path`; ok with the sum, or an error.
rl_result rl_fs_dir_size(rl_string path);
// True when `path` is a symlink; ok with the bool, or an error.
rl_result rl_fs_is_symlink(rl_string path);
// Open `file` with `mode` (`r`, `w`, `a`, `r+`, `w+`, `a+`).
rl_result rl_fs_open(rl_string file, rl_string mode);
// Close a file handle id; ok null on success, or an error.
rl_result rl_fs_close(int64_t handle_id);
// Read up to `n` bytes from a handle; ok with the text, or an error.
rl_result rl_fs_read_handle(int64_t handle_id, int64_t n);
// Write `data` to a handle; ok with the byte count, or an error.
rl_result rl_fs_write_handle(int64_t handle_id, rl_string data);
// Seek a handle; ok with the new position, or an error.
rl_result rl_fs_seek(int64_t handle_id, int64_t offset, int64_t whence);
// Flush a writable handle; ok null on success, or an error.
rl_result rl_fs_flush(int64_t handle_id);
// Read from a handle to EOF; ok with the text, or an error.
rl_result rl_fs_read_all(int64_t handle_id);
// Read one line from a handle; ok with the line, or an error.
rl_result rl_fs_readline(int64_t handle_id);
// Rename to `new_name` in the same directory; ok with the new full path,
// or an error.
rl_result rl_fs_rename_file(rl_string path, rl_string new_name);

// ---- process ----
// Value of the environment variable `key` (owned copy), or a null
// string (data == NULL) when unset.
rl_string rl_process_env(rl_string key);
// Current working directory of the process, or an error.
rl_result rl_process_cwd(void);
// Change directory; ok null on success, or an error.
rl_result rl_process_set_cwd(rl_string path);
// Run `cmd` through the shell and capture stdout (trailing newlines
// stripped); ok with the output, or an error when it cannot run.
rl_result rl_process_exec(rl_string cmd);
// Run `cmd` in the foreground; ok with the exit code, or an error.
rl_result rl_process_exec_fg(rl_string cmd);
// OS name like Rust's `std::env::consts::OS` (`linux`, `macos`, ...).
rl_string rl_process_os_name(void);
// Spawn `cmd` in the background; ok with the child pid, or an error.
rl_result rl_process_exec_background(rl_string cmd);
// True while a spawned pid is still running (false for unknown pids).
bool rl_process_running(int64_t pid);
// Reap a spawned pid; ok with its exit code, or an error.
rl_result rl_process_wait_pid(int64_t pid);
// Send SIGTERM / SIGKILL; ok null on success, or an error.
rl_result rl_process_term_pid(int64_t pid);
rl_result rl_process_kill_pid(int64_t pid);
// Run `cmd` through the shell; ok with the exit code, or an error.
rl_result rl_process_exec_code(rl_string cmd);
// Run `cmd` through the shell; ok with one element per output line,
// or an error.
rl_result rl_process_exec_lines(rl_string cmd);
// Same, but with `env` assignments (e.g. `"A=1 B=2"`) prepended.
rl_result rl_process_with_exec(rl_string env, rl_string cmd);
rl_result rl_process_with_exec_code(rl_string env, rl_string cmd);
rl_result rl_process_with_exec_lines(rl_string env, rl_string cmd);
// Set an environment variable; ok null on success, or an error.
rl_result rl_process_set_env(rl_string key, rl_string value);
// Remove an environment variable; ok null on success, or an error.
rl_result rl_process_remove_env(rl_string key);
// All environment variable names as an array of strings.
rl_array rl_process_env_keys(void);
// CPU architecture like Rust consts::ARCH ("x86_64", "aarch64", ...).
rl_string rl_process_arch(void);
// Number of available CPUs, or 1 when unknown.
int64_t rl_process_num_cpus(void);
// Parent process id.
int64_t rl_process_parent_pid(void);
// True when kill(pid, 0) succeeds or reports EPERM.
bool rl_process_exists(int64_t pid);
// Run exe plus args in the foreground; ok with the exit code, or an error.
rl_result rl_process_with_exec_fg(rl_string exe, rl_string cmd);
// Run cmd with input on stdin; ok with stdout, or an error.
rl_result rl_process_exec_with_stdin(rl_string cmd, rl_string input);
// Same with exe plus args and input on stdin; ok with stdout, or an error.
rl_result rl_process_with_exec_with_stdin(rl_string exe, rl_string cmd, rl_string input);
// Run cmd with envs pairs; ok with stdout, or an error.
rl_result rl_process_exec_with_env(rl_string cmd, rl_array envs);
// Same with exe plus args and envs pairs; ok with stdout, or an error.
rl_result rl_process_with_exec_with_env(rl_string exe, rl_string cmd, rl_array envs);
// Run cmd in dir; ok with stdout, or an error.
rl_result rl_process_exec_with_cwd(rl_string cmd, rl_string dir);
// Same with exe plus args in dir; ok with stdout, or an error.
rl_result rl_process_with_exec_with_cwd(rl_string exe, rl_string cmd, rl_string dir);
// Run cmd with timeout; ok with stdout, or an error on timeout.
rl_result rl_process_exec_with_timeout(rl_string cmd, int64_t timeout_ms);
// Spawn exe plus args in the background; ok with pid, or an error.
rl_result rl_process_with_exec_background(rl_string exe, rl_string cmd);
// Pipe cmd1 into cmd2; ok with final stdout, or an error.
rl_result rl_process_pipe(rl_string cmd1, rl_string cmd2);
// Pipe all cmds; ok with final stdout, or an error.
rl_result rl_process_pipe_all(rl_array cmds);
// Command-line arguments (excluding argv[0]) as an array of strings.
rl_array rl_process_args(void);
// Snapshot argv at startup; generated `main` calls this first.
void rl_store_args(int argc, char **argv);

// ---- core ----
// Mirrors `core::` intrinsics. Typed getters abort out-of-bounds, missing
// keys and mistyped values; the boxed forms serve dynamic containers with
// declaration-driven unboxing (`rl_unbox_*`). Maps are string-keyed.
rl_array rl_core_arr_new(void);
rl_array rl_core_arr_push(rl_array a, rl_value v);
int64_t rl_core_arr_get_i64(rl_array a, int64_t i);
double rl_core_arr_get_f64(rl_array a, int64_t i);
bool rl_core_arr_get_bool(rl_array a, int64_t i);
rl_string rl_core_arr_get_str(rl_array a, int64_t i);
rl_array rl_core_arr_get_arr(rl_array a, int64_t i);
rl_map rl_core_arr_get_map(rl_array a, int64_t i);
rl_value rl_core_arr_get_boxed(rl_array a, int64_t i);
rl_array rl_core_arr_set(rl_array a, int64_t i, rl_value v);
rl_value rl_core_map_get_boxed(rl_map m, rl_string k);
int64_t rl_core_map_get_i64(rl_map m, rl_string k);
double rl_core_map_get_f64(rl_map m, rl_string k);
bool rl_core_map_get_bool(rl_map m, rl_string k);
rl_string rl_core_map_get_str(rl_map m, rl_string k);
rl_array rl_core_map_get_arr(rl_map m, rl_string k);
rl_map rl_core_map_get_map(rl_map m, rl_string k);
rl_map rl_core_map_set(rl_map m, rl_string k, rl_value v);
rl_array rl_core_map_keys(rl_map m);
rl_set rl_core_set_add(rl_set s, rl_value v);
bool rl_core_set_has(rl_set s, rl_value v);
rl_string rl_core_type_of(rl_result v);
rl_array rl_core_arr_remove(rl_array a, int64_t i);
rl_map rl_core_map_remove(rl_map m, rl_string k);
bool rl_core_map_has(rl_map m, rl_string k);
rl_set rl_core_set_remove(rl_set s, rl_value v);
int64_t rl_core_arr_len(rl_array a);
int64_t rl_core_map_len(rl_map m);
int64_t rl_core_set_len(rl_set s);
int64_t rl_core_str_len(rl_string s);
uint8_t rl_core_str_get_byte(rl_string s, int64_t i);
rl_string rl_core_str_slice(rl_string s, int64_t start, int64_t end);
rl_string rl_core_str_concat(rl_string a, rl_string b);
int64_t rl_core_syscall6(int64_t nr, int64_t a1, int64_t a2, int64_t a3,
    int64_t a4, int64_t a5, int64_t a6);

// ---- byte buffers (core::__buf_*) ----
// Fixed table of 256 live buffers; ids are slot+1. Growth doubles from
// 16 bytes; shrink and clear keep capacity.
int64_t rl_buf_new(void);
int64_t rl_buf_len(int64_t id);
void rl_buf_push_byte(int64_t id, uint8_t byte);
uint8_t rl_buf_get_byte(int64_t id, int64_t i);
void rl_buf_set_byte(int64_t id, int64_t i, uint8_t byte);
void rl_buf_append(int64_t id, rl_string s);
rl_string rl_buf_slice(int64_t id, int64_t start, int64_t end);
void rl_buf_clear(int64_t id);
rl_string rl_buf_to_string(int64_t id);
void rl_buf_free(int64_t id);
int64_t rl_buf_addr(int64_t id);
void rl_buf_resize(int64_t id, int64_t n);

// ---- cli ----
// Mirrors `std::cli`. The arg parser drops everything through the first
// `--`, reads spec maps with string values, and returns string/bool/array
// values plus a `"_"` positional array. Editable input is plain line reads
// with history threading (no arrow-key editing).
rl_result rl_cli_parse_args(rl_array spec);
rl_result rl_cli_parse_args_or_exit(rl_array spec);
rl_result rl_cli_usage(rl_array spec);
rl_string rl_cli_prompt(rl_string msg);
rl_string rl_cli_prompt_password(rl_string msg);
bool rl_cli_prompt_confirm(rl_string msg);
rl_string rl_cli_prompt_choice(rl_string msg, rl_array options);
rl_result rl_cli_shell_split(rl_string s);
rl_string rl_cli_shell_join(rl_array parts);
rl_string rl_cli_read_line_editable(rl_string msg);
rl_result rl_cli_read_line_with_history(rl_string msg, rl_array history);
rl_result rl_cli_progress_bar(int64_t current, int64_t total, rl_string label);
rl_result rl_cli_spinner_tick(int64_t frame);

// ---- crypto ----
// Mirrors `std::crypto`. Byte arrays are int64-element arrays holding
// 0-255. Password hashing needs libargon2 (rlt links it when generated
// code mentions rl_crypto_password).
rl_array rl_crypto_sha256(rl_array data);
rl_array rl_crypto_sha512(rl_array data);
rl_array rl_crypto_sha1(rl_array data);
rl_array rl_crypto_md5(rl_array data);
rl_array rl_crypto_hmac_sha256(rl_array key, rl_array data);
rl_array rl_crypto_hmac_sha512(rl_array key, rl_array data);
bool rl_crypto_constant_time_eq(rl_array a, rl_array b);
rl_array rl_crypto_secure_random_bytes(int64_t count);
rl_array rl_crypto_secure_token(int64_t count);
rl_string rl_crypto_secure_token_hex(int64_t count);
rl_string rl_crypto_secure_token_urlsafe(int64_t count);
rl_string rl_crypto_base64_encode(rl_array data);
rl_result rl_crypto_base64_decode(rl_string s);
rl_string rl_crypto_base64_url_encode(rl_array data);
rl_result rl_crypto_base64_url_decode(rl_string s);
rl_string rl_crypto_hex_encode(rl_array data);
rl_result rl_crypto_hex_decode(rl_string s);
rl_string rl_crypto_uuid_v4(void);
rl_string rl_crypto_uuid_v7(void);
rl_result rl_crypto_uuid_parse(rl_string s);
rl_string rl_crypto_password_hash(rl_string password);
bool rl_crypto_password_verify(rl_string password, rl_string hash);

// ---- serialize ----
// Mirrors `std::serialize`. YAML needs libyaml (rlt links it when
// generated code mentions rl_serialize_yaml).
rl_result rl_serialize_json_parse(rl_string s);
rl_string rl_serialize_json_stringify(rl_result v);
rl_string rl_serialize_json_stringify_pretty(rl_result v);
bool rl_serialize_json_is_valid(rl_string s);
rl_result rl_serialize_json_get(rl_result v, rl_string path);
rl_result rl_serialize_csv_parse(rl_string s);
rl_result rl_serialize_csv_parse_with_delimiter(rl_string s, rl_string delim);
rl_string rl_serialize_csv_stringify(rl_array rows);
rl_result rl_serialize_csv_parse_headers(rl_string s);
rl_result rl_serialize_toml_parse(rl_string s);
rl_result rl_serialize_toml_stringify(rl_result v);
rl_result rl_serialize_ini_parse(rl_string s);
rl_result rl_serialize_ini_stringify(rl_result v);
rl_result rl_serialize_yaml_parse(rl_string s);
rl_result rl_serialize_yaml_stringify(rl_result v);

// Dynamic index for result-held containers (dynamic unwraps): maps by
// string key, arrays by int index. Errors pass through.
rl_result rl_dynamic_get(rl_result target, rl_result key);

// ---- time ----
// Format a Unix timestamp with a strftime-style `pattern`, or an error.
rl_result rl_time_format_time(int64_t timestamp, rl_string pattern);
// Format as `YYYY-MM-DD` in local time, or an error.
rl_result rl_time_format_date_str(int64_t timestamp);
// Format as `HH:MM:SS` in local time, or an error.
rl_result rl_time_format_time_str(int64_t timestamp);
// Split into `[year, month, day, hour, min, sec]`, or an error.
rl_result rl_time_parts(int64_t timestamp);

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
// Delete the file at `path`; ok null on success, or an error.
rl_result rl_io_delete_file(rl_string path);
// True when stdin is a terminal (used for cursor hide/show only).
bool rl_io_isatty(void);
// Write to stderr without / with a trailing newline.
void rl_io_eprint(rl_string msg);
void rl_io_eprintln(rl_string msg);
// Read all of stdin until EOF; ok with the text, or an error.
rl_result rl_io_read_all_stdin(void);
// Decode a byte array as UTF-8; ok with the string, or an error.
rl_result rl_io_decode_utf8(rl_array bytes);
// Encode a string as UTF-8 bytes; bare array of byte values.
rl_array rl_io_encode_utf8(rl_string s);

// ---- types ----
// `to_string` over a result payload: int/float/bool/char/string,
// err otherwise.
rl_result rl_types_to_string(rl_result x);
// `to_bin` over a result payload: int/byte/bool/char/string, err otherwise.
rl_result rl_types_to_bin(rl_result x);
// `to_hex` over a result payload: int/byte/char/string, err otherwise.
rl_result rl_types_to_hex(rl_result x);
// `to_oct` over a result payload: int/byte/char/string, err otherwise.
rl_result rl_types_to_oct(rl_result x);
// `to_int` over a result payload: int/float/bool/char/string, err otherwise.
rl_result rl_to_int(rl_result x);
// `to_float` over a result payload: float/int/bool/string, err otherwise.
rl_result rl_to_float(rl_result x);
// `to_bool` over a result payload: bool/int/float/null/string, err otherwise.
rl_result rl_to_bool(rl_result x);

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
// Reseed the C RNG (RL rand_seed); later values are deterministic.
void rl_rand_seed(int64_t seed);
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
// Set algebra over boxed values; equality mirrors rl_value_eq.
// Each returns a fresh set, or ok bool for the subset tests.
rl_set rl_set_union(rl_set a, rl_set b);
rl_set rl_set_intersection(rl_set a, rl_set b);
rl_set rl_set_difference(rl_set a, rl_set b);
rl_set rl_set_symmetric_difference(rl_set a, rl_set b);
rl_result rl_set_is_subset(rl_set a, rl_set b);
rl_result rl_set_is_superset(rl_set a, rl_set b);
// Lookup with a default value (boxed); ok with the value or the default.
// The insert variant stores the default when the key is missing and
// returns the default as ok; both report errors for missing keys.
rl_result rl_map_get_or_s(rl_map m, rl_string key, rl_value def);
rl_result rl_map_get_or_insert_s(rl_map *m, rl_string key, rl_value def);
// Legacy int-default variants for dynamically typed defaults.
rl_result rl_map_get_or(rl_map m, rl_string key, int64_t def);
rl_result rl_map_get_or_insert(rl_map *m, rl_string key, int64_t def);
// Binary heap over arrays (min-heap on ints and floats; other element
// kinds keep insertion order). Push returns the new heap; pop returns
// the heap without its root; peek returns the root value.
rl_result rl_heap_push(rl_array a, int64_t v);
rl_result rl_heap_push_v(rl_array a, rl_value v);
rl_result rl_heap_pop(rl_array a);
rl_result rl_heap_peek(rl_array a);
rl_result rl_heap_peek_t(rl_array a, int32_t tag);
// Deque over arrays: push inserts at the front, pop drops the front.
rl_result rl_deque_push_front(rl_array a, int64_t v);
rl_result rl_deque_push_front_v(rl_array a, rl_value v);
rl_result rl_deque_pop_front(rl_array a);
// Bisect over sorted int arrays; sorted_insert keeps ascending order.
rl_result rl_bisect_left(rl_array a, int64_t v);
rl_result rl_bisect_right(rl_array a, int64_t v);
rl_result rl_sorted_insert(rl_array a, int64_t v);

// ---- array (generic) ----
// Elementwise array utilities backing the RL `arr_*` functions.
// Mutating ops grow the buffer in place; out-of-range access returns
// an RL error instead of trapping. Sorts are ascending numeric.
// Append `v` / drop the last element (ok with the new array), or an error.
rl_result rl_arr_push(rl_array a, int64_t v);
// First / last element by tag constant, or an error when empty.
rl_result rl_arr_first_t(rl_array a, int32_t tag);
rl_result rl_arr_last_t(rl_array a, int32_t tag);
// Membership test / first index over any element type.
rl_result rl_arr_contains_v(rl_array a, rl_value v);
rl_result rl_arr_index_of_v(rl_array a, rl_value v);
// Append a boxed value to an array of any element type; ok with the new
// array, or an error when the value does not fit the element width.
rl_result rl_arr_push_v(rl_array a, rl_value v);
rl_result rl_arr_pop(rl_array a);
// Insert `v` at `idx` / drop the element at `idx`.
rl_result rl_arr_insert(rl_array a, int64_t idx, int64_t v);
// Insert a boxed value at `idx` for any element type.
rl_result rl_arr_insert_v(rl_array a, int64_t idx, rl_value v);
rl_result rl_arr_remove(rl_array a, int64_t idx);
// Reversed / concatenated copies (inputs unchanged).
rl_array rl_arr_reverse(rl_array a);
rl_array rl_arr_concat(rl_array a, rl_array b);
// First / last element, or an error when empty.
rl_result rl_arr_first(rl_array a);
rl_result rl_arr_last(rl_array a);
// Copy with duplicates removed by tag equality, keeping first-seen order.
rl_array rl_arr_unique_t(rl_array a, int32_t tag);
// Copy of `[start, end)` with clamping.
rl_array rl_arr_slice(rl_array a, int64_t start, int64_t end);
// Membership test / first index (or -1) wrapped as results.
rl_result rl_arr_contains(rl_array a, int64_t v);
rl_result rl_arr_index_of(rl_array a, int64_t v);
// Fresh array of `count` copies of `v`.
rl_array rl_arr_fill(int64_t v, int64_t count);
// Fresh array of `count` copies of a boxed value.
rl_array rl_arr_fill_v(rl_value v, int64_t count);
// Stepped integer sequence, or an error for a zero step.
rl_result rl_arr_range(int64_t start, int64_t end, int64_t step);
// Sum / product / max / min by tag (float or int), erroring on empty input.
rl_result rl_arr_sum_t(rl_array a, int32_t tag);
rl_result rl_arr_product_t(rl_array a, int32_t tag);
rl_result rl_arr_max_t(rl_array a, int32_t tag);
rl_result rl_arr_min_t(rl_array a, int32_t tag);
// Sorted copy by tag (input unchanged).
rl_array rl_arr_sort_t(rl_array a, int32_t tag);
// One-level flatten of nested arrays.
rl_array rl_arr_flatten(rl_array a);
// Pairwise tuples up to the shorter length.
rl_result rl_arr_zip(rl_array a, rl_array b);
// Pairwise tuples into a struct layout (`es_tuple` bytes, second field
// at `off_b`), copying `es_a` / `es_b` bytes per side.
rl_result rl_arr_zip_t(rl_array a, rl_array b, uint64_t es_tuple, uint64_t off_b, uint64_t es_a, uint64_t es_b);
// Split into chunks of `size` / sliding windows of `size`; ok with an
// array of sub-arrays, or an error for a bad size.
rl_result rl_arr_chunk(rl_array a, int64_t size);
rl_result rl_arr_windows(rl_array a, int64_t size);
// Copy with elements `i` and `j` exchanged; error when out of bounds.
rl_result rl_arr_swap(rl_array a, int64_t i, int64_t j);
// First `n` elements of the endlessly repeated array; error when negative.
rl_result rl_arr_cycle_take(rl_array a, int64_t n);
// Pairwise tuples padded with `fill` up to the longer length.
rl_result rl_arr_zip_longest_t(rl_array a, rl_array b, rl_value fill, uint64_t es_tuple, uint64_t off_b, uint64_t es_a, uint64_t es_b);
// Legacy int-fill variant for dynamically typed fills.
rl_result rl_arr_zip_longest(rl_array a, rl_array b, int64_t fill, uint64_t es_tuple, uint64_t off_b, uint64_t es_a, uint64_t es_b);

// ---- closure-consuming array functions ----
// Higher-order array ops: each element (boxed as `rl_result` by payload
// `tag`) is fed to the RL lambda; callback errors propagate as errors.
// Keep elements where `pred` returns true.
rl_result rl_arr_filter_closure(rl_array arr, rl_closure pred, int32_t tag);
// Apply `fn` to every element, collecting the homogeneous outputs.
rl_result rl_arr_map_closure(rl_array arr, rl_closure fn, int32_t tag);
// First element where `pred` holds (null when none).
rl_result rl_arr_find_closure(rl_array arr, rl_closure pred, int32_t tag);
// Left fold starting from `init`.
rl_result rl_arr_reduce_closure(rl_array arr, rl_closure fn, rl_result init, int32_t tag);
// Index of the first match, or -1 when absent.
rl_result rl_arr_find_index_closure(rl_array arr, rl_closure pred, int32_t tag);
// True when `pred` holds for all / for at least one element.
rl_result rl_arr_all_closure(rl_array arr, rl_closure pred, int32_t tag);
rl_result rl_arr_any_closure(rl_array arr, rl_closure pred, int32_t tag);
// Run `fn` for side effects; ok null unless a call errors.
rl_result rl_arr_for_each_closure(rl_array arr, rl_closure fn, int32_t tag);
// Map, then concatenate one level of the resulting arrays.
rl_result rl_arr_flat_map_closure(rl_array arr, rl_closure fn, int32_t tag);
// Sort using `cmp(a, b)` returning negative / zero / positive.
rl_result rl_arr_sort_by_closure(rl_array arr, rl_closure cmp, int32_t tag);
// Split into matching and rest by predicate; ok with the pair.
rl_result rl_arr_partition_closure(rl_array arr, rl_closure pred, int32_t tag);
// Element with the greatest / smallest mapped key.
rl_result rl_arr_max_by_closure(rl_array arr, rl_closure fn, int32_t tag);
rl_result rl_arr_min_by_closure(rl_array arr, rl_closure fn, int32_t tag);

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
// Ok with `[cols, rows]`, or an error when the size is unknown.
rl_result rl_term_size(void);
// Query cursor via DSR; ok with [x, y] or an error on timeout/non-tty.
rl_result rl_term_get_cursor_pos(void);
// Request a terminal size (may be ignored by the emulator).
rl_result rl_term_set_size(int64_t cols, int64_t rows);
// Set the window title to `title`.
rl_result rl_term_set_title(rl_string title);
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
// Print a value without moving to a new line (shared rendering with
// `format`: bare values print raw, results print decorated).
void rl_term_print_inline(rl_fmt_arg arg);
// Read one key press as an array of key-code strings (blocks); ok with
// the array, or an error on EOF.
rl_result rl_term_read_key(void);
// Ok with true when input arrives within `ms` milliseconds.
rl_result rl_term_poll(int64_t ms);

// ---- result unwrap (with error checking) ----
// Checked unwrap used by RL `unwrap`: aborts with a message when `r`
// is an error instead of silently reading a dead union member.
// Identity on ok (payload untouched); lets declarations hold dynamic
// unwraps as results without mistyping the storage.
rl_result rl_result_unwrap_result(rl_result r);
int64_t rl_result_unwrap_i64(rl_result r);
double rl_result_unwrap_f64(rl_result r);
bool rl_result_unwrap_bool(rl_result r);
rl_string rl_result_unwrap_str(rl_result r);
// Checked unwrap of an array payload; aborts on error like the rest.
rl_array rl_result_unwrap_arr(rl_result r);
// Checked unwrap of a map payload; aborts on error like the rest.
rl_map rl_result_unwrap_map(rl_result r);
// Checked unwrap of a set payload; aborts on error like the rest.
rl_set rl_result_unwrap_set(rl_result r);
// Checked unwrap of a boxed closure payload; aborts on error like the rest.
rl_closure rl_result_unwrap_closure(rl_result r);
// Unwrap an error payload; aborts when given an ok value.
rl_string rl_result_unwrap_err_str(rl_result r);
int64_t rl_result_unwrap_err_i64(rl_result r);
double rl_result_unwrap_err_f64(rl_result r);
bool rl_result_unwrap_err_bool(rl_result r);
// Checked unwrap of the i64 payload (generic fallback used by codegen).
rl_result rl_result_unwrap_auto(rl_result r);
// Length of a string/array/map/set result payload, or an error.
rl_result rl_len_result(rl_result v);

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
// Return ok bool true when payload tag is array.
rl_result rl_is_array(rl_result x);
// Return ok bool true when payload tag is map.
rl_result rl_is_map(rl_result x);
// Return ok bool true when payload tag is set.
rl_result rl_is_set(rl_result x);
// Tuples never travel boxed; always false at runtime (literals fold).
rl_result rl_is_tuple(rl_result x);
// Return ok bool true when payload is a closure.
rl_result rl_is_function(rl_result x);
// Narrow ints erase to I64 at runtime; literals fold statically.
rl_result rl_is_uint(rl_result x);
// Narrow ints erase to I64 at runtime; literals fold statically.
rl_result rl_is_sbyte(rl_result x);
// Narrow ints erase to I64 at runtime; literals fold statically.
rl_result rl_is_bsbyte(rl_result x);
// Narrow ints erase to I64 at runtime; literals fold statically.
rl_result rl_is_bbyte(rl_result x);
// Narrow ints erase to I64 at runtime; literals fold statically.
rl_result rl_is_sint(rl_result x);
// Narrow ints erase to I64 at runtime; literals fold statically.
rl_result rl_is_suint(rl_result x);
// Small float erases to F64 at runtime; literals fold statically.
rl_result rl_is_sfloat(rl_result x);
// Handle kind testers: true only for live ids of that kind. Bare ints
// and other payloads are always false. Gui has no C backend and always
// returns false.
rl_result rl_is_c_handle(rl_result x);
rl_result rl_is_net_handle(rl_result x);
rl_result rl_is_http_handle(rl_result x);
rl_result rl_is_audio_handle(rl_result x);
rl_result rl_is_gui_handle(rl_result x);
rl_result rl_is_file_handle(rl_result x);

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
// shared objects, and call their symbols. Handles are tagged integer ids
// into an internal table (`rl_c_handle` is the raw dlopen pointer type).
// Each domain uses its own base so kinds never overlap and small bare
// ints never collide with real handles.
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
// `"127.0.0.1:8080"`. Each open socket is a tagged handle id; results hold
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
// Half-close the read side, write side, or both ("read" / "write" / "both").
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
// Receive one datagram / datagram plus sender address as a 2-tuple
// (data string, sender "ip:port" string) in a single element array.
rl_result rl_net_udp_recv(int64_t handle_id, int64_t max_bytes);
rl_result rl_net_udp_recv_from(int64_t handle_id, int64_t max_bytes);
// Close the socket.
rl_result rl_net_udp_close(int64_t handle_id);
// DNS lookup of `"host:port"`; result holds an array of `"ip"` strings.
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
// GET / POST shorthand; result holds a 2-tuple (status int, body string)
// in a single element array. Non-2xx statuses are still ok.
rl_result rl_http_get(rl_string url);
rl_result rl_http_post(rl_string url, rl_string body, rl_string content_type, int has_content_type);
// Full client request; result holds a 2-tuple (status int, body string)
// in a single element array. `headers` is an array of 2-tuples
// (name string, value string). Pass `has_body` / `has_headers` 0 to skip.
rl_result rl_http_request(rl_string method, rl_string url, rl_string body, int has_body, rl_array headers, int has_headers);

// ---- std::audio (playback via miniaudio) ----
// Blocking and async sound playback plus file metadata. Playing sounds
// are int64 handle ids into an internal table (tagged so kinds never
// overlap and small bare ints never collide with real handles); results
// hold either the value (null, bool, float, int, handle id, string
// array, metadata tuple) or an RL error. Unknown or stopped ids fail
// with an RL error and never crash.
// Play a file to completion; ok null on success.
rl_result rl_audio_play_file(rl_string path);
// Start async playback; ok with the sound handle id.
rl_result rl_audio_play_file_async(rl_string path);
// Play a sine tone of `freq` Hz for `duration_ms` and block; ok null.
rl_result rl_audio_beep(double freq, int64_t duration_ms);
// Pause / resume a live sound; ok null.
rl_result rl_audio_sound_pause(int64_t handle_id);
rl_result rl_audio_sound_resume(int64_t handle_id);
// Stop a sound and release its handle; ok null.
rl_result rl_audio_sound_stop(int64_t handle_id);
// True when the sound is paused; ok bool.
rl_result rl_audio_sound_is_paused(int64_t handle_id);
// Per-sound volume (base, before the master volume); ok null.
rl_result rl_audio_sound_set_volume(int64_t handle_id, double volume);
// Per-sound base volume; ok float.
rl_result rl_audio_sound_get_volume(int64_t handle_id);
// Playback speed multiplier; ok null.
rl_result rl_audio_sound_set_speed(int64_t handle_id, double speed);
// Seek to `position_ms` from the start; ok null.
rl_result rl_audio_sound_seek(int64_t handle_id, int64_t position_ms);
// True when playback reached the end; ok bool.
rl_result rl_audio_sound_is_finished(int64_t handle_id);
// Block until playback reaches the end; ok null.
rl_result rl_audio_sound_wait(int64_t handle_id);
// Playback device names; ok with a string array.
rl_result rl_audio_list_output_devices(void);
// Select the device used for future playback; ok null.
rl_result rl_audio_set_output_device(rl_string name);
// Master volume applied to every sound; ok null.
rl_result rl_audio_set_master_volume(double volume);
// File duration in milliseconds; ok int.
rl_result rl_audio_duration(rl_string path);
// File metadata as (channels, sample rate, duration ms, format name)
// in a single element array; ok with the tuple.
rl_result rl_audio_file_info(rl_string path);

#endif
