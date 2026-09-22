#define _GNU_SOURCE
#define _POSIX_C_SOURCE 200809L
#include "rl_runtime.h"

// Single-header audio backend: compiled only for programs using it
// (rlt defines RL_USE_AUDIO then), so other programs skip the ~4MB header.
#ifdef RL_USE_AUDIO
#define MINIAUDIO_IMPLEMENTATION
#include "miniaudio.h"
#endif

// Forward declarations for helpers used before their definitions.
static const char *_rl_tag_name(enum rl_type_tag tag);
static char *_rl_trim_copy(rl_string s, uint64_t *out_len);
static bool _rl_utf8_decode(const char *s, uint64_t len, uint32_t *code, uint64_t *used);
static void _rl_abort(void);

// Tagged handle ids: each domain adds its base to a small index so
// kinds never overlap and small bare ints never collide with handles.
#define RL_HANDLE_C_BASE ((int64_t)0x1000000)
#define RL_HANDLE_NET_BASE ((int64_t)0x2000000)
#define RL_HANDLE_HTTP_BASE ((int64_t)0x3000000)
#define RL_HANDLE_FILE_BASE ((int64_t)0x4000000)
#define RL_HANDLE_AUDIO_BASE ((int64_t)0x5000000)

// Invoke a boxed closure value: unboxes, aborts loudly when the value
// is not a closure (e.g. calling a result that holds no closure).
rl_result rl_closure_call_checked(rl_result callee, rl_result *args, uint64_t argc) {
    if (!callee.is_ok || callee.tag != RL_TAG_CLOSURE || callee.data.closure == NULL) {
        fprintf(stderr, "error: value is not callable\n");
        _rl_abort();
    }
    return rl_closure_call(*callee.data.closure, args, argc);
}

// Build a closure value, heap-copying the captures so the closure
// outlives its definition site (returned or stored closures).
rl_closure rl_closure_new_heap(rl_closure_fn fn, rl_result *captures, uint64_t capture_count) {
    rl_result *heap = NULL;
    if (capture_count > 0) {
        heap = malloc(capture_count * sizeof(rl_result));
        memcpy(heap, captures, capture_count * sizeof(rl_result));
    }
    rl_closure c = { .fn = fn, .captures = heap, .capture_count = capture_count };
    return c;
}

// Abort loud failures only after flushing pending output, so earlier
// prints are never lost when stdout is block-buffered (pipes).
static void _rl_abort(void) {
    fflush(stdout);
    fflush(stderr);
    abort();
}


// ---- string ----
// Borrowed string buffers and basic ops; concat allocates.
rl_string rl_str_literal(const char *s, uint64_t len) {
    rl_string str = { .data = s, .len = len, .rc = 0 };
    return str;
}

// Length of the string in bytes.
uint64_t rl_str_len(rl_string s) { return s.len; }

// Allocate a new string holding `a` followed by `b` (caller owns).
rl_string rl_str_concat(rl_string a, rl_string b) {
    uint64_t total = a.len + b.len;
    char *buf = (char *)malloc(total + 1);
    memcpy(buf, a.data, a.len);
    memcpy(buf + a.len, b.data, b.len);
    buf[total] = '\0';
    rl_string result = { .data = buf, .len = total, .rc = 1 };
    return result;
}

// Byte-wise equality comparison.
bool rl_str_eq(rl_string a, rl_string b) {
    if (a.len != b.len) return false;
    return memcmp(a.data, b.data, a.len) == 0;
}

// Format one interpolation argument into a fresh string (caller owns via
// out). `bare` marks plain values (render the payload, e.g. `5`); wrapped
// results render with their `ok(...)` / `err(...)` decoration, mirroring
// the VM's Display.
static void _rl_fmt_arg_to_str(rl_fmt_arg arg, char **out, uint64_t *out_len) {
    rl_result v = arg.v;
    char buf[160];
    int n = 0;
    if (!arg.bare) {
        if (!v.is_ok) { n = snprintf(buf, sizeof(buf), "err(%d)", v.err_code); }
        else switch (v.tag) {
            case RL_TAG_NULL: n = snprintf(buf, sizeof(buf), "ok(null)"); break;
            case RL_TAG_I64: n = snprintf(buf, sizeof(buf), "ok(%ld)", (long)v.data.i64); break;
            case RL_TAG_F64: n = snprintf(buf, sizeof(buf), "ok(%g)", v.data.f64); break;
            case RL_TAG_BOOL: n = snprintf(buf, sizeof(buf), "ok(%s)", v.data.boolean ? "true" : "false"); break;
            case RL_TAG_CHAR: n = snprintf(buf, sizeof(buf), "ok(%c)", (char)(unsigned char)v.data.i64); break;
            case RL_TAG_STR: n = snprintf(buf, sizeof(buf), "ok(%.*s)", (int)v.data.str.len, v.data.str.data); break;
            case RL_TAG_CLOSURE: n = snprintf(buf, sizeof(buf), "ok(<fn>)"); break;
            default: n = snprintf(buf, sizeof(buf), "ok(<value>)"); break;
        }
        char *dup = malloc(n + 1);
        memcpy(dup, buf, n + 1);
        *out = dup;
        *out_len = n;
        return;
    }
    if (!v.is_ok) { n = snprintf(buf, sizeof(buf), "err(%d)", v.err_code); }
    else switch (v.tag) {
        case RL_TAG_NULL: n = snprintf(buf, sizeof(buf), "null"); break;
        case RL_TAG_I64: n = snprintf(buf, sizeof(buf), "%ld", (long)v.data.i64); break;
        case RL_TAG_F64: n = snprintf(buf, sizeof(buf), "%g", v.data.f64); break;
        case RL_TAG_BOOL: n = snprintf(buf, sizeof(buf), "%s", v.data.boolean ? "true" : "false"); break;
        case RL_TAG_CHAR: n = snprintf(buf, sizeof(buf), "%c", (char)(unsigned char)v.data.i64); break;
        case RL_TAG_STR: { char *d = malloc(v.data.str.len + 1); memcpy(d, v.data.str.data, v.data.str.len); d[v.data.str.len] = '\0'; *out = d; *out_len = v.data.str.len; return; }
        case RL_TAG_CLOSURE: n = snprintf(buf, sizeof(buf), "<fn>"); break;
        default: n = snprintf(buf, sizeof(buf), "<value>"); break;
    }
    char *dup = malloc(n + 1);
    memcpy(dup, buf, n + 1);
    *out = dup;
    *out_len = n;
}

// Concatenate argc arguments (used by `concat`).
rl_string rl_str_concat_variadic(rl_fmt_arg *args, uint64_t argc) {
    uint64_t total = 0;
    char **parts = malloc(argc * sizeof(char *));
    uint64_t *part_lens = malloc(argc * sizeof(uint64_t));
    for (uint64_t i = 0; i < argc; i++) {
        _rl_fmt_arg_to_str(args[i], &parts[i], &part_lens[i]);
        total += part_lens[i];
    }
    char *buf = malloc(total + 1);
    uint64_t pos = 0;
    for (uint64_t i = 0; i < argc; i++) {
        memcpy(buf + pos, parts[i], part_lens[i]);
        pos += part_lens[i];
        free(parts[i]);
    }
    buf[total] = '\0';
    free(parts);
    free(part_lens);
    return (rl_string){ .data = buf, .len = total, .rc = 1 };
}

// Interpolate args into a `{}` template string (caller owns). Returns a
// bare string like the VM; arity mismatches abort like a VM error.
rl_string rl_str_format(rl_string tmpl, rl_fmt_arg *args, uint64_t argc) {
    uint64_t total = 0;
    // Worst case alternates literal segments and placeholders: argc
    // arguments plus argc + 1 literal runs.
    uint64_t cap = 2 * argc + 1;
    char **parts = malloc(cap * sizeof(char *));
    uint64_t *part_lens = malloc(cap * sizeof(uint64_t));
    uint64_t part_count = 0;
    uint64_t arg_idx = 0;
    uint64_t i = 0;
    while (i < tmpl.len) {
        if (tmpl.data[i] == '{' && i + 1 < tmpl.len && tmpl.data[i + 1] == '}') {
            if (arg_idx >= argc) {
                for (uint64_t j = 0; j < part_count; j++) free(parts[j]);
                free(parts); free(part_lens);
                fprintf(stderr, "error: format() has placeholder(s) with no matching argument\n");
                _rl_abort();
            }
            _rl_fmt_arg_to_str(args[arg_idx], &parts[part_count], &part_lens[part_count]);
            total += part_lens[part_count];
            part_count++;
            arg_idx++;
            i += 2;
        } else {
            uint64_t start = i;
            while (i < tmpl.len && !(tmpl.data[i] == '{' && i + 1 < tmpl.len && tmpl.data[i + 1] == '}')) i++;
            uint64_t seg_len = i - start;
            parts[part_count] = malloc(seg_len + 1);
            memcpy(parts[part_count], tmpl.data + start, seg_len);
            parts[part_count][seg_len] = '\0';
            part_lens[part_count] = seg_len;
            total += seg_len;
            part_count++;
        }
    }
    if (arg_idx < argc) {
        for (uint64_t j = 0; j < part_count; j++) free(parts[j]);
        free(parts); free(part_lens);
        fprintf(stderr, "error: format() received more arguments than placeholders\n");
        _rl_abort();
    }
    char *buf = malloc(total + 1);
    uint64_t pos = 0;
    for (uint64_t j = 0; j < part_count; j++) {
        memcpy(buf + pos, parts[j], part_lens[j]);
        pos += part_lens[j];
        free(parts[j]);
    }
    buf[total] = '\0';
    free(parts);
    free(part_lens);
    return (rl_string){ .data = buf, .len = total, .rc = 1 };
}

// Pairwise tuples up to the shorter length.
rl_result rl_arr_zip(rl_array a, rl_array b) {
    uint64_t min_len = a.len < b.len ? a.len : b.len;
    int64_t *a_elems = (int64_t *)a.data;
    int64_t *b_elems = (int64_t *)b.data;
    uint64_t cap = 16;
    int64_t *buf = malloc(cap * sizeof(int64_t));
    uint64_t count = 0;
    for (uint64_t i = 0; i < min_len; i++) {
        if (count + 2 > cap) { cap *= 2; buf = realloc(buf, cap * sizeof(int64_t)); }
        buf[count++] = a_elems[i];
        buf[count++] = b_elems[i];
    }
    return rl_ok_arr(rl_arr_from_vals(buf, count, sizeof(int64_t)));
}

// Pairwise tuples: element `i` of each side lands in `.field_0` /
// `.field_1` of a tuple struct with total size `es_tuple` and second
// field offset `off_b`. Sides copy `es_a` / `es_b` bytes each.
rl_result rl_arr_zip_t(rl_array a, rl_array b, uint64_t es_tuple, uint64_t off_b, uint64_t es_a, uint64_t es_b) {
    uint64_t min_len = a.len < b.len ? a.len : b.len;
    uint64_t cap = min_len > 0 ? min_len : 1;
    char *buf = malloc(cap * es_tuple);
    for (uint64_t i = 0; i < min_len; i++) {
        char *slot = buf + i * es_tuple;
        memset(slot, 0, es_tuple);
        if (a.data) memcpy(slot, (char *)a.data + i * es_a, es_a);
        if (b.data) memcpy(slot + off_b, (char *)b.data + i * es_b, es_b);
    }
    rl_array out;
    out.data = min_len > 0 ? buf : NULL;
    if (min_len == 0) free(buf);
    out.len = min_len;
    out.cap = min_len;
    out.elem_size = (int32_t)es_tuple;
    out.type_tag = RL_TAG_I64;
    return rl_ok_arr(out);
}

// ---- result type ----

// Wrap a null in a successful result.
rl_result rl_ok_null(void) {
    rl_result r = { .is_ok = true, .tag = RL_TAG_NULL, .err_code = 0 };
    return r;
}

// Wrap an int64 value in a successful result.
rl_result rl_ok_i64(int64_t v) {
    rl_result r = { .is_ok = true, .tag = RL_TAG_I64, .data.i64 = v, .err_code = 0 };
    return r;
}

// Wrap a float64 value in a successful result.
rl_result rl_ok_f64(double v) {
    rl_result r = { .is_ok = true, .tag = RL_TAG_F64, .data.f64 = v, .err_code = 0 };
    return r;
}

// Wrap a bool value in a successful result.
rl_result rl_ok_bool(bool v) {
    rl_result r = { .is_ok = true, .tag = RL_TAG_BOOL, .data.boolean = v, .err_code = 0 };
    return r;
}

// Wrap a string value in a successful result.
rl_result rl_ok_str(rl_string v) {
    // A null string wraps as null, so `is_null`/`type_of` see through it.
    if (v.data == NULL) return rl_ok_null();
    rl_result r = { .is_ok = true, .tag = RL_TAG_STR, .data.str = v, .err_code = 0 };
    return r;
}

// Wrap an array value in a successful result.
rl_result rl_ok_arr(rl_array v) {
    rl_result r = { .is_ok = true, .tag = RL_TAG_ARR, .data.arr = v, .err_code = 0 };
    return r;
}

// Wrap a map value in a successful result.
rl_result rl_ok_map(rl_map v) {
    rl_result r = { .is_ok = true, .tag = RL_TAG_MAP, .data.map = v, .err_code = 0 };
    return r;
}

// Wrap a set value in a successful result.
rl_result rl_ok_set(rl_set v) {
    rl_result r = { .is_ok = true, .tag = RL_TAG_SET, .data.set = v, .err_code = 0 };
    return r;
}

// Build an error result with code and C string message.
static rl_result rl_make_err(int64_t code, const char *msg) {
    rl_string s = rl_str_literal(msg, strlen(msg));
    rl_result r = { .is_ok = false, .tag = RL_TAG_STR, .data.str = s, .err_code = (int32_t)code };
    return r;
}

// Build a failed result carrying a message.
rl_result rl_err_msg(rl_string msg) {
    rl_result r = { .is_ok = false, .tag = RL_TAG_STR, .data.str = msg, .err_code = 0 };
    return r;
}

// Build a failed result with numeric code plus message.
rl_result rl_err_code(int64_t code, rl_string msg) {
    rl_result r = { .is_ok = false, .tag = RL_TAG_STR, .data.str = msg, .err_code = (int32_t)code };
    return r;
}

// Build a failed result carrying a numeric code.
rl_result rl_err(int64_t v) {
    rl_result r = { .is_ok = false, .tag = RL_TAG_I64, .data.i64 = v, .err_code = 0 };
    return r;
}

// Alias of rl_err kept for older generated code.
rl_result rl_error(int64_t v) {
    rl_result r = { .is_ok = false, .tag = RL_TAG_I64, .data.i64 = v, .err_code = -1 };
    return r;
}

// ---- array type ----

// Copy count elements of elem_size bytes into a new array.
rl_array rl_arr_from_vals(const void *vals, uint64_t count, int32_t elem_size) {
    rl_array arr;
    arr.len = count;
    arr.cap = count;
    arr.elem_size = elem_size;
    arr.type_tag = RL_TAG_I64;
    if (count == 0) {
        arr.data = NULL;
    } else {
        arr.data = malloc(count * elem_size);
        memcpy(arr.data, vals, count * elem_size);
    }
    return arr;
}

// Copy count elements with an explicit payload tag (literals record
// their element kind so later ops dispatch correctly).
rl_array rl_arr_from_vals_tag(const void *vals, uint64_t count, int32_t elem_size, int32_t tag) {
    rl_array arr = rl_arr_from_vals(vals, count, elem_size);
    arr.type_tag = tag;
    return arr;
}

// Allocate an empty array for elements of elem_size bytes.
rl_array rl_arr_new(int32_t elem_size) {
    rl_array arr = { .data = NULL, .len = 0, .cap = 0, .elem_size = elem_size, .type_tag = RL_TAG_I64 };
    return arr;
}

// ---- map type ----

// Allocate an empty map.
rl_map rl_map_new(void) {
    rl_map m = { .entries = NULL, .len = 0, .cap = 0 };
    return m;
}

// Grow map capacity to hold at least needed entries.
static void rl_map_grow(rl_map *m, uint64_t needed) {
    if (m->cap >= needed) return;
    uint64_t new_cap = m->cap == 0 ? 8 : m->cap * 2;
    while (new_cap < needed) new_cap *= 2;
    m->entries = realloc(m->entries, new_cap * sizeof(rl_map_entry));
    m->cap = new_cap;
}

// Insert or overwrite `key` (a copy of the key string is kept).
void rl_map_set(rl_map *m, const char *key, rl_value val) {
    for (uint64_t i = 0; i < m->len; i++) {
        if (strcmp(m->entries[i].key, key) == 0) {
            m->entries[i].value = val;
            return;
        }
    }
    rl_map_grow(m, m->len + 1);
    m->entries[m->len].key = strdup(key);
    m->entries[m->len].value = val;
    m->len++;
}

// Look up `key`; returns a null-valued `rl_value` when absent.
rl_value rl_map_get(rl_map m, const char *key) {
    for (uint64_t i = 0; i < m.len; i++) {
        if (strcmp(m.entries[i].key, key) == 0) {
            return m.entries[i].value;
        }
    }
    rl_value null_val = { .tag = RL_VTAG_NULL, .data.i64 = 0 };
    return null_val;
}

// True when `key` is present.
bool rl_map_contains(rl_map m, const char *key) {
    for (uint64_t i = 0; i < m.len; i++) {
        if (strcmp(m.entries[i].key, key) == 0) return true;
    }
    return false;
}

// Number of entries in the map.
uint64_t rl_map_len(rl_map m) { return m.len; }

// Delete `key` if present (no-op otherwise).
void rl_map_remove(rl_map *m, const char *key) {
    for (uint64_t i = 0; i < m->len; i++) {
        if (strcmp(m->entries[i].key, key) == 0) {
            free(m->entries[i].key);
            m->entries[i] = m->entries[m->len - 1];
            m->len--;
            return;
        }
    }
}

// ---- set type ----

// Allocate an empty set.
rl_set rl_set_new(void) {
    rl_set s = { .data = NULL, .len = 0, .cap = 0 };
    return s;
}

// Grow set buffer to hold at least needed elements.
static void rl_set_grow(rl_set *s, uint64_t needed) {
    if (s->cap >= needed) return;
    uint64_t new_cap = s->cap == 0 ? 8 : s->cap * 2;
    while (new_cap < needed) new_cap *= 2;
    s->data = realloc(s->data, new_cap * sizeof(rl_value));
    s->cap = new_cap;
}

// Compare two boxed values for equality by tag and payload.
static bool rl_value_eq(rl_value a, rl_value b) {
    if (a.tag != b.tag) return false;
    switch (a.tag) {
        case RL_VTAG_NULL: return true;
        case RL_VTAG_I64: return a.data.i64 == b.data.i64;
        case RL_VTAG_F64: return a.data.f64 == b.data.f64;
        case RL_VTAG_BOOL: return a.data.boolean == b.data.boolean;
        case RL_VTAG_CHAR: return a.data.i64 == b.data.i64;
        case RL_VTAG_STR: return rl_str_eq(a.data.str, b.data.str);
        default: return false;
    }
}

// Insert `val` unless an equal value is already present.
void rl_set_add(rl_set *s, rl_value val) {
    for (uint64_t i = 0; i < s->len; i++) {
        if (rl_value_eq(s->data[i], val)) return;
    }
    rl_set_grow(s, s->len + 1);
    s->data[s->len++] = val;
}

// True when an equal value is present.
bool rl_set_contains(rl_set s, rl_value val) {
    for (uint64_t i = 0; i < s.len; i++) {
        if (rl_value_eq(s.data[i], val)) return true;
    }
    return false;
}

// Number of elements in the set.
uint64_t rl_set_len(rl_set s) { return s.len; }

// Delete the first element equal to `val` (no-op when absent).
void rl_set_remove(rl_set *s, rl_value val) {
    for (uint64_t i = 0; i < s->len; i++) {
        if (rl_value_eq(s->data[i], val)) {
            s->data[i] = s->data[s->len - 1];
            s->len--;
            return;
        }
    }
}

// ---- print functions ----

// Print double with shortest round-trip precision.
static void rl_print_f64(double v) {
    if (isnan(v)) { printf("NaN"); return; }
    if (isinf(v) > 0) { printf("inf"); return; }
    if (isinf(v) < 0) { printf("-inf"); return; }
    char buf[64];
    int best_prec = 17;
    for (int prec = 1; prec <= 17; prec++) {
        char tmp[64];
        snprintf(tmp, sizeof(tmp), "%.*g", prec, v);
        if (strtod(tmp, NULL) == v) {
            best_prec = prec;
            break;
        }
    }
    snprintf(buf, sizeof(buf), "%.*g", best_prec, v);
    printf("%s", buf);
}

// Print int64 payload (inner helper, no newline).
static void rl_print_i64_val(int64_t v) { printf("%ld", v); }
// Print float64 payload (inner helper, no newline).
static void rl_print_f64_val(double v) { rl_print_f64(v); }
// Print bool payload (inner helper, no newline).
static void rl_print_bool_val(bool v) { printf(v ? "true" : "false"); }
// Print string payload bytes (inner helper, no newline).
static void rl_print_str_val(rl_string v) { printf("%.*s", (int)v.len, v.data); }
// Print array payload (inner helper, no newline).
static void rl_print_arr_val(rl_array v) { rl_print_rl_array(v); }
// Print map payload (inner helper, no newline).
static void rl_print_map_val(rl_map v) { rl_print_rl_map(v); }
// Print set payload (inner helper, no newline).
static void rl_print_set_val(rl_set v) { rl_print_rl_set(v); }

// Print int64 value with no trailing newline.
void rl_print_int64(int64_t v) { printf("%ld", v); }
// Print float64 value with no trailing newline.
void rl_print_float64(double v) { rl_print_f64(v); }
// Print bool value with no trailing newline.
void rl_print_bool(bool v) { printf(v ? "true" : "false"); }
// Print char value with no trailing newline.
void rl_print_char(char v) { printf("%c", v); }
// Print string bytes with no trailing newline.
void rl_print_str(rl_string v) {
    if (v.data == NULL) { printf("null"); return; }
    printf("%.*s", (int)v.len, v.data);
}
// Print pointer as <ptr:...> with no trailing newline.
void rl_print_ptr(void *v) { printf("<ptr:%p>", v); }
// Print null with no trailing newline.
void rl_print_null(void) { printf("null"); }

// Print int64 value plus a trailing newline.
void rl_println_int64(int64_t v) { printf("%ld\n", v); }
// Print float64 value plus a trailing newline.
void rl_println_float64(double v) { rl_print_f64(v); printf("\n"); }
// Print bool value plus a trailing newline.
void rl_println_bool(bool v) { printf("%s\n", v ? "true" : "false"); }
// Print char value plus a trailing newline.
void rl_println_char(char v) { printf("%c\n", v); }
// Print string bytes plus a trailing newline.
void rl_println_str(rl_string v) {
    if (v.data == NULL) { printf("null\n"); return; }
    printf("%.*s\n", (int)v.len, v.data);
}
// Print pointer as <ptr:...> plus a trailing newline.
void rl_println_ptr(void *v) { printf("<ptr:%p>\n", v); }
// Print null plus a trailing newline.
void rl_println_null(void) { printf("null\n"); }

// Print result as ok(...) or err(...) (inner helper).
static void rl_print_result_inner(rl_result v) {
    if (v.is_ok) {
        printf("ok(");
        switch (v.tag) {
             case RL_TAG_NULL: printf("null"); break;
            case RL_TAG_I64: rl_print_i64_val(v.data.i64); break;
            case RL_TAG_F64: rl_print_f64_val(v.data.f64); break;
            case RL_TAG_BOOL: rl_print_bool_val(v.data.boolean); break;
            case RL_TAG_CHAR: printf("%c", (char)(unsigned char)v.data.i64); break;
            case RL_TAG_STR: printf("%.*s", (int)v.data.str.len, v.data.str.data); break;
            case RL_TAG_ARR: rl_print_arr_val(v.data.arr); break;
            case RL_TAG_MAP: rl_print_map_val(v.data.map); break;
            case RL_TAG_SET: rl_print_set_val(v.data.set); break;
            case RL_TAG_CLOSURE: printf("<fn>"); break;
        }
        printf(")");
    } else {
        printf("err(");
        switch (v.tag) {
            case RL_TAG_NULL: printf("null"); break;
            case RL_TAG_I64: rl_print_i64_val(v.data.i64); break;
            case RL_TAG_F64: rl_print_f64_val(v.data.f64); break;
            case RL_TAG_BOOL: rl_print_bool_val(v.data.boolean); break;
            case RL_TAG_CHAR: printf("%c", (char)(unsigned char)v.data.i64); break;
            case RL_TAG_STR: printf("%.*s", (int)v.data.str.len, v.data.str.data); break;
            case RL_TAG_ARR: rl_print_arr_val(v.data.arr); break;
            case RL_TAG_MAP: rl_print_map_val(v.data.map); break;
            case RL_TAG_SET: rl_print_set_val(v.data.set); break;
            case RL_TAG_CLOSURE: printf("<fn>"); break;
        }
        printf(")");
    }
}

// Print a result payload as ok(...) or err(...).
void rl_print_result(rl_result v) { rl_print_result_inner(v); }
// Print a result payload as ok(...) or err(...) plus newline.
void rl_println_result(rl_result v) { rl_print_result_inner(v); printf("\n"); }

// Print result payload without ok/err wrapper (inner helper).
static void rl_print_raw_inner(rl_result v) {
    switch (v.tag) {
        case RL_TAG_NULL: printf("null"); break;
        case RL_TAG_I64: rl_print_i64_val(v.data.i64); break;
        case RL_TAG_F64: rl_print_f64_val(v.data.f64); break;
        case RL_TAG_BOOL: printf("%s", v.data.boolean ? "true" : "false"); break;
        case RL_TAG_CHAR: printf("%c", (char)(unsigned char)v.data.i64); break;
        case RL_TAG_STR: printf("%.*s", (int)v.data.str.len, v.data.str.data); break;
        case RL_TAG_ARR: rl_print_arr_val(v.data.arr); break;
        case RL_TAG_MAP: rl_print_map_val(v.data.map); break;
        case RL_TAG_SET: rl_print_set_val(v.data.set); break;
        case RL_TAG_CLOSURE: printf("<fn>"); break;
    }
}
// Print a result payload without type decoration.
void rl_print_raw(rl_result v) { rl_print_raw_inner(v); }
// Print a result payload without decoration plus newline.
void rl_println_raw(rl_result v) { rl_print_raw_inner(v); printf("\n"); }

// Print array in RL literal syntax ([1, 2]).
void rl_print_rl_array(rl_array v) {
    printf("[");
    for (uint64_t i = 0; i < v.len; i++) {
        if (i > 0) printf(", ");
        if (v.type_tag == RL_TAG_CHAR) {
            printf("%c", ((char *)v.data)[i]);
        } else if (v.elem_size == sizeof(rl_array) && v.type_tag == RL_TAG_ARR) {
            rl_print_rl_array(((rl_array *)v.data)[i]);
        } else if (v.type_tag == RL_TAG_F64 && v.elem_size == sizeof(double)) {
            { double fv = ((double *)v.data)[i]; rl_print_f64(fv); }
        } else if (v.elem_size == sizeof(int64_t)) {
            printf("%ld", (long)((int64_t *)v.data)[i]);
        } else if (v.elem_size == sizeof(double)) {
            { double fv = ((double *)v.data)[i]; rl_print_f64(fv); }
        } else if (v.elem_size == sizeof(rl_string)) {
            rl_string s = ((rl_string *)v.data)[i];
            printf("%.*s", (int)s.len, s.data);
        } else if (v.elem_size == sizeof(bool)) {
            printf("%s", ((bool *)v.data)[i] ? "true" : "false");
        } else {
            printf("...");
        }
    }
    printf("]");
}

// Print array in RL syntax plus a trailing newline.
void rl_println_rl_array(rl_array v) {
    rl_print_rl_array(v);
    printf("\n");
}

// Print a boxed map/set element value.
static void _rl_print_value(rl_value v) {
    switch (v.tag) {
        case RL_VTAG_NULL: printf("null"); break;
        case RL_VTAG_I64: printf("%ld", (long)v.data.i64); break;
        case RL_VTAG_F64: printf("%g", v.data.f64); break;
        case RL_VTAG_BOOL: printf("%s", v.data.boolean ? "true" : "false"); break;
        case RL_VTAG_CHAR: printf("%c", (char)(unsigned char)v.data.i64); break;
        case RL_VTAG_STR: printf("%.*s", (int)v.data.str.len, v.data.str.data); break;
        case RL_VTAG_ARR: rl_print_rl_array(v.data.arr); break;
        case RL_VTAG_CLOSURE: printf("<fn>"); break;
        default: printf("<value>"); break;
    }
}

// Print map in RL literal syntax ({k: v}).
void rl_print_rl_map(rl_map v) {
    printf("{");
    for (uint64_t i = 0; i < v.len; i++) {
        if (i > 0) printf(", ");
        printf("%s: ", v.entries[i].key);
        _rl_print_value(v.entries[i].value);
    }
    printf("}");
}

// Print map in RL syntax plus a trailing newline.
void rl_println_rl_map(rl_map v) {
    rl_print_rl_map(v);
    printf("\n");
}

// Print set values in braces.
void rl_print_rl_set(rl_set v) {
    printf("{");
    for (uint64_t i = 0; i < v.len; i++) {
        if (i > 0) printf(", ");
        _rl_print_value(v.data[i]);
    }
    printf("}");
}

// Print set values in braces plus a trailing newline.
void rl_println_rl_set(rl_set v) {
    rl_print_rl_set(v);
    printf("\n");
}

// Print a closure as an opaque <fn> placeholder.
void rl_print_closure(rl_closure v) {
    printf("<fn>");
}

// Print a closure placeholder plus a trailing newline.
void rl_println_closure(rl_closure v) {
    rl_print_closure(v);
    printf("\n");
}

// ---- math ----

// Return n! (0 for negative n).
int64_t rl_math_factorial(int64_t n) {
    if (n < 0) return 0;
    int64_t result = 1;
    for (int64_t i = 2; i <= n; i++) { result *= i; }
    return result;
}

// Greatest common divisor of a and b.
int64_t rl_math_gcd(int64_t a, int64_t b) {
    a = a < 0 ? -a : a;
    b = b < 0 ? -b : b;
    while (b) { int64_t t = b; b = a % b; a = t; }
    return a;
}

// Least common multiple of a and b (0 when either is 0).
int64_t rl_math_lcm(int64_t a, int64_t b) {
    if (a == 0 || b == 0) return 0;
    a = a < 0 ? -a : a;
    b = b < 0 ? -b : b;
    return (a / rl_math_gcd(a, b)) * b;
}

// True when n is prime.
bool rl_math_is_prime(int64_t n) {
    if (n < 2) return false;
    if (n == 2) return true;
    if (n % 2 == 0) return false;
    for (int64_t i = 3; i * i <= n; i += 2) {
        if (n % i == 0) return false;
    }
    return true;
}

// Nth Fibonacci number (0 for n <= 0).
int64_t rl_math_fibonacci(int64_t n) {
    if (n <= 0) return 0;
    if (n == 1) return 1;
    int64_t a = 0, b = 1;
    for (int64_t i = 2; i <= n; i++) {
        int64_t t = a + b;
        a = b;
        b = t;
    }
    return b;
}

// Rotate left (64-bit, shift mod 64 like the VM).
rl_result rl_bitwise_rotate_left(int64_t a, int64_t shift) {
    uint32_t s = (uint32_t)shift;
    uint32_t n = s % 64;
    uint64_t u = (uint64_t)a;
    uint64_t r = n == 0 ? u : (u << n) | (u >> (64 - n));
    return rl_ok_i64((int64_t)r);
}

// Rotate right (64-bit, shift mod 64 like the VM).
rl_result rl_bitwise_rotate_right(int64_t a, int64_t shift) {
    uint32_t s = (uint32_t)shift;
    uint32_t n = s % 64;
    uint64_t u = (uint64_t)a;
    uint64_t r = n == 0 ? u : (u >> n) | (u << (64 - n));
    return rl_ok_i64((int64_t)r);
}

// Copy of `a` with bit `n` set.
rl_result rl_bitwise_bit_set(int64_t a, int64_t n) {
    uint32_t pos = (uint32_t)n;
    if (pos >= 64) return rl_ok_i64(a);
    return rl_ok_i64(a | ((int64_t)1 << pos));
}

// Copy of `a` with bit `n` cleared.
rl_result rl_bitwise_bit_clear(int64_t a, int64_t n) {
    uint32_t pos = (uint32_t)n;
    if (pos >= 64) return rl_ok_i64(a);
    return rl_ok_i64(a & ~((int64_t)1 << pos));
}

// Copy of `a` with bit `n` flipped.
rl_result rl_bitwise_bit_toggle(int64_t a, int64_t n) {
    uint32_t pos = (uint32_t)n;
    if (pos >= 64) return rl_ok_i64(a);
    return rl_ok_i64(a ^ ((int64_t)1 << pos));
}

// True when bit `n` of `a` is set.
rl_result rl_bitwise_bit_is_set(int64_t a, int64_t n) {
    uint32_t pos = (uint32_t)n;
    if (pos >= 64) return rl_ok_bool(false);
    return rl_ok_bool((a & ((int64_t)1 << pos)) != 0);
}

// ---- time ----

// Wall-clock time in milliseconds since the Unix epoch.
int64_t rl_time_now_ms(void) {
    struct timespec ts;
    clock_gettime(CLOCK_REALTIME, &ts);
    return (int64_t)ts.tv_sec * 1000 + ts.tv_nsec / 1000000;
}

// Monotonic nanos since first call (RL monotonic_now).
int64_t rl_time_monotonic_now(void) {
    static struct timespec start;
    static int started = 0;
    struct timespec now;
    clock_gettime(CLOCK_MONOTONIC, &now);
    if (!started) {
        start = now;
        started = 1;
        return 0;
    }
    return (int64_t)(now.tv_sec - start.tv_sec) * 1000000000LL + (int64_t)(now.tv_nsec - start.tv_nsec);
}

// ---- fs ----

// Create a single directory; result is an error when it fails.
rl_result rl_fs_mkdir(rl_string path) {
    char buf[path.len + 1];
    memcpy(buf, path.data, path.len);
    buf[path.len] = '\0';
    int rc = mkdir(buf, 0755);
    if (rc == 0) return rl_ok_i64(0);
    return rl_make_err(-1, "mkdir failed");
}

// ---- string ----

// out-of-range indexes clamp instead of trapping. ASCII case conversion.
rl_string rl_str_to_upper(rl_string s) {
    char *buf = malloc(s.len + 1);
    for (uint64_t i = 0; i < s.len; i++) {
        buf[i] = toupper((unsigned char)s.data[i]);
    }
    buf[s.len] = '\0';
    rl_string result = { .data = buf, .len = s.len, .rc = 1 };
    return result;
}

// out-of-range indexes clamp instead of trapping. ASCII case conversion.
rl_string rl_str_to_lower(rl_string s) {
    char *buf = malloc(s.len + 1);
    for (uint64_t i = 0; i < s.len; i++) {
        buf[i] = tolower((unsigned char)s.data[i]);
    }
    buf[s.len] = '\0';
    rl_string result = { .data = buf, .len = s.len, .rc = 1 };
    return result;
}

// Count leading whitespace bytes.
static uint64_t rl_str_skip_space_start(const char *s, uint64_t len) {
    uint64_t i = 0;
    while (i < len && (s[i] == ' ' || s[i] == '\t' || s[i] == '\n' || s[i] == '\r')) i++;
    return i;
}

// Return offset of trailing whitespace start.
static uint64_t rl_str_skip_space_end(const char *s, uint64_t len) {
    uint64_t i = len;
    while (i > 0 && (s[i-1] == ' ' || s[i-1] == '\t' || s[i-1] == '\n' || s[i-1] == '\r')) i--;
    return i;
}

// Strip whitespace on both sides, or on one side only.
rl_string rl_str_trim(rl_string s) {
    uint64_t start = rl_str_skip_space_start(s.data, s.len);
    uint64_t end = rl_str_skip_space_end(s.data, s.len);
    if (start >= end) {
        rl_string result = { .data = "", .len = 0, .rc = 1 };
        return result;
    }
    char *buf = malloc(end - start + 1);
    memcpy(buf, s.data + start, end - start);
    buf[end - start] = '\0';
    rl_string result = { .data = buf, .len = end - start, .rc = 1 };
    return result;
}

// Strip whitespace on both sides, or on one side only.
rl_string rl_str_trim_start(rl_string s) {
    uint64_t start = rl_str_skip_space_start(s.data, s.len);
    char *buf = malloc(s.len - start + 1);
    memcpy(buf, s.data + start, s.len - start);
    buf[s.len - start] = '\0';
    rl_string result = { .data = buf, .len = s.len - start, .rc = 1 };
    return result;
}

// Strip whitespace on both sides, or on one side only.
rl_string rl_str_trim_end(rl_string s) {
    uint64_t end = rl_str_skip_space_end(s.data, s.len);
    char *buf = malloc(end + 1);
    memcpy(buf, s.data, end);
    buf[end] = '\0';
    rl_string result = { .data = buf, .len = end, .rc = 1 };
    return result;
}

// Substring predicates.
bool rl_str_contains(rl_string haystack, rl_string needle) {
    if (needle.len == 0) return true;
    if (needle.len > haystack.len) return false;
    for (uint64_t i = 0; i <= haystack.len - needle.len; i++) {
        if (memcmp(haystack.data + i, needle.data, needle.len) == 0) return true;
    }
    return false;
}

// Substring predicates.
bool rl_str_starts_with(rl_string s, rl_string prefix) {
    if (prefix.len > s.len) return false;
    return memcmp(s.data, prefix.data, prefix.len) == 0;
}

// Substring predicates.
bool rl_str_ends_with(rl_string s, rl_string suffix) {
    if (suffix.len > s.len) return false;
    return memcmp(s.data + s.len - suffix.len, suffix.data, suffix.len) == 0;
}

// Replace every occurrence of `from` with `to`.
rl_string rl_str_replace(rl_string s, rl_string from, rl_string to) {
    if (from.len == 0) {
        char *buf = malloc(s.len + 1);
        memcpy(buf, s.data, s.len);
        buf[s.len] = '\0';
        rl_string result = { .data = buf, .len = s.len, .rc = 1 };
        return result;
    }
    uint64_t count = 0;
    uint64_t pos = 0;
    while (pos <= s.len) {
        if (pos + from.len <= s.len && memcmp(s.data + pos, from.data, from.len) == 0) {
            count++;
            pos += from.len;
        } else {
            pos++;
        }
    }
    if (count == 0) {
        char *buf = malloc(s.len + 1);
        memcpy(buf, s.data, s.len);
        buf[s.len] = '\0';
        rl_string result = { .data = buf, .len = s.len, .rc = 1 };
        return result;
    }
    uint64_t new_len = s.len - count * from.len + count * to.len;
    char *buf = malloc(new_len + 1);
    uint64_t w = 0;
    pos = 0;
    while (pos < s.len) {
        if (pos + from.len <= s.len && memcmp(s.data + pos, from.data, from.len) == 0) {
            memcpy(buf + w, to.data, to.len);
            w += to.len;
            pos += from.len;
        } else {
            buf[w++] = s.data[pos++];
        }
    }
    buf[new_len] = '\0';
    rl_string result = { .data = buf, .len = new_len, .rc = 1 };
    return result;
}

// Repeat `s` `count` times (empty string for `count <= 0`).
rl_string rl_str_repeat(rl_string s, int64_t count) {
    if (count <= 0 || s.len == 0) {
        rl_string result = { .data = "", .len = 0, .rc = 1 };
        return result;
    }
    uint64_t total = s.len * (uint64_t)count;
    char *buf = malloc(total + 1);
    for (int64_t i = 0; i < count; i++) {
        memcpy(buf + i * s.len, s.data, s.len);
    }
    buf[total] = '\0';
    rl_string result = { .data = buf, .len = total, .rc = 1 };
    return result;
}

// Byte offset of the first `needle` hit, or -1 when absent.
int64_t rl_str_index_of(rl_string haystack, rl_string needle) {
    if (needle.len == 0) return 0;
    if (needle.len > haystack.len) return -1;
    for (uint64_t i = 0; i <= haystack.len - needle.len; i++) {
        if (memcmp(haystack.data + i, needle.data, needle.len) == 0) return (int64_t)i;
    }
    return -1;
}

// Number of non-overlapping `needle` occurrences.
int64_t rl_str_count(rl_string haystack, rl_string needle) {
    if (needle.len == 0) return 0;
    int64_t count = 0;
    uint64_t pos = 0;
    while (pos <= haystack.len) {
        if (pos + needle.len <= haystack.len && memcmp(haystack.data + pos, needle.data, needle.len) == 0) {
            count++;
            pos += needle.len;
        } else {
            pos++;
        }
    }
    return count;
}

// Pad with `c` up to `width` bytes on the left / right.
rl_string rl_str_pad_left(rl_string s, int64_t width, char c) {
    int64_t pad = width - (int64_t)s.len;
    if (pad <= 0) {
        char *buf = malloc(s.len + 1);
        memcpy(buf, s.data, s.len);
        buf[s.len] = '\0';
        rl_string result = { .data = buf, .len = s.len, .rc = 1 };
        return result;
    }
    uint64_t total = (uint64_t)pad + s.len;
    char *buf = malloc(total + 1);
    for (int64_t i = 0; i < pad; i++) buf[i] = c;
    memcpy(buf + pad, s.data, s.len);
    buf[total] = '\0';
    rl_string result = { .data = buf, .len = total, .rc = 1 };
    return result;
}

// Pad with `c` up to `width` bytes on the left / right.
rl_string rl_str_pad_right(rl_string s, int64_t width, char c) {
    int64_t pad = width - (int64_t)s.len;
    if (pad <= 0) {
        char *buf = malloc(s.len + 1);
        memcpy(buf, s.data, s.len);
        buf[s.len] = '\0';
        rl_string result = { .data = buf, .len = s.len, .rc = 1 };
        return result;
    }
    uint64_t total = s.len + (uint64_t)pad;
    char *buf = malloc(total + 1);
    memcpy(buf, s.data, s.len);
    for (int64_t i = 0; i < pad; i++) buf[s.len + i] = c;
    buf[total] = '\0';
    rl_string result = { .data = buf, .len = total, .rc = 1 };
    return result;
}

// Byte-range slice `[start, end)` with clamping.
rl_string rl_str_slice(rl_string s, int64_t start, int64_t end) {
    if (start < 0) start = 0;
    if (end > (int64_t)s.len) end = (int64_t)s.len;
    if (start >= end) {
        rl_string result = { .data = "", .len = 0, .rc = 1 };
        return result;
    }
    uint64_t len = (uint64_t)(end - start);
    char *buf = malloc(len + 1);
    memcpy(buf, s.data + start, len);
    buf[len] = '\0';
    rl_string result = { .data = buf, .len = len, .rc = 1 };
    return result;
}

// Byte-reversed copy.
rl_string rl_str_reverse(rl_string s) {
    char *buf = malloc(s.len + 1);
    uint64_t j = 0;
    uint64_t i = s.len;
    while (i > 0) {
        unsigned char c = (unsigned char)s.data[i - 1];
        uint64_t char_len;
        if (c < 0x80) char_len = 1;
        else if (c < 0xE0) char_len = 2;
        else if (c < 0xF0) char_len = 3;
        else char_len = 4;
        i -= char_len;
        memcpy(buf + j, s.data + i, char_len);
        j += char_len;
    }
    buf[s.len] = '\0';
    rl_string result = { .data = buf, .len = s.len, .rc = 1 };
    return result;
}

// Raw bytes and one-char strings for each byte.
rl_array rl_str_bytes(rl_string s) {
    int64_t *buf = malloc(s.len * sizeof(int64_t));
    for (uint64_t i = 0; i < s.len; i++) {
        buf[i] = (int64_t)(unsigned char)s.data[i];
    }
    return rl_arr_from_vals(buf, s.len, sizeof(int64_t));
}

// Raw bytes and one-char strings for each byte.
rl_array rl_str_chars(rl_string s) {
    uint64_t cap = 16;
    char *buf = malloc(cap);
    uint64_t count = 0;
    uint64_t i = 0;
    while (i < s.len) {
        unsigned char c = (unsigned char)s.data[i];
        uint64_t char_len;
        char ch;
        if (c < 0x80) {
            char_len = 1;
            ch = (char)c;
        } else if (c < 0xE0) {
            char_len = 2;
            ch = (char)(c & 0x1F);
        } else if (c < 0xF0) {
            char_len = 3;
            ch = (char)(c & 0x0F);
        } else {
            char_len = 4;
            ch = (char)(c & 0x07);
        }
        for (uint64_t j = 1; j < char_len && i + j < s.len; j++) {
            ch = (ch << 6) | ((unsigned char)s.data[i + j] & 0x3F);
        }
        if (count >= cap) {
            cap *= 2;
            buf = realloc(buf, cap);
        }
        buf[count++] = ch;
        i += char_len;
    }
    rl_array arr;
    arr.data = buf;
    arr.len = count;
    arr.cap = cap;
    arr.elem_size = sizeof(char);
    arr.type_tag = RL_TAG_CHAR;
    return arr;
}

// Byte at `index` (0 when out of range).
char rl_str_char_at(rl_string s, int64_t index) {
    if (index < 0) return '\0';
    uint64_t pos = 0;
    int64_t char_idx = 0;
    while (pos < s.len) {
        unsigned char c = (unsigned char)s.data[pos];
        uint64_t char_len;
        int64_t codepoint;
        if (c < 0x80) { char_len = 1; codepoint = c; }
        else if (c < 0xE0) { char_len = 2; codepoint = c & 0x1F; }
        else if (c < 0xF0) { char_len = 3; codepoint = c & 0x0F; }
        else { char_len = 4; codepoint = c & 0x07; }
        for (uint64_t j = 1; j < char_len && pos + j < s.len; j++) {
            codepoint = (codepoint << 6) | ((unsigned char)s.data[pos + j] & 0x3F);
        }
        if (char_idx == index) return (char)codepoint;
        pos += char_len;
        char_idx++;
    }
    return '\0';
}

// Join an array of strings with `delim` between elements.
rl_string rl_str_join(rl_array arr, rl_string delim) {
    if (arr.len == 0) {
        rl_string result = { .data = "", .len = 0, .rc = 1 };
        return result;
    }
    uint64_t cap = arr.len * 8 + arr.len * delim.len + 1;
    char *buf = malloc(cap);
    uint64_t w = 0;
    int64_t *elems = (int64_t *)arr.data;
    for (uint64_t i = 0; i < arr.len; i++) {
        if (i > 0) {
            memcpy(buf + w, delim.data, delim.len);
            w += delim.len;
        }
        char tmp[32];
        int len = snprintf(tmp, sizeof(tmp), "%ld", (long)elems[i]);
        memcpy(buf + w, tmp, len);
        w += len;
    }
    buf[w] = '\0';
    rl_string result = { .data = buf, .len = w, .rc = 1 };
    return result;
}

// Append one element rendering to the join buffer, growing as needed.
static void _rl_join_push(char **buf, uint64_t *w, uint64_t *cap, const char *s, uint64_t n) {
    while (*w + n + 1 > *cap) {
        *cap *= 2;
        *buf = realloc(*buf, *cap);
    }
    memcpy(*buf + *w, s, n);
    *w += n;
}

// Join an array of any element type with `delim`; elements render like
// the VM's Display (strings raw, bools/floats formatted, ints decimal).
rl_string rl_str_join_t(rl_array arr, rl_string delim, int32_t tag) {
    if (arr.len == 0) {
        rl_string result = { .data = "", .len = 0, .rc = 1 };
        return result;
    }
    uint64_t es = arr.elem_size ? (uint64_t)arr.elem_size : sizeof(int64_t);
    uint64_t cap = 64;
    char *buf = malloc(cap);
    uint64_t w = 0;
    for (uint64_t i = 0; i < arr.len; i++) {
        if (i > 0 && delim.data != NULL) {
            _rl_join_push(&buf, &w, &cap, delim.data, delim.len);
        }
        char *slot = arr.data ? (char *)arr.data + i * es : NULL;
        if (tag == RL_TAG_STR && es == sizeof(rl_string) && slot) {
            rl_string s;
            memcpy(&s, slot, sizeof(rl_string));
            if (s.data != NULL) _rl_join_push(&buf, &w, &cap, s.data, s.len);
        } else if (tag == RL_TAG_F64) {
            double v = 0;
            if (slot) memcpy(&v, slot, es < sizeof(double) ? es : sizeof(double));
            char tmp[32];
            int len = snprintf(tmp, sizeof(tmp), "%g", v);
            _rl_join_push(&buf, &w, &cap, tmp, len);
        } else if (tag == RL_TAG_BOOL) {
            bool b = slot && *(uint8_t *)slot != 0;
            _rl_join_push(&buf, &w, &cap, b ? "true" : "false", b ? 4 : 5);
        } else if (tag == RL_TAG_CHAR) {
            int64_t v = 0;
            if (slot) memcpy(&v, slot, es < sizeof(int64_t) ? es : sizeof(int64_t));
            char tmp[8];
            int len = snprintf(tmp, sizeof(tmp), "%c", (char)(unsigned char)v);
            _rl_join_push(&buf, &w, &cap, tmp, len);
        } else if (tag == RL_TAG_NULL) {
            _rl_join_push(&buf, &w, &cap, "null", 4);
        } else {
            int64_t v = 0;
            if (slot) memcpy(&v, slot, es < sizeof(int64_t) ? es : sizeof(int64_t));
            char tmp[32];
            int len = snprintf(tmp, sizeof(tmp), "%ld", (long)v);
            _rl_join_push(&buf, &w, &cap, tmp, len);
        }
    }
    buf[w] = '\0';
    rl_string result = { .data = buf, .len = w, .rc = 1 };
    return result;
}

// Split on `delim` into an array of strings.
rl_array rl_str_split(rl_string s, rl_string delim) {
    if (delim.len == 0 || s.len == 0) {
        rl_array arr = { .data = NULL, .len = 0, .cap = 0, .elem_size = sizeof(rl_string), .type_tag = RL_TAG_STR };
        return arr;
    }
    uint64_t cap = 16;
    rl_string *buf = malloc(cap * sizeof(rl_string));
    uint64_t count = 0;
    uint64_t pos = 0;
    while (pos <= s.len) {
        uint64_t next = pos;
        bool found = false;
        while (next + delim.len <= s.len) {
            if (memcmp(s.data + next, delim.data, delim.len) == 0) {
                found = true;
                break;
            }
            next++;
        }
        uint64_t seg_len = found ? next - pos : s.len - pos;
        if (count >= cap) {
            cap *= 2;
            buf = realloc(buf, cap * sizeof(rl_string));
        }
        char *seg = malloc(seg_len + 1);
        memcpy(seg, s.data + pos, seg_len);
        seg[seg_len] = '\0';
        buf[count] = (rl_string){ .data = seg, .len = seg_len, .rc = 1 };
        count++;
        if (found) {
            pos = next + delim.len;
        } else {
            break;
        }
    }
    rl_array arr;
    arr.data = buf;
    arr.len = count;
    arr.cap = cap;
    arr.elem_size = sizeof(rl_string);
    arr.type_tag = RL_TAG_STR;
    return arr;
}

// Count Unicode codepoints (UTF-8) in `s`.
static uint64_t _rl_str_char_count(rl_string s) {
    if (s.data == NULL) return 0;
    uint64_t n = 0;
    for (uint64_t i = 0; i < s.len; i++) {
        if (((unsigned char)s.data[i] & 0xC0) != 0x80) n++;
    }
    return n;
}

// Remainder after `prefix`, or an error when missing.
rl_result rl_str_strip_prefix(rl_string s, rl_string prefix) {
    if (s.data == NULL || prefix.data == NULL) return rl_err(-1);
    if (prefix.len <= s.len && memcmp(s.data, prefix.data, prefix.len) == 0) {
        uint64_t rest = s.len - prefix.len;
        char *buf = malloc(rest + 1);
        memcpy(buf, s.data + prefix.len, rest);
        buf[rest] = '\0';
        rl_string out = { .data = buf, .len = rest, .rc = 1 };
        return rl_ok_str(out);
    }
    int n = snprintf(NULL, 0, "strip_prefix: string does not start with \"%.*s\"",
        (int)prefix.len, prefix.data);
    char *msg = malloc((uint64_t)n + 1);
    snprintf(msg, (uint64_t)n + 1, "strip_prefix: string does not start with \"%.*s\"",
        (int)prefix.len, prefix.data);
    return rl_err_msg((rl_string){ .data = msg, .len = (uint64_t)n, .rc = 1 });
}

// Remainder without `suffix`, or an error when missing.
rl_result rl_str_strip_suffix(rl_string s, rl_string suffix) {
    if (s.data == NULL || suffix.data == NULL) return rl_err(-1);
    if (suffix.len <= s.len && memcmp(s.data + s.len - suffix.len, suffix.data, suffix.len) == 0) {
        uint64_t rest = s.len - suffix.len;
        char *buf = malloc(rest + 1);
        memcpy(buf, s.data, rest);
        buf[rest] = '\0';
        rl_string out = { .data = buf, .len = rest, .rc = 1 };
        return rl_ok_str(out);
    }
    int n = snprintf(NULL, 0, "strip_suffix: string does not end with \"%.*s\"",
        (int)suffix.len, suffix.data);
    char *msg = malloc((uint64_t)n + 1);
    snprintf(msg, (uint64_t)n + 1, "strip_suffix: string does not end with \"%.*s\"",
        (int)suffix.len, suffix.data);
    return rl_err_msg((rl_string){ .data = msg, .len = (uint64_t)n, .rc = 1 });
}

// Char index of the last `needle` hit, or -1 when absent.
int64_t rl_str_last_index_of(rl_string s, rl_string needle) {
    if (s.data == NULL || needle.data == NULL) return -1;
    if (needle.len == 0) return (int64_t)_rl_str_char_count(s);
    if (needle.len > s.len) return -1;
    int64_t found = -1;
    for (uint64_t i = 0; i + needle.len <= s.len; i++) {
        if (memcmp(s.data + i, needle.data, needle.len) == 0) found = (int64_t)i;
    }
    if (found < 0) return -1;
    uint64_t n = 0;
    for (int64_t i = 0; i < found; i++) {
        if (((unsigned char)s.data[i] & 0xC0) != 0x80) n++;
    }
    return (int64_t)n;
}

// Split on the first `sep` into [before, after], or an error.
rl_result rl_str_split_once(rl_string s, rl_string sep) {
    if (s.data == NULL || sep.data == NULL) return rl_err(-1);
    uint64_t at = s.len + 1;
    if (sep.len == 0) {
        at = 0;
    } else if (sep.len <= s.len) {
        for (uint64_t i = 0; i + sep.len <= s.len; i++) {
            if (memcmp(s.data + i, sep.data, sep.len) == 0) { at = i; break; }
        }
    }
    if (at > s.len) {
        int n = snprintf(NULL, 0, "split_once: separator \"%.*s\" not found in string",
            (int)sep.len, sep.data);
        char *msg = malloc((uint64_t)n + 1);
        snprintf(msg, (uint64_t)n + 1, "split_once: separator \"%.*s\" not found in string",
            (int)sep.len, sep.data);
        return rl_err_msg((rl_string){ .data = msg, .len = (uint64_t)n, .rc = 1 });
    }
    uint64_t before_len = at;
    uint64_t after_len = s.len - at - sep.len;
    char *b = malloc(before_len + 1);
    memcpy(b, s.data, before_len);
    b[before_len] = '\0';
    char *a = malloc(after_len + 1);
    memcpy(a, s.data + at + sep.len, after_len);
    a[after_len] = '\0';
    rl_string *buf = malloc(2 * sizeof(rl_string));
    buf[0] = (rl_string){ .data = b, .len = before_len, .rc = 1 };
    buf[1] = (rl_string){ .data = a, .len = after_len, .rc = 1 };
    rl_array arr = { .data = buf, .len = 2, .cap = 2,
        .elem_size = sizeof(rl_string), .type_tag = RL_TAG_STR };
    return rl_ok_arr(arr);
}

// One element per line (handles \n and \r\n).
rl_array rl_str_lines(rl_string s) {
    rl_array empty = { .data = NULL, .len = 0, .cap = 0,
        .elem_size = sizeof(rl_string), .type_tag = RL_TAG_STR };
    if (s.data == NULL || s.len == 0) return empty;
    uint64_t cap = 16;
    rl_string *buf = malloc(cap * sizeof(rl_string));
    uint64_t count = 0;
    uint64_t pos = 0;
    while (pos < s.len) {
        uint64_t next = pos;
        while (next < s.len && s.data[next] != '\n') next++;
        uint64_t seg_len = next - pos;
        if (seg_len > 0 && s.data[pos + seg_len - 1] == '\r') seg_len--;
        if (count >= cap) { cap *= 2; buf = realloc(buf, cap * sizeof(rl_string)); }
        char *seg = malloc(seg_len + 1);
        memcpy(seg, s.data + pos, seg_len);
        seg[seg_len] = '\0';
        buf[count++] = (rl_string){ .data = seg, .len = seg_len, .rc = 1 };
        if (next >= s.len) break;
        pos = next + 1;
    }
    rl_array arr = { .data = buf, .len = count, .cap = cap,
        .elem_size = sizeof(rl_string), .type_tag = RL_TAG_STR };
    return arr;
}

// True for ASCII whitespace bytes (wrap word splitting).
static bool _rl_is_wrap_space(char c) {
    return c == ' ' || c == '\t' || c == '\n' || c == '\r' || c == '\f' || c == '\v';
}

// Word-wrap to `width` bytes (copy when `width <= 0`).
rl_string rl_str_wrap(rl_string s, int64_t width) {
    rl_string null_str = { .data = NULL, .len = 0, .rc = 0 };
    if (s.data == NULL) return null_str;
    if (width <= 0) {
        char *buf = malloc(s.len + 1);
        memcpy(buf, s.data, s.len);
        buf[s.len] = '\0';
        return (rl_string){ .data = buf, .len = s.len, .rc = 1 };
    }
    uint64_t w = (uint64_t)width;
    uint64_t cap = s.len + 1;
    char *out = malloc(cap);
    uint64_t len = 0;
    uint64_t line_len = 0;
    uint64_t pos = 0;
    bool first = true;
    while (pos < s.len) {
        while (pos < s.len && _rl_is_wrap_space(s.data[pos])) pos++;
        if (pos >= s.len) break;
        uint64_t start = pos;
        while (pos < s.len && !_rl_is_wrap_space(s.data[pos])) pos++;
        uint64_t word_len = pos - start;
        if (first) {
            if (len + word_len >= cap) { cap = len + word_len + 1; out = realloc(out, cap); }
            memcpy(out + len, s.data + start, word_len);
            len += word_len;
            line_len = word_len;
            first = false;
        } else if (line_len + 1 + word_len <= w) {
            if (len + 1 + word_len >= cap) { cap = len + 1 + word_len + 1; out = realloc(out, cap); }
            out[len++] = ' ';
            memcpy(out + len, s.data + start, word_len);
            len += word_len;
            line_len += 1 + word_len;
        } else {
            if (len + 1 + word_len >= cap) { cap = len + 1 + word_len + 1; out = realloc(out, cap); }
            out[len++] = '\n';
            memcpy(out + len, s.data + start, word_len);
            len += word_len;
            line_len = word_len;
        }
    }
    out[len] = '\0';
    return (rl_string){ .data = out, .len = len, .rc = 1 };
}

// Prefix every line with `prefix`.
rl_string rl_str_indent(rl_string s, rl_string prefix) {
    rl_string null_str = { .data = NULL, .len = 0, .rc = 0 };
    if (s.data == NULL || prefix.data == NULL) return null_str;
    rl_array ls = rl_str_lines(s);
    if (ls.len == 0) {
        char *buf = malloc(1);
        buf[0] = '\0';
        return (rl_string){ .data = buf, .len = 0, .rc = 1 };
    }
    rl_string *lines = (rl_string *)ls.data;
    uint64_t total = 0;
    for (uint64_t i = 0; i < ls.len; i++) total += prefix.len + lines[i].len;
    total += ls.len - 1;
    char *out = malloc(total + 1);
    uint64_t w = 0;
    for (uint64_t i = 0; i < ls.len; i++) {
        if (i > 0) out[w++] = '\n';
        memcpy(out + w, prefix.data, prefix.len);
        w += prefix.len;
        memcpy(out + w, lines[i].data, lines[i].len);
        w += lines[i].len;
    }
    out[total] = '\0';
    return (rl_string){ .data = out, .len = total, .rc = 1 };
}

// Count leading spaces/tabs (dedent indent width).
static uint64_t _rl_dedent_width(rl_string line) {
    uint64_t n = 0;
    while (n < line.len && (line.data[n] == ' ' || line.data[n] == '\t')) n++;
    return n;
}

// Remove the common leading indent.
rl_string rl_str_dedent(rl_string s) {
    rl_string null_str = { .data = NULL, .len = 0, .rc = 0 };
    if (s.data == NULL) return null_str;
    rl_array ls = rl_str_lines(s);
    if (ls.len == 0) {
        char *buf = malloc(1);
        buf[0] = '\0';
        return (rl_string){ .data = buf, .len = 0, .rc = 1 };
    }
    rl_string *lines = (rl_string *)ls.data;
    uint64_t min_w = (uint64_t)-1;
    for (uint64_t i = 0; i < ls.len; i++) {
        if (lines[i].len == 0) continue;
        uint64_t cur = _rl_dedent_width(lines[i]);
        if (cur < min_w) min_w = cur;
    }
    if (min_w == (uint64_t)-1) min_w = 0;
    uint64_t total = 0;
    for (uint64_t i = 0; i < ls.len; i++) {
        uint64_t keep = lines[i].len == 0 ? 0 :
            (lines[i].len > min_w ? lines[i].len - min_w : 0);
        total += keep;
    }
    total += ls.len - 1;
    char *out = malloc(total + 1);
    uint64_t w = 0;
    for (uint64_t i = 0; i < ls.len; i++) {
        if (i > 0) out[w++] = '\n';
        if (lines[i].len == 0) continue;
        uint64_t strip = min_w < lines[i].len ? min_w : lines[i].len;
        uint64_t keep = lines[i].len - strip;
        memcpy(out + w, lines[i].data + strip, keep);
        w += keep;
    }
    out[w] = '\0';
    return (rl_string){ .data = out, .len = w, .rc = 1 };
}

// One diff row: text plus -1|0|1.
typedef struct { rl_string field_0; int64_t field_1; } _rl_diff_tuple;

// Line diff as [(text, -1|0|1)], always ok.
rl_result rl_str_diff_lines(rl_string a, rl_string b) {
    if (a.data == NULL || b.data == NULL) return rl_err(-1);
    rl_array la = rl_str_lines(a);
    rl_array lb = rl_str_lines(b);
    rl_string *xa = (rl_string *)la.data;
    rl_string *xb = (rl_string *)lb.data;
    uint64_t max_len = la.len > lb.len ? la.len : lb.len;
    uint64_t cap = max_len * 2 + 1;
    if (cap < 1) cap = 1;
    _rl_diff_tuple *buf = malloc(cap * sizeof(_rl_diff_tuple));
    uint64_t count = 0;
    for (uint64_t i = 0; i < max_len; i++) {
        bool has_a = i < la.len;
        bool has_b = i < lb.len;
        if (has_a && has_b && xa[i].len == xb[i].len
            && memcmp(xa[i].data, xb[i].data, xa[i].len) == 0) {
            char *t = malloc(xa[i].len + 1);
            memcpy(t, xa[i].data, xa[i].len);
            t[xa[i].len] = '\0';
            buf[count].field_0 = (rl_string){ .data = t, .len = xa[i].len, .rc = 1 };
            buf[count].field_1 = 0;
            count++;
        } else if (has_a && has_b) {
            uint64_t al = xa[i].len + 1;
            char *t1 = malloc(al + 1);
            t1[0] = '-';
            memcpy(t1 + 1, xa[i].data, xa[i].len);
            t1[al] = '\0';
            buf[count].field_0 = (rl_string){ .data = t1, .len = al, .rc = 1 };
            buf[count].field_1 = -1;
            count++;
            uint64_t bl = xb[i].len + 1;
            char *t2 = malloc(bl + 1);
            t2[0] = '+';
            memcpy(t2 + 1, xb[i].data, xb[i].len);
            t2[bl] = '\0';
            buf[count].field_0 = (rl_string){ .data = t2, .len = bl, .rc = 1 };
            buf[count].field_1 = 1;
            count++;
        } else if (has_a) {
            uint64_t al = xa[i].len + 1;
            char *t1 = malloc(al + 1);
            t1[0] = '-';
            memcpy(t1 + 1, xa[i].data, xa[i].len);
            t1[al] = '\0';
            buf[count].field_0 = (rl_string){ .data = t1, .len = al, .rc = 1 };
            buf[count].field_1 = -1;
            count++;
        } else {
            uint64_t bl = xb[i].len + 1;
            char *t2 = malloc(bl + 1);
            t2[0] = '+';
            memcpy(t2 + 1, xb[i].data, xb[i].len);
            t2[bl] = '\0';
            buf[count].field_0 = (rl_string){ .data = t2, .len = bl, .rc = 1 };
            buf[count].field_1 = 1;
            count++;
        }
    }
    rl_array arr = { .data = buf, .len = count, .cap = cap,
        .elem_size = sizeof(_rl_diff_tuple), .type_tag = RL_TAG_I64 };
    return rl_ok_arr(arr);
}

// True for alphabetic codepoints (ASCII plus common letter ranges).
static bool _rl_cp_is_alpha(uint32_t cp) {
    if (cp < 128) return (cp >= 'A' && cp <= 'Z') || (cp >= 'a' && cp <= 'z');
    if (cp >= 0xC0 && cp <= 0xD6) return true;
    if (cp >= 0xD8 && cp <= 0xF6) return true;
    if (cp >= 0xF8 && cp <= 0x2AF) return true;
    if (cp >= 0x370 && cp <= 0x3FF) return true;
    if (cp >= 0x400 && cp <= 0x4FF) return true;
    if (cp >= 0x530 && cp <= 0x58F) return true;
    if (cp >= 0x590 && cp <= 0x5FF) return true;
    if (cp >= 0x600 && cp <= 0x6FF) {
        if (cp >= 0x660 && cp <= 0x669) return false;
        if (cp >= 0x6F0 && cp <= 0x6F9) return false;
        return true;
    }
    if (cp >= 0x3040 && cp <= 0x30FF) return true;
    if (cp >= 0x4E00 && cp <= 0x9FFF) return true;
    if (cp >= 0xAC00 && cp <= 0xD7AF) return true;
    return false;
}

// True for whitespace codepoints (ASCII plus NBSP and common spaces).
static bool _rl_cp_is_space(uint32_t cp) {
    if (cp == ' ' || cp == '\t' || cp == '\n' || cp == '\r' || cp == '\f' || cp == '\v') return true;
    if (cp == 0xA0 || cp == 0x1680 || cp == 0x2028 || cp == 0x2029
        || cp == 0x202F || cp == 0x205F || cp == 0x3000) return true;
    if (cp >= 0x2000 && cp <= 0x200A) return true;
    return false;
}

// True when non-empty and all chars are alphabetic.
bool rl_str_is_alpha(rl_string s) {
    if (s.data == NULL || s.len == 0) return false;
    uint64_t pos = 0;
    while (pos < s.len) {
        uint32_t cp = 0;
        uint64_t used = 0;
        if (!_rl_utf8_decode(s.data + pos, s.len - pos, &cp, &used)) return false;
        if (!_rl_cp_is_alpha(cp)) return false;
        pos += used;
    }
    return true;
}

// True when non-empty and all chars are ASCII digits.
bool rl_str_is_numeric(rl_string s) {
    if (s.data == NULL || s.len == 0) return false;
    for (uint64_t i = 0; i < s.len; i++) {
        unsigned char c = (unsigned char)s.data[i];
        if (c < '0' || c > '9') return false;
    }
    return true;
}

// True when non-empty and all chars are whitespace.
bool rl_str_is_whitespace(rl_string s) {
    if (s.data == NULL || s.len == 0) return false;
    uint64_t pos = 0;
    while (pos < s.len) {
        uint32_t cp = 0;
        uint64_t used = 0;
        if (!_rl_utf8_decode(s.data + pos, s.len - pos, &cp, &used)) return false;
        if (!_rl_cp_is_space(cp)) return false;
        pos += used;
    }
    return true;
}

// True for uppercase codepoints (ASCII plus Latin-1).
static bool _rl_cp_is_upper(uint32_t cp) {
    if (cp >= 'A' && cp <= 'Z') return true;
    if (cp >= 0xC0 && cp <= 0xD6) return true;
    if (cp >= 0xD8 && cp <= 0xDE) return true;
    return false;
}

// True for lowercase codepoints (ASCII plus Latin-1).
static bool _rl_cp_is_lower(uint32_t cp) {
    if (cp >= 'a' && cp <= 'z') return true;
    if (cp == 0xDF) return true;
    if (cp >= 0xE0 && cp <= 0xF6) return true;
    if (cp >= 0xF8 && cp <= 0xFF) return true;
    return false;
}

// True for numeric codepoints (ASCII plus common digit ranges).
static bool _rl_cp_is_numeric(uint32_t cp) {
    if (cp >= '0' && cp <= '9') return true;
    if (cp >= 0x660 && cp <= 0x669) return true;
    if (cp >= 0x6F0 && cp <= 0x6F9) return true;
    if (cp == 0xB2 || cp == 0xB3 || cp == 0xB9) return true;
    return false;
}

// True for control codepoints.
static bool _rl_cp_is_control(uint32_t cp) {
    if (cp < 0x20) return true;
    if (cp == 0x7F) return true;
    if (cp >= 0x80 && cp <= 0x9F) return true;
    return false;
}

// Two-letter category of the first char, or an error when empty.
rl_result rl_str_unicode_category(rl_string s) {
    if (s.data == NULL || s.len == 0) {
        return rl_err_msg(rl_str_literal("unicode_category: empty string", 30));
    }
    uint32_t cp = 0;
    uint64_t used = 0;
    if (!_rl_utf8_decode(s.data, s.len, &cp, &used)) cp = (unsigned char)s.data[0];
    const char *cat;
    if (_rl_cp_is_alpha(cp)) {
        if (_rl_cp_is_upper(cp)) cat = "Lu";
        else if (_rl_cp_is_lower(cp)) cat = "Ll";
        else cat = "Lt";
    } else if (_rl_cp_is_numeric(cp)) {
        cat = "Nd";
    } else if (_rl_cp_is_space(cp)) {
        cat = "Zs";
    } else if (!_rl_cp_is_control(cp)) {
        cat = "Po";
    } else {
        cat = "Cc";
    }
    uint64_t len = 2;
    char *buf = malloc(3);
    memcpy(buf, cat, 3);
    return rl_ok_str((rl_string){ .data = buf, .len = len, .rc = 1 });
}

// ---- debug ----

// Abort with message (RL panic).
void rl_panic(rl_string msg) {
    if (msg.len > 0) {
        fprintf(stderr, "panic: %.*s\n", (int)msg.len, msg.data);
    } else {
        fprintf(stderr, "panic\n");
    }
    exit(1);
}

// Abort as unreachable code.
void rl_unreachable(void) {
    fprintf(stderr, "error: reached unreachable code\n");
    exit(1);
}

// Abort as unimplemented (RL todo).
void rl_todo(void) {
    fprintf(stderr, "error: not yet implemented\n");
    exit(1);
}

// Abort reporting a failed `assert_eq` (got `a`, wanted `b`).
void rl_assert_fail(rl_string label, int64_t a, int64_t b) {
    fprintf(stderr, "assertion failed: %.*s: %ld != %ld\n",
            (int)label.len, label.data, (long)a, (long)b);
    exit(1);
}

// Abort reporting a failed `assert` with a custom message.
void rl_assert_fail_msg(rl_string label, rl_string msg) {
    fprintf(stderr, "assertion failed: %.*s: %.*s\n",
            (int)label.len, label.data, (int)msg.len, msg.data);
    exit(1);
}

// RL type name for a numeric type tag (for `type_of`).
rl_string rl_type_of(int64_t type_tag) {
    const char *name;
    switch (type_tag) {
        case 1: name = "int"; break;
        case 2: name = "float"; break;
        case 3: name = "bool"; break;
        case 4: name = "string"; break;
        case 5: name = "null"; break;
        case 6: name = "char"; break;
        case 7: name = "byte"; break;
        case 8: name = "arr"; break;
        case 9: name = "map"; break;
        case 10: name = "set"; break;
        case 11: name = "tuple"; break;
        case 12: name = "function"; break;
        case 13: name = "closure"; break;
        case 14: name = "record"; break;
        case 15: name = "tag"; break;
        case 16: name = "ok"; break;
        case 17: name = "err"; break;
        case 18: name = "error"; break;
        default: name = "unknown"; break;
    }
    uint64_t len = strlen(name);
    char *buf = malloc(len + 1);
    memcpy(buf, name, len + 1);
    rl_string result = { .data = buf, .len = len, .rc = 1 };
    return result;
}

// RL type name for a result value: ok/err plus payload kinds.
rl_string rl_type_of_result(rl_result v) {
    const char *name;
    if (!v.is_ok) name = "err";
    else switch (v.tag) {
        case RL_TAG_NULL: name = "null"; break;
        case RL_TAG_I64: name = "int"; break;
        case RL_TAG_F64: name = "float"; break;
        case RL_TAG_BOOL: name = "bool"; break;
        case RL_TAG_CHAR: name = "char"; break;
        case RL_TAG_STR: name = "string"; break;
        case RL_TAG_ARR: name = "arr"; break;
        case RL_TAG_MAP: name = "map"; break;
        case RL_TAG_SET: name = "set"; break;
        case RL_TAG_CLOSURE: name = "closure"; break;
        default: name = "unknown"; break;
    }
    uint64_t len = strlen(name);
    char *buf = malloc(len + 1);
    memcpy(buf, name, len + 1);
    rl_string result = { .data = buf, .len = len, .rc = 1 };
    return result;
}

// Print a result to stderr in dbg format and return it unchanged.
rl_result rl_dbg_value(rl_result v) {
    fprintf(stderr, "[dbg] ");
    if (v.is_ok) {
        fprintf(stderr, "ok(");
        switch (v.tag) {
            case RL_TAG_NULL: fprintf(stderr, "null"); break;
            case RL_TAG_I64: fprintf(stderr, "%ld", (long)v.data.i64); break;
            case RL_TAG_F64: fprintf(stderr, "%g", v.data.f64); break;
            case RL_TAG_BOOL: fprintf(stderr, "%s", v.data.boolean ? "true" : "false"); break;
            case RL_TAG_CHAR: fprintf(stderr, "%c", (char)(unsigned char)v.data.i64); break;
            case RL_TAG_STR: fprintf(stderr, "%.*s", (int)v.data.str.len, v.data.str.data); break;
            case RL_TAG_ARR: fprintf(stderr, "<array>"); break;
            case RL_TAG_MAP: fprintf(stderr, "<map>"); break;
            case RL_TAG_SET: fprintf(stderr, "<set>"); break;
            case RL_TAG_CLOSURE: fprintf(stderr, "<fn>"); break;
        }
        fprintf(stderr, ") (ok)\n");
    } else {
        fprintf(stderr, "err(%d) (err)\n", v.err_code);
    }
    return v;
}

// Print int64 value to stderr and return it unchanged (RL dbg).
int64_t rl_dbg_int64(int64_t v) {
    fprintf(stderr, "[dbg] %ld (int)\n", (long)v);
    return v;
}

// Print float64 value to stderr and return it unchanged.
double rl_dbg_float64(double v) {
    fprintf(stderr, "[dbg] %.15g (float)\n", v);
    return v;
}

// Print bool value to stderr and return it unchanged.
bool rl_dbg_bool(bool v) {
    fprintf(stderr, "[dbg] %s (bool)\n", v ? "true" : "false");
    return v;
}

// Print string value to stderr and return it unchanged.
rl_string rl_dbg_str(rl_string v) {
    fprintf(stderr, "[dbg] \"%.*s\" (string)\n", (int)v.len, v.data);
    return v;
}

// Print a yellow warning to stderr (RL warn).
void rl_debug_warn(rl_string msg) {
    if (msg.data == NULL) {
        fprintf(stderr, "\x1b[33m[warn]\x1b[0m \n");
    } else {
        fprintf(stderr, "\x1b[33m[warn]\x1b[0m %.*s\n", (int)msg.len, msg.data);
    }
}

#include <execinfo.h>

// Capture a backtrace (RL stack_trace); bare string, empty when unsupported.
rl_string rl_debug_stack_trace(void) {
    void *frames[64];
    int n = backtrace(frames, 64);
    char **syms = backtrace_symbols(frames, n);
    if (n <= 0 || !syms) {
        rl_string empty = { .data = "", .len = 0, .rc = 1 };
        return empty;
    }
    size_t total = 0;
    for (int i = 0; i < n; i++) total += strlen(syms[i]) + 1;
    char *buf = malloc(total + 1);
    size_t pos = 0;
    for (int i = 0; i < n; i++) {
        size_t l = strlen(syms[i]);
        memcpy(buf + pos, syms[i], l);
        pos += l;
        buf[pos++] = '\n';
    }
    buf[pos] = '\0';
    free(syms);
    rl_string out = { .data = buf, .len = pos, .rc = 1 };
    return out;
}

// ---- path ----

// Pure path parsing (no filesystem access except the `is_*` checks).
rl_string rl_path_extension(rl_string path) {
    int64_t last_dot = -1;
    for (int64_t i = (int64_t)path.len - 1; i >= 0; i--) {
        if (path.data[i] == '.') { last_dot = i; break; }
    }
    if (last_dot < 0 || (uint64_t)last_dot >= path.len - 1) {
        rl_string result = { .data = "", .len = 0, .rc = 1 };
        return result;
    }
    uint64_t ext_len = path.len - (uint64_t)last_dot - 1;
    char *buf = malloc(ext_len + 1);
    memcpy(buf, path.data + last_dot + 1, ext_len);
    buf[ext_len] = '\0';
    rl_string result = { .data = buf, .len = ext_len, .rc = 1 };
    return result;
}

// Pure path parsing (no filesystem access except the `is_*` checks).
rl_string rl_path_filename(rl_string path) {
    int64_t last_sep = -1;
    for (int64_t i = (int64_t)path.len - 1; i >= 0; i--) {
        if (path.data[i] == '/') { last_sep = i; break; }
    }
    uint64_t start = (last_sep >= 0) ? (uint64_t)(last_sep + 1) : 0;
    uint64_t len = path.len - start;
    char *buf = malloc(len + 1);
    memcpy(buf, path.data + start, len);
    buf[len] = '\0';
    rl_string result = { .data = buf, .len = len, .rc = 1 };
    return result;
}

// Pure path parsing (no filesystem access except the `is_*` checks).
rl_string rl_path_parent(rl_string path) {
    int64_t last_sep = -1;
    for (int64_t i = (int64_t)path.len - 1; i >= 0; i--) {
        if (path.data[i] == '/') { last_sep = i; break; }
    }
    if (last_sep < 0) {
        rl_string result = { .data = ".", .len = 1, .rc = 1 };
        return result;
    }
    if (last_sep == 0) {
        rl_string result = { .data = "/", .len = 1, .rc = 1 };
        return result;
    }
    char *buf = malloc((uint64_t)last_sep + 1);
    memcpy(buf, path.data, (uint64_t)last_sep);
    buf[last_sep] = '\0';
    rl_string result = { .data = buf, .len = (uint64_t)last_sep, .rc = 1 };
    return result;
}

// Pure path parsing (no filesystem access except the `is_*` checks).
rl_string rl_path_stem(rl_string path) {
    int64_t last_sep = -1;
    int64_t last_dot = -1;
    for (int64_t i = (int64_t)path.len - 1; i >= 0; i--) {
        if (path.data[i] == '/' && last_sep < 0) last_sep = i;
        if (path.data[i] == '.' && last_dot < 0) last_dot = i;
    }
    uint64_t start = (last_sep >= 0) ? (uint64_t)(last_sep + 1) : 0;
    uint64_t end = (last_dot > last_sep) ? (uint64_t)last_dot : path.len;
    uint64_t len = end - start;
    char *buf = malloc(len + 1);
    memcpy(buf, path.data + start, len);
    buf[len] = '\0';
    rl_string result = { .data = buf, .len = len, .rc = 1 };
    return result;
}

// Drop the last component; join/push append one (push mutates in spirit, both return a fresh string).
rl_string rl_path_pop(rl_string path) {
    return rl_path_parent(path);
}

// Drop the last component; join/push append one (push mutates in spirit, both return a fresh string).
rl_string rl_path_join(rl_string path, rl_string target) {
    // A null input poisons the whole join (mirrors error propagation).
    if (path.data == NULL || target.data == NULL) {
        rl_string null_str = { .data = NULL, .len = 0, .rc = 0 };
        return null_str;
    }
    if (target.len == 0) {
        char *buf = malloc(path.len + 1);
        memcpy(buf, path.data, path.len);
        buf[path.len] = '\0';
        rl_string result = { .data = buf, .len = path.len, .rc = 1 };
        return result;
    }
    uint64_t total = path.len + 1 + target.len;
    char *buf = malloc(total + 1);
    memcpy(buf, path.data, path.len);
    buf[path.len] = '/';
    memcpy(buf + path.len + 1, target.data, target.len);
    buf[total] = '\0';
    rl_string result = { .data = buf, .len = total, .rc = 1 };
    return result;
}

// Drop the last component; join/push append one (push mutates in spirit, both return a fresh string).
rl_string rl_path_push(rl_string path, rl_string target) {
    return rl_path_join(path, target);
}

// Drop the last component; join/push append one (push mutates in spirit, both return a fresh string).
rl_string rl_path_set_extension(rl_string path, rl_string ext) {
    int64_t last_dot = -1;
    for (int64_t i = (int64_t)path.len - 1; i >= 0; i--) {
        if (path.data[i] == '.') { last_dot = i; break; }
    }
    uint64_t base_len = (last_dot >= 0) ? (uint64_t)last_dot : path.len;
    uint64_t total = base_len + 1 + ext.len;
    char *buf = malloc(total + 1);
    memcpy(buf, path.data, base_len);
    buf[base_len] = '.';
    memcpy(buf + base_len + 1, ext.data, ext.len);
    buf[total] = '\0';
    rl_string result = { .data = buf, .len = total, .rc = 1 };
    return result;
}

// Filesystem checks: true when the path exists and is a dir / file.
bool rl_path_is_dir(rl_string path) {
    char buf[path.len + 1];
    memcpy(buf, path.data, path.len);
    buf[path.len] = '\0';
    struct stat st;
    if (stat(buf, &st) != 0) return false;
    return S_ISDIR(st.st_mode);
}

// Filesystem checks: true when the path exists and is a dir / file.
bool rl_path_is_file(rl_string path) {
    char buf[path.len + 1];
    memcpy(buf, path.data, path.len);
    buf[path.len] = '\0';
    struct stat st;
    if (stat(buf, &st) != 0) return false;
    return S_ISREG(st.st_mode);
}

// True when `path` starts with `/`.
bool rl_path_is_absolute(rl_string path) {
    if (path.data == NULL || path.len == 0) return false;
    return path.data[0] == '/';
}

// True when `path` does not start with `/`.
bool rl_path_is_relative(rl_string path) {
    if (path.data == NULL || path.len == 0) return true;
    return path.data[0] != '/';
}

// Split `s` into components: `is_abs` for leading `/`, `parts` holds
// non-empty segments (`.` and `..` kept literally). Caller frees `parts`
// items and the array itself.
static void _rl_path_split_parts(rl_string s, bool *is_abs,
    char ***parts, uint64_t **lens, uint64_t *count) {
    *is_abs = s.len > 0 && s.data[0] == '/';
    uint64_t cap = 16;
    char **pp = malloc(cap * sizeof(char *));
    uint64_t *ll = malloc(cap * sizeof(uint64_t));
    uint64_t n = 0;
    uint64_t i = 0;
    while (i < s.len) {
        while (i < s.len && s.data[i] == '/') i++;
        if (i >= s.len) break;
        uint64_t start = i;
        while (i < s.len && s.data[i] != '/') i++;
        uint64_t seg = i - start;
        if (n >= cap) {
            cap *= 2;
            pp = realloc(pp, cap * sizeof(char *));
            ll = realloc(ll, cap * sizeof(uint64_t));
        }
        pp[n] = (char *)(s.data + start);
        ll[n] = seg;
        n++;
    }
    *parts = pp;
    *lens = ll;
    *count = n;
}

// True when `path` starts with `base` at a component boundary.
bool rl_path_starts_with(rl_string path, rl_string base) {
    if (path.data == NULL || base.data == NULL) return false;
    bool pa_abs = false, ba_abs = false;
    char **pp = NULL, **bp = NULL;
    uint64_t *pl = NULL, *bl = NULL;
    uint64_t pn = 0, bn = 0;
    _rl_path_split_parts(path, &pa_abs, &pp, &pl, &pn);
    _rl_path_split_parts(base, &ba_abs, &bp, &bl, &bn);
    bool ok = true;
    if (pa_abs != ba_abs) ok = false;
    else {
        // Rust ignores `.` segments when comparing, so filter first.
        uint64_t pfn = 0;
        for (uint64_t i = 0; i < pn; i++) {
            if (pl[i] == 1 && pp[i][0] == '.') continue;
            pp[pfn] = pp[i];
            pl[pfn] = pl[i];
            pfn++;
        }
        uint64_t bfn = 0;
        for (uint64_t i = 0; i < bn; i++) {
            if (bl[i] == 1 && bp[i][0] == '.') continue;
            bp[bfn] = bp[i];
            bl[bfn] = bl[i];
            bfn++;
        }
        if (bfn > pfn) ok = false;
        else {
            for (uint64_t i = 0; i < bfn; i++) {
                if (pl[i] != bl[i] || memcmp(pp[i], bp[i], pl[i]) != 0) {
                    ok = false;
                    break;
                }
            }
        }
    }
    free(pp);
    free(pl);
    free(bp);
    free(bl);
    return ok;
}

// True when `path` ends with `child` at a component boundary.
bool rl_path_ends_with(rl_string path, rl_string child) {
    if (path.data == NULL || child.data == NULL) return false;
    bool pa_abs = false, ca_abs = false;
    char **pp = NULL, **cp = NULL;
    uint64_t *pl = NULL, *cl = NULL;
    uint64_t pn = 0, cn = 0;
    _rl_path_split_parts(path, &pa_abs, &pp, &pl, &pn);
    _rl_path_split_parts(child, &ca_abs, &cp, &cl, &cn);
    // Filter `.` out of both sides (Rust ignores `.`).
    uint64_t pcap = pn + 1;
    char **pf = malloc(pcap * sizeof(char *));
    uint64_t *lf = malloc(pcap * sizeof(uint64_t));
    uint64_t pfn = 0;
    for (uint64_t i = 0; i < pn; i++) {
        if (pl[i] == 1 && pp[i][0] == '.') continue;
        pf[pfn] = pp[i];
        lf[pfn] = pl[i];
        pfn++;
    }
    uint64_t ccap = cn + 1;
    char **cf = malloc(ccap * sizeof(char *));
    uint64_t *mf = malloc(ccap * sizeof(uint64_t));
    uint64_t cfn = 0;
    for (uint64_t i = 0; i < cn; i++) {
        if (cl[i] == 1 && cp[i][0] == '.') continue;
        cf[cfn] = cp[i];
        mf[cfn] = cl[i];
        cfn++;
    }
    bool ok = true;
    if (cfn > pfn) ok = false;
    else {
        for (uint64_t i = 0; i < cfn; i++) {
            uint64_t pi = pfn - cfn + i;
            if (lf[pi] != mf[i] || memcmp(pf[pi], cf[i], lf[pi]) != 0) {
                ok = false;
                break;
            }
        }
    }
    free(pp);
    free(pl);
    free(cp);
    free(cl);
    free(pf);
    free(lf);
    free(cf);
    free(mf);
    return ok;
}

// Lexically clean `.` and duplicate separators (keeps `..`).
rl_string rl_path_normalize(rl_string path) {
    rl_string null_str = { .data = NULL, .len = 0, .rc = 0 };
    if (path.data == NULL) return null_str;
    if (path.len == 0) {
        char *buf = malloc(1);
        buf[0] = '\0';
        return (rl_string){ .data = buf, .len = 0, .rc = 1 };
    }
    bool is_abs = path.data[0] == '/';
    bool dotdot_lead = path.len >= 2 && path.data[0] == '.' && path.data[1] == '.';
    uint64_t cap = 16;
    char **pp = malloc(cap * sizeof(char *));
    uint64_t *pl = malloc(cap * sizeof(uint64_t));
    uint64_t n = 0;
    uint64_t i = 0;
    bool first_seg = true;
    while (i < path.len) {
        while (i < path.len && path.data[i] == '/') i++;
        if (i >= path.len) break;
        uint64_t start = i;
        while (i < path.len && path.data[i] != '/') i++;
        uint64_t seg = i - start;
        if (seg == 1 && path.data[start] == '.') {
            if (first_seg && !is_abs) {
                if (n >= cap) {
                    cap *= 2;
                    pp = realloc(pp, cap * sizeof(char *));
                    pl = realloc(pl, cap * sizeof(uint64_t));
                }
                pp[n] = (char *)(path.data + start);
                pl[n] = seg;
                n++;
            }
        } else {
            if (n >= cap) {
                cap *= 2;
                pp = realloc(pp, cap * sizeof(char *));
                pl = realloc(pl, cap * sizeof(uint64_t));
            }
            pp[n] = (char *)(path.data + start);
            pl[n] = seg;
            n++;
        }
        first_seg = false;
    }
    if (n == 0) {
        free(pp);
        free(pl);
        if (is_abs) {
            char *buf = malloc(2);
            buf[0] = '/';
            buf[1] = '\0';
            return (rl_string){ .data = buf, .len = 1, .rc = 1 };
        }
        char *buf = malloc(path.len + 1);
        memcpy(buf, path.data, path.len);
        buf[path.len] = '\0';
        return (rl_string){ .data = buf, .len = path.len, .rc = 1 };
    }
    uint64_t total = 0;
    for (uint64_t k = 0; k < n; k++) total += pl[k];
    total += n - 1;
    if (is_abs) total += 1;
    char *out = malloc(total + 1);
    uint64_t w = 0;
    if (is_abs) out[w++] = '/';
    for (uint64_t k = 0; k < n; k++) {
        if (k > 0) out[w++] = '/';
        memcpy(out + w, pp[k], pl[k]);
        w += pl[k];
    }
    out[w] = '\0';
    free(pp);
    free(pl);
    bool starts_dd = w >= 2 && out[0] == '.' && out[1] == '.';
    if (dotdot_lead && !starts_dd) {
        uint64_t nlen = w + 3;
        char *nbuf = malloc(nlen + 1);
        memcpy(nbuf, "../", 3);
        memcpy(nbuf + 3, out, w + 1);
        free(out);
        out = nbuf;
        w = nlen;
    }
    if (is_abs && out[0] != '/') {
        uint64_t nlen = w + 1;
        char *nbuf = malloc(nlen + 1);
        nbuf[0] = '/';
        memcpy(nbuf + 1, out, w + 1);
        free(out);
        out = nbuf;
        w = nlen;
    }
    return (rl_string){ .data = out, .len = w, .rc = 1 };
}

// Absolute form via cwd when relative, or an error.
rl_result rl_path_absolute(rl_string path) {
    if (path.data == NULL) return rl_err(-1);
    if (path.len > 0 && path.data[0] == '/') {
        char *buf = malloc(path.len + 1);
        memcpy(buf, path.data, path.len);
        buf[path.len] = '\0';
        return rl_ok_str((rl_string){ .data = buf, .len = path.len, .rc = 1 });
    }
    char cwd[4096];
    if (getcwd(cwd, sizeof(cwd)) == NULL) {
        int e = errno;
        char tmp[256];
        int n = snprintf(tmp, sizeof(tmp), "path_absolute: %s", strerror(e));
        char *msg = malloc((uint64_t)n + 1);
        memcpy(msg, tmp, (uint64_t)n + 1);
        return rl_err_msg((rl_string){ .data = msg, .len = (uint64_t)n, .rc = 1 });
    }
    uint64_t cl = strlen(cwd);
    uint64_t total = cl + 1 + path.len;
    char *out = malloc(total + 1);
    memcpy(out, cwd, cl);
    out[cl] = '/';
    memcpy(out + cl + 1, path.data, path.len);
    out[total] = '\0';
    return rl_ok_str((rl_string){ .data = out, .len = total, .rc = 1 });
}

// Canonical form via realpath, or an error.
rl_result rl_path_canonicalize(rl_string path) {
    if (path.data == NULL) return rl_err(-1);
    char *tmp = malloc(path.len + 1);
    memcpy(tmp, path.data, path.len);
    tmp[path.len] = '\0';
    char resolved[4096];
    if (realpath(tmp, resolved) == NULL) {
        int e = errno;
        const char *es = strerror(e);
        int n = snprintf(NULL, 0, "path_canonicalize: %.*s: %s (os error %d)",
            (int)path.len, path.data, es, e);
        char *msg = malloc((uint64_t)n + 1);
        snprintf(msg, (uint64_t)n + 1, "path_canonicalize: %.*s: %s (os error %d)",
            (int)path.len, path.data, es, e);
        free(tmp);
        return rl_err_msg((rl_string){ .data = msg, .len = (uint64_t)n, .rc = 1 });
    }
    free(tmp);
    uint64_t len = strlen(resolved);
    char *out = malloc(len + 1);
    memcpy(out, resolved, len + 1);
    return rl_ok_str((rl_string){ .data = out, .len = len, .rc = 1 });
}

// Replace a leading `~` with $HOME (copy when unset).
rl_string rl_path_expand_home(rl_string path) {
    rl_string null_str = { .data = NULL, .len = 0, .rc = 0 };
    if (path.data == NULL) return null_str;
    if (path.len == 0 || path.data[0] != '~') {
        char *buf = malloc(path.len + 1);
        memcpy(buf, path.data, path.len);
        buf[path.len] = '\0';
        return (rl_string){ .data = buf, .len = path.len, .rc = 1 };
    }
    const char *home = getenv("HOME");
    if (home == NULL) {
        char *buf = malloc(path.len + 1);
        memcpy(buf, path.data, path.len);
        buf[path.len] = '\0';
        return (rl_string){ .data = buf, .len = path.len, .rc = 1 };
    }
    uint64_t hl = strlen(home);
    uint64_t rest = path.len - 1;
    char *buf = malloc(hl + rest + 1);
    memcpy(buf, home, hl);
    memcpy(buf + hl, path.data + 1, rest);
    buf[hl + rest] = '\0';
    return (rl_string){ .data = buf, .len = hl + rest, .rc = 1 };
}

// Make an owned copy of `len` bytes.
static rl_string _rl_path_copy(const char *p, uint64_t len) {
    char *buf = malloc(len + 1);
    memcpy(buf, p, len);
    buf[len] = '\0';
    return (rl_string){ .data = buf, .len = len, .rc = 1 };
}

// [parent, file] pair for `path`.
rl_array rl_path_split(rl_string path) {
    rl_array empty = { .data = NULL, .len = 0, .cap = 0,
        .elem_size = sizeof(rl_string), .type_tag = RL_TAG_STR };
    if (path.data == NULL) return empty;
    uint64_t len = path.len;
    while (len > 0 && path.data[len - 1] == '/') len--;
    if (len == 0) {
        rl_string *buf = malloc(2 * sizeof(rl_string));
        buf[0] = _rl_path_copy("", 0);
        buf[1] = _rl_path_copy("", 0);
        rl_array arr = { .data = buf, .len = 2, .cap = 2,
            .elem_size = sizeof(rl_string), .type_tag = RL_TAG_STR };
        return arr;
    }
    int64_t last = -1;
    for (uint64_t i = 0; i < len; i++) {
        if (path.data[i] == '/') last = (int64_t)i;
    }
    rl_string parent;
    rl_string file;
    if (last < 0) {
        parent = _rl_path_copy("", 0);
        file = _rl_path_copy(path.data, len);
    } else if (last == 0) {
        parent = _rl_path_copy("/", 1);
        file = _rl_path_copy(path.data + 1, len - 1);
    } else {
        parent = _rl_path_copy(path.data, (uint64_t)last);
        file = _rl_path_copy(path.data + last + 1, len - (uint64_t)last - 1);
    }
    rl_string *buf = malloc(2 * sizeof(rl_string));
    buf[0] = parent;
    buf[1] = file;
    rl_array arr = { .data = buf, .len = 2, .cap = 2,
        .elem_size = sizeof(rl_string), .type_tag = RL_TAG_STR };
    return arr;
}

// [stem, extension-with-dot] pair for `path`.
rl_array rl_path_split_extension(rl_string path) {
    rl_array empty = { .data = NULL, .len = 0, .cap = 0,
        .elem_size = sizeof(rl_string), .type_tag = RL_TAG_STR };
    if (path.data == NULL) return empty;
    uint64_t len = path.len;
    while (len > 0 && path.data[len - 1] == '/') len--;
    uint64_t fstart = 0;
    for (uint64_t i = 0; i < len; i++) {
        if (path.data[i] == '/') fstart = i + 1;
    }
    uint64_t flen = len - fstart;
    int64_t dot = -1;
    for (uint64_t i = 0; i < flen; i++) {
        if (path.data[fstart + i] == '.') dot = (int64_t)i;
    }
    rl_string stem;
    rl_string ext;
    if (dot <= 0) {
        stem = _rl_path_copy(path.data + fstart, flen);
        ext = _rl_path_copy("", 0);
    } else {
        stem = _rl_path_copy(path.data + fstart, (uint64_t)dot);
        uint64_t elen = flen - (uint64_t)dot;
        ext = _rl_path_copy(path.data + fstart + dot, elen);
    }
    rl_string *buf = malloc(2 * sizeof(rl_string));
    buf[0] = stem;
    buf[1] = ext;
    rl_array arr = { .data = buf, .len = 2, .cap = 2,
        .elem_size = sizeof(rl_string), .type_tag = RL_TAG_STR };
    return arr;
}

// One element per path component (`/` first when absolute).
rl_array rl_path_components(rl_string path) {
    rl_array empty = { .data = NULL, .len = 0, .cap = 0,
        .elem_size = sizeof(rl_string), .type_tag = RL_TAG_STR };
    if (path.data == NULL || path.len == 0) return empty;
    bool is_abs = path.data[0] == '/';
    uint64_t cap = 16;
    rl_string *buf = malloc(cap * sizeof(rl_string));
    uint64_t count = 0;
    if (is_abs) {
        buf[count++] = _rl_path_copy("/", 1);
    }
    uint64_t i = 0;
    while (i < path.len) {
        while (i < path.len && path.data[i] == '/') i++;
        if (i >= path.len) break;
        uint64_t start = i;
        while (i < path.len && path.data[i] != '/') i++;
        uint64_t seg = i - start;
        if (count >= cap) {
            cap *= 2;
            buf = realloc(buf, cap * sizeof(rl_string));
        }
        buf[count++] = _rl_path_copy(path.data + start, seg);
    }
    if (!is_abs && count == 0) {
        free(buf);
        return empty;
    }
    if (is_abs && count == 1) {
        rl_array arr = { .data = buf, .len = 1, .cap = cap,
            .elem_size = sizeof(rl_string), .type_tag = RL_TAG_STR };
        return arr;
    }
    rl_array arr = { .data = buf, .len = count, .cap = cap,
        .elem_size = sizeof(rl_string), .type_tag = RL_TAG_STR };
    return arr;
}

// Replace the file name with `name`.
rl_string rl_path_with_file_name(rl_string path, rl_string name) {
    rl_string null_str = { .data = NULL, .len = 0, .rc = 0 };
    if (path.data == NULL || name.data == NULL) return null_str;
    if (path.len == 0) return _rl_path_copy(name.data, name.len);
    uint64_t len = path.len;
    while (len > 1 && path.data[len - 1] == '/') len--;
    if (len == 1 && path.data[0] == '/') {
        uint64_t total = 1 + name.len;
        char *buf = malloc(total + 1);
        buf[0] = '/';
        memcpy(buf + 1, name.data, name.len);
        buf[total] = '\0';
        return (rl_string){ .data = buf, .len = total, .rc = 1 };
    }
    int64_t last = -1;
    for (uint64_t i = 0; i < len; i++) {
        if (path.data[i] == '/') last = (int64_t)i;
    }
    if (last < 0) return _rl_path_copy(name.data, name.len);
    if (last == 0) {
        uint64_t total = 1 + name.len;
        char *buf = malloc(total + 1);
        buf[0] = '/';
        memcpy(buf + 1, name.data, name.len);
        buf[total] = '\0';
        return (rl_string){ .data = buf, .len = total, .rc = 1 };
    }
    uint64_t dir = (uint64_t)last;
    uint64_t total = dir + 1 + name.len;
    char *buf = malloc(total + 1);
    memcpy(buf, path.data, dir);
    buf[dir] = '/';
    memcpy(buf + dir + 1, name.data, name.len);
    buf[total] = '\0';
    return (rl_string){ .data = buf, .len = total, .rc = 1 };
}

// Build an absolute component list for `p` using `cwd` when relative.
static void _rl_path_abs_parts(rl_string p, const char *cwd, uint64_t cwd_len,
    char ***out_pp, uint64_t **out_ll, uint64_t *out_n) {
    bool p_abs = p.len > 0 && p.data[0] == '/';
    uint64_t cap = 32;
    char **pp = malloc(cap * sizeof(char *));
    uint64_t *ll = malloc(cap * sizeof(uint64_t));
    uint64_t n = 0;
    if (!p_abs) {
        uint64_t i = 0;
        while (i < cwd_len) {
            while (i < cwd_len && cwd[i] == '/') i++;
            if (i >= cwd_len) break;
            uint64_t s = i;
            while (i < cwd_len && cwd[i] != '/') i++;
            if (n >= cap) {
                cap *= 2;
                pp = realloc(pp, cap * sizeof(char *));
                ll = realloc(ll, cap * sizeof(uint64_t));
            }
            pp[n] = (char *)(cwd + s);
            ll[n] = i - s;
            n++;
        }
    }
    uint64_t i = 0;
    while (i < p.len) {
        while (i < p.len && p.data[i] == '/') i++;
        if (i >= p.len) break;
        uint64_t s = i;
        while (i < p.len && p.data[i] != '/') i++;
        if (n >= cap) {
            cap *= 2;
            pp = realloc(pp, cap * sizeof(char *));
            ll = realloc(ll, cap * sizeof(uint64_t));
        }
        pp[n] = (char *)(p.data + s);
        ll[n] = i - s;
        n++;
    }
    // Drop `.` segments (Rust ignores them when comparing).
    uint64_t w = 0;
    for (uint64_t k = 0; k < n; k++) {
        if (ll[k] == 1 && pp[k][0] == '.') continue;
        pp[w] = pp[k];
        ll[w] = ll[k];
        w++;
    }
    *out_pp = pp;
    *out_ll = ll;
    *out_n = w;
}

// Relative route from `from` to `to`, or an error.
rl_result rl_path_relative(rl_string from, rl_string to) {
    if (from.data == NULL || to.data == NULL) return rl_err(-1);
    char cwd[4096];
    if (getcwd(cwd, sizeof(cwd)) == NULL) {
        int e = errno;
        char tmp[256];
        int n = snprintf(tmp, sizeof(tmp), "path_relative: %s", strerror(e));
        char *msg = malloc((uint64_t)n + 1);
        memcpy(msg, tmp, (uint64_t)n + 1);
        return rl_err_msg((rl_string){ .data = msg, .len = (uint64_t)n, .rc = 1 });
    }
    uint64_t cwd_len = strlen(cwd);
    char **fp = NULL, **tp = NULL;
    uint64_t *fl = NULL, *tl = NULL;
    uint64_t fn = 0, tn = 0;
    _rl_path_abs_parts(from, cwd, cwd_len, &fp, &fl, &fn);
    _rl_path_abs_parts(to, cwd, cwd_len, &tp, &tl, &tn);
    uint64_t common = 0;
    while (common < fn && common < tn && fl[common] == tl[common]
        && memcmp(fp[common], tp[common], fl[common]) == 0) common++;
    uint64_t ups = fn - common;
    uint64_t total = 0;
    for (uint64_t k = 0; k < ups; k++) total += (k > 0 ? 1 : 0) + 2;
    for (uint64_t k = common; k < tn; k++) total += (total > 0 ? 1 : 0) + tl[k];
    char *out = malloc(total + 1);
    uint64_t w = 0;
    for (uint64_t k = 0; k < ups; k++) {
        if (w > 0) out[w++] = '/';
        out[w++] = '.';
        out[w++] = '.';
    }
    for (uint64_t k = common; k < tn; k++) {
        if (w > 0) out[w++] = '/';
        memcpy(out + w, tp[k], tl[k]);
        w += tl[k];
    }
    out[w] = '\0';
    free(fp);
    free(fl);
    free(tp);
    free(tl);
    return rl_ok_str((rl_string){ .data = out, .len = w, .rc = 1 });
}

// Join all `parts` (absolute parts reset the base).
rl_string rl_path_join_many(rl_array parts) {
    rl_string null_str = { .data = NULL, .len = 0, .rc = 0 };
    if (parts.data == NULL) {
        char *buf = malloc(1);
        buf[0] = '\0';
        return (rl_string){ .data = buf, .len = 0, .rc = 1 };
    }
    rl_string *elems = (rl_string *)parts.data;
    uint64_t cap = 256;
    char *buf = malloc(cap);
    uint64_t len = 0;
    for (uint64_t i = 0; i < parts.len; i++) {
        rl_string p = elems[i];
        if (p.data == NULL) {
            free(buf);
            return null_str;
        }
        if (p.len > 0 && p.data[0] == '/') {
            if (p.len + 1 > cap) { cap = p.len + 1; buf = realloc(buf, cap); }
            memcpy(buf, p.data, p.len);
            len = p.len;
            continue;
        }
        if (len == 0) {
            if (p.len + 1 > cap) { cap = p.len + 1; buf = realloc(buf, cap); }
            memcpy(buf, p.data, p.len);
            len = p.len;
            continue;
        }
        uint64_t need = len + (buf[len - 1] == '/' ? 0 : 1) + p.len;
        if (need + 1 > cap) { cap = need + 1; buf = realloc(buf, cap); }
        if (buf[len - 1] != '/') buf[len++] = '/';
        memcpy(buf + len, p.data, p.len);
        len += p.len;
    }
    buf[len] = '\0';
    return (rl_string){ .data = buf, .len = len, .rc = 1 };
}

// ---- fs ----

// Size of the file at path.
// Size of the file at `path` in bytes, or an error.
rl_result rl_fs_file_size(rl_string path) {
    if (path.data == NULL) return rl_err(-1);
    char buf[path.len + 1];
    memcpy(buf, path.data, path.len);
    buf[path.len] = '\0';
    struct stat st;
    if (stat(buf, &st) != 0) return rl_err(-1);
    return rl_ok_i64((int64_t)st.st_size);
}

// Last-modified time of the file at `path` (Unix seconds), or an error.
rl_result rl_fs_file_modified(rl_string path) {
    if (path.data == NULL) return rl_err(-1);
    char buf[path.len + 1];
    memcpy(buf, path.data, path.len);
    buf[path.len] = '\0';
    struct stat st;
    if (stat(buf, &st) != 0) return rl_err(-1);
    return rl_ok_i64((int64_t)st.st_mtime);
}

// Creation time of the file at `path` (Unix seconds), or an error.
rl_result rl_fs_file_created(rl_string path) {
    if (path.data == NULL) return rl_err(-1);
    char buf[path.len + 1];
    memcpy(buf, path.data, path.len);
    buf[path.len] = '\0';
    struct stat st;
    if (stat(buf, &st) != 0) return rl_err(-1);
    return rl_ok_i64((int64_t)st.st_ctime);
}

// Create an empty file (or update its timestamps), or an error.
rl_result rl_fs_touch(rl_string path) {
    if (path.data == NULL) return rl_err(-1);
    char buf[path.len + 1];
    memcpy(buf, path.data, path.len);
    buf[path.len] = '\0';
    FILE *f = fopen(buf, "ab");
    if (!f) return rl_err(-1);
    fclose(f);
    return rl_ok_null();
}

// Copy `src` to `dst`; the result holds 0 on success, or an error.
rl_result rl_fs_copy_file(rl_string src, rl_string dst) {
    if (src.data == NULL || dst.data == NULL) return rl_err(-1);
    char sbuf[src.len + 1];
    memcpy(sbuf, src.data, src.len);
    sbuf[src.len] = '\0';
    char dbuf[dst.len + 1];
    memcpy(dbuf, dst.data, dst.len);
    dbuf[dst.len] = '\0';
    FILE *fin = fopen(sbuf, "rb");
    if (!fin) return rl_err(-1);
    FILE *fout = fopen(dbuf, "wb");
    if (!fout) { fclose(fin); return rl_err(-1); }
    char chunk[8192];
    size_t n;
    while ((n = fread(chunk, 1, sizeof(chunk), fin)) > 0) {
        fwrite(chunk, 1, n, fout);
    }
    fclose(fin);
    fclose(fout);
    return rl_ok_i64(0);
}

// Create `path` plus missing parents; ok null on success, or an error.
rl_result rl_fs_mkdir_all(rl_string path) {
    if (path.data == NULL) return rl_err(-1);
    char buf[path.len + 1];
    memcpy(buf, path.data, path.len);
    buf[path.len] = '\0';
    for (char *p = buf + 1; *p; p++) {
        if (*p == '/') {
            *p = '\0';
            mkdir(buf, 0755);
            *p = '/';
        }
    }
    if (mkdir(buf, 0755) != 0 && errno != EEXIST) return rl_err(-1);
    return rl_ok_null();
}

// Remove the directory at `path`; ok null on success, or an error.
rl_result rl_fs_rmdir(rl_string path) {
    if (path.data == NULL) return rl_err(-1);
    char buf[path.len + 1];
    memcpy(buf, path.data, path.len);
    buf[path.len] = '\0';
    if (rmdir(buf) != 0) return rl_err(-1);
    return rl_ok_null();
}

// Move `src` to `dst` (same filesystem); ok null on success, or an error.
rl_result rl_fs_move_file(rl_string src, rl_string dst) {
    if (src.data == NULL || dst.data == NULL) return rl_err(-1);
    char sbuf[src.len + 1];
    memcpy(sbuf, src.data, src.len);
    sbuf[src.len] = '\0';
    char dbuf[dst.len + 1];
    memcpy(dbuf, dst.data, dst.len);
    dbuf[dst.len] = '\0';
    if (rename(sbuf, dbuf) != 0) return rl_err(-1);
    return rl_ok_null();
}

// Delete the directory tree at `path`; ok null on success, or an error.
rl_result rl_fs_rmdir_all(rl_string path) {
    if (path.data == NULL) return rl_err(-1);
    char buf[path.len + 1];
    memcpy(buf, path.data, path.len);
    buf[path.len] = '\0';
    if (rmdir(buf) != 0) return rl_err(-1);
    return rl_ok_null();
}

// Full paths of entries in the directory at `path`, or an error.
rl_result rl_fs_list_dir(rl_string path) {
    if (path.data == NULL) return rl_err(-1);
    char buf[path.len + 1];
    memcpy(buf, path.data, path.len);
    buf[path.len] = '\0';
    DIR *d = opendir(buf);
    if (!d) return rl_err(-1);
    uint64_t cap = 16;
    rl_string *entries = malloc(cap * sizeof(rl_string));
    uint64_t count = 0;
    struct dirent *ent;
    while ((ent = readdir(d)) != NULL) {
        if (strcmp(ent->d_name, ".") == 0 || strcmp(ent->d_name, "..") == 0) continue;
        if (count >= cap) {
            cap *= 2;
            entries = realloc(entries, cap * sizeof(rl_string));
        }
        uint64_t name_len = strlen(ent->d_name);
        bool need_sep = path.len > 0 && path.data[path.len - 1] != '/';
        uint64_t full_len = path.len + (need_sep ? 1 : 0) + name_len;
        char *name = malloc(full_len + 1);
        memcpy(name, path.data, path.len);
        if (need_sep) name[path.len] = '/';
        memcpy(name + path.len + (need_sep ? 1 : 0), ent->d_name, name_len + 1);
        entries[count] = (rl_string){ .data = name, .len = full_len, .rc = 1 };
        count++;
    }
    closedir(d);
    rl_array arr;
    arr.data = entries;
    arr.len = count;
    arr.cap = cap;
    arr.elem_size = sizeof(rl_string);
    arr.type_tag = RL_TAG_STR;
    return rl_ok_arr(arr);
}

// Rename to `new_name` in the same directory; ok with the new full path,
// or an error.
rl_result rl_fs_rename_file(rl_string path, rl_string new_name) {
    if (path.data == NULL || new_name.data == NULL) return rl_err(-1);
    char pbuf[path.len + 1];
    memcpy(pbuf, path.data, path.len);
    pbuf[path.len] = '\0';
    // Parent directory of `path` (empty means root or relative).
    uint64_t dir_len = 0;
    bool rooted = false;
    for (uint64_t i = 0; i < path.len; i++) {
        if (path.data[i] == '/') { dir_len = i; rooted = true; }
    }
    uint64_t full_len;
    char *full;
    if (!rooted) {
        full_len = new_name.len;
        full = malloc(full_len + 1);
        memcpy(full, new_name.data, new_name.len);
    } else if (dir_len == 0) {
        full_len = 1 + new_name.len;
        full = malloc(full_len + 1);
        full[0] = '/';
        memcpy(full + 1, new_name.data, new_name.len);
    } else {
        full_len = dir_len + 1 + new_name.len;
        full = malloc(full_len + 1);
        memcpy(full, path.data, dir_len);
        full[dir_len] = '/';
        memcpy(full + dir_len + 1, new_name.data, new_name.len);
    }
    full[full_len] = '\0';
    if (rename(pbuf, full) != 0) { free(full); return rl_err(-1); }
    rl_string result = { .data = full, .len = full_len, .rc = 1 };
    return rl_ok_str(result);
}

// File names of entries in the directory at `path`, or an error.
rl_result rl_fs_list_dir_names(rl_string path) {
    if (path.data == NULL) return rl_err(-1);
    char buf[path.len + 1];
    memcpy(buf, path.data, path.len);
    buf[path.len] = '\0';
    DIR *d = opendir(buf);
    if (!d) {
        int e = errno;
        const char *es = strerror(e);
        int n = snprintf(NULL, 0, "list_dir_names: failed to read \"%.*s\": %s (os error %d)", (int)path.len, path.data, es, e);
        char *msg = malloc((uint64_t)n + 1);
        snprintf(msg, (uint64_t)n + 1, "list_dir_names: failed to read \"%.*s\": %s (os error %d)", (int)path.len, path.data, es, e);
        return rl_err_msg((rl_string){ .data = msg, .len = (uint64_t)n, .rc = 1 });
    }
    uint64_t cap = 16;
    rl_string *entries = malloc(cap * sizeof(rl_string));
    uint64_t count = 0;
    struct dirent *ent;
    while ((ent = readdir(d)) != NULL) {
        if (strcmp(ent->d_name, ".") == 0 || strcmp(ent->d_name, "..") == 0) continue;
        if (count >= cap) {
            cap *= 2;
            entries = realloc(entries, cap * sizeof(rl_string));
        }
        uint64_t name_len = strlen(ent->d_name);
        char *name = malloc(name_len + 1);
        memcpy(name, ent->d_name, name_len + 1);
        entries[count] = (rl_string){ .data = name, .len = name_len, .rc = 1 };
        count++;
    }
    closedir(d);
    rl_array arr;
    arr.data = entries;
    arr.len = count;
    arr.cap = cap;
    arr.elem_size = sizeof(rl_string);
    arr.type_tag = RL_TAG_STR;
    return rl_ok_arr(arr);
}

// Last-accessed time of the file at `path` (Unix seconds), or an error.
rl_result rl_fs_file_accessed(rl_string path) {
    if (path.data == NULL) return rl_err(-1);
    char buf[path.len + 1];
    memcpy(buf, path.data, path.len);
    buf[path.len] = '\0';
    struct stat st;
    if (stat(buf, &st) != 0) {
        int e = errno;
        const char *es = strerror(e);
        int n = snprintf(NULL, 0, "file_accessed: failed to read \"%.*s\": %s (os error %d)", (int)path.len, path.data, es, e);
        char *msg = malloc((uint64_t)n + 1);
        snprintf(msg, (uint64_t)n + 1, "file_accessed: failed to read \"%.*s\": %s (os error %d)", (int)path.len, path.data, es, e);
        return rl_err_msg((rl_string){ .data = msg, .len = (uint64_t)n, .rc = 1 });
    }
    return rl_ok_i64((int64_t)st.st_atime);
}

// Raw mode bits of the file at `path`, or an error.
rl_result rl_fs_file_permissions(rl_string path) {
    if (path.data == NULL) return rl_err(-1);
    char buf[path.len + 1];
    memcpy(buf, path.data, path.len);
    buf[path.len] = '\0';
    struct stat st;
    if (stat(buf, &st) != 0) {
        int e = errno;
        const char *es = strerror(e);
        int n = snprintf(NULL, 0, "file_permissions: failed to read \"%.*s\": %s (os error %d)", (int)path.len, path.data, es, e);
        char *msg = malloc((uint64_t)n + 1);
        snprintf(msg, (uint64_t)n + 1, "file_permissions: failed to read \"%.*s\": %s (os error %d)", (int)path.len, path.data, es, e);
        return rl_err_msg((rl_string){ .data = msg, .len = (uint64_t)n, .rc = 1 });
    }
    return rl_ok_i64((int64_t)st.st_mode);
}

// Set permission bits on `path`; ok null on success, or an error.
rl_result rl_fs_set_permissions(rl_string path, int64_t mode) {
    if (path.data == NULL) return rl_err(-1);
    char buf[path.len + 1];
    memcpy(buf, path.data, path.len);
    buf[path.len] = '\0';
    struct stat st;
    if (stat(buf, &st) != 0) {
        int e = errno;
        const char *es = strerror(e);
        int n = snprintf(NULL, 0, "set_permissions: failed to read \"%.*s\": %s (os error %d)", (int)path.len, path.data, es, e);
        char *msg = malloc((uint64_t)n + 1);
        snprintf(msg, (uint64_t)n + 1, "set_permissions: failed to read \"%.*s\": %s (os error %d)", (int)path.len, path.data, es, e);
        return rl_err_msg((rl_string){ .data = msg, .len = (uint64_t)n, .rc = 1 });
    }
    if (chmod(buf, (mode_t)(mode & 07777)) != 0) {
        int e = errno;
        const char *es = strerror(e);
        int n = snprintf(NULL, 0, "set_permissions: failed to set permissions on \"%.*s\": %s (os error %d)", (int)path.len, path.data, es, e);
        char *msg = malloc((uint64_t)n + 1);
        snprintf(msg, (uint64_t)n + 1, "set_permissions: failed to set permissions on \"%.*s\": %s (os error %d)", (int)path.len, path.data, es, e);
        return rl_err_msg((rl_string){ .data = msg, .len = (uint64_t)n, .rc = 1 });
    }
    return rl_ok_null();
}

// Fresh temp file path; ok with the path, or an error.
rl_result rl_fs_temp_file(void) {
    const char *tmpdir = getenv("TMPDIR");
    if (!tmpdir) tmpdir = "/tmp";
    char tmpl[4096];
    snprintf(tmpl, sizeof(tmpl), "%s/.tmpXXXXXX", tmpdir);
    int fd = mkstemp(tmpl);
    if (fd < 0) {
        int e = errno;
        const char *es = strerror(e);
        int n = snprintf(NULL, 0, "temp_file: %s (os error %d)", es, e);
        char *msg = malloc((uint64_t)n + 1);
        snprintf(msg, (uint64_t)n + 1, "temp_file: %s (os error %d)", es, e);
        return rl_err_msg((rl_string){ .data = msg, .len = (uint64_t)n, .rc = 1 });
    }
    close(fd);
    uint64_t len = strlen(tmpl);
    char *out = malloc(len + 1);
    memcpy(out, tmpl, len + 1);
    return rl_ok_str((rl_string){ .data = out, .len = len, .rc = 1 });
}

// Fresh temp file path inside `dir`; ok with the path, or an error.
rl_result rl_fs_temp_file_in(rl_string dir) {
    if (dir.data == NULL) return rl_err(-1);
    char dbuf[dir.len + 1];
    memcpy(dbuf, dir.data, dir.len);
    dbuf[dir.len] = '\0';
    uint64_t need = dir.len + 12;
    char *tmpl = malloc(need + 1);
    memcpy(tmpl, dir.data, dir.len);
    memcpy(tmpl + dir.len, "/.tmpXXXXXX", 12);
    int fd = mkstemp(tmpl);
    if (fd < 0) {
        int e = errno;
        const char *es = strerror(e);
        int n = snprintf(NULL, 0, "temp_file_in: %s (os error %d) at path \"%s\"", es, e, tmpl);
        char *msg = malloc((uint64_t)n + 1);
        snprintf(msg, (uint64_t)n + 1, "temp_file_in: %s (os error %d) at path \"%s\"", es, e, tmpl);
        free(tmpl);
        return rl_err_msg((rl_string){ .data = msg, .len = (uint64_t)n, .rc = 1 });
    }
    close(fd);
    uint64_t len = strlen(tmpl);
    char *out = malloc(len + 1);
    memcpy(out, tmpl, len + 1);
    free(tmpl);
    return rl_ok_str((rl_string){ .data = out, .len = len, .rc = 1 });
}

// Truncate `path` to `len` bytes; ok null on success, or an error.
rl_result rl_fs_truncate_file(rl_string path, int64_t len) {
    if (path.data == NULL) return rl_err(-1);
    char buf[path.len + 1];
    memcpy(buf, path.data, path.len);
    buf[path.len] = '\0';
    int fd = open(buf, O_RDONLY);
    if (fd < 0) {
        int e = errno;
        const char *es = strerror(e);
        int n = snprintf(NULL, 0, "truncate_file: failed to open \"%.*s\": %s (os error %d)", (int)path.len, path.data, es, e);
        char *msg = malloc((uint64_t)n + 1);
        snprintf(msg, (uint64_t)n + 1, "truncate_file: failed to open \"%.*s\": %s (os error %d)", (int)path.len, path.data, es, e);
        return rl_err_msg((rl_string){ .data = msg, .len = (uint64_t)n, .rc = 1 });
    }
    uint64_t want = len < 0 ? 0 : (uint64_t)len;
    if (ftruncate(fd, (off_t)want) != 0) {
        int e = errno;
        const char *es = strerror(e);
        int n = snprintf(NULL, 0, "truncate_file: failed to truncate \"%.*s\": %s (os error %d)", (int)path.len, path.data, es, e);
        char *msg = malloc((uint64_t)n + 1);
        snprintf(msg, (uint64_t)n + 1, "truncate_file: failed to truncate \"%.*s\": %s (os error %d)", (int)path.len, path.data, es, e);
        close(fd);
        return rl_err_msg((rl_string){ .data = msg, .len = (uint64_t)n, .rc = 1 });
    }
    close(fd);
    return rl_ok_null();
}

// True when `c` is a path separator.
static bool _rl_glob_is_sep(char c) {
    return c == '/';
}

// Validate a glob pattern like the glob crate; 0 when valid.
static int _rl_glob_validate(const char *pat, uint64_t len, uint64_t *out_pos, const char **out_msg) {
    static const char *wild = "wildcards are either regular `*` or recursive `**`";
    static const char *rec = "recursive wildcards must form a single path component";
    static const char *range = "invalid range pattern";
    uint64_t i = 0;
    while (i < len) {
        char c = pat[i];
        if (c == '*') {
            uint64_t old = i;
            while (i < len && pat[i] == '*') i++;
            uint64_t count = i - old;
            if (count > 2) {
                *out_pos = old + 2;
                *out_msg = wild;
                return 1;
            }
            if (count == 2) {
                bool prev_sep = (old == 0) || (old >= 1 && _rl_glob_is_sep(pat[old - 1]) && !(old == 1 && len >= 2 && pat[0] == '*' && pat[1] == '*')) ;
                // Mirror the crate: valid only as a whole component.
                bool starts_ok = (i == 2) || (old > 0 && _rl_glob_is_sep(pat[old - 1]));
                if (starts_ok) {
                    if (i < len && _rl_glob_is_sep(pat[i])) {
                        i++;
                    } else if (i == len) {
                    } else {
                        *out_pos = i;
                        *out_msg = rec;
                        return 1;
                    }
                } else {
                    (void)prev_sep;
                    *out_pos = old > 0 ? old - 1 : old;
                    *out_msg = rec;
                    return 1;
                }
            }
        } else if (c == '[') {
            bool neg = (i + 1 < len && pat[i + 1] == '!');
            if (neg) {
                if (i + 4 > len) {
                    *out_pos = i;
                    *out_msg = range;
                    return 1;
                }
                uint64_t j = i + 3;
                bool found = false;
                while (j < len) {
                    if (pat[j] == ']') { found = true; break; }
                    j++;
                }
                if (!found) {
                    *out_pos = i;
                    *out_msg = range;
                    return 1;
                }
                i = j + 1;
            } else {
                if (i + 3 > len) {
                    *out_pos = i;
                    *out_msg = range;
                    return 1;
                }
                uint64_t j = i + 2;
                bool found = false;
                while (j < len) {
                    if (pat[j] == ']') { found = true; break; }
                    j++;
                }
                if (!found) {
                    *out_pos = i;
                    *out_msg = range;
                    return 1;
                }
                i = j + 1;
            }
        } else if (c == '?') {
            i++;
        } else {
            i++;
        }
    }
    return 0;
}

// True when `s` holds no glob metacharacters.
static bool _rl_glob_has_meta(const char *s, uint64_t len) {
    for (uint64_t i = 0; i < len; i++) {
        if (s[i] == '*' || s[i] == '?' || s[i] == '[') return true;
    }
    return false;
}

// Compare two C strings for qsort (lexicographic).
static int _rl_glob_cmp(const void *a, const void *b) {
    const char *sa = *(const char *const *)a;
    const char *sb = *(const char *const *)b;
    return strcmp(sa, sb);
}

// Join `base` and `name` into a fresh path (caller owns).
static char *_rl_glob_join(const char *base, uint64_t base_len, const char *name, uint64_t name_len, uint64_t *out_len) {
    uint64_t total;
    char *out;
    if (base_len == 0 || (base_len == 1 && base[0] == '.')) {
        total = name_len;
        out = malloc(total + 1);
        memcpy(out, name, name_len);
    } else if (base_len > 0 && base[base_len - 1] == '/') {
        total = base_len + name_len;
        out = malloc(total + 1);
        memcpy(out, base, base_len);
        memcpy(out + base_len, name, name_len);
    } else {
        total = base_len + 1 + name_len;
        out = malloc(total + 1);
        memcpy(out, base, base_len);
        out[base_len] = '/';
        memcpy(out + base_len + 1, name, name_len);
    }
    out[total] = '\0';
    if (out_len) *out_len = total;
    return out;
}

// Recursively match `comps[0..ncomps]` under `base`.
static void _rl_glob_walk(const char *base, char **comps, uint64_t *comp_lens, uint64_t ncomps, char ***out, uint64_t *count, uint64_t *cap) {
    if (ncomps == 0) {
        struct stat st;
        if (stat(base, &st) == 0) {
            uint64_t bl = strlen(base);
            if (*count >= *cap) {
                *cap = *cap == 0 ? 16 : *cap * 2;
                *out = realloc(*out, *cap * sizeof(char *));
            }
            char *dup = malloc(bl + 1);
            memcpy(dup, base, bl + 1);
            (*out)[*count] = dup;
            (*count)++;
        }
        return;
    }
    if (strcmp(comps[0], "**") == 0) {
        _rl_glob_walk(base, comps + 1, comp_lens + 1, ncomps - 1, out, count, cap);
        DIR *d = opendir(base);
        if (!d) return;
        struct dirent *ent;
        while ((ent = readdir(d)) != NULL) {
            if (strcmp(ent->d_name, ".") == 0 || strcmp(ent->d_name, "..") == 0) continue;
            uint64_t bl = strlen(base);
            uint64_t nl = strlen(ent->d_name);
            uint64_t fl = 0;
            char *full = _rl_glob_join(base, bl, ent->d_name, nl, &fl);
            struct stat st;
            bool is_dir = (stat(full, &st) == 0 && S_ISDIR(st.st_mode));
            if (is_dir) {
                _rl_glob_walk(full, comps, comp_lens, ncomps, out, count, cap);
            }
            free(full);
        }
        closedir(d);
        return;
    }
    DIR *d = opendir(base);
    if (!d) return;
    struct dirent *ent;
    while ((ent = readdir(d)) != NULL) {
        if (strcmp(ent->d_name, ".") == 0 || strcmp(ent->d_name, "..") == 0) continue;
        if (fnmatch(comps[0], ent->d_name, 0) != 0) continue;
        uint64_t bl = strlen(base);
        uint64_t nl = strlen(ent->d_name);
        uint64_t fl = 0;
        char *full = _rl_glob_join(base, bl, ent->d_name, nl, &fl);
        if (ncomps == 1) {
            if (*count >= *cap) {
                *cap = *cap == 0 ? 16 : *cap * 2;
                *out = realloc(*out, *cap * sizeof(char *));
            }
            (*out)[*count] = full;
            (*count)++;
        } else {
            struct stat st;
            bool is_dir = (stat(full, &st) == 0 && S_ISDIR(st.st_mode));
            if (is_dir) {
                _rl_glob_walk(full, comps + 1, comp_lens + 1, ncomps - 1, out, count, cap);
            }
            free(full);
        }
    }
    closedir(d);
}

// Paths matching `pattern` (`*`, `**`, `?`, `[...]`); ok with the list.
rl_result rl_fs_glob(rl_string pattern) {
    if (pattern.data == NULL) return rl_err(-1);
    char *pat = malloc(pattern.len + 1);
    memcpy(pat, pattern.data, pattern.len);
    pat[pattern.len] = '\0';
    uint64_t plen = pattern.len;
    while (plen > 1 && pat[plen - 1] == '/') { pat[--plen] = '\0'; }
    uint64_t err_pos = 0;
    const char *err_msg = NULL;
    if (_rl_glob_validate(pat, plen, &err_pos, &err_msg)) {
        int n = snprintf(NULL, 0, "glob: invalid pattern \"%.*s\": Pattern syntax error near position %llu: %s", (int)pattern.len, pattern.data, (unsigned long long)err_pos, err_msg);
        char *msg = malloc((uint64_t)n + 1);
        snprintf(msg, (uint64_t)n + 1, "glob: invalid pattern \"%.*s\": Pattern syntax error near position %llu: %s", (int)pattern.len, pattern.data, (unsigned long long)err_pos, err_msg);
        free(pat);
        return rl_err_msg((rl_string){ .data = msg, .len = (uint64_t)n, .rc = 1 });
    }
    if (!_rl_glob_has_meta(pat, plen)) {
        struct stat st;
        if (stat(pat, &st) == 0) {
            rl_string *buf = malloc(sizeof(rl_string));
            char *dup = malloc(plen + 1);
            memcpy(dup, pat, plen + 1);
            buf[0] = (rl_string){ .data = dup, .len = plen, .rc = 1 };
            free(pat);
            rl_array arr = { .data = buf, .len = 1, .cap = 1, .elem_size = sizeof(rl_string), .type_tag = RL_TAG_STR };
            return rl_ok_arr(arr);
        }
        free(pat);
        rl_array empty = { .data = NULL, .len = 0, .cap = 0, .elem_size = sizeof(rl_string), .type_tag = RL_TAG_STR };
        return rl_ok_arr(empty);
    }
    bool is_abs = plen > 0 && pat[0] == '/';
    uint64_t cap_c = 16;
    char **comps = malloc(cap_c * sizeof(char *));
    uint64_t *clens = malloc(cap_c * sizeof(uint64_t));
    uint64_t ncomps = 0;
    uint64_t i = 0;
    while (i < plen) {
        while (i < plen && pat[i] == '/') i++;
        if (i >= plen) break;
        uint64_t s = i;
        while (i < plen && pat[i] != '/') i++;
        uint64_t sl = i - s;
        if (ncomps >= cap_c) {
            cap_c *= 2;
            comps = realloc(comps, cap_c * sizeof(char *));
            clens = realloc(clens, cap_c * sizeof(uint64_t));
        }
        char *c = malloc(sl + 1);
        memcpy(c, pat + s, sl);
        c[sl] = '\0';
        comps[ncomps] = c;
        clens[ncomps] = sl;
        ncomps++;
    }
    uint64_t base_end = 0;
    while (base_end < ncomps && !_rl_glob_has_meta(comps[base_end], clens[base_end])) base_end++;
    char base[4096];
    if (base_end == 0) {
        if (is_abs) snprintf(base, sizeof(base), "/");
        else snprintf(base, sizeof(base), ".");
    } else {
        if (is_abs) {
            uint64_t w = 0;
            base[w++] = '/';
            for (uint64_t k = 0; k < base_end; k++) {
                if (k > 0) base[w++] = '/';
                memcpy(base + w, comps[k], clens[k]);
                w += clens[k];
            }
            base[w] = '\0';
        } else {
            uint64_t w = 0;
            for (uint64_t k = 0; k < base_end; k++) {
                if (k > 0) base[w++] = '/';
                memcpy(base + w, comps[k], clens[k]);
                w += clens[k];
            }
            base[w] = '\0';
        }
    }
    char **out = NULL;
    uint64_t count = 0;
    uint64_t cap = 0;
    _rl_glob_walk(base, comps + base_end, clens + base_end, ncomps - base_end, &out, &count, &cap);
    for (uint64_t k = 0; k < ncomps; k++) free(comps[k]);
    free(comps);
    free(clens);
    free(pat);
    if (count > 1) qsort(out, count, sizeof(char *), _rl_glob_cmp);
    if (count == 0) {
        free(out);
        rl_array empty = { .data = NULL, .len = 0, .cap = 0, .elem_size = sizeof(rl_string), .type_tag = RL_TAG_STR };
        return rl_ok_arr(empty);
    }
    rl_string *buf = malloc(count * sizeof(rl_string));
    for (uint64_t k = 0; k < count; k++) {
        uint64_t l = strlen(out[k]);
        buf[k] = (rl_string){ .data = out[k], .len = l, .rc = 1 };
    }
    free(out);
    rl_array arr = { .data = buf, .len = count, .cap = count, .elem_size = sizeof(rl_string), .type_tag = RL_TAG_STR };
    return rl_ok_arr(arr);
}

// All paths under `path` depth-first; ok with the list, or an error.
rl_result rl_fs_walk_dir(rl_string path) {
    if (path.data == NULL) return rl_err(-1);
    char *root = malloc(path.len + 1);
    memcpy(root, path.data, path.len);
    root[path.len] = '\0';
    uint64_t cap_s = 16;
    char **stack = malloc(cap_s * sizeof(char *));
    uint64_t nstack = 0;
    stack[nstack++] = root;
    uint64_t cap_r = 16;
    rl_string *results = malloc(cap_r * sizeof(rl_string));
    uint64_t nres = 0;
    while (nstack > 0) {
        char *dir = stack[--nstack];
        DIR *d = opendir(dir);
        if (!d) {
            int e = errno;
            const char *es = strerror(e);
            int n = snprintf(NULL, 0, "walk_dir: failed to read \"%s\": %s (os error %d)", dir, es, e);
            char *msg = malloc((uint64_t)n + 1);
            snprintf(msg, (uint64_t)n + 1, "walk_dir: failed to read \"%s\": %s (os error %d)", dir, es, e);
            for (uint64_t k = 0; k < nstack; k++) free(stack[k]);
            free(stack);
            free(dir);
            for (uint64_t k = 0; k < nres; k++) free((void *)results[k].data);
            free(results);
            return rl_err_msg((rl_string){ .data = msg, .len = (uint64_t)n, .rc = 1 });
        }
        struct dirent *ent;
        while ((ent = readdir(d)) != NULL) {
            if (strcmp(ent->d_name, ".") == 0 || strcmp(ent->d_name, "..") == 0) continue;
            uint64_t dl = strlen(dir);
            uint64_t nl = strlen(ent->d_name);
            bool need_sep = dl > 0 && dir[dl - 1] != '/';
            uint64_t fl = dl + (need_sep ? 1 : 0) + nl;
            char *full = malloc(fl + 1);
            memcpy(full, dir, dl);
            if (need_sep) full[dl] = '/';
            memcpy(full + dl + (need_sep ? 1 : 0), ent->d_name, nl + 1);
            struct stat st;
            bool is_dir = (stat(full, &st) == 0 && S_ISDIR(st.st_mode));
            if (is_dir) {
                if (nstack >= cap_s) {
                    cap_s *= 2;
                    stack = realloc(stack, cap_s * sizeof(char *));
                }
                stack[nstack++] = full;
            }
            if (nres >= cap_r) {
                cap_r *= 2;
                results = realloc(results, cap_r * sizeof(rl_string));
            }
            uint64_t rl = strlen(full);
            char *dup = malloc(rl + 1);
            memcpy(dup, full, rl + 1);
            results[nres++] = (rl_string){ .data = dup, .len = rl, .rc = 1 };
            if (is_dir) {
                // keep full on the stack; do not free here
            } else {
                free(full);
            }
        }
        closedir(d);
        free(dir);
    }
    free(stack);
    rl_array arr;
    arr.data = results;
    arr.len = nres;
    arr.cap = cap_r;
    arr.elem_size = sizeof(rl_string);
    arr.type_tag = RL_TAG_STR;
    return rl_ok_arr(arr);
}

// Create a symlink from `src` to `dst`; ok null on success, or an error.
rl_result rl_fs_symlink(rl_string src, rl_string dst) {
    if (src.data == NULL || dst.data == NULL) return rl_err(-1);
    char sbuf[src.len + 1];
    memcpy(sbuf, src.data, src.len);
    sbuf[src.len] = '\0';
    char dbuf[dst.len + 1];
    memcpy(dbuf, dst.data, dst.len);
    dbuf[dst.len] = '\0';
    if (symlink(sbuf, dbuf) != 0) {
        int e = errno;
        const char *es = strerror(e);
        int n = snprintf(NULL, 0, "symlink: failed to create symlink from \"%.*s\" to \"%.*s\": %s (os error %d)", (int)src.len, src.data, (int)dst.len, dst.data, es, e);
        char *msg = malloc((uint64_t)n + 1);
        snprintf(msg, (uint64_t)n + 1, "symlink: failed to create symlink from \"%.*s\" to \"%.*s\": %s (os error %d)", (int)src.len, src.data, (int)dst.len, dst.data, es, e);
        return rl_err_msg((rl_string){ .data = msg, .len = (uint64_t)n, .rc = 1 });
    }
    return rl_ok_null();
}

// Target of the symlink at `path`, or an error.
rl_result rl_fs_readlink(rl_string path) {
    if (path.data == NULL) return rl_err(-1);
    char buf[path.len + 1];
    memcpy(buf, path.data, path.len);
    buf[path.len] = '\0';
    uint64_t cap = 256;
    char *out = malloc(cap);
    ssize_t n = readlink(buf, out, cap);
    while (n >= 0 && (uint64_t)n >= cap) {
        cap *= 2;
        out = realloc(out, cap);
        n = readlink(buf, out, cap);
    }
    if (n < 0) {
        int e = errno;
        const char *es = strerror(e);
        int m = snprintf(NULL, 0, "readlink: failed to read symlink \"%.*s\": %s (os error %d)", (int)path.len, path.data, es, e);
        char *msg = malloc((uint64_t)m + 1);
        snprintf(msg, (uint64_t)m + 1, "readlink: failed to read symlink \"%.*s\": %s (os error %d)", (int)path.len, path.data, es, e);
        free(out);
        return rl_err_msg((rl_string){ .data = msg, .len = (uint64_t)m, .rc = 1 });
    }
    out[n] = '\0';
    char *dup = malloc((uint64_t)n + 1);
    memcpy(dup, out, (uint64_t)n + 1);
    free(out);
    return rl_ok_str((rl_string){ .data = dup, .len = (uint64_t)n, .rc = 1 });
}

// Create a hard link from `src` to `dst`; ok null on success, or an error.
rl_result rl_fs_hardlink(rl_string src, rl_string dst) {
    if (src.data == NULL || dst.data == NULL) return rl_err(-1);
    char sbuf[src.len + 1];
    memcpy(sbuf, src.data, src.len);
    sbuf[src.len] = '\0';
    char dbuf[dst.len + 1];
    memcpy(dbuf, dst.data, dst.len);
    dbuf[dst.len] = '\0';
    if (link(sbuf, dbuf) != 0) {
        int e = errno;
        const char *es = strerror(e);
        int n = snprintf(NULL, 0, "hardlink: failed to create hard link from \"%.*s\" to \"%.*s\": %s (os error %d)", (int)src.len, src.data, (int)dst.len, dst.data, es, e);
        char *msg = malloc((uint64_t)n + 1);
        snprintf(msg, (uint64_t)n + 1, "hardlink: failed to create hard link from \"%.*s\" to \"%.*s\": %s (os error %d)", (int)src.len, src.data, (int)dst.len, dst.data, es, e);
        return rl_err_msg((rl_string){ .data = msg, .len = (uint64_t)n, .rc = 1 });
    }
    return rl_ok_null();
}

// Canonical path of `path`; ok with the path, or an error.
rl_result rl_fs_realpath(rl_string path) {
    if (path.data == NULL) return rl_err(-1);
    char buf[path.len + 1];
    memcpy(buf, path.data, path.len);
    buf[path.len] = '\0';
    char resolved[4096];
    if (realpath(buf, resolved) == NULL) {
        int e = errno;
        const char *es = strerror(e);
        int n = snprintf(NULL, 0, "realpath: failed to resolve \"%.*s\": %s (os error %d)", (int)path.len, path.data, es, e);
        char *msg = malloc((uint64_t)n + 1);
        snprintf(msg, (uint64_t)n + 1, "realpath: failed to resolve \"%.*s\": %s (os error %d)", (int)path.len, path.data, es, e);
        return rl_err_msg((rl_string){ .data = msg, .len = (uint64_t)n, .rc = 1 });
    }
    uint64_t len = strlen(resolved);
    char *out = malloc(len + 1);
    memcpy(out, resolved, len + 1);
    return rl_ok_str((rl_string){ .data = out, .len = len, .rc = 1 });
}

// Create `path` and parents; 0 on success, errno otherwise.
static int _rl_fs_mkdir_p(const char *path) {
    char *tmp = strdup(path);
    uint64_t len = strlen(tmp);
    while (len > 1 && tmp[len - 1] == '/') { tmp[--len] = '\0'; }
    for (char *p = tmp + 1; *p; p++) {
        if (*p == '/') {
            *p = '\0';
            if (mkdir(tmp, 0755) != 0 && errno != EEXIST) {
                int e = errno;
                free(tmp);
                return e;
            }
            *p = '/';
        }
    }
    if (mkdir(tmp, 0755) != 0 && errno != EEXIST) {
        int e = errno;
        free(tmp);
        return e;
    }
    free(tmp);
    return 0;
}

// Advisory exclusive lock on `path`; ok null on success, or an error.
rl_result rl_fs_lock_file(rl_string path) {
    if (path.data == NULL) return rl_err(-1);
    char buf[path.len + 1];
    memcpy(buf, path.data, path.len);
    buf[path.len] = '\0';
    int fd = open(buf, O_WRONLY | O_CREAT, 0666);
    if (fd < 0) {
        int e = errno;
        const char *es = strerror(e);
        int n = snprintf(NULL, 0, "lock_file: failed to open \"%.*s\": %s (os error %d)", (int)path.len, path.data, es, e);
        char *msg = malloc((uint64_t)n + 1);
        snprintf(msg, (uint64_t)n + 1, "lock_file: failed to open \"%.*s\": %s (os error %d)", (int)path.len, path.data, es, e);
        return rl_err_msg((rl_string){ .data = msg, .len = (uint64_t)n, .rc = 1 });
    }
    if (flock(fd, LOCK_EX) != 0) {
        int n = snprintf(NULL, 0, "lock_file: failed to lock \"%.*s\"", (int)path.len, path.data);
        char *msg = malloc((uint64_t)n + 1);
        snprintf(msg, (uint64_t)n + 1, "lock_file: failed to lock \"%.*s\"", (int)path.len, path.data);
        close(fd);
        return rl_err_msg((rl_string){ .data = msg, .len = (uint64_t)n, .rc = 1 });
    }
    return rl_ok_null();
}

// Release the advisory lock on `path`; ok null on success, or an error.
rl_result rl_fs_unlock_file(rl_string path) {
    if (path.data == NULL) return rl_err(-1);
    char buf[path.len + 1];
    memcpy(buf, path.data, path.len);
    buf[path.len] = '\0';
    int fd = open(buf, O_WRONLY);
    if (fd < 0) {
        int e = errno;
        const char *es = strerror(e);
        int n = snprintf(NULL, 0, "unlock_file: failed to open \"%.*s\": %s (os error %d)", (int)path.len, path.data, es, e);
        char *msg = malloc((uint64_t)n + 1);
        snprintf(msg, (uint64_t)n + 1, "unlock_file: failed to open \"%.*s\": %s (os error %d)", (int)path.len, path.data, es, e);
        return rl_err_msg((rl_string){ .data = msg, .len = (uint64_t)n, .rc = 1 });
    }
    if (flock(fd, LOCK_UN) != 0) {
        int n = snprintf(NULL, 0, "unlock_file: failed to unlock \"%.*s\"", (int)path.len, path.data);
        char *msg = malloc((uint64_t)n + 1);
        snprintf(msg, (uint64_t)n + 1, "unlock_file: failed to unlock \"%.*s\"", (int)path.len, path.data);
        close(fd);
        return rl_err_msg((rl_string){ .data = msg, .len = (uint64_t)n, .rc = 1 });
    }
    close(fd);
    return rl_ok_null();
}

// Copy one regular file; 0 on success, errno otherwise.
static int _rl_fs_copy_one(const char *src, const char *dst) {
    FILE *fin = fopen(src, "rb");
    if (!fin) return errno;
    FILE *fout = fopen(dst, "wb");
    if (!fout) {
        int e = errno;
        fclose(fin);
        return e;
    }
    char chunk[8192];
    size_t n;
    while ((n = fread(chunk, 1, sizeof(chunk), fin)) > 0) {
        if (fwrite(chunk, 1, n, fout) != n) {
            int e = errno ? errno : EIO;
            fclose(fin);
            fclose(fout);
            return e;
        }
    }
    fclose(fin);
    fclose(fout);
    return 0;
}

// Copy the directory tree at `src` to `dst`; ok null, or an error.
rl_result rl_fs_copy_dir(rl_string src, rl_string dst) {
    if (src.data == NULL || dst.data == NULL) return rl_err(-1);
    char sbuf[src.len + 1];
    memcpy(sbuf, src.data, src.len);
    sbuf[src.len] = '\0';
    char dbuf[dst.len + 1];
    memcpy(dbuf, dst.data, dst.len);
    dbuf[dst.len] = '\0';
    struct stat sst;
    if (stat(sbuf, &sst) != 0 || !S_ISDIR(sst.st_mode)) {
        int n = snprintf(NULL, 0, "copy_dir: source \"%.*s\" is not a directory", (int)src.len, src.data);
        char *msg = malloc((uint64_t)n + 1);
        snprintf(msg, (uint64_t)n + 1, "copy_dir: source \"%.*s\" is not a directory", (int)src.len, src.data);
        return rl_err_msg((rl_string){ .data = msg, .len = (uint64_t)n, .rc = 1 });
    }
    int ce = _rl_fs_mkdir_p(dbuf);
    if (ce != 0) {
        const char *es = strerror(ce);
        int n = snprintf(NULL, 0, "copy_dir: failed to create \"%.*s\": %s (os error %d)", (int)dst.len, dst.data, es, ce);
        char *msg = malloc((uint64_t)n + 1);
        snprintf(msg, (uint64_t)n + 1, "copy_dir: failed to create \"%.*s\": %s (os error %d)", (int)dst.len, dst.data, es, ce);
        return rl_err_msg((rl_string){ .data = msg, .len = (uint64_t)n, .rc = 1 });
    }
    uint64_t cap_s = 16;
    char **stack = malloc(cap_s * sizeof(char *));
    uint64_t nstack = 0;
    stack[nstack++] = strdup(sbuf);
    uint64_t sroot_len = strlen(sbuf);
    while (nstack > 0) {
        char *dir = stack[--nstack];
        DIR *d = opendir(dir);
        if (!d) {
            int e = errno;
            const char *es = strerror(e);
            int n = snprintf(NULL, 0, "copy_dir: failed to read \"%s\": %s (os error %d)", dir, es, e);
            char *msg = malloc((uint64_t)n + 1);
            snprintf(msg, (uint64_t)n + 1, "copy_dir: failed to read \"%s\": %s (os error %d)", dir, es, e);
            for (uint64_t k = 0; k < nstack; k++) free(stack[k]);
            free(stack);
            free(dir);
            return rl_err_msg((rl_string){ .data = msg, .len = (uint64_t)n, .rc = 1 });
        }
        struct dirent *ent;
        while ((ent = readdir(d)) != NULL) {
            if (strcmp(ent->d_name, ".") == 0 || strcmp(ent->d_name, "..") == 0) continue;
            uint64_t dl = strlen(dir);
            uint64_t nl = strlen(ent->d_name);
            char *full = malloc(dl + 1 + nl + 1);
            memcpy(full, dir, dl);
            full[dl] = '/';
            memcpy(full + dl + 1, ent->d_name, nl + 1);
            const char *rel = full + sroot_len;
            if (*rel == '/') rel++;
            uint64_t dl2 = strlen(dbuf);
            uint64_t rl2 = strlen(rel);
            char *target = malloc(dl2 + 1 + rl2 + 1);
            memcpy(target, dbuf, dl2);
            target[dl2] = '/';
            memcpy(target + dl2 + 1, rel, rl2 + 1);
            struct stat est;
            bool is_dir = (stat(full, &est) == 0 && S_ISDIR(est.st_mode));
            if (is_dir) {
                int me = _rl_fs_mkdir_p(target);
                if (me != 0) {
                    const char *es = strerror(me);
                    int n = snprintf(NULL, 0, "copy_dir: failed to create \"%s\": %s (os error %d)", target, es, me);
                    char *msg = malloc((uint64_t)n + 1);
                    snprintf(msg, (uint64_t)n + 1, "copy_dir: failed to create \"%s\": %s (os error %d)", target, es, me);
                    free(full);
                    free(target);
                    closedir(d);
                    for (uint64_t k = 0; k < nstack; k++) free(stack[k]);
                    free(stack);
                    free(dir);
                    return rl_err_msg((rl_string){ .data = msg, .len = (uint64_t)n, .rc = 1 });
                }
                if (nstack >= cap_s) {
                    cap_s *= 2;
                    stack = realloc(stack, cap_s * sizeof(char *));
                }
                stack[nstack++] = full;
            } else {
                char *slash = strrchr(target, '/');
                if (slash) {
                    *slash = '\0';
                    int pe = _rl_fs_mkdir_p(target);
                    *slash = '/';
                    if (pe != 0) {
                        const char *es = strerror(pe);
                        int n = snprintf(NULL, 0, "copy_dir: failed to create \"%s\": %s (os error %d)", target, es, pe);
                        char *msg = malloc((uint64_t)n + 1);
                        snprintf(msg, (uint64_t)n + 1, "copy_dir: failed to create \"%s\": %s (os error %d)", target, es, pe);
                        free(full);
                        free(target);
                        closedir(d);
                        for (uint64_t k = 0; k < nstack; k++) free(stack[k]);
                        free(stack);
                        free(dir);
                        return rl_err_msg((rl_string){ .data = msg, .len = (uint64_t)n, .rc = 1 });
                    }
                }
                int fe = _rl_fs_copy_one(full, target);
                if (fe != 0) {
                    const char *es = strerror(fe);
                    int n = snprintf(NULL, 0, "copy_dir: failed to copy \"%s\" to \"%s\": %s (os error %d)", full, target, es, fe);
                    char *msg = malloc((uint64_t)n + 1);
                    snprintf(msg, (uint64_t)n + 1, "copy_dir: failed to copy \"%s\" to \"%s\": %s (os error %d)", full, target, es, fe);
                    free(full);
                    free(target);
                    closedir(d);
                    for (uint64_t k = 0; k < nstack; k++) free(stack[k]);
                    free(stack);
                    free(dir);
                    return rl_err_msg((rl_string){ .data = msg, .len = (uint64_t)n, .rc = 1 });
                }
                free(full);
            }
            free(target);
        }
        closedir(d);
        free(dir);
    }
    free(stack);
    return rl_ok_null();
}

// Total byte size under `path`; ok with the sum, or an error.
rl_result rl_fs_dir_size(rl_string path) {
    if (path.data == NULL) return rl_err(-1);
    char *root = malloc(path.len + 1);
    memcpy(root, path.data, path.len);
    root[path.len] = '\0';
    uint64_t cap_s = 16;
    char **stack = malloc(cap_s * sizeof(char *));
    uint64_t nstack = 0;
    stack[nstack++] = root;
    int64_t total = 0;
    while (nstack > 0) {
        char *dir = stack[--nstack];
        DIR *d = opendir(dir);
        if (!d) {
            int e = errno;
            const char *es = strerror(e);
            int n = snprintf(NULL, 0, "dir_size: failed to read \"%s\": %s (os error %d)", dir, es, e);
            char *msg = malloc((uint64_t)n + 1);
            snprintf(msg, (uint64_t)n + 1, "dir_size: failed to read \"%s\": %s (os error %d)", dir, es, e);
            for (uint64_t k = 0; k < nstack; k++) free(stack[k]);
            free(stack);
            free(dir);
            return rl_err_msg((rl_string){ .data = msg, .len = (uint64_t)n, .rc = 1 });
        }
        struct dirent *ent;
        while ((ent = readdir(d)) != NULL) {
            if (strcmp(ent->d_name, ".") == 0 || strcmp(ent->d_name, "..") == 0) continue;
            uint64_t dl = strlen(dir);
            uint64_t nl = strlen(ent->d_name);
            bool need_sep = dl > 0 && dir[dl - 1] != '/';
            uint64_t fl = dl + (need_sep ? 1 : 0) + nl;
            char *full = malloc(fl + 1);
            memcpy(full, dir, dl);
            if (need_sep) full[dl] = '/';
            memcpy(full + dl + (need_sep ? 1 : 0), ent->d_name, nl + 1);
            struct stat st;
            if (stat(full, &st) == 0 && S_ISDIR(st.st_mode)) {
                if (nstack >= cap_s) {
                    cap_s *= 2;
                    stack = realloc(stack, cap_s * sizeof(char *));
                }
                stack[nstack++] = full;
            } else {
                struct stat fst;
                if (stat(full, &fst) != 0) {
                    int e = errno;
                    const char *es = strerror(e);
                    int n = snprintf(NULL, 0, "dir_size: failed to stat \"%s\": %s (os error %d)", full, es, e);
                    char *msg = malloc((uint64_t)n + 1);
                    snprintf(msg, (uint64_t)n + 1, "dir_size: failed to stat \"%s\": %s (os error %d)", full, es, e);
                    free(full);
                    closedir(d);
                    for (uint64_t k = 0; k < nstack; k++) free(stack[k]);
                    free(stack);
                    free(dir);
                    return rl_err_msg((rl_string){ .data = msg, .len = (uint64_t)n, .rc = 1 });
                }
                total += (int64_t)fst.st_size;
                free(full);
            }
        }
        closedir(d);
        free(dir);
    }
    free(stack);
    return rl_ok_i64(total);
}

// True when `path` is a symlink; ok with the bool, or an error.
rl_result rl_fs_is_symlink(rl_string path) {
    if (path.data == NULL) return rl_err(-1);
    char buf[path.len + 1];
    memcpy(buf, path.data, path.len);
    buf[path.len] = '\0';
    struct stat st;
    if (lstat(buf, &st) != 0) {
        int e = errno;
        const char *es = strerror(e);
        int n = snprintf(NULL, 0, "is_symlink: failed to read \"%.*s\": %s (os error %d)", (int)path.len, path.data, es, e);
        char *msg = malloc((uint64_t)n + 1);
        snprintf(msg, (uint64_t)n + 1, "is_symlink: failed to read \"%.*s\": %s (os error %d)", (int)path.len, path.data, es, e);
        return rl_err_msg((rl_string){ .data = msg, .len = (uint64_t)n, .rc = 1 });
    }
    return rl_ok_bool(S_ISLNK(st.st_mode));
}

// File handle table: int64 ids into FILE pointers.
#define _RL_FS_MAX_HANDLES 256
static FILE *_rl_fs_files[_RL_FS_MAX_HANDLES];
static bool _rl_fs_readable[_RL_FS_MAX_HANDLES];
static bool _rl_fs_writable[_RL_FS_MAX_HANDLES];
static bool _rl_fs_used[_RL_FS_MAX_HANDLES];
static int _rl_fs_handle_count = 0;

// Look up a live file handle; NULL when unknown or closed.
static int64_t _rl_fs_idx(int64_t tagged) {
    int64_t idx = tagged - RL_HANDLE_FILE_BASE;
    if (idx < 0 || idx >= _rl_fs_handle_count) return -1;
    return idx;
}
static FILE *_rl_fs_get(int64_t tagged, bool *readable, bool *writable) {
    int64_t idx = _rl_fs_idx(tagged);
    if (idx < 0) return NULL;
    if (!_rl_fs_used[idx] || !_rl_fs_files[idx]) return NULL;
    if (readable) *readable = _rl_fs_readable[idx];
    if (writable) *writable = _rl_fs_writable[idx];
    return _rl_fs_files[idx];
}

// Open `file` with `mode` (`r`, `w`, `a`, `r+`, `w+`, `a+`).
rl_result rl_fs_open(rl_string file, rl_string mode) {
    if (file.data == NULL || mode.data == NULL) return rl_err(-1);
    char fbuf[file.len + 1];
    memcpy(fbuf, file.data, file.len);
    fbuf[file.len] = '\0';
    char mbuf[mode.len + 1];
    memcpy(mbuf, mode.data, mode.len);
    mbuf[mode.len] = '\0';
    bool is_r = strcmp(mbuf, "r") == 0;
    bool is_w = strcmp(mbuf, "w") == 0;
    bool is_a = strcmp(mbuf, "a") == 0;
    bool is_rp = strcmp(mbuf, "r+") == 0;
    bool is_wp = strcmp(mbuf, "w+") == 0;
    bool is_ap = strcmp(mbuf, "a+") == 0;
    if (!is_r && !is_w && !is_a && !is_rp && !is_wp && !is_ap) {
        int n = snprintf(NULL, 0, "open: invalid mode \"%s\" (expected r, w, a, r+, w+, a+)", mbuf);
        char *msg = malloc((uint64_t)n + 1);
        snprintf(msg, (uint64_t)n + 1, "open: invalid mode \"%s\" (expected r, w, a, r+, w+, a+)", mbuf);
        return rl_err_msg((rl_string){ .data = msg, .len = (uint64_t)n, .rc = 1 });
    }
    if (_rl_fs_handle_count >= _RL_FS_MAX_HANDLES) {
        const char *m = "open: too many open handles";
        uint64_t ml = strlen(m);
        char *dup = malloc(ml + 1);
        memcpy(dup, m, ml + 1);
        return rl_err_msg((rl_string){ .data = dup, .len = ml, .rc = 1 });
    }
    FILE *fp = NULL;
    if (is_r) fp = fopen(fbuf, "rb");
    else if (is_w) fp = fopen(fbuf, "wb");
    else if (is_a) fp = fopen(fbuf, "ab");
    else if (is_rp) {
        fp = fopen(fbuf, "r+b");
        if (!fp && errno == ENOENT) fp = fopen(fbuf, "w+b");
    } else if (is_wp) fp = fopen(fbuf, "w+b");
    else if (is_ap) fp = fopen(fbuf, "a+b");
    if (!fp) {
        int e = errno;
        const char *es = strerror(e);
        int n = snprintf(NULL, 0, "open: failed to open \"%.*s\": %s (os error %d)", (int)file.len, file.data, es, e);
        char *msg = malloc((uint64_t)n + 1);
        snprintf(msg, (uint64_t)n + 1, "open: failed to open \"%.*s\": %s (os error %d)", (int)file.len, file.data, es, e);
        return rl_err_msg((rl_string){ .data = msg, .len = (uint64_t)n, .rc = 1 });
    }
    int idx = _rl_fs_handle_count++;
    _rl_fs_files[idx] = fp;
    _rl_fs_readable[idx] = is_r || is_rp || is_wp || is_ap;
    _rl_fs_writable[idx] = is_w || is_a || is_rp || is_wp || is_ap;
    _rl_fs_used[idx] = true;
    return rl_ok_i64(RL_HANDLE_FILE_BASE + (int64_t)idx);
}

// Close a file handle id; ok null on success, or an error.
rl_result rl_fs_close(int64_t handle_id) {
    FILE *fp = _rl_fs_get(handle_id, NULL, NULL);
    if (!fp) {
        const char *m = "close: invalid handle";
        uint64_t ml = strlen(m);
        char *dup = malloc(ml + 1);
        memcpy(dup, m, ml + 1);
        return rl_err_msg((rl_string){ .data = dup, .len = ml, .rc = 1 });
    }
    fclose(fp);
    int64_t cidx = _rl_fs_idx(handle_id);
    if (cidx >= 0) {
        _rl_fs_files[cidx] = NULL;
        _rl_fs_used[cidx] = false;
    }
    return rl_ok_null();
}

// Read up to `n` bytes from a handle; ok with the text, or an error.
rl_result rl_fs_read_handle(int64_t handle_id, int64_t n) {
    bool readable = false;
    FILE *fp = _rl_fs_get(handle_id, &readable, NULL);
    if (!fp) {
        const char *m = "read: invalid handle";
        uint64_t ml = strlen(m);
        char *dup = malloc(ml + 1);
        memcpy(dup, m, ml + 1);
        return rl_err_msg((rl_string){ .data = dup, .len = ml, .rc = 1 });
    }
    if (!readable) {
        const char *m = "read: handle is not open for reading";
        uint64_t ml = strlen(m);
        char *dup = malloc(ml + 1);
        memcpy(dup, m, ml + 1);
        return rl_err_msg((rl_string){ .data = dup, .len = ml, .rc = 1 });
    }
    if (n <= 0) {
        char *empty = malloc(1);
        empty[0] = '\0';
        return rl_ok_str((rl_string){ .data = empty, .len = 0, .rc = 1 });
    }
    int64_t ridx = _rl_fs_idx(handle_id);
    bool writable = ridx >= 0 ? _rl_fs_writable[ridx] : false;
    if (writable) fflush(fp);
    uint64_t want = (uint64_t)n;
    char *buf = malloc(want + 1);
    clearerr(fp);
    size_t got = fread(buf, 1, want, fp);
    if (got == 0 && ferror(fp)) {
        int e = errno;
        const char *es = strerror(e);
        int m = snprintf(NULL, 0, "read: %s (os error %d)", es, e);
        char *msg = malloc((uint64_t)m + 1);
        snprintf(msg, (uint64_t)m + 1, "read: %s (os error %d)", es, e);
        free(buf);
        return rl_err_msg((rl_string){ .data = msg, .len = (uint64_t)m, .rc = 1 });
    }
    buf[got] = '\0';
    char *out = malloc(got + 1);
    memcpy(out, buf, got + 1);
    free(buf);
    return rl_ok_str((rl_string){ .data = out, .len = got, .rc = 1 });
}

// Write `data` to a handle; ok with the byte count, or an error.
rl_result rl_fs_write_handle(int64_t handle_id, rl_string data) {
    if (data.data == NULL) return rl_err(-1);
    bool writable = false;
    FILE *fp = _rl_fs_get(handle_id, NULL, &writable);
    if (!fp) {
        const char *m = "write: invalid handle";
        uint64_t ml = strlen(m);
        char *dup = malloc(ml + 1);
        memcpy(dup, m, ml + 1);
        return rl_err_msg((rl_string){ .data = dup, .len = ml, .rc = 1 });
    }
    if (!writable) {
        const char *m = "write: handle is not open for writing";
        uint64_t ml = strlen(m);
        char *dup = malloc(ml + 1);
        memcpy(dup, m, ml + 1);
        return rl_err_msg((rl_string){ .data = dup, .len = ml, .rc = 1 });
    }
    int64_t widx = _rl_fs_idx(handle_id);
    bool readable = widx >= 0 ? _rl_fs_readable[widx] : false;
    if (readable) fseek(fp, 0, SEEK_CUR);
    clearerr(fp);
    size_t w = fwrite(data.data, 1, data.len, fp);
    if (w != data.len && ferror(fp)) {
        int e = errno;
        const char *es = strerror(e);
        int m = snprintf(NULL, 0, "write: %s (os error %d)", es, e);
        char *msg = malloc((uint64_t)m + 1);
        snprintf(msg, (uint64_t)m + 1, "write: %s (os error %d)", es, e);
        return rl_err_msg((rl_string){ .data = msg, .len = (uint64_t)m, .rc = 1 });
    }
    return rl_ok_i64((int64_t)data.len);
}

// Seek a handle; ok with the new position, or an error.
rl_result rl_fs_seek(int64_t handle_id, int64_t offset, int64_t whence) {
    FILE *fp = _rl_fs_get(handle_id, NULL, NULL);
    if (!fp) {
        const char *m = "seek: invalid handle";
        uint64_t ml = strlen(m);
        char *dup = malloc(ml + 1);
        memcpy(dup, m, ml + 1);
        return rl_err_msg((rl_string){ .data = dup, .len = ml, .rc = 1 });
    }
    int how = 0;
    if (whence == 0) how = SEEK_SET;
    else if (whence == 1) how = SEEK_CUR;
    else if (whence == 2) how = SEEK_END;
    else {
        int n = snprintf(NULL, 0, "seek: invalid whence %ld (expected 0, 1, or 2)", (long)whence);
        char *msg = malloc((uint64_t)n + 1);
        snprintf(msg, (uint64_t)n + 1, "seek: invalid whence %ld (expected 0, 1, or 2)", (long)whence);
        return rl_err_msg((rl_string){ .data = msg, .len = (uint64_t)n, .rc = 1 });
    }
    int64_t off = offset;
    if (whence == 0 && off < 0) off = 0;
    clearerr(fp);
    if (fseeko(fp, (off_t)off, how) != 0) {
        int e = errno;
        const char *es = strerror(e);
        int n = snprintf(NULL, 0, "seek: %s (os error %d)", es, e);
        char *msg = malloc((uint64_t)n + 1);
        snprintf(msg, (uint64_t)n + 1, "seek: %s (os error %d)", es, e);
        return rl_err_msg((rl_string){ .data = msg, .len = (uint64_t)n, .rc = 1 });
    }
    off_t pos = ftello(fp);
    return rl_ok_i64((int64_t)pos);
}

// Flush a writable handle; ok null on success, or an error.
rl_result rl_fs_flush(int64_t handle_id) {
    bool writable = false;
    FILE *fp = _rl_fs_get(handle_id, NULL, &writable);
    if (!fp) {
        const char *m = "flush: invalid handle";
        uint64_t ml = strlen(m);
        char *dup = malloc(ml + 1);
        memcpy(dup, m, ml + 1);
        return rl_err_msg((rl_string){ .data = dup, .len = ml, .rc = 1 });
    }
    if (!writable) {
        const char *m = "flush: handle is not open for writing";
        uint64_t ml = strlen(m);
        char *dup = malloc(ml + 1);
        memcpy(dup, m, ml + 1);
        return rl_err_msg((rl_string){ .data = dup, .len = ml, .rc = 1 });
    }
    if (fflush(fp) != 0) {
        int e = errno;
        const char *es = strerror(e);
        int n = snprintf(NULL, 0, "flush: %s (os error %d)", es, e);
        char *msg = malloc((uint64_t)n + 1);
        snprintf(msg, (uint64_t)n + 1, "flush: %s (os error %d)", es, e);
        return rl_err_msg((rl_string){ .data = msg, .len = (uint64_t)n, .rc = 1 });
    }
    return rl_ok_null();
}

// Read from a handle to EOF; ok with the text, or an error.
rl_result rl_fs_read_all(int64_t handle_id) {
    bool readable = false;
    FILE *fp = _rl_fs_get(handle_id, &readable, NULL);
    if (!fp) {
        const char *m = "read_all: invalid handle";
        uint64_t ml = strlen(m);
        char *dup = malloc(ml + 1);
        memcpy(dup, m, ml + 1);
        return rl_err_msg((rl_string){ .data = dup, .len = ml, .rc = 1 });
    }
    if (!readable) {
        const char *m = "read_all: handle is not open for reading";
        uint64_t ml = strlen(m);
        char *dup = malloc(ml + 1);
        memcpy(dup, m, ml + 1);
        return rl_err_msg((rl_string){ .data = dup, .len = ml, .rc = 1 });
    }
    int64_t aidx = _rl_fs_idx(handle_id);
    bool writable = aidx >= 0 ? _rl_fs_writable[aidx] : false;
    if (writable) fflush(fp);
    clearerr(fp);
    uint64_t cap = 4096;
    char *buf = malloc(cap);
    uint64_t len = 0;
    for (;;) {
        if (len >= cap) {
            cap *= 2;
            buf = realloc(buf, cap);
        }
        size_t r = fread(buf + len, 1, cap - len, fp);
        len += r;
        if (r == 0) {
            if (ferror(fp)) {
                int e = errno;
                const char *es = strerror(e);
                int m = snprintf(NULL, 0, "read_all: %s (os error %d)", es, e);
                char *msg = malloc((uint64_t)m + 1);
                snprintf(msg, (uint64_t)m + 1, "read_all: %s (os error %d)", es, e);
                free(buf);
                return rl_err_msg((rl_string){ .data = msg, .len = (uint64_t)m, .rc = 1 });
            }
            break;
        }
        if (feof(fp)) break;
    }
    char *out = malloc(len + 1);
    memcpy(out, buf, len);
    out[len] = '\0';
    free(buf);
    return rl_ok_str((rl_string){ .data = out, .len = len, .rc = 1 });
}

// True for ASCII whitespace trimmed by readline.
static bool _rl_fs_is_space(char c) {
    return c == ' ' || c == '\t' || c == '\n' || c == '\r' || c == '\f' || c == '\v';
}

// Read one line from a handle; ok with the line, or an error.
rl_result rl_fs_readline(int64_t handle_id) {
    bool readable = false;
    FILE *fp = _rl_fs_get(handle_id, &readable, NULL);
    if (!fp) {
        const char *m = "readline: invalid handle";
        uint64_t ml = strlen(m);
        char *dup = malloc(ml + 1);
        memcpy(dup, m, ml + 1);
        return rl_err_msg((rl_string){ .data = dup, .len = ml, .rc = 1 });
    }
    if (!readable) {
        const char *m = "readline: handle is not open for reading";
        uint64_t ml = strlen(m);
        char *dup = malloc(ml + 1);
        memcpy(dup, m, ml + 1);
        return rl_err_msg((rl_string){ .data = dup, .len = ml, .rc = 1 });
    }
    int64_t lidx = _rl_fs_idx(handle_id);
    bool writable = lidx >= 0 ? _rl_fs_writable[lidx] : false;
    if (writable) fflush(fp);
    clearerr(fp);
    uint64_t cap = 256;
    char *buf = malloc(cap);
    uint64_t len = 0;
    int c;
    bool any = false;
    while ((c = fgetc(fp)) != EOF) {
        any = true;
        if (len >= cap) {
            cap *= 2;
            buf = realloc(buf, cap);
        }
        buf[len++] = (char)c;
        if (c == '\n') break;
    }
    if (!any) {
        if (ferror(fp)) {
            int e = errno;
            const char *es = strerror(e);
            int m = snprintf(NULL, 0, "readline: %s (os error %d)", es, e);
            char *msg = malloc((uint64_t)m + 1);
            snprintf(msg, (uint64_t)m + 1, "readline: %s (os error %d)", es, e);
            free(buf);
            return rl_err_msg((rl_string){ .data = msg, .len = (uint64_t)m, .rc = 1 });
        }
        free(buf);
        char *empty = malloc(1);
        empty[0] = '\0';
        return rl_ok_str((rl_string){ .data = empty, .len = 0, .rc = 1 });
    }
    while (len > 0 && _rl_fs_is_space(buf[len - 1])) len--;
    char *out = malloc(len + 1);
    memcpy(out, buf, len);
    out[len] = '\0';
    free(buf);
    return rl_ok_str((rl_string){ .data = out, .len = len, .rc = 1 });
}

// ---- process ----

// Current working directory of the process.
// Value of the environment variable `key` (owned copy), or a null
// string (data == NULL) when unset. Matches the VM: bare string or null.
rl_string rl_process_env(rl_string key) {
    char buf[key.len + 1];
    memcpy(buf, key.data, key.len);
    buf[key.len] = '\0';
    const char *val = getenv(buf);
    if (val == NULL) {
        rl_string result = { .data = NULL, .len = 0, .rc = 0 };
        return result;
    }
    uint64_t len = strlen(val);
    char *out = malloc(len + 1);
    memcpy(out, val, len + 1);
    rl_string result = { .data = out, .len = len, .rc = 1 };
    return result;
}

// Current working directory, or an error.
rl_result rl_process_cwd(void) {
    char buf[4096];
    if (getcwd(buf, sizeof(buf)) == NULL) return rl_err(-1);
    uint64_t len = strlen(buf);
    char *out = malloc(len + 1);
    memcpy(out, buf, len + 1);
    rl_string result = { .data = out, .len = len, .rc = 1 };
    return rl_ok_str(result);
}

// Change directory; ok null on success, or an error.
rl_result rl_process_set_cwd(rl_string path) {
    if (path.data == NULL) return rl_err(-1);
    char buf[path.len + 1];
    memcpy(buf, path.data, path.len);
    buf[path.len] = '\0';
    if (chdir(buf) != 0) return rl_err(-1);
    return rl_ok_null();
}

// Run `cmd` through the shell and capture stdout (variants return
// the exit code or one array element per output line instead).
// Run `cmd` through the shell and capture stdout (trailing newlines
// stripped); ok with the output, or an error when it cannot run.
rl_result rl_process_exec(rl_string cmd) {
    if (cmd.data == NULL) return rl_err(-1);
    char buf[cmd.len + 1];
    memcpy(buf, cmd.data, cmd.len);
    buf[cmd.len] = '\0';
    FILE *fp = popen(buf, "r");
    if (!fp) return rl_err(-1);
    uint64_t cap = 4096;
    char *out = malloc(cap);
    uint64_t len = 0;
    size_t n;
    while ((n = fread(out + len, 1, cap - len, fp)) > 0) {
        len += n;
        if (len >= cap) {
            cap *= 2;
            out = realloc(out, cap);
        }
    }
    pclose(fp);
    out[len] = '\0';
    while (len > 0 && out[len - 1] == '\n') {
        len--;
    }
    rl_string result = { .data = out, .len = len, .rc = 1 };
    return rl_ok_str(result);
}

// Run `cmd` through the shell in the foreground; ok with the exit code,
// or an error when it cannot run.
rl_result rl_process_exec_fg(rl_string cmd) {
    if (cmd.data == NULL) return rl_err(-1);
    char buf[cmd.len + 1];
    memcpy(buf, cmd.data, cmd.len);
    buf[cmd.len] = '\0';
    int code = system(buf);
    if (code == -1) return rl_err(-1);
    return rl_ok_i64((int64_t)WEXITSTATUS(code));
}

// Run `cmd` through the shell; ok with the exit code, or an error.
rl_result rl_process_exec_code(rl_string cmd) {
    if (cmd.data == NULL) return rl_err(-1);
    char buf[cmd.len + 1];
    memcpy(buf, cmd.data, cmd.len);
    buf[cmd.len] = '\0';
    int code = system(buf);
    if (code == -1) return rl_err(-1);
    return rl_ok_i64((int64_t)WEXITSTATUS(code));
}

// Run `cmd` through the shell; ok with one array element per output
// line, or an error.
rl_result rl_process_exec_lines(rl_string cmd) {
    rl_result output = rl_process_exec(cmd);
    if (!output.is_ok) return output;
    rl_string nl = { .data = "\n", .len = 1, .rc = 1 };
    return rl_ok_arr(rl_str_split(output.data.str, nl));
}

// Same, but with `env` assignments (e.g. `"A=1 B=2"`) prepended.
rl_result rl_process_with_exec(rl_string env, rl_string cmd) {
    if (env.data == NULL || cmd.data == NULL) return rl_err(-1);
    uint64_t total = env.len + 1 + cmd.len;
    char buf[total + 1];
    memcpy(buf, env.data, env.len);
    buf[env.len] = ' ';
    memcpy(buf + env.len + 1, cmd.data, cmd.len);
    buf[total] = '\0';
    rl_string combined = { .data = buf, .len = total, .rc = 0 };
    return rl_process_exec(combined);
}

// Same, but with `env` assignments (e.g. `"A=1 B=2"`) prepended.
rl_result rl_process_with_exec_code(rl_string env, rl_string cmd) {
    if (env.data == NULL || cmd.data == NULL) return rl_err(-1);
    uint64_t total = env.len + 1 + cmd.len;
    char buf[total + 1];
    memcpy(buf, env.data, env.len);
    buf[env.len] = ' ';
    memcpy(buf + env.len + 1, cmd.data, cmd.len);
    buf[total] = '\0';
    rl_string combined = { .data = buf, .len = total, .rc = 0 };
    return rl_process_exec_code(combined);
}

// Same, but with `env` assignments (e.g. `"A=1 B=2"`) prepended.
rl_result rl_process_with_exec_lines(rl_string env, rl_string cmd) {
    if (env.data == NULL || cmd.data == NULL) return rl_err(-1);
    uint64_t total = env.len + 1 + cmd.len;
    char buf[total + 1];
    memcpy(buf, env.data, env.len);
    buf[env.len] = ' ';
    memcpy(buf + env.len + 1, cmd.data, cmd.len);
    buf[total] = '\0';
    rl_string combined = { .data = buf, .len = total, .rc = 0 };
    return rl_process_exec_lines(combined);
}

// OS name like Rust's `std::env::consts::OS` (`linux`, `macos`, ...).
rl_string rl_process_os_name(void) {
#if defined(__linux__)
    return rl_str_literal("linux", 5);
#elif defined(__APPLE__)
    return rl_str_literal("macos", 5);
#elif defined(_WIN32)
    return rl_str_literal("windows", 7);
#else
    return rl_str_literal("unknown", 7);
#endif
}

// ---- background processes ----
// PIDs spawned by `exec_background`, with cached exit statuses so a
// reaped child still answers `wait_pid` like the VM's child table.
#define _RL_MAX_BG 256
static pid_t _rl_bg_pids[_RL_MAX_BG];
static bool _rl_bg_done[_RL_MAX_BG];
static int64_t _rl_bg_status[_RL_MAX_BG];
static int _rl_bg_count = 0;

static int _rl_bg_find(pid_t pid) {
    for (int i = 0; i < _rl_bg_count; i++) {
        if (_rl_bg_pids[i] == pid) return i;
    }
    return -1;
}

// Fork and run `cmd` through the shell; the child PID, or -1.
static pid_t _rl_spawn_shell(const char *cmd) {
    pid_t pid = fork();
    if (pid < 0) return -1;
    if (pid == 0) {
        execl("/bin/sh", "sh", "-c", cmd, (char *)NULL);
        _exit(127);
    }
    return pid;
}

// Spawn `cmd` in the background; ok with the child pid, or an error.
rl_result rl_process_exec_background(rl_string cmd) {
    if (cmd.data == NULL) return rl_err(-1);
    if (_rl_bg_count >= _RL_MAX_BG) return rl_err(-1);
    char buf[cmd.len + 1];
    memcpy(buf, cmd.data, cmd.len);
    buf[cmd.len] = '\0';
    pid_t pid = _rl_spawn_shell(buf);
    if (pid < 0) return rl_err(-1);
    _rl_bg_pids[_rl_bg_count] = pid;
    _rl_bg_done[_rl_bg_count] = false;
    _rl_bg_status[_rl_bg_count] = 0;
    _rl_bg_count++;
    return rl_ok_i64((int64_t)pid);
}

// True while a spawned pid is still running (false for unknown pids,
// mirroring the VM's child table).
bool rl_process_running(int64_t pid) {
    int idx = _rl_bg_find((pid_t)pid);
    if (idx < 0 || _rl_bg_done[idx]) return false;
    int status = 0;
    pid_t r = waitpid((pid_t)pid, &status, WNOHANG);
    if (r == 0) return true;
    _rl_bg_done[idx] = true;
    _rl_bg_status[idx] =
        (r > 0 && WIFEXITED(status)) ? (int64_t)WEXITSTATUS(status) : -1;
    return false;
}

// Reap a spawned pid; ok with its exit code (-1 when signaled), or an
// error for untracked pids.
rl_result rl_process_wait_pid(int64_t pid) {
    int idx = _rl_bg_find((pid_t)pid);
    if (idx < 0) {
        char *msg = malloc(64);
        int n = snprintf(msg, 64, "wait_pid: no tracked process with pid %ld", (long)pid);
        return rl_err_msg((rl_string){ .data = msg, .len = (uint64_t)n, .rc = 1 });
    }
    pid_t p = _rl_bg_pids[idx];
    bool done = _rl_bg_done[idx];
    int64_t status = _rl_bg_status[idx];
    _rl_bg_pids[idx] = _rl_bg_pids[_rl_bg_count - 1];
    _rl_bg_done[idx] = _rl_bg_done[_rl_bg_count - 1];
    _rl_bg_status[idx] = _rl_bg_status[_rl_bg_count - 1];
    _rl_bg_count--;
    if (done) return rl_ok_i64(status);
    int st = 0;
    if (waitpid(p, &st, 0) < 0) {
        char *msg = malloc(64);
        int n = snprintf(msg, 64, "wait_pid: failed waiting for pid %ld", (long)pid);
        return rl_err_msg((rl_string){ .data = msg, .len = (uint64_t)n, .rc = 1 });
    }
    return rl_ok_i64(WIFEXITED(st) ? (int64_t)WEXITSTATUS(st) : -1);
}

// Send SIGTERM / SIGKILL; ok null on success, or an error.
rl_result rl_process_term_pid(int64_t pid) {
    if (kill((pid_t)pid, SIGTERM) != 0) {
        char *msg = malloc(64);
        int n = snprintf(msg, 64, "term_pid: failed to send SIGTERM to pid %ld", (long)pid);
        return rl_err_msg((rl_string){ .data = msg, .len = (uint64_t)n, .rc = 1 });
    }
    return rl_ok_null();
}

rl_result rl_process_kill_pid(int64_t pid) {
    if (kill((pid_t)pid, SIGKILL) != 0) {
        char *msg = malloc(64);
        int n = snprintf(msg, 64, "kill_pid: failed to send SIGKILL to pid %ld", (long)pid);
        return rl_err_msg((rl_string){ .data = msg, .len = (uint64_t)n, .rc = 1 });
    }
    return rl_ok_null();
}

// Set an environment variable; ok null on success, or an error.
rl_result rl_process_set_env(rl_string key, rl_string value) {
    if (key.data == NULL || value.data == NULL) return rl_err(-1);
    char kbuf[key.len + 1];
    memcpy(kbuf, key.data, key.len);
    kbuf[key.len] = '\0';
    char vbuf[value.len + 1];
    memcpy(vbuf, value.data, value.len);
    vbuf[value.len] = '\0';
    if (setenv(kbuf, vbuf, 1) != 0) return rl_err(-1);
    return rl_ok_null();
}

// Remove an environment variable; ok null on success, or an error.
rl_result rl_process_remove_env(rl_string key) {
    if (key.data == NULL) return rl_err(-1);
    char kbuf[key.len + 1];
    memcpy(kbuf, key.data, key.len);
    kbuf[key.len] = '\0';
    if (unsetenv(kbuf) != 0) return rl_err(-1);
    return rl_ok_null();
}

// All environment variable names as an array of strings.
rl_array rl_process_env_keys(void) {
    extern char **environ;
    rl_array empty = { .data = NULL, .len = 0, .cap = 0, .elem_size = sizeof(rl_string), .type_tag = RL_TAG_STR };
    if (environ == NULL) return empty;
    uint64_t count = 0;
    for (char **e = environ; *e != NULL; e++) count++;
    if (count == 0) return empty;
    rl_string *buf = malloc(count * sizeof(rl_string));
    uint64_t n = 0;
    for (char **e = environ; *e != NULL; e++) {
        char *eq = strchr(*e, '=');
        uint64_t klen = eq ? (uint64_t)(eq - *e) : strlen(*e);
        char *out = malloc(klen + 1);
        memcpy(out, *e, klen);
        out[klen] = '\0';
        buf[n++] = (rl_string){ .data = out, .len = klen, .rc = 1 };
    }
    rl_array arr = { .data = buf, .len = n, .cap = n, .elem_size = sizeof(rl_string), .type_tag = RL_TAG_STR };
    return arr;
}

// CPU architecture like Rust consts::ARCH.
rl_string rl_process_arch(void) {
#if defined(__x86_64__) || defined(_M_X64)
    return rl_str_literal("x86_64", 6);
#elif defined(__aarch64__) || defined(_M_ARM64)
    return rl_str_literal("aarch64", 7);
#elif defined(__arm__) || defined(_M_ARM)
    return rl_str_literal("arm", 3);
#elif defined(__i386__) || defined(_M_IX86)
    return rl_str_literal("x86", 3);
#elif defined(__riscv) && __riscv_xlen == 64
    return rl_str_literal("riscv64", 7);
#elif defined(__mips64__)
    return rl_str_literal("mips64", 6);
#elif defined(__mips__)
    return rl_str_literal("mips", 4);
#elif defined(__powerpc64__)
    return rl_str_literal("powerpc64", 9);
#elif defined(__powerpc__)
    return rl_str_literal("powerpc", 7);
#elif defined(__s390x__)
    return rl_str_literal("s390x", 5);
#else
    return rl_str_literal("unknown", 7);
#endif
}

// Number of available CPUs, or 1 when unknown.
int64_t rl_process_num_cpus(void) {
    long n = sysconf(_SC_NPROCESSORS_ONLN);
    if (n < 1) return 1;
    return (int64_t)n;
}

// Parent process id.
int64_t rl_process_parent_pid(void) {
    return (int64_t)getppid();
}

// True exactly when kill(pid, 0) succeeds, mirroring the VM (which
// reports false for EPERM pids and true for pid 0's process group).
bool rl_process_exists(int64_t pid) {
    return kill((pid_t)pid, 0) == 0;
}

// Run exe plus args in the foreground; ok with the exit code, or an error.
rl_result rl_process_with_exec_fg(rl_string exe, rl_string cmd) {
    if (exe.data == NULL || cmd.data == NULL) return rl_err(-1);
    uint64_t total = exe.len + 1 + cmd.len;
    char buf[total + 1];
    memcpy(buf, exe.data, exe.len);
    buf[exe.len] = ' ';
    memcpy(buf + exe.len + 1, cmd.data, cmd.len);
    buf[total] = '\0';
    rl_string combined = { .data = buf, .len = total, .rc = 0 };
    return rl_process_exec_fg(combined);
}

// Helper to capture shell output with stdin input via fork.
static rl_result _rl_stdin_capture(const char *cmd_cstr, rl_string input, const char *prefix, const char *orig_cmd, uint64_t orig_len, bool use_prefix_only) {
    int in_pipe[2];
    int out_pipe[2];
    if (pipe(in_pipe) != 0) {
        int e = errno;
        const char *es = strerror(e);
        int n;
        char *msg;
        if (use_prefix_only) {
            n = snprintf(NULL, 0, "%s: failed: %s (os error %d)", prefix, es, e);
            msg = malloc((uint64_t)n + 1);
            snprintf(msg, (uint64_t)n + 1, "%s: failed: %s (os error %d)", prefix, es, e);
        } else {
            n = snprintf(NULL, 0, "%s: failed to run \"%.*s\": %s (os error %d)", prefix, (int)orig_len, orig_cmd, es, e);
            msg = malloc((uint64_t)n + 1);
            snprintf(msg, (uint64_t)n + 1, "%s: failed to run \"%.*s\": %s (os error %d)", prefix, (int)orig_len, orig_cmd, es, e);
        }
        return rl_err_msg((rl_string){ .data = msg, .len = (uint64_t)n, .rc = 1 });
    }
    if (pipe(out_pipe) != 0) {
        int e = errno;
        const char *es = strerror(e);
        int n;
        char *msg;
        if (use_prefix_only) {
            n = snprintf(NULL, 0, "%s: failed: %s (os error %d)", prefix, es, e);
            msg = malloc((uint64_t)n + 1);
            snprintf(msg, (uint64_t)n + 1, "%s: failed: %s (os error %d)", prefix, es, e);
        } else {
            n = snprintf(NULL, 0, "%s: failed to run \"%.*s\": %s (os error %d)", prefix, (int)orig_len, orig_cmd, es, e);
            msg = malloc((uint64_t)n + 1);
            snprintf(msg, (uint64_t)n + 1, "%s: failed to run \"%.*s\": %s (os error %d)", prefix, (int)orig_len, orig_cmd, es, e);
        }
        close(in_pipe[0]); close(in_pipe[1]);
        return rl_err_msg((rl_string){ .data = msg, .len = (uint64_t)n, .rc = 1 });
    }
    pid_t pid = fork();
    if (pid < 0) {
        int e = errno;
        const char *es = strerror(e);
        int n;
        char *msg;
        if (use_prefix_only) {
            n = snprintf(NULL, 0, "%s: failed: %s (os error %d)", prefix, es, e);
            msg = malloc((uint64_t)n + 1);
            snprintf(msg, (uint64_t)n + 1, "%s: failed: %s (os error %d)", prefix, es, e);
        } else {
            n = snprintf(NULL, 0, "%s: failed to run \"%.*s\": %s (os error %d)", prefix, (int)orig_len, orig_cmd, es, e);
            msg = malloc((uint64_t)n + 1);
            snprintf(msg, (uint64_t)n + 1, "%s: failed to run \"%.*s\": %s (os error %d)", prefix, (int)orig_len, orig_cmd, es, e);
        }
        close(in_pipe[0]); close(in_pipe[1]); close(out_pipe[0]); close(out_pipe[1]);
        return rl_err_msg((rl_string){ .data = msg, .len = (uint64_t)n, .rc = 1 });
    }
    if (pid == 0) {
        dup2(in_pipe[0], STDIN_FILENO);
        dup2(out_pipe[1], STDOUT_FILENO);
        close(in_pipe[0]); close(in_pipe[1]); close(out_pipe[0]); close(out_pipe[1]);
        execl("/bin/sh", "sh", "-c", cmd_cstr, (char *)NULL);
        _exit(127);
    }
    close(in_pipe[0]); close(out_pipe[1]);
    uint64_t written = 0;
    while (written < input.len) {
        ssize_t w = write(in_pipe[1], input.data + written, input.len - written);
        if (w < 0) {
            if (errno == EINTR) continue;
            break;
        }
        if (w == 0) break;
        written += (uint64_t)w;
    }
    close(in_pipe[1]);
    uint64_t cap = 4096;
    char *out = malloc(cap);
    uint64_t len = 0;
    for (;;) {
        if (len >= cap) { cap *= 2; out = realloc(out, cap); }
        ssize_t r = read(out_pipe[0], out + len, cap - len);
        if (r < 0) {
            if (errno == EINTR) continue;
            break;
        }
        if (r == 0) break;
        len += (uint64_t)r;
    }
    close(out_pipe[0]);
    int status = 0;
    while (waitpid(pid, &status, 0) < 0) {
        if (errno != EINTR) {
            int e = errno;
            const char *es = strerror(e);
            int n = snprintf(NULL, 0, "%s: failed: %s (os error %d)", prefix, es, e);
            char *msg = malloc((uint64_t)n + 1);
            snprintf(msg, (uint64_t)n + 1, "%s: failed: %s (os error %d)", prefix, es, e);
            free(out);
            return rl_err_msg((rl_string){ .data = msg, .len = (uint64_t)n, .rc = 1 });
        }
    }
    out[len] = '\0';
    while (len > 0 && out[len - 1] == '\n') len--;
    rl_string result = { .data = out, .len = len, .rc = 1 };
    return rl_ok_str(result);
}

// Run cmd with input on stdin; ok with stdout, or an error.
rl_result rl_process_exec_with_stdin(rl_string cmd, rl_string input) {
    if (cmd.data == NULL || input.data == NULL) return rl_err(-1);
    char buf[cmd.len + 1];
    memcpy(buf, cmd.data, cmd.len);
    buf[cmd.len] = '\0';
    return _rl_stdin_capture(buf, input, "exec_with_stdin", cmd.data, cmd.len, false);
}

// Same with exe plus args and input on stdin; ok with stdout, or an error.
rl_result rl_process_with_exec_with_stdin(rl_string exe, rl_string cmd, rl_string input) {
    if (exe.data == NULL || cmd.data == NULL || input.data == NULL) return rl_err(-1);
    uint64_t total = exe.len + 1 + cmd.len;
    char *buf = malloc(total + 1);
    memcpy(buf, exe.data, exe.len);
    buf[exe.len] = ' ';
    memcpy(buf + exe.len + 1, cmd.data, cmd.len);
    buf[total] = '\0';
    rl_result r = _rl_stdin_capture(buf, input, "with_exec_with_stdin", cmd.data, cmd.len, true);
    free(buf);
    return r;
}

// True when a byte is safe to leave unquoted in shell env assignments.
static bool _rl_env_char_safe(char c) {
    if (c >= 'A' && c <= 'Z') return true;
    if (c >= 'a' && c <= 'z') return true;
    if (c >= '0' && c <= '9') return true;
    if (c == '_' || c == '@' || c == '%' || c == '+' || c == '=' || c == ':' || c == ',' || c == '.' || c == '/' || c == '-' ) return true;
    return false;
}

// Build "K=V " prefix from envs pairs; returns malloced buffer with len.
static char *_rl_env_prefix(rl_array envs, uint64_t *out_len) {
    uint64_t cap = 256;
    char *buf = malloc(cap);
    uint64_t len = 0;
    if (envs.data == NULL || envs.len == 0) {
        buf[0] = '\0';
        *out_len = 0;
        return buf;
    }
    rl_array *pairs = (rl_array *)envs.data;
    for (uint64_t i = 0; i < envs.len; i++) {
        rl_array pair = pairs[i];
        if (pair.len != 2 || pair.data == NULL) continue;
        rl_string *kv = (rl_string *)pair.data;
        rl_string k = kv[0];
        rl_string v = kv[1];
        if (k.data == NULL || v.data == NULL) continue;
        if (k.len == 0) continue;
        bool need_quote = (v.len == 0);
        for (uint64_t j = 0; j < v.len && !need_quote; j++) {
            if (!_rl_env_char_safe(v.data[j])) need_quote = true;
        }
        uint64_t need = k.len + 1 + v.len + 4 + 1;
        if (need_quote) need += v.len + 2;
        while (len + need + 1 > cap) { cap *= 2; buf = realloc(buf, cap); }
        memcpy(buf + len, k.data, k.len);
        len += k.len;
        buf[len++] = '=';
        if (!need_quote) {
            memcpy(buf + len, v.data, v.len);
            len += v.len;
        } else if (v.len == 0) {
            buf[len++] = '\'';
            buf[len++] = '\'';
        } else {
            buf[len++] = '\'';
            for (uint64_t j = 0; j < v.len; j++) {
                if (v.data[j] == '\'') {
                    memcpy(buf + len, "'\\''", 4);
                    len += 4;
                } else {
                    buf[len++] = v.data[j];
                }
            }
            buf[len++] = '\'';
        }
        buf[len++] = ' ';
    }
    buf[len] = '\0';
    *out_len = len;
    return buf;
}

// Run cmd with envs pairs; ok with stdout, or an error.
rl_result rl_process_exec_with_env(rl_string cmd, rl_array envs) {
    if (cmd.data == NULL) return rl_err(-1);
    uint64_t plen = 0;
    char *prefix = _rl_env_prefix(envs, &plen);
    char *buf = NULL;
    uint64_t total = 0;
    if (plen > 0) {
        // Use export so $VAR expands after assignment. Plain
        // "K=V cmd" would expand $VAR before assignment.
        const char *head = "export ";
        const char *mid = "; ";
        uint64_t hlen = 7;
        uint64_t mlen = 2;
        total = hlen + plen + mlen + cmd.len;
        buf = malloc(total + 1);
        memcpy(buf, head, hlen);
        memcpy(buf + hlen, prefix, plen);
        memcpy(buf + hlen + plen, mid, mlen);
        memcpy(buf + hlen + plen + mlen, cmd.data, cmd.len);
        buf[total] = '\0';
    } else {
        total = cmd.len;
        buf = malloc(total + 1);
        memcpy(buf, cmd.data, cmd.len);
        buf[total] = '\0';
    }
    free(prefix);
    rl_string combined = { .data = buf, .len = total, .rc = 1 };
    rl_result r = rl_process_exec(combined);
    if (!r.is_ok) {
        int e = errno;
        const char *es = strerror(e);
        int n = snprintf(NULL, 0, "exec_with_env: failed to run \"%.*s\": %s (os error %d)", (int)cmd.len, cmd.data, es, e);
        char *msg = malloc((uint64_t)n + 1);
        snprintf(msg, (uint64_t)n + 1, "exec_with_env: failed to run \"%.*s\": %s (os error %d)", (int)cmd.len, cmd.data, es, e);
        free(buf);
        return rl_err_msg((rl_string){ .data = msg, .len = (uint64_t)n, .rc = 1 });
    }
    free(buf);
    return r;
}

// Same with exe plus args and envs pairs; ok with stdout, or an error.
rl_result rl_process_with_exec_with_env(rl_string exe, rl_string cmd, rl_array envs) {
    if (exe.data == NULL || cmd.data == NULL) return rl_err(-1);
    uint64_t base = exe.len + 1 + cmd.len;
    char *basebuf = malloc(base + 1);
    memcpy(basebuf, exe.data, exe.len);
    basebuf[exe.len] = ' ';
    memcpy(basebuf + exe.len + 1, cmd.data, cmd.len);
    basebuf[base] = '\0';
    uint64_t plen = 0;
    char *prefix = _rl_env_prefix(envs, &plen);
    char *buf = NULL;
    uint64_t total = 0;
    if (plen > 0) {
        const char *head = "export ";
        const char *mid = "; ";
        uint64_t hlen = 7;
        uint64_t mlen = 2;
        total = hlen + plen + mlen + base;
        buf = malloc(total + 1);
        memcpy(buf, head, hlen);
        memcpy(buf + hlen, prefix, plen);
        memcpy(buf + hlen + plen, mid, mlen);
        memcpy(buf + hlen + plen + mlen, basebuf, base);
        buf[total] = '\0';
    } else {
        total = base;
        buf = malloc(total + 1);
        memcpy(buf, basebuf, base);
        buf[total] = '\0';
    }
    free(prefix);
    free(basebuf);
    rl_string combined = { .data = buf, .len = total, .rc = 1 };
    rl_result r = rl_process_exec(combined);
    if (!r.is_ok) {
        int e = errno;
        const char *es = strerror(e);
        int n = snprintf(NULL, 0, "with_exec_with_env: failed: %s (os error %d)", es, e);
        char *msg = malloc((uint64_t)n + 1);
        snprintf(msg, (uint64_t)n + 1, "with_exec_with_env: failed: %s (os error %d)", es, e);
        free(buf);
        return rl_err_msg((rl_string){ .data = msg, .len = (uint64_t)n, .rc = 1 });
    }
    free(buf);
    return r;
}

// Helper to capture shell output in dir via fork.
static rl_result _rl_cwd_capture(const char *cmd_cstr, const char *dir_cstr, const char *prefix, const char *orig_cmd, uint64_t orig_len, const char *orig_dir, uint64_t orig_dir_len, bool use_prefix_only) {
    int out_pipe[2];
    if (pipe(out_pipe) != 0) {
        int e = errno;
        const char *es = strerror(e);
        int n;
        char *msg;
        if (use_prefix_only) {
            n = snprintf(NULL, 0, "%s: failed: %s (os error %d)", prefix, es, e);
            msg = malloc((uint64_t)n + 1);
            snprintf(msg, (uint64_t)n + 1, "%s: failed: %s (os error %d)", prefix, es, e);
        } else {
            n = snprintf(NULL, 0, "%s: failed to run \"%.*s\" in \"%.*s\": %s (os error %d)", prefix, (int)orig_len, orig_cmd, (int)orig_dir_len, orig_dir, es, e);
            msg = malloc((uint64_t)n + 1);
            snprintf(msg, (uint64_t)n + 1, "%s: failed to run \"%.*s\" in \"%.*s\": %s (os error %d)", prefix, (int)orig_len, orig_cmd, (int)orig_dir_len, orig_dir, es, e);
        }
        return rl_err_msg((rl_string){ .data = msg, .len = (uint64_t)n, .rc = 1 });
    }
    pid_t pid = fork();
    if (pid < 0) {
        int e = errno;
        const char *es = strerror(e);
        int n;
        char *msg;
        if (use_prefix_only) {
            n = snprintf(NULL, 0, "%s: failed: %s (os error %d)", prefix, es, e);
            msg = malloc((uint64_t)n + 1);
            snprintf(msg, (uint64_t)n + 1, "%s: failed: %s (os error %d)", prefix, es, e);
        } else {
            n = snprintf(NULL, 0, "%s: failed to run \"%.*s\" in \"%.*s\": %s (os error %d)", prefix, (int)orig_len, orig_cmd, (int)orig_dir_len, orig_dir, es, e);
            msg = malloc((uint64_t)n + 1);
            snprintf(msg, (uint64_t)n + 1, "%s: failed to run \"%.*s\" in \"%.*s\": %s (os error %d)", prefix, (int)orig_len, orig_cmd, (int)orig_dir_len, orig_dir, es, e);
        }
        close(out_pipe[0]); close(out_pipe[1]);
        return rl_err_msg((rl_string){ .data = msg, .len = (uint64_t)n, .rc = 1 });
    }
    if (pid == 0) {
        if (chdir(dir_cstr) != 0) _exit(127);
        dup2(out_pipe[1], STDOUT_FILENO);
        close(out_pipe[0]); close(out_pipe[1]);
        execl("/bin/sh", "sh", "-c", cmd_cstr, (char *)NULL);
        _exit(127);
    }
    close(out_pipe[1]);
    uint64_t cap = 4096;
    char *out = malloc(cap);
    uint64_t len = 0;
    for (;;) {
        if (len >= cap) { cap *= 2; out = realloc(out, cap); }
        ssize_t r = read(out_pipe[0], out + len, cap - len);
        if (r < 0) {
            if (errno == EINTR) continue;
            break;
        }
        if (r == 0) break;
        len += (uint64_t)r;
    }
    close(out_pipe[0]);
    int status = 0;
    while (waitpid(pid, &status, 0) < 0) {
        if (errno != EINTR) {
            int e = errno;
            const char *es = strerror(e);
            int n;
            char *msg;
            if (use_prefix_only) {
                n = snprintf(NULL, 0, "%s: failed: %s (os error %d)", prefix, es, e);
                msg = malloc((uint64_t)n + 1);
                snprintf(msg, (uint64_t)n + 1, "%s: failed: %s (os error %d)", prefix, es, e);
            } else {
                n = snprintf(NULL, 0, "%s: failed to run \"%.*s\" in \"%.*s\": %s (os error %d)", prefix, (int)orig_len, orig_cmd, (int)orig_dir_len, orig_dir, es, e);
                msg = malloc((uint64_t)n + 1);
                snprintf(msg, (uint64_t)n + 1, "%s: failed to run \"%.*s\" in \"%.*s\": %s (os error %d)", prefix, (int)orig_len, orig_cmd, (int)orig_dir_len, orig_dir, es, e);
            }
            free(out);
            return rl_err_msg((rl_string){ .data = msg, .len = (uint64_t)n, .rc = 1 });
        }
    }
    if (WIFEXITED(status) && WEXITSTATUS(status) == 127) {
        struct stat st;
        if (stat(dir_cstr, &st) != 0 || !S_ISDIR(st.st_mode)) {
            int e = errno;
            if (stat(dir_cstr, &st) == 0 && !S_ISDIR(st.st_mode)) e = ENOTDIR;
            const char *es = strerror(e);
            int n;
            char *msg;
            if (use_prefix_only) {
                n = snprintf(NULL, 0, "%s: failed: %s (os error %d)", prefix, es, e);
                msg = malloc((uint64_t)n + 1);
                snprintf(msg, (uint64_t)n + 1, "%s: failed: %s (os error %d)", prefix, es, e);
            } else {
                n = snprintf(NULL, 0, "%s: failed to run \"%.*s\" in \"%.*s\": %s (os error %d)", prefix, (int)orig_len, orig_cmd, (int)orig_dir_len, orig_dir, es, e);
                msg = malloc((uint64_t)n + 1);
                snprintf(msg, (uint64_t)n + 1, "%s: failed to run \"%.*s\" in \"%.*s\": %s (os error %d)", prefix, (int)orig_len, orig_cmd, (int)orig_dir_len, orig_dir, es, e);
            }
            free(out);
            return rl_err_msg((rl_string){ .data = msg, .len = (uint64_t)n, .rc = 1 });
        }
    }
    out[len] = '\0';
    while (len > 0 && out[len - 1] == '\n') len--;
    rl_string result = { .data = out, .len = len, .rc = 1 };
    return rl_ok_str(result);
}

// Run cmd in dir; ok with stdout, or an error.
rl_result rl_process_exec_with_cwd(rl_string cmd, rl_string dir) {
    if (cmd.data == NULL || dir.data == NULL) return rl_err(-1);
    char cbuf[cmd.len + 1];
    memcpy(cbuf, cmd.data, cmd.len);
    cbuf[cmd.len] = '\0';
    char dbuf[dir.len + 1];
    memcpy(dbuf, dir.data, dir.len);
    dbuf[dir.len] = '\0';
    struct stat st;
    if (stat(dbuf, &st) != 0) {
        int e = errno;
        const char *es = strerror(e);
        int n = snprintf(NULL, 0, "exec_with_cwd: failed to run \"%.*s\" in \"%.*s\": %s (os error %d)", (int)cmd.len, cmd.data, (int)dir.len, dir.data, es, e);
        char *msg = malloc((uint64_t)n + 1);
        snprintf(msg, (uint64_t)n + 1, "exec_with_cwd: failed to run \"%.*s\" in \"%.*s\": %s (os error %d)", (int)cmd.len, cmd.data, (int)dir.len, dir.data, es, e);
        return rl_err_msg((rl_string){ .data = msg, .len = (uint64_t)n, .rc = 1 });
    }
    if (!S_ISDIR(st.st_mode)) {
        int e = ENOTDIR;
        const char *es = strerror(e);
        int n = snprintf(NULL, 0, "exec_with_cwd: failed to run \"%.*s\" in \"%.*s\": %s (os error %d)", (int)cmd.len, cmd.data, (int)dir.len, dir.data, es, e);
        char *msg = malloc((uint64_t)n + 1);
        snprintf(msg, (uint64_t)n + 1, "exec_with_cwd: failed to run \"%.*s\" in \"%.*s\": %s (os error %d)", (int)cmd.len, cmd.data, (int)dir.len, dir.data, es, e);
        return rl_err_msg((rl_string){ .data = msg, .len = (uint64_t)n, .rc = 1 });
    }
    return _rl_cwd_capture(cbuf, dbuf, "exec_with_cwd", cmd.data, cmd.len, dir.data, dir.len, false);
}

// Same with exe plus args in dir; ok with stdout, or an error.
rl_result rl_process_with_exec_with_cwd(rl_string exe, rl_string cmd, rl_string dir) {
    if (exe.data == NULL || cmd.data == NULL || dir.data == NULL) return rl_err(-1);
    uint64_t total = exe.len + 1 + cmd.len;
    char *buf = malloc(total + 1);
    memcpy(buf, exe.data, exe.len);
    buf[exe.len] = ' ';
    memcpy(buf + exe.len + 1, cmd.data, cmd.len);
    buf[total] = '\0';
    char dbuf[dir.len + 1];
    memcpy(dbuf, dir.data, dir.len);
    dbuf[dir.len] = '\0';
    struct stat st;
    if (stat(dbuf, &st) != 0) {
        int e = errno;
        const char *es = strerror(e);
        int n = snprintf(NULL, 0, "with_exec_with_cwd: failed: %s (os error %d)", es, e);
        char *msg = malloc((uint64_t)n + 1);
        snprintf(msg, (uint64_t)n + 1, "with_exec_with_cwd: failed: %s (os error %d)", es, e);
        free(buf);
        return rl_err_msg((rl_string){ .data = msg, .len = (uint64_t)n, .rc = 1 });
    }
    if (!S_ISDIR(st.st_mode)) {
        int e = ENOTDIR;
        const char *es = strerror(e);
        int n = snprintf(NULL, 0, "with_exec_with_cwd: failed: %s (os error %d)", es, e);
        char *msg = malloc((uint64_t)n + 1);
        snprintf(msg, (uint64_t)n + 1, "with_exec_with_cwd: failed: %s (os error %d)", es, e);
        free(buf);
        return rl_err_msg((rl_string){ .data = msg, .len = (uint64_t)n, .rc = 1 });
    }
    rl_result r = _rl_cwd_capture(buf, dbuf, "with_exec_with_cwd", cmd.data, cmd.len, dir.data, dir.len, true);
    free(buf);
    return r;
}

// Run cmd with timeout; ok with stdout, or an error on timeout.
rl_result rl_process_exec_with_timeout(rl_string cmd, int64_t timeout_ms) {
    if (cmd.data == NULL) return rl_err(-1);
    char cbuf[cmd.len + 1];
    memcpy(cbuf, cmd.data, cmd.len);
    cbuf[cmd.len] = '\0';
    int out_pipe[2];
    if (pipe(out_pipe) != 0) {
        int e = errno;
        const char *es = strerror(e);
        int n = snprintf(NULL, 0, "exec_with_timeout: failed to run \"%.*s\": %s (os error %d)", (int)cmd.len, cmd.data, es, e);
        char *msg = malloc((uint64_t)n + 1);
        snprintf(msg, (uint64_t)n + 1, "exec_with_timeout: failed to run \"%.*s\": %s (os error %d)", (int)cmd.len, cmd.data, es, e);
        return rl_err_msg((rl_string){ .data = msg, .len = (uint64_t)n, .rc = 1 });
    }
    pid_t pid = fork();
    if (pid < 0) {
        int e = errno;
        const char *es = strerror(e);
        int n = snprintf(NULL, 0, "exec_with_timeout: failed to run \"%.*s\": %s (os error %d)", (int)cmd.len, cmd.data, es, e);
        char *msg = malloc((uint64_t)n + 1);
        snprintf(msg, (uint64_t)n + 1, "exec_with_timeout: failed to run \"%.*s\": %s (os error %d)", (int)cmd.len, cmd.data, es, e);
        close(out_pipe[0]); close(out_pipe[1]);
        return rl_err_msg((rl_string){ .data = msg, .len = (uint64_t)n, .rc = 1 });
    }
    if (pid == 0) {
        dup2(out_pipe[1], STDOUT_FILENO);
        close(out_pipe[0]); close(out_pipe[1]);
        execl("/bin/sh", "sh", "-c", cbuf, (char *)NULL);
        _exit(127);
    }
    close(out_pipe[1]);
    int64_t wait_ms = timeout_ms < 0 ? 0 : timeout_ms;
    struct timespec start;
    clock_gettime(CLOCK_MONOTONIC, &start);
    int status = 0;
    bool done = false;
    bool exited = false;
    while (!done) {
        pid_t r = waitpid(pid, &status, WNOHANG);
        if (r < 0) {
            if (errno == EINTR) continue;
            int e = errno;
            const char *es = strerror(e);
            int n = snprintf(NULL, 0, "exec_with_timeout: %s (os error %d)", es, e);
            char *msg = malloc((uint64_t)n + 1);
            snprintf(msg, (uint64_t)n + 1, "exec_with_timeout: %s (os error %d)", es, e);
            close(out_pipe[0]);
            return rl_err_msg((rl_string){ .data = msg, .len = (uint64_t)n, .rc = 1 });
        }
        if (r > 0) {
            exited = true;
            done = true;
            break;
        }
        struct timespec now;
        clock_gettime(CLOCK_MONOTONIC, &now);
        int64_t elapsed = (int64_t)(now.tv_sec - start.tv_sec) * 1000 + (int64_t)(now.tv_nsec - start.tv_nsec) / 1000000;
        if (elapsed >= wait_ms) {
            kill(pid, SIGKILL);
            while (waitpid(pid, &status, 0) < 0) {
                if (errno != EINTR) break;
            }
            close(out_pipe[0]);
            int n = snprintf(NULL, 0, "exec_with_timeout: \"%.*s\" timed out after %ldms", (int)cmd.len, cmd.data, (long)timeout_ms);
            char *msg = malloc((uint64_t)n + 1);
            snprintf(msg, (uint64_t)n + 1, "exec_with_timeout: \"%.*s\" timed out after %ldms", (int)cmd.len, cmd.data, (long)timeout_ms);
            return rl_err_msg((rl_string){ .data = msg, .len = (uint64_t)n, .rc = 1 });
        }
        usleep(50000);
    }
    (void)exited;
    uint64_t cap = 4096;
    char *out = malloc(cap);
    uint64_t len = 0;
    for (;;) {
        if (len >= cap) { cap *= 2; out = realloc(out, cap); }
        ssize_t rr = read(out_pipe[0], out + len, cap - len);
        if (rr < 0) {
            if (errno == EINTR) continue;
            break;
        }
        if (rr == 0) break;
        len += (uint64_t)rr;
    }
    close(out_pipe[0]);
    out[len] = '\0';
    while (len > 0 && out[len - 1] == '\n') len--;
    if (WIFEXITED(status)) {
        int code = WEXITSTATUS(status);
        if (code == 0) {
            rl_string result = { .data = out, .len = len, .rc = 1 };
            return rl_ok_str(result);
        }
        int n = snprintf(NULL, 0, "exec_with_timeout: \"%.*s\" exited with code %d", (int)cmd.len, cmd.data, code);
        char *msg = malloc((uint64_t)n + 1);
        snprintf(msg, (uint64_t)n + 1, "exec_with_timeout: \"%.*s\" exited with code %d", (int)cmd.len, cmd.data, code);
        free(out);
        return rl_err_msg((rl_string){ .data = msg, .len = (uint64_t)n, .rc = 1 });
    }
    int code = -1;
    int n = snprintf(NULL, 0, "exec_with_timeout: \"%.*s\" exited with code %d", (int)cmd.len, cmd.data, code);
    char *msg = malloc((uint64_t)n + 1);
    snprintf(msg, (uint64_t)n + 1, "exec_with_timeout: \"%.*s\" exited with code %d", (int)cmd.len, cmd.data, code);
    free(out);
    return rl_err_msg((rl_string){ .data = msg, .len = (uint64_t)n, .rc = 1 });
}

// Spawn exe plus args in the background; ok with pid, or an error.
rl_result rl_process_with_exec_background(rl_string exe, rl_string cmd) {
    if (exe.data == NULL || cmd.data == NULL) return rl_err(-1);
    uint64_t total = exe.len + 1 + cmd.len;
    char buf[total + 1];
    memcpy(buf, exe.data, exe.len);
    buf[exe.len] = ' ';
    memcpy(buf + exe.len + 1, cmd.data, cmd.len);
    buf[total] = '\0';
    rl_string combined = { .data = buf, .len = total, .rc = 0 };
    return rl_process_exec_background(combined);
}

// Pipe cmd1 into cmd2; ok with final stdout, or an error.
rl_result rl_process_pipe(rl_string cmd1, rl_string cmd2) {
    if (cmd1.data == NULL || cmd2.data == NULL) return rl_err(-1);
    uint64_t total = cmd1.len + 3 + cmd2.len;
    char *buf = malloc(total + 1);
    memcpy(buf, cmd1.data, cmd1.len);
    memcpy(buf + cmd1.len, " | ", 3);
    memcpy(buf + cmd1.len + 3, cmd2.data, cmd2.len);
    buf[total] = '\0';
    rl_string combined = { .data = buf, .len = total, .rc = 1 };
    rl_result r = rl_process_exec(combined);
    free(buf);
    return r;
}

// Pipe all cmds; ok with final stdout, or an error.
rl_result rl_process_pipe_all(rl_array cmds) {
    if (cmds.len == 0) {
        return rl_err_msg(rl_str_literal("pipe_all: command list is empty", 31));
    }
    if (cmds.data == NULL) return rl_err(-1);
    if (cmds.len == 1) {
        rl_string *elems = (rl_string *)cmds.data;
        return rl_process_exec(elems[0]);
    }
    rl_string *elems = (rl_string *)cmds.data;
    uint64_t total = 0;
    for (uint64_t i = 0; i < cmds.len; i++) {
        if (elems[i].data == NULL) return rl_err(-1);
        total += elems[i].len;
        if (i + 1 < cmds.len) total += 3;
    }
    char *buf = malloc(total + 1);
    uint64_t pos = 0;
    for (uint64_t i = 0; i < cmds.len; i++) {
        memcpy(buf + pos, elems[i].data, elems[i].len);
        pos += elems[i].len;
        if (i + 1 < cmds.len) {
            memcpy(buf + pos, " | ", 3);
            pos += 3;
        }
    }
    buf[total] = '\0';
    rl_string combined = { .data = buf, .len = total, .rc = 1 };
    rl_result r = rl_process_exec(combined);
    free(buf);
    return r;
}

static int _rl_stored_argc = 0;
static char **_rl_stored_argv = NULL;

// Snapshot argv at startup; generated `main` calls this first.
void rl_store_args(int argc, char **argv) {
    _rl_stored_argc = argc;
    _rl_stored_argv = argv;
    // Unbuffered stdout on TTYs: crossterm flushes after every command,
    // so frames, modals and help render immediately instead of stalling
    // in the stdio buffer. Pipes and files are line-buffered like Rust's
    // stdout, keeping newline prints ordered against stderr.
    if (isatty(STDOUT_FILENO)) {
        setvbuf(stdout, NULL, _IONBF, 0);
    } else {
        setvbuf(stdout, NULL, _IOLBF, 0);
    }
}

// Command-line arguments (excluding argv[0]) as an array of strings.
rl_array rl_process_args(void) {
    if (!_rl_stored_argv) {
        rl_array arr = { .data = NULL, .len = 0, .cap = 0, .elem_size = sizeof(rl_string), .type_tag = RL_TAG_STR };
        return arr;
    }
    uint64_t len = (uint64_t)(_rl_stored_argc > 0 ? _rl_stored_argc - 1 : 0);
    rl_string *buf = malloc(len * sizeof(rl_string));
    for (uint64_t i = 0; i < len; i++) {
        const char *s = _rl_stored_argv[i + 1];
        uint64_t slen = strlen(s);
        char *sdup = malloc(slen + 1);
        memcpy(sdup, s, slen + 1);
        buf[i] = (rl_string){ .data = sdup, .len = slen };
    }
    rl_array arr = { .data = buf, .len = len, .cap = len, .elem_size = sizeof(rl_string), .type_tag = RL_TAG_STR };
    return arr;
}

// ---- cli ----
// Mirrors `std::cli` in rl-std (hand-rolled arg parser, prompts, progress).
// The parser shares the VM's semantics exactly: spec is an array of maps
// with string values (`flag` is the string "true"/"false" because RL maps
// are homogeneous), `--name value` / `--name=value` / `-s value` forms,
// everything through the first `--` is dropped (runner preamble on the VM,
// explicit separator for compiled binaries), leftovers land in `"_"`.

// Owned copy of an rl_string as a NUL-terminated C string.
static char *_rl_cli_cstr(rl_string s) {
    char *out = malloc(s.len + 1);
    if (s.len > 0 && s.data != NULL) memcpy(out, s.data, s.len);
    out[s.len] = '\0';
    return out;
}

static rl_string _rl_cli_str_owned(const char *s, uint64_t n) {
    char *dup = malloc(n + 1);
    memcpy(dup, s, n);
    dup[n] = '\0';
    rl_string out = { .data = dup, .len = n };
    return out;
}

static rl_result _rl_cli_err(const char *msg) {
    return rl_err_msg(rl_str_literal(msg, strlen(msg)));
}

// Fetches a string field from a spec map. Missing key -> NULL (caller
// decides); present but non-string -> error message in *err_out.
static char *_rl_cli_spec_str(rl_map m, const char *key, const char **err_out) {
    char *out = NULL;
    *err_out = NULL;
    rl_string k = rl_str_literal(key, strlen(key));
    rl_result got = rl_map_get_s(m, k);
    if (!got.is_ok) return NULL;
    if (got.tag != RL_TAG_STR) {
        *err_out = "spec value must be a string";
        return NULL;
    }
    rl_string s = got.data.str;
    out = malloc(s.len + 1);
    if (s.len > 0 && s.data != NULL) memcpy(out, s.data, s.len);
    out[s.len] = '\0';
    return out;
}

typedef struct { char *name; char short_c; int has_short; int is_flag; char *def; char *help; } _rl_cli_opt;

static void _rl_cli_opts_free(_rl_cli_opt *opts, uint64_t n) {
    for (uint64_t i = 0; i < n; i++) {
        free(opts[i].name);
        free(opts[i].def);
        free(opts[i].help);
    }
    free(opts);
}

// Parses the spec array into options. Returns NULL on success, else a
// static error message (usage text is built separately).
static const char *_rl_cli_read_spec(rl_array spec, _rl_cli_opt **out_opts, uint64_t *out_n) {
    _rl_cli_opt *opts = NULL;
    uint64_t n = 0;
    rl_map *maps = (rl_map *)spec.data;
    for (uint64_t i = 0; i < spec.len; i++) {
        _rl_cli_opt o;
        memset(&o, 0, sizeof(o));
        const char *err = NULL;
        o.name = _rl_cli_spec_str(maps[i], "name", &err);
        if (err != NULL) { _rl_cli_opts_free(opts, n); return err; }
        if (o.name == NULL) { _rl_cli_opts_free(opts, n); return "spec entry is missing \"name\""; }
        if (o.name[0] == '\0' || o.name[0] == '-') { _rl_cli_opts_free(opts, n); free(o.name); return "bad option name"; }
        char *flag_w = _rl_cli_spec_str(maps[i], "flag", &err);
        if (err != NULL) { _rl_cli_opts_free(opts, n); free(o.name); return err; }
        if (flag_w != NULL) {
            if (!strcmp(flag_w, "true") || !strcmp(flag_w, "1") || !strcmp(flag_w, "yes") || !strcmp(flag_w, "y")) {
                o.is_flag = 1;
            } else if (!strcmp(flag_w, "false") || !strcmp(flag_w, "0") || !strcmp(flag_w, "no") || !strcmp(flag_w, "n")) {
                o.is_flag = 0;
            } else {
                _rl_cli_opts_free(opts, n);
                free(o.name);
                free(flag_w);
                return "flag must be \"true\" or \"false\"";
            }
            free(flag_w);
        }
        char *short_w = _rl_cli_spec_str(maps[i], "short", &err);
        if (err != NULL) { _rl_cli_opts_free(opts, n); free(o.name); return err; }
        if (short_w != NULL) {
            if (short_w[0] == '\0' || short_w[1] != '\0') {
                _rl_cli_opts_free(opts, n);
                free(o.name);
                free(short_w);
                return "short must be one character";
            }
            o.short_c = short_w[0];
            o.has_short = 1;
            free(short_w);
        }
        o.def = _rl_cli_spec_str(maps[i], "default", &err);
        if (err != NULL) { _rl_cli_opts_free(opts, n); free(o.name); return err; }
        o.help = _rl_cli_spec_str(maps[i], "help", &err);
        if (err != NULL) { _rl_cli_opts_free(opts, n); free(o.name); free(o.def); return err; }
        if (o.is_flag && o.def != NULL) {
            _rl_cli_opts_free(opts, n);
            free(o.name);
            free(o.def);
            free(o.help);
            return "flag cannot have a default";
        }
        _rl_cli_opt *grown = realloc(opts, (n + 1) * sizeof(_rl_cli_opt));
        if (grown == NULL) { _rl_cli_opts_free(opts, n); free(o.name); free(o.def); free(o.help); return "out of memory"; }
        opts = grown;
        opts[n++] = o;
    }
    *out_opts = opts;
    *out_n = n;
    return NULL;
}

// Appends `text` to a heap usage buffer (realloc-growing).
static void _rl_cli_usage_add(char **buf, uint64_t *len, uint64_t *cap, const char *text) {
    uint64_t tlen = strlen(text);
    if (*len + tlen + 1 > *cap) {
        uint64_t ncap = (*cap == 0 ? 256 : *cap * 2) + tlen;
        char *nbuf = realloc(*buf, ncap);
        if (nbuf == NULL) return;
        *buf = nbuf;
        *cap = ncap;
    }
    memcpy(*buf + *len, text, tlen);
    *len += tlen;
    (*buf)[*len] = '\0';
}

static char *_rl_cli_usage(_rl_cli_opt *opts, uint64_t n, const char *err) {
    char *buf = NULL;
    uint64_t len = 0, cap = 0;
    if (err != NULL) {
        _rl_cli_usage_add(&buf, &len, &cap, err);
        _rl_cli_usage_add(&buf, &len, &cap, "\n\n");
    }
    _rl_cli_usage_add(&buf, &len, &cap, "usage: program [options] [--] [args...]\n\noptions:\n");
    for (uint64_t i = 0; i < n; i++) {
        char line[512];
        int at = snprintf(line, sizeof(line), "  --%s", opts[i].name);
        if (opts[i].has_short) at += snprintf(line + at, sizeof(line) - (uint64_t)at, ", -%c", opts[i].short_c);
        if (opts[i].is_flag) at += snprintf(line + at, sizeof(line) - (uint64_t)at, "  (flag)");
        if (opts[i].def != NULL) at += snprintf(line + at, sizeof(line) - (uint64_t)at, "  (default: %s)", opts[i].def);
        if (opts[i].help != NULL) at += snprintf(line + at, sizeof(line) - (uint64_t)at, "  %s", opts[i].help);
        (void)at;
        _rl_cli_usage_add(&buf, &len, &cap, line);
        _rl_cli_usage_add(&buf, &len, &cap, "\n");
    }
    if (buf == NULL) {
        buf = malloc(1);
        if (buf != NULL) buf[0] = '\0';
    }
    return buf;
}

// Stores (name, value) pairs; later entries win like the VM's set_value.
typedef struct { char *name; rl_value val; } _rl_cli_kv;

static void _rl_cli_kv_set(_rl_cli_kv **kvs, uint64_t *n, uint64_t *cap, const char *name, rl_value val) {
    for (uint64_t i = 0; i < *n; i++) {
        if (!strcmp((*kvs)[i].name, name)) {
            (*kvs)[i].val = val;
            return;
        }
    }
    if (*n == *cap) {
        uint64_t ncap = (*cap == 0 ? 8 : *cap * 2);
        _rl_cli_kv *grown = realloc(*kvs, ncap * sizeof(_rl_cli_kv));
        if (grown == NULL) return;
        *kvs = grown;
        *cap = ncap;
    }
    (*kvs)[*n].name = malloc(strlen(name) + 1);
    if ((*kvs)[*n].name == NULL) return;
    memcpy((*kvs)[*n].name, name, strlen(name) + 1);
    (*kvs)[*n].val = val;
    (*n)++;
}

// Parses argv against opts. Returns NULL + sets *err_msg on failure
// (caller frees neither; messages are static or usage-owned).
static rl_map _rl_cli_parse_argv(_rl_cli_opt *opts, uint64_t nopts, const char **err_msg) {
    rl_map out = rl_map_new();
    int argc = _rl_stored_argc;
    char **argv = _rl_stored_argv;
    // Drop argv[0] plus everything through the first `--`.
    int start = 1;
    for (int i = 1; i < argc; i++) {
        if (!strcmp(argv[i], "--")) { start = i + 1; break; }
    }
    _rl_cli_kv *kvs = NULL;
    uint64_t nkvs = 0, capkvs = 0;
    int only_positional = 0;
    rl_string *pos = NULL;
    uint64_t npos = 0, cappos = 0;
    *err_msg = NULL;
    for (int i = start; i < argc; i++) {
        const char *arg = argv[i];
        if (only_positional || arg[0] != '-' || arg[1] == '\0' || !strcmp(arg, "-") || !strcmp(arg, "--")) {
            if (!strcmp(arg, "--") && !only_positional) { only_positional = 1; continue; }
            if (npos == cappos) {
                uint64_t ncap = (cappos == 0 ? 8 : cappos * 2);
                rl_string *grown = realloc(pos, ncap * sizeof(rl_string));
                if (grown == NULL) { *err_msg = "out of memory"; goto done; }
                pos = grown;
                cappos = ncap;
            }
            pos[npos++] = _rl_cli_str_owned(arg, strlen(arg));
            continue;
        }
        int is_long = (arg[0] == '-' && arg[1] == '-');
        const char *body = is_long ? arg + 2 : arg + 1;
        char key[256];
        const char *inline_val = NULL;
        if (is_long) {
            const char *eq = strchr(body, '=');
            if (eq != NULL) {
                uint64_t klen = (uint64_t)(eq - body);
                if (klen >= sizeof(key)) klen = sizeof(key) - 1;
                memcpy(key, body, klen);
                key[klen] = '\0';
                inline_val = eq + 1;
            } else {
                snprintf(key, sizeof(key), "%s", body);
            }
        } else {
            snprintf(key, sizeof(key), "%s", body);
        }
        _rl_cli_opt *opt = NULL;
        for (uint64_t k = 0; k < nopts; k++) {
            if (!strcmp(opts[k].name, key)) { opt = &opts[k]; break; }
            if (!is_long && opts[k].has_short && key[0] == opts[k].short_c && key[1] == '\0') {
                opt = &opts[k];
                break;
            }
        }
        if (opt == NULL) {
            char *use = _rl_cli_usage(opts, nopts, NULL);
            static char unknown_buf[1024];
            snprintf(unknown_buf, sizeof(unknown_buf), "unknown argument: %s\n\n%s", arg, use);
            free(use);
            *err_msg = unknown_buf;
            goto done;
        }
        if (opt->is_flag) {
            int val = 1;
            if (inline_val != NULL) {
                if (!strcmp(inline_val, "true") || !strcmp(inline_val, "1") || !strcmp(inline_val, "yes") || !strcmp(inline_val, "y")) {
                    val = 1;
                } else if (!strcmp(inline_val, "false") || !strcmp(inline_val, "0") || !strcmp(inline_val, "no") || !strcmp(inline_val, "n")) {
                    val = 0;
                } else {
                    static char flag_buf[256];
                    snprintf(flag_buf, sizeof(flag_buf), "flag --%s expects true/false, got \"%s\"", opt->name, inline_val);
                    *err_msg = flag_buf;
                    goto done;
                }
            }
            rl_value v;
            v.tag = RL_VTAG_BOOL;
            v.data.boolean = val;
            _rl_cli_kv_set(&kvs, &nkvs, &capkvs, opt->name, v);
            continue;
        }
        const char *val = inline_val;
        if (val == NULL) {
            i++;
            if (i >= argc) {
                static char need_buf[256];
                snprintf(need_buf, sizeof(need_buf), "option --%s expects a value", opt->name);
                *err_msg = need_buf;
                goto done;
            }
            val = argv[i];
        }
        rl_value v;
        v.tag = RL_VTAG_STR;
        v.data.str = _rl_cli_str_owned(val, strlen(val));
        _rl_cli_kv_set(&kvs, &nkvs, &capkvs, opt->name, v);
    }
    for (uint64_t k = 0; k < nopts; k++) {
        int seen = 0;
        for (uint64_t j = 0; j < nkvs; j++) {
            if (!strcmp(kvs[j].name, opts[k].name)) { seen = 1; break; }
        }
        if (seen) continue;
        if (opts[k].is_flag) {
            rl_value v;
            v.tag = RL_VTAG_BOOL;
            v.data.boolean = 0;
            _rl_cli_kv_set(&kvs, &nkvs, &capkvs, opts[k].name, v);
        } else if (opts[k].def != NULL) {
            rl_value v;
            v.tag = RL_VTAG_STR;
            v.data.str = _rl_cli_str_owned(opts[k].def, strlen(opts[k].def));
            _rl_cli_kv_set(&kvs, &nkvs, &capkvs, opts[k].name, v);
        } else {
            static char missing_buf[256];
            snprintf(missing_buf, sizeof(missing_buf), "missing required option: --%s", opts[k].name);
            *err_msg = missing_buf;
            goto done;
        }
    }
done:;
    rl_value parr;
    parr.tag = RL_VTAG_ARR;
    rl_string *pbuf = malloc((npos > 0 ? npos : 1) * sizeof(rl_string));
    for (uint64_t j = 0; j < npos; j++) pbuf[j] = pos[j];
    free(pos);
    rl_array pa = { .data = pbuf, .len = npos, .cap = npos, .elem_size = (int32_t)sizeof(rl_string), .type_tag = RL_TAG_STR };
    parr.data.arr = pa;
    _rl_cli_kv_set(&kvs, &nkvs, &capkvs, "_", parr);
    for (uint64_t j = 0; j < nkvs; j++) {
        rl_map_set(&out, kvs[j].name, kvs[j].val);
        free(kvs[j].name);
    }
    free(kvs);
    return out;
}

rl_result rl_cli_parse_args(rl_array spec) {
    _rl_cli_opt *opts = NULL;
    uint64_t nopts = 0;
    const char *spec_err = _rl_cli_read_spec(spec, &opts, &nopts);
    if (spec_err != NULL) return _rl_cli_err(spec_err);
    const char *err_msg = NULL;
    rl_map out = _rl_cli_parse_argv(opts, nopts, &err_msg);
    _rl_cli_opts_free(opts, nopts);
    if (err_msg != NULL) return _rl_cli_err(err_msg);
    return rl_ok_map(out);
}

rl_result rl_cli_parse_args_or_exit(rl_array spec) {
    _rl_cli_opt *opts = NULL;
    uint64_t nopts = 0;
    const char *spec_err = _rl_cli_read_spec(spec, &opts, &nopts);
    if (spec_err != NULL) {
        fprintf(stderr, "%s\n", spec_err);
        exit(2);
    }
    const char *err_msg = NULL;
    rl_map out = _rl_cli_parse_argv(opts, nopts, &err_msg);
    if (err_msg != NULL) {
        fprintf(stderr, "%s\n", err_msg);
        _rl_cli_opts_free(opts, nopts);
        exit(2);
    }
    _rl_cli_opts_free(opts, nopts);
    return rl_ok_map(out);
}

rl_result rl_cli_usage(rl_array spec) {
    _rl_cli_opt *opts = NULL;
    uint64_t nopts = 0;
    const char *spec_err = _rl_cli_read_spec(spec, &opts, &nopts);
    if (spec_err != NULL) return _rl_cli_err(spec_err);
    char *text = _rl_cli_usage(opts, nopts, NULL);
    _rl_cli_opts_free(opts, nopts);
    rl_string s = _rl_cli_str_owned(text, strlen(text));
    free(text);
    return rl_ok_str(s);
}

// ---- cli prompts ----

static char *_rl_cli_read_line(FILE *in, FILE *out, const char *prompt_text) {
    if (prompt_text != NULL) {
        fputs(prompt_text, out);
        fflush(out);
    }
    char *line = NULL;
    size_t cap = 0;
    ssize_t n = getline(&line, &cap, in);
    if (n < 0) {
        free(line);
        line = malloc(1);
        if (line != NULL) line[0] = '\0';
        return line;
    }
    while (n > 0 && (line[n - 1] == '\n' || line[n - 1] == '\r')) line[--n] = '\0';
    return line;
}

rl_string rl_cli_prompt(rl_string msg) {
    char *m = _rl_cli_cstr(msg);
    char *line = _rl_cli_read_line(stdin, stdout, m);
    free(m);
    rl_string s = _rl_cli_str_owned(line, strlen(line));
    free(line);
    return s;
}

rl_string rl_cli_prompt_password(rl_string msg) {
    // No-echo read, mirroring rpassword: /dev/tty only. Without a
    // controlling terminal there is no prompt and no read (rpassword
    // surfaces this as an error, which the VM maps to "").
    FILE *tty = fopen("/dev/tty", "r+");
    if (tty == NULL) return _rl_cli_str_owned("", 0);
    char *m = _rl_cli_cstr(msg);
    fputs(m, tty);
    fflush(tty);
    free(m);
    struct termios oldt, newt;
    int echoed = 0;
    int fd = fileno(tty);
    if (tcgetattr(fd, &oldt) == 0) {
        newt = oldt;
        newt.c_lflag &= (tcflag_t)~ECHO;
        if (tcsetattr(fd, TCSANOW, &newt) == 0) echoed = 1;
    }
    char *line = _rl_cli_read_line(tty, tty, NULL);
    if (echoed) {
        tcsetattr(fd, TCSANOW, &oldt);
        fputc('\n', tty);
        fflush(tty);
    }
    fclose(tty);
    rl_string s = _rl_cli_str_owned(line, strlen(line));
    free(line);
    return s;
}

bool rl_cli_prompt_confirm(rl_string msg) {
    char *m = _rl_cli_cstr(msg);
    char full[1024];
    snprintf(full, sizeof(full), "%s [y/n] ", m);
    free(m);
    char *line = _rl_cli_read_line(stdin, stdout, full);
    for (char *p = line; *p != '\0'; p++) {
        if (*p >= 'A' && *p <= 'Z') *p = (char)(*p + ('a' - 'A'));
    }
    int yes = (!strcmp(line, "y") || !strcmp(line, "yes"));
    free(line);
    return yes;
}

rl_string rl_cli_prompt_choice(rl_string msg, rl_array options) {
    if (options.len == 0) return _rl_cli_str_owned("", 0);
    rl_string *items = (rl_string *)options.data;
    char *m = _rl_cli_cstr(msg);
    for (;;) {
        printf("%s\n", m);
        for (uint64_t i = 0; i < options.len; i++) {
            char *text = _rl_cli_cstr(items[i]);
            printf("  %llu. %s\n", (unsigned long long)(i + 1), text);
            free(text);
        }
        fflush(stdout);
        char *line = _rl_cli_read_line(stdin, stdout, NULL);
        if (line[0] == '\0' && feof(stdin)) {
            free(line);
            free(m);
            return _rl_cli_str_owned("", 0);
        }
        char *end = NULL;
        unsigned long pick = strtoul(line, &end, 10);
        if (end != line && *end == '\0' && pick >= 1 && pick <= options.len) {
            free(line);
            free(m);
            rl_string s = items[pick - 1];
            return _rl_cli_str_owned(s.data, s.len);
        }
        int exact = 0;
        for (uint64_t i = 0; i < options.len; i++) {
            char *text = _rl_cli_cstr(items[i]);
            if (!strcmp(text, line)) exact = 1;
            free(text);
            if (exact) break;
        }
        if (exact) {
            rl_string s = _rl_cli_str_owned(line, strlen(line));
            free(line);
            free(m);
            return s;
        }
        free(line);
        printf("pick 1-%llu or one of the listed values\n", (unsigned long long)options.len);
    }
}

// ---- cli shell words ----
// Mirrors shell-words: SQL-style single quotes (literal), double quotes
// (backslash escapes $, `, ", \ and newline), backslash escapes any char
// outside quotes, whitespace separates. Unterminated quotes are errors.

static int _rl_cli_shell_push(char **buf, uint64_t *len, uint64_t *cap, char c) {
    if (*len + 1 >= *cap) {
        uint64_t ncap = (*cap == 0 ? 32 : *cap * 2);
        char *grown = realloc(*buf, ncap);
        if (grown == NULL) return -1;
        *buf = grown;
        *cap = ncap;
    }
    (*buf)[(*len)++] = c;
    return 0;
}

rl_result rl_cli_shell_split(rl_string s) {
    const char *p = (s.data != NULL) ? s.data : "";
    const char *end = p + s.len;
    rl_string *parts = NULL;
    uint64_t nparts = 0, capparts = 0;
    while (p < end) {
        while (p < end && (*p == ' ' || *p == '\t' || *p == '\n' || *p == '\r')) p++;
        if (p >= end) break;
        char *word = NULL;
        uint64_t wlen = 0, wcap = 0;
        int closed = 1;
        while (p < end && *p != ' ' && *p != '\t' && *p != '\n' && *p != '\r') {
            if (*p == '\'') {
                closed = 0;
                p++;
                while (p < end && *p != '\'') {
                    if (_rl_cli_shell_push(&word, &wlen, &wcap, *p)) goto oom;
                    p++;
                }
                if (p >= end) goto unterminated;
                closed = 1;
                p++;
            } else if (*p == '"') {
                closed = 0;
                p++;
                while (p < end && *p != '"') {
                    if (*p == '\\' && p + 1 < end
                        && (p[1] == '$' || p[1] == '`' || p[1] == '"' || p[1] == '\\' || p[1] == '\n')) {
                        p++;
                        if (_rl_cli_shell_push(&word, &wlen, &wcap, *p)) goto oom;
                        p++;
                    } else {
                        if (_rl_cli_shell_push(&word, &wlen, &wcap, *p)) goto oom;
                        p++;
                    }
                }
                if (p >= end) goto unterminated;
                closed = 1;
                p++;
            } else if (*p == '\\' && p + 1 < end) {
                p++;
                if (_rl_cli_shell_push(&word, &wlen, &wcap, *p)) goto oom;
                p++;
            } else {
                if (_rl_cli_shell_push(&word, &wlen, &wcap, *p)) goto oom;
                p++;
            }
        }
        (void)closed;
        if (nparts == capparts) {
            uint64_t ncap = (capparts == 0 ? 8 : capparts * 2);
            rl_string *grown = realloc(parts, ncap * sizeof(rl_string));
            if (grown == NULL) goto oom;
            parts = grown;
            capparts = ncap;
        }
        char *wdup = malloc(wlen + 1);
        if (wdup == NULL) goto oom;
        if (wlen > 0) memcpy(wdup, word, wlen);
        wdup[wlen] = '\0';
        free(word);
        word = NULL;
        parts[nparts++] = (rl_string){ .data = wdup, .len = wlen };
        continue;
    oom:
        free(word);
        for (uint64_t i = 0; i < nparts; i++) free((void *)parts[i].data);
        free(parts);
        return _rl_cli_err("shell_split: out of memory");
    unterminated:
        free(word);
        for (uint64_t i = 0; i < nparts; i++) free((void *)parts[i].data);
        free(parts);
        return _rl_cli_err("shell_split: unterminated quote");
    }
    rl_array arr = { .data = parts, .len = nparts, .cap = capparts,
        .elem_size = (int32_t)sizeof(rl_string), .type_tag = RL_TAG_STR };
    return rl_ok_arr(arr);
}

static int _rl_cli_needs_quote(const char *s, uint64_t n) {
    if (n == 0) return 1;
    for (uint64_t i = 0; i < n; i++) {
        char c = s[i];
        if (c == ' ' || c == '\t' || c == '\n' || c == '\'' || c == '"' || c == '\\'
            || c == '$' || c == '`' || c == '!' || c == '(' || c == ')' || c == '&'
            || c == '|' || c == ';' || c == '<' || c == '>' || c == '*' || c == '?'
            || c == '#' || c == '~') {
            return 1;
        }
    }
    return 0;
}

rl_string rl_cli_shell_join(rl_array parts) {
    rl_string *items = (rl_string *)parts.data;
    char *out = NULL;
    uint64_t len = 0, cap = 0;
    for (uint64_t i = 0; i < parts.len; i++) {
        if (i > 0) {
            if (_rl_cli_shell_push(&out, &len, &cap, ' ')) break;
        }
        const char *s = (items[i].data != NULL) ? items[i].data : "";
        uint64_t n = items[i].len;
        if (!_rl_cli_needs_quote(s, n)) {
            for (uint64_t k = 0; k < n; k++) {
                if (_rl_cli_shell_push(&out, &len, &cap, s[k])) break;
            }
        } else {
            if (_rl_cli_shell_push(&out, &len, &cap, '\'')) break;
            for (uint64_t k = 0; k < n; k++) {
                if (s[k] == '\'') {
                    const char *esc = "'\\''";
                    for (int e = 0; e < 4; e++) {
                        if (_rl_cli_shell_push(&out, &len, &cap, esc[e])) break;
                    }
                } else {
                    if (_rl_cli_shell_push(&out, &len, &cap, s[k])) break;
                }
            }
            _rl_cli_shell_push(&out, &len, &cap, '\'');
        }
    }
    if (out == NULL) {
        out = malloc(1);
        if (out != NULL) out[0] = '\0';
    } else {
        if (_rl_cli_shell_push(&out, &len, &cap, '\0')) {
            out[len] = '\0';
        }
        len = strlen(out);
    }
    rl_string r = { .data = out, .len = len };
    return r;
}

// ---- cli editable input ----
// Reduced scope (documented): plain line reads with history threading,
// no arrow-key editing. A vendored line editor (linenoise-style) is the
// follow-up for full rustyline parity.

typedef struct { rl_string field_0; rl_array field_1; } _rl_tuple_sarr;

static rl_result _rl_ok_tuple_sarr(rl_string line, rl_array hist) {
    _rl_tuple_sarr *slot = malloc(sizeof(_rl_tuple_sarr));
    slot->field_0 = line;
    slot->field_1 = hist;
    rl_array out;
    out.data = slot;
    out.len = 1;
    out.cap = 1;
    out.elem_size = (int32_t)sizeof(_rl_tuple_sarr);
    out.type_tag = RL_TAG_I64;
    return rl_ok_arr(out);
}

rl_string rl_cli_read_line_editable(rl_string msg) {
    // rustyline prints no prompt on non-tty stdin; mirror that.
    char *m = isatty(STDIN_FILENO) ? _rl_cli_cstr(msg) : NULL;
    char *line = _rl_cli_read_line(stdin, stdout, m);
    free(m);
    rl_string s = _rl_cli_str_owned(line, strlen(line));
    free(line);
    return s;
}

rl_result rl_cli_read_line_with_history(rl_string msg, rl_array history) {
    char *m = isatty(STDIN_FILENO) ? _rl_cli_cstr(msg) : NULL;
    char *line = _rl_cli_read_line(stdin, stdout, m);
    free(m);
    uint64_t hlen = strlen(line) > 0 ? history.len + 1 : history.len;
    rl_string *buf = malloc((hlen > 0 ? hlen : 1) * sizeof(rl_string));
    rl_string *items = (rl_string *)history.data;
    for (uint64_t i = 0; i < history.len; i++) {
        buf[i] = _rl_cli_str_owned(items[i].data, items[i].len);
    }
    if (strlen(line) > 0) {
        buf[history.len] = _rl_cli_str_owned(line, strlen(line));
    }
    rl_string ls = _rl_cli_str_owned(line, strlen(line));
    free(line);
    rl_array hist = { .data = buf, .len = hlen, .cap = hlen,
        .elem_size = (int32_t)sizeof(rl_string), .type_tag = RL_TAG_STR };
    return _rl_ok_tuple_sarr(ls, hist);
}

// ---- crypto ----
// Mirrors `std::crypto`. Compact from-spec hash implementations; every
// round constant below was verified against the sha2/sha1/md-5 crate
// sources. Byte arrays are int64-element arrays holding 0-255 (the
// transpiler-wide convention, matching rl_io_read_bytes). Out-of-range
// inputs abort, mirroring the VM's loud runtime type error.

static uint32_t _rl_rotr32(uint32_t x, unsigned n) { return (x >> n) | (x << (32 - n)); }
static uint64_t _rl_rotr64(uint64_t x, unsigned n) { return (x >> n) | (x << (64 - n)); }

// ---- SHA-256 ----

static const uint32_t _RL_SHA256_K[64] = {
    0x428a2f98, 0x71374491, 0xb5c0fbcf, 0xe9b5dba5, 0x3956c25b, 0x59f111f1, 0x923f82a4, 0xab1c5ed5,
    0xd807aa98, 0x12835b01, 0x243185be, 0x550c7dc3, 0x72be5d74, 0x80deb1fe, 0x9bdc06a7, 0xc19bf174,
    0xe49b69c1, 0xefbe4786, 0x0fc19dc6, 0x240ca1cc, 0x2de92c6f, 0x4a7484aa, 0x5cb0a9dc, 0x76f988da,
    0x983e5152, 0xa831c66d, 0xb00327c8, 0xbf597fc7, 0xc6e00bf3, 0xd5a79147, 0x06ca6351, 0x14292967,
    0x27b70a85, 0x2e1b2138, 0x4d2c6dfc, 0x53380d13, 0x650a7354, 0x766a0abb, 0x81c2c92e, 0x92722c85,
    0xa2bfe8a1, 0xa81a664b, 0xc24b8b70, 0xc76c51a3, 0xd192e819, 0xd6990624, 0xf40e3585, 0x106aa070,
    0x19a4c116, 0x1e376c08, 0x2748774c, 0x34b0bcb5, 0x391c0cb3, 0x4ed8aa4a, 0x5b9cca4f, 0x682e6ff3,
    0x748f82ee, 0x78a5636f, 0x84c87814, 0x8cc70208, 0x90befffa, 0xa4506ceb, 0xbef9a3f7, 0xc67178f2,
};

typedef struct { uint32_t h[8]; uint64_t len; uint8_t buf[64]; size_t buflen; } _rl_sha256_ctx;

static void _rl_sha256_init(_rl_sha256_ctx *c) {
    c->h[0] = 0x6a09e667; c->h[1] = 0xbb67ae85; c->h[2] = 0x3c6ef372; c->h[3] = 0xa54ff53a;
    c->h[4] = 0x510e527f; c->h[5] = 0x9b05688c; c->h[6] = 0x1f83d9ab; c->h[7] = 0x5be0cd19;
    c->len = 0;
    c->buflen = 0;
}

static void _rl_sha256_block(_rl_sha256_ctx *c, const uint8_t *p) {
    uint32_t w[64];
    for (int i = 0; i < 16; i++) {
        w[i] = ((uint32_t)p[4 * i] << 24) | ((uint32_t)p[4 * i + 1] << 16)
            | ((uint32_t)p[4 * i + 2] << 8) | (uint32_t)p[4 * i + 3];
    }
    for (int i = 16; i < 64; i++) {
        uint32_t s0 = _rl_rotr32(w[i - 15], 7) ^ _rl_rotr32(w[i - 15], 18) ^ (w[i - 15] >> 3);
        uint32_t s1 = _rl_rotr32(w[i - 2], 17) ^ _rl_rotr32(w[i - 2], 19) ^ (w[i - 2] >> 10);
        w[i] = w[i - 16] + s0 + w[i - 7] + s1;
    }
    uint32_t a = c->h[0], b = c->h[1], c2 = c->h[2], d = c->h[3];
    uint32_t e = c->h[4], f = c->h[5], g = c->h[6], h = c->h[7];
    for (int i = 0; i < 64; i++) {
        uint32_t S1 = _rl_rotr32(e, 6) ^ _rl_rotr32(e, 11) ^ _rl_rotr32(e, 25);
        uint32_t ch = (e & f) ^ (~e & g);
        uint32_t t1 = h + S1 + ch + _RL_SHA256_K[i] + w[i];
        uint32_t S0 = _rl_rotr32(a, 2) ^ _rl_rotr32(a, 13) ^ _rl_rotr32(a, 22);
        uint32_t mj = (a & b) ^ (a & c2) ^ (b & c2);
        uint32_t t2 = S0 + mj;
        h = g; g = f; f = e; e = d + t1; d = c2; c2 = b; b = a; a = t1 + t2;
    }
    c->h[0] += a; c->h[1] += b; c->h[2] += c2; c->h[3] += d;
    c->h[4] += e; c->h[5] += f; c->h[6] += g; c->h[7] += h;
}

static void _rl_sha256_update(_rl_sha256_ctx *c, const uint8_t *data, size_t n) {
    c->len += (uint64_t)n;
    while (n > 0) {
        size_t take = 64 - c->buflen;
        if (take > n) take = n;
        memcpy(c->buf + c->buflen, data, take);
        c->buflen += take;
        data += take;
        n -= take;
        if (c->buflen == 64) {
            _rl_sha256_block(c, c->buf);
            c->buflen = 0;
        }
    }
}

static void _rl_sha256_final(_rl_sha256_ctx *c, uint8_t out[32]) {
    uint64_t bitlen = c->len * 8;
    uint8_t pad = 0x80;
    _rl_sha256_update(c, &pad, 1);
    uint8_t zero = 0;
    while (c->buflen != 56) _rl_sha256_update(c, &zero, 1);
    uint8_t lenbuf[8];
    for (int i = 0; i < 8; i++) lenbuf[i] = (uint8_t)(bitlen >> (56 - 8 * i));
    // Bypass update (length must NOT include padding): fill + compress.
    memcpy(c->buf + 56, lenbuf, 8);
    _rl_sha256_block(c, c->buf);
    for (int i = 0; i < 8; i++) {
        out[4 * i] = (uint8_t)(c->h[i] >> 24);
        out[4 * i + 1] = (uint8_t)(c->h[i] >> 16);
        out[4 * i + 2] = (uint8_t)(c->h[i] >> 8);
        out[4 * i + 3] = (uint8_t)c->h[i];
    }
}

// ---- SHA-512 ----

static const uint64_t _RL_SHA512_K[80] = {
    0x428a2f98d728ae22ULL, 0x7137449123ef65cdULL, 0xb5c0fbcfec4d3b2fULL, 0xe9b5dba58189dbbcULL,
    0x3956c25bf348b538ULL, 0x59f111f1b605d019ULL, 0x923f82a4af194f9bULL, 0xab1c5ed5da6d8118ULL,
    0xd807aa98a3030242ULL, 0x12835b0145706fbeULL, 0x243185be4ee4b28cULL, 0x550c7dc3d5ffb4e2ULL,
    0x72be5d74f27b896fULL, 0x80deb1fe3b1696b1ULL, 0x9bdc06a725c71235ULL, 0xc19bf174cf692694ULL,
    0xe49b69c19ef14ad2ULL, 0xefbe4786384f25e3ULL, 0x0fc19dc68b8cd5b5ULL, 0x240ca1cc77ac9c65ULL,
    0x2de92c6f592b0275ULL, 0x4a7484aa6ea6e483ULL, 0x5cb0a9dcbd41fbd4ULL, 0x76f988da831153b5ULL,
    0x983e5152ee66dfabULL, 0xa831c66d2db43210ULL, 0xb00327c898fb213fULL, 0xbf597fc7beef0ee4ULL,
    0xc6e00bf33da88fc2ULL, 0xd5a79147930aa725ULL, 0x06ca6351e003826fULL, 0x142929670a0e6e70ULL,
    0x27b70a8546d22ffcULL, 0x2e1b21385c26c926ULL, 0x4d2c6dfc5ac42aedULL, 0x53380d139d95b3dfULL,
    0x650a73548baf63deULL, 0x766a0abb3c77b2a8ULL, 0x81c2c92e47edaee6ULL, 0x92722c851482353bULL,
    0xa2bfe8a14cf10364ULL, 0xa81a664bbc423001ULL, 0xc24b8b70d0f89791ULL, 0xc76c51a30654be30ULL,
    0xd192e819d6ef5218ULL, 0xd69906245565a910ULL, 0xf40e35855771202aULL, 0x106aa07032bbd1b8ULL,
    0x19a4c116b8d2d0c8ULL, 0x1e376c085141ab53ULL, 0x2748774cdf8eeb99ULL, 0x34b0bcb5e19b48a8ULL,
    0x391c0cb3c5c95a63ULL, 0x4ed8aa4ae3418acbULL, 0x5b9cca4f7763e373ULL, 0x682e6ff3d6b2b8a3ULL,
    0x748f82ee5defb2fcULL, 0x78a5636f43172f60ULL, 0x84c87814a1f0ab72ULL, 0x8cc702081a6439ecULL,
    0x90befffa23631e28ULL, 0xa4506cebde82bde9ULL, 0xbef9a3f7b2c67915ULL, 0xc67178f2e372532bULL,
    0xca273eceea26619cULL, 0xd186b8c721c0c207ULL, 0xeada7dd6cde0eb1eULL, 0xf57d4f7fee6ed178ULL,
    0x06f067aa72176fbaULL, 0x0a637dc5a2c898a6ULL, 0x113f9804bef90daeULL, 0x1b710b35131c471bULL,
    0x28db77f523047d84ULL, 0x32caab7b40c72493ULL, 0x3c9ebe0a15c9bebcULL, 0x431d67c49c100d4cULL,
    0x4cc5d4becb3e42b6ULL, 0x597f299cfc657e2aULL, 0x5fcb6fab3ad6faecULL, 0x6c44198c4a475817ULL,
};

typedef struct { uint64_t h[8]; uint64_t len; uint8_t buf[128]; size_t buflen; } _rl_sha512_ctx;

static void _rl_sha512_init(_rl_sha512_ctx *c) {
    c->h[0] = 0x6a09e667f3bcc908ULL; c->h[1] = 0xbb67ae8584caa73bULL;
    c->h[2] = 0x3c6ef372fe94f82bULL; c->h[3] = 0xa54ff53a5f1d36f1ULL;
    c->h[4] = 0x510e527fade682d1ULL; c->h[5] = 0x9b05688c2b3e6c1fULL;
    c->h[6] = 0x1f83d9abfb41bd6bULL; c->h[7] = 0x5be0cd19137e2179ULL;
    c->len = 0;
    c->buflen = 0;
}

static void _rl_sha512_block(_rl_sha512_ctx *c, const uint8_t *p) {
    uint64_t w[80];
    for (int i = 0; i < 16; i++) {
        w[i] = ((uint64_t)p[8 * i] << 56) | ((uint64_t)p[8 * i + 1] << 48)
            | ((uint64_t)p[8 * i + 2] << 40) | ((uint64_t)p[8 * i + 3] << 32)
            | ((uint64_t)p[8 * i + 4] << 24) | ((uint64_t)p[8 * i + 5] << 16)
            | ((uint64_t)p[8 * i + 6] << 8) | (uint64_t)p[8 * i + 7];
    }
    for (int i = 16; i < 80; i++) {
        uint64_t s0 = _rl_rotr64(w[i - 15], 1) ^ _rl_rotr64(w[i - 15], 8) ^ (w[i - 15] >> 7);
        uint64_t s1 = _rl_rotr64(w[i - 2], 19) ^ _rl_rotr64(w[i - 2], 61) ^ (w[i - 2] >> 6);
        w[i] = w[i - 16] + s0 + w[i - 7] + s1;
    }
    uint64_t a = c->h[0], b = c->h[1], c2 = c->h[2], d = c->h[3];
    uint64_t e = c->h[4], f = c->h[5], g = c->h[6], h = c->h[7];
    for (int i = 0; i < 80; i++) {
        uint64_t S1 = _rl_rotr64(e, 14) ^ _rl_rotr64(e, 18) ^ _rl_rotr64(e, 41);
        uint64_t ch = (e & f) ^ (~e & g);
        uint64_t t1 = h + S1 + ch + _RL_SHA512_K[i] + w[i];
        uint64_t S0 = _rl_rotr64(a, 28) ^ _rl_rotr64(a, 34) ^ _rl_rotr64(a, 39);
        uint64_t mj = (a & b) ^ (a & c2) ^ (b & c2);
        uint64_t t2 = S0 + mj;
        h = g; g = f; f = e; e = d + t1; d = c2; c2 = b; b = a; a = t1 + t2;
    }
    c->h[0] += a; c->h[1] += b; c->h[2] += c2; c->h[3] += d;
    c->h[4] += e; c->h[5] += f; c->h[6] += g; c->h[7] += h;
}

static void _rl_sha512_update(_rl_sha512_ctx *c, const uint8_t *data, size_t n) {
    c->len += (uint64_t)n;
    while (n > 0) {
        size_t take = 128 - c->buflen;
        if (take > n) take = n;
        memcpy(c->buf + c->buflen, data, take);
        c->buflen += take;
        data += take;
        n -= take;
        if (c->buflen == 128) {
            _rl_sha512_block(c, c->buf);
            c->buflen = 0;
        }
    }
}

static void _rl_sha512_final(_rl_sha512_ctx *c, uint8_t out[64]) {
    // 128-bit length: high 64 bits are zero for any real input.
    uint64_t bitlen = c->len * 8;
    uint8_t pad = 0x80;
    _rl_sha512_update(c, &pad, 1);
    uint8_t zero = 0;
    while (c->buflen != 112) _rl_sha512_update(c, &zero, 1);
    uint8_t lenbuf[16];
    memset(lenbuf, 0, 8);
    for (int i = 0; i < 8; i++) lenbuf[8 + i] = (uint8_t)(bitlen >> (56 - 8 * i));
    memcpy(c->buf + 112, lenbuf, 16);
    _rl_sha512_block(c, c->buf);
    for (int i = 0; i < 8; i++) {
        for (int k = 0; k < 8; k++) out[8 * i + k] = (uint8_t)(c->h[i] >> (56 - 8 * k));
    }
}

// ---- SHA-1 ----

typedef struct { uint32_t h[5]; uint64_t len; uint8_t buf[64]; size_t buflen; } _rl_sha1_ctx;

static void _rl_sha1_init(_rl_sha1_ctx *c) {
    c->h[0] = 0x67452301; c->h[1] = 0xefcdab89; c->h[2] = 0x98badcfe;
    c->h[3] = 0x10325476; c->h[4] = 0xc3d2e1f0;
    c->len = 0;
    c->buflen = 0;
}

static void _rl_sha1_block(_rl_sha1_ctx *c, const uint8_t *p) {
    uint32_t w[80];
    for (int i = 0; i < 16; i++) {
        w[i] = ((uint32_t)p[4 * i] << 24) | ((uint32_t)p[4 * i + 1] << 16)
            | ((uint32_t)p[4 * i + 2] << 8) | (uint32_t)p[4 * i + 3];
    }
    for (int i = 16; i < 80; i++) {
        w[i] = _rl_rotr32(w[i - 3] ^ w[i - 8] ^ w[i - 14] ^ w[i - 16], 31);
    }
    uint32_t a = c->h[0], b = c->h[1], cc = c->h[2], d = c->h[3], e = c->h[4];
    for (int i = 0; i < 80; i++) {
        uint32_t f, k;
        if (i < 20) { f = (b & cc) | (~b & d); k = 0x5a827999; }
        else if (i < 40) { f = b ^ cc ^ d; k = 0x6ed9eba1; }
        else if (i < 60) { f = (b & cc) | (b & d) | (cc & d); k = 0x8f1bbcdc; }
        else { f = b ^ cc ^ d; k = 0xca62c1d6; }
        uint32_t t = _rl_rotr32(a, 27) + f + e + k + w[i];
        e = d; d = cc; cc = _rl_rotr32(b, 2); b = a; a = t;
    }
    c->h[0] += a; c->h[1] += b; c->h[2] += cc; c->h[3] += d; c->h[4] += e;
}

static void _rl_sha1_update(_rl_sha1_ctx *c, const uint8_t *data, size_t n) {
    c->len += (uint64_t)n;
    while (n > 0) {
        size_t take = 64 - c->buflen;
        if (take > n) take = n;
        memcpy(c->buf + c->buflen, data, take);
        c->buflen += take;
        data += take;
        n -= take;
        if (c->buflen == 64) {
            _rl_sha1_block(c, c->buf);
            c->buflen = 0;
        }
    }
}

static void _rl_sha1_final(_rl_sha1_ctx *c, uint8_t out[20]) {
    uint64_t bitlen = c->len * 8;
    uint8_t pad = 0x80;
    _rl_sha1_update(c, &pad, 1);
    uint8_t zero = 0;
    while (c->buflen != 56) _rl_sha1_update(c, &zero, 1);
    uint8_t lenbuf[8];
    for (int i = 0; i < 8; i++) lenbuf[i] = (uint8_t)(bitlen >> (56 - 8 * i));
    memcpy(c->buf + 56, lenbuf, 8);
    _rl_sha1_block(c, c->buf);
    for (int i = 0; i < 5; i++) {
        out[4 * i] = (uint8_t)(c->h[i] >> 24);
        out[4 * i + 1] = (uint8_t)(c->h[i] >> 16);
        out[4 * i + 2] = (uint8_t)(c->h[i] >> 8);
        out[4 * i + 3] = (uint8_t)c->h[i];
    }
}

// ---- MD5 ----

static const uint32_t _RL_MD5_K[64] = {
    0xd76aa478, 0xe8c7b756, 0x242070db, 0xc1bdceee, 0xf57c0faf, 0x4787c62a, 0xa8304613, 0xfd469501,
    0x698098d8, 0x8b44f7af, 0xffff5bb1, 0x895cd7be, 0x6b901122, 0xfd987193, 0xa679438e, 0x49b40821,
    0xf61e2562, 0xc040b340, 0x265e5a51, 0xe9b6c7aa, 0xd62f105d, 0x02441453, 0xd8a1e681, 0xe7d3fbc8,
    0x21e1cde6, 0xc33707d6, 0xf4d50d87, 0x455a14ed, 0xa9e3e905, 0xfcefa3f8, 0x676f02d9, 0x8d2a4c8a,
    0xfffa3942, 0x8771f681, 0x6d9d6122, 0xfde5380c, 0xa4beea44, 0x4bdecfa9, 0xf6bb4b60, 0xbebfbc70,
    0x289b7ec6, 0xeaa127fa, 0xd4ef3085, 0x04881d05, 0xd9d4d039, 0xe6db99e5, 0x1fa27cf8, 0xc4ac5665,
    0xf4292244, 0x432aff97, 0xab9423a7, 0xfc93a039, 0x655b59c3, 0x8f0ccc92, 0xffeff47d, 0x85845dd1,
    0x6fa87e4f, 0xfe2ce6e0, 0xa3014314, 0x4e0811a1, 0xf7537e82, 0xbd3af235, 0x2ad7d2bb, 0xeb86d391,
};

static const uint8_t _RL_MD5_S[64] = {
    7, 12, 17, 22, 7, 12, 17, 22, 7, 12, 17, 22, 7, 12, 17, 22,
    5, 9, 14, 20, 5, 9, 14, 20, 5, 9, 14, 20, 5, 9, 14, 20,
    4, 11, 16, 23, 4, 11, 16, 23, 4, 11, 16, 23, 4, 11, 16, 23,
    6, 10, 15, 21, 6, 10, 15, 21, 6, 10, 15, 21, 6, 10, 15, 21,
};

typedef struct { uint32_t h[4]; uint64_t len; uint8_t buf[64]; size_t buflen; } _rl_md5_ctx;

static void _rl_md5_init(_rl_md5_ctx *c) {
    c->h[0] = 0x67452301; c->h[1] = 0xefcdab89; c->h[2] = 0x98badcfe; c->h[3] = 0x10325476;
    c->len = 0;
    c->buflen = 0;
}

static void _rl_md5_block(_rl_md5_ctx *c, const uint8_t *p) {
    uint32_t m[16];
    for (int i = 0; i < 16; i++) {
        m[i] = (uint32_t)p[4 * i] | ((uint32_t)p[4 * i + 1] << 8)
            | ((uint32_t)p[4 * i + 2] << 16) | ((uint32_t)p[4 * i + 3] << 24);
    }
    uint32_t a = c->h[0], b = c->h[1], cc = c->h[2], d = c->h[3];
    for (int i = 0; i < 64; i++) {
        uint32_t f;
        unsigned g;
        if (i < 16) { f = (b & cc) | (~b & d); g = (unsigned)i; }
        else if (i < 32) { f = (d & b) | (~d & cc); g = (unsigned)((5 * i + 1) % 16); }
        else if (i < 48) { f = b ^ cc ^ d; g = (unsigned)((3 * i + 5) % 16); }
        else { f = cc ^ (b | ~d); g = (unsigned)((7 * i) % 16); }
        f = f + a + _RL_MD5_K[i] + m[g];
        a = d; d = cc; cc = b;
        b = b + _rl_rotr32(f, 32 - _RL_MD5_S[i]);
    }
    c->h[0] += a; c->h[1] += b; c->h[2] += cc; c->h[3] += d;
}

static void _rl_md5_update(_rl_md5_ctx *c, const uint8_t *data, size_t n) {
    c->len += (uint64_t)n;
    while (n > 0) {
        size_t take = 64 - c->buflen;
        if (take > n) take = n;
        memcpy(c->buf + c->buflen, data, take);
        c->buflen += take;
        data += take;
        n -= take;
        if (c->buflen == 64) {
            _rl_md5_block(c, c->buf);
            c->buflen = 0;
        }
    }
}

static void _rl_md5_final(_rl_md5_ctx *c, uint8_t out[16]) {
    uint64_t bitlen = c->len * 8;
    uint8_t pad = 0x80;
    _rl_md5_update(c, &pad, 1);
    uint8_t zero = 0;
    while (c->buflen != 56) _rl_md5_update(c, &zero, 1);
    // Little-endian length, bypassing update like the others.
    for (int i = 0; i < 8; i++) c->buf[56 + i] = (uint8_t)(bitlen >> (8 * i));
    _rl_md5_block(c, c->buf);
    for (int i = 0; i < 4; i++) {
        out[4 * i] = (uint8_t)c->h[i];
        out[4 * i + 1] = (uint8_t)(c->h[i] >> 8);
        out[4 * i + 2] = (uint8_t)(c->h[i] >> 16);
        out[4 * i + 3] = (uint8_t)(c->h[i] >> 24);
    }
}

// ---- crypto HMAC ----
// Generic HMAC over the init/update/final triples above. Keys longer
// than the block hash down first, exactly like the hmac crate.

typedef struct {
    void (*init)(void *ctx);
    void (*update)(void *ctx, const uint8_t *data, size_t n);
    void (*final)(void *ctx, uint8_t *out);
    size_t ctx_size;
    size_t block_size;
    size_t out_len;
} _rl_hash_ops;

static void _rl_hmac(const _rl_hash_ops *ops, const uint8_t *key, size_t keylen,
        const uint8_t *data, size_t datalen, uint8_t *out) {
    uint8_t keybuf[128];
    memset(keybuf, 0, ops->block_size);
    if (keylen > ops->block_size) {
        uint8_t condensed[64];
        uint8_t cbuf[256];
        ops->init(cbuf);
        ops->update(cbuf, key, keylen);
        ops->final(cbuf, condensed);
        memcpy(keybuf, condensed, ops->out_len);
    } else {
        memcpy(keybuf, key, keylen);
    }
    uint8_t ipad[128], opad[128];
    for (size_t i = 0; i < ops->block_size; i++) {
        ipad[i] = keybuf[i] ^ 0x36;
        opad[i] = keybuf[i] ^ 0x5c;
    }
    uint8_t inner[64];
    uint8_t cbuf[256];
    ops->init(cbuf);
    ops->update(cbuf, ipad, ops->block_size);
    ops->update(cbuf, data, datalen);
    ops->final(cbuf, inner);
    ops->init(cbuf);
    ops->update(cbuf, opad, ops->block_size);
    ops->update(cbuf, inner, ops->out_len);
    ops->final(cbuf, out);
}

static void _rl_sha256_init_v(void *c) { _rl_sha256_init((_rl_sha256_ctx *)c); }
static void _rl_sha256_update_v(void *c, const uint8_t *d, size_t n) {
    _rl_sha256_update((_rl_sha256_ctx *)c, d, n);
}
static void _rl_sha256_final_v(void *c, uint8_t *o) { _rl_sha256_final((_rl_sha256_ctx *)c, o); }
static void _rl_sha512_init_v(void *c) { _rl_sha512_init((_rl_sha512_ctx *)c); }
static void _rl_sha512_update_v(void *c, const uint8_t *d, size_t n) {
    _rl_sha512_update((_rl_sha512_ctx *)c, d, n);
}
static void _rl_sha512_final_v(void *c, uint8_t *o) { _rl_sha512_final((_rl_sha512_ctx *)c, o); }

static const _rl_hash_ops _RL_OPS_SHA256 = {
    _rl_sha256_init_v, _rl_sha256_update_v, _rl_sha256_final_v,
    sizeof(_rl_sha256_ctx), 64, 32,
};
static const _rl_hash_ops _RL_OPS_SHA512 = {
    _rl_sha512_init_v, _rl_sha512_update_v, _rl_sha512_final_v,
    sizeof(_rl_sha512_ctx), 128, 64,
};

// ---- crypto byte plumbing ----
// Byte arrays are int64-element arrays holding 0-255. Literals arrive
// that way; out-of-range elements abort like the VM's type error.

static uint8_t *_rl_crypto_bytes(rl_array a, uint64_t *n_out) {
    if (a.elem_size != (int32_t)sizeof(int64_t)) {
        fprintf(stderr, "error: expected array[byte]\n");
        _rl_abort();
    }
    int64_t *elems = (int64_t *)a.data;
    uint8_t *out = malloc(a.len > 0 ? a.len : 1);
    for (uint64_t i = 0; i < a.len; i++) {
        if (elems[i] < 0 || elems[i] > 255) {
            free(out);
            fprintf(stderr, "error: byte value %lld out of range 0-255\n", (long long)elems[i]);
            _rl_abort();
        }
        out[i] = (uint8_t)elems[i];
    }
    *n_out = a.len;
    return out;
}

static rl_array _rl_crypto_push_bytes(const uint8_t *data, uint64_t n) {
    int64_t *buf = malloc((n > 0 ? n : 1) * sizeof(int64_t));
    for (uint64_t i = 0; i < n; i++) buf[i] = (int64_t)data[i];
    rl_array arr = { .data = buf, .len = n, .cap = n,
        .elem_size = (int32_t)sizeof(int64_t), .type_tag = RL_TAG_I64 };
    return arr;
}

static rl_string _rl_crypto_push_str(const char *data, uint64_t n) {
    char *dup = malloc(n + 1);
    memcpy(dup, data, n);
    dup[n] = '\0';
    rl_string s = { .data = dup, .len = n };
    return s;
}

rl_array rl_crypto_sha256(rl_array data) {
    uint64_t n = 0;
    uint8_t *bytes = _rl_crypto_bytes(data, &n);
    _rl_sha256_ctx c;
    _rl_sha256_init(&c);
    _rl_sha256_update(&c, bytes, (size_t)n);
    uint8_t out[32];
    _rl_sha256_final(&c, out);
    free(bytes);
    return _rl_crypto_push_bytes(out, 32);
}

rl_array rl_crypto_sha512(rl_array data) {
    uint64_t n = 0;
    uint8_t *bytes = _rl_crypto_bytes(data, &n);
    _rl_sha512_ctx c;
    _rl_sha512_init(&c);
    _rl_sha512_update(&c, bytes, (size_t)n);
    uint8_t out[64];
    _rl_sha512_final(&c, out);
    free(bytes);
    return _rl_crypto_push_bytes(out, 64);
}

rl_array rl_crypto_sha1(rl_array data) {
    uint64_t n = 0;
    uint8_t *bytes = _rl_crypto_bytes(data, &n);
    _rl_sha1_ctx c;
    _rl_sha1_init(&c);
    _rl_sha1_update(&c, bytes, (size_t)n);
    uint8_t out[20];
    _rl_sha1_final(&c, out);
    free(bytes);
    return _rl_crypto_push_bytes(out, 20);
}

rl_array rl_crypto_md5(rl_array data) {
    uint64_t n = 0;
    uint8_t *bytes = _rl_crypto_bytes(data, &n);
    _rl_md5_ctx c;
    _rl_md5_init(&c);
    _rl_md5_update(&c, bytes, (size_t)n);
    uint8_t out[16];
    _rl_md5_final(&c, out);
    free(bytes);
    return _rl_crypto_push_bytes(out, 16);
}

rl_array rl_crypto_hmac_sha256(rl_array key, rl_array data) {
    uint64_t nk = 0, nd = 0;
    uint8_t *k = _rl_crypto_bytes(key, &nk);
    uint8_t *d = _rl_crypto_bytes(data, &nd);
    uint8_t out[32];
    _rl_hmac(&_RL_OPS_SHA256, k, (size_t)nk, d, (size_t)nd, out);
    free(k);
    free(d);
    return _rl_crypto_push_bytes(out, 32);
}

rl_array rl_crypto_hmac_sha512(rl_array key, rl_array data) {
    uint64_t nk = 0, nd = 0;
    uint8_t *k = _rl_crypto_bytes(key, &nk);
    uint8_t *d = _rl_crypto_bytes(data, &nd);
    uint8_t out[64];
    _rl_hmac(&_RL_OPS_SHA512, k, (size_t)nk, d, (size_t)nd, out);
    free(k);
    free(d);
    return _rl_crypto_push_bytes(out, 64);
}

bool rl_crypto_constant_time_eq(rl_array a, rl_array b) {
    uint64_t na = 0, nb = 0;
    uint8_t *ba = _rl_crypto_bytes(a, &na);
    uint8_t *bb = _rl_crypto_bytes(b, &nb);
    // Lengths fold into the diff like subtle's slice ct_eq; the data
    // loop always runs over the shared prefix.
    uint64_t diff = na ^ nb;
    uint64_t n = (na < nb) ? na : nb;
    for (uint64_t i = 0; i < n; i++) diff |= (uint64_t)(ba[i] ^ bb[i]);
    free(ba);
    free(bb);
    return diff == 0;
}

// ---- crypto base64 / hex ----

static const char _RL_B64_STD[] = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
static const char _RL_B64_URL[] = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789-_";

static rl_string _rl_crypto_b64_encode(const uint8_t *data, uint64_t n, const char *alpha, int pad) {
    uint64_t outlen = ((n + 2) / 3) * 4;
    char *out = malloc(outlen + 1);
    uint64_t o = 0;
    for (uint64_t i = 0; i < n; i += 3) {
        uint32_t triple = (uint32_t)data[i] << 16;
        int rem = (int)(n - i);
        if (rem > 1) triple |= (uint32_t)data[i + 1] << 8;
        if (rem > 2) triple |= data[i + 2];
        out[o++] = alpha[(triple >> 18) & 63];
        out[o++] = alpha[(triple >> 12) & 63];
        if (rem > 1) {
            out[o++] = alpha[(triple >> 6) & 63];
        } else if (pad) {
            out[o++] = '=';
        }
        if (rem > 2) {
            out[o++] = alpha[triple & 63];
        } else if (pad) {
            out[o++] = '=';
        }
    }
    out[o] = '\0';
    rl_string s = { .data = out, .len = o };
    return s;
}

static int _rl_crypto_b64_val(char c, const char *alpha) {
    if (c >= 'A' && c <= 'Z') return c - 'A';
    if (c >= 'a' && c <= 'z') return c - 'a' + 26;
    if (c >= '0' && c <= '9') return c - '0' + 52;
    if (c == alpha[62]) return 62;
    if (c == alpha[63]) return 63;
    return -1;
}

// Decodes base64; pad!=0 requires standard padding, pad==0 forbids it
// (URL_SAFE_NO_PAD). Returns NULL + *err on invalid input.
static uint8_t *_rl_crypto_b64_decode(const char *s, uint64_t n, const char *alpha, int pad,
        uint64_t *out_n, const char **err) {
    *err = NULL;
    if (!pad && n % 4 == 1) {
        *err = "invalid base64 length";
        return NULL;
    }
    uint64_t cap = (n / 4) * 3 + 3;
    uint8_t *out = malloc(cap > 0 ? cap : 1);
    uint64_t o = 0;
    uint64_t i = 0;
    while (i < n) {
        int vals[4];
        int got = 0, padding = 0;
        for (int k = 0; k < 4 && i < n; k++, i++) {
            if (pad && s[i] == '=') {
                vals[k] = 0;
                padding++;
                got++;
            } else {
                int v = _rl_crypto_b64_val(s[i], alpha);
                if (v < 0) {
                    free(out);
                    *err = "invalid base64 character";
                    return NULL;
                }
                if (padding > 0) {
                    free(out);
                    *err = "misplaced base64 padding";
                    return NULL;
                }
                vals[k] = v;
                got++;
            }
        }
        if (got < 4) {
            if (pad || got == 1) {
                free(out);
                *err = "truncated base64 input";
                return NULL;
            }
            // Unpadded tail: 2 chars -> 1 byte, 3 chars -> 2 bytes.
            uint32_t triple = ((uint32_t)vals[0] << 18) | ((uint32_t)vals[1] << 12);
            if (got == 3) triple |= (uint32_t)vals[2] << 6;
            out[o++] = (uint8_t)(triple >> 16);
            if (got == 3) out[o++] = (uint8_t)(triple >> 8);
            break;
        }
        uint32_t triple = ((uint32_t)vals[0] << 18) | ((uint32_t)vals[1] << 12)
            | ((uint32_t)vals[2] << 6) | (uint32_t)vals[3];
        // Padding only valid as the last 1-2 chars of the final group.
        if (padding > 0 && i < n) {
            free(out);
            *err = "misplaced base64 padding";
            return NULL;
        }
        if (padding > 2) {
            free(out);
            *err = "invalid base64 padding";
            return NULL;
        }
        out[o++] = (uint8_t)(triple >> 16);
        if (padding < 2) out[o++] = (uint8_t)(triple >> 8);
        if (padding < 1) out[o++] = (uint8_t)triple;
    }
    *out_n = o;
    return out;
}

rl_string rl_crypto_base64_encode(rl_array data) {
    uint64_t n = 0;
    uint8_t *bytes = _rl_crypto_bytes(data, &n);
    rl_string s = _rl_crypto_b64_encode(bytes, n, _RL_B64_STD, 1);
    free(bytes);
    return s;
}

rl_result rl_crypto_base64_decode(rl_string s) {
    const char *data = (s.data != NULL) ? s.data : "";
    uint64_t n = 0;
    const char *err = NULL;
    uint8_t *bytes = _rl_crypto_b64_decode(data, s.len, _RL_B64_STD, 1, &n, &err);
    if (err != NULL) return _rl_cli_err(err);
    rl_array arr = _rl_crypto_push_bytes(bytes, n);
    free(bytes);
    return rl_ok_arr(arr);
}

rl_string rl_crypto_base64_url_encode(rl_array data) {
    uint64_t n = 0;
    uint8_t *bytes = _rl_crypto_bytes(data, &n);
    rl_string s = _rl_crypto_b64_encode(bytes, n, _RL_B64_URL, 0);
    free(bytes);
    return s;
}

rl_result rl_crypto_base64_url_decode(rl_string s) {
    const char *data = (s.data != NULL) ? s.data : "";
    uint64_t n = 0;
    const char *err = NULL;
    uint8_t *bytes = _rl_crypto_b64_decode(data, s.len, _RL_B64_URL, 0, &n, &err);
    if (err != NULL) return _rl_cli_err(err);
    rl_array arr = _rl_crypto_push_bytes(bytes, n);
    free(bytes);
    return rl_ok_arr(arr);
}

static const char _RL_HEX[] = "0123456789abcdef";

rl_string rl_crypto_hex_encode(rl_array data) {
    uint64_t n = 0;
    uint8_t *bytes = _rl_crypto_bytes(data, &n);
    char *out = malloc(n * 2 + 1);
    for (uint64_t i = 0; i < n; i++) {
        out[2 * i] = _RL_HEX[bytes[i] >> 4];
        out[2 * i + 1] = _RL_HEX[bytes[i] & 15];
    }
    out[n * 2] = '\0';
    free(bytes);
    rl_string s = { .data = out, .len = n * 2 };
    return s;
}

static int _rl_crypto_hex_val(char c) {
    if (c >= '0' && c <= '9') return c - '0';
    if (c >= 'a' && c <= 'f') return c - 'a' + 10;
    if (c >= 'A' && c <= 'F') return c - 'A' + 10;
    return -1;
}

rl_result rl_crypto_hex_decode(rl_string s) {
    const char *data = (s.data != NULL) ? s.data : "";
    if (s.len % 2 != 0) return _rl_cli_err("hex_decode: odd length");
    uint8_t *out = malloc(s.len / 2 > 0 ? s.len / 2 : 1);
    for (uint64_t i = 0; i < s.len; i += 2) {
        int hi = _rl_crypto_hex_val(data[i]);
        int lo = _rl_crypto_hex_val(data[i + 1]);
        if (hi < 0 || lo < 0) {
            free(out);
            return _rl_cli_err("hex_decode: invalid hex character");
        }
        out[i / 2] = (uint8_t)((hi << 4) | lo);
    }
    rl_array arr = _rl_crypto_push_bytes(out, s.len / 2);
    free(out);
    return rl_ok_arr(arr);
}

// ---- crypto RNG ----

static void _rl_crypto_random(uint8_t *buf, uint64_t n) {
    // getrandom() first (fast, no fd), /dev/urandom fallback. Failure
    // aborts like the VM's expect: predictable bytes are worse than none.
    uint64_t done = 0;
    while (done < n) {
        ssize_t r = getrandom(buf + done, (size_t)(n - done), 0);
        if (r < 0) break;
        done += (uint64_t)r;
    }
    if (done < n) {
        FILE *f = fopen("/dev/urandom", "rb");
        if (f != NULL) {
            while (done < n) {
                size_t r = fread(buf + done, 1, (size_t)(n - done), f);
                if (r == 0) break;
                done += r;
            }
            fclose(f);
        }
    }
    if (done < n) {
        fprintf(stderr, "error: secure_random_bytes: OS entropy failure\n");
        _rl_abort();
    }
}

rl_array rl_crypto_secure_random_bytes(int64_t count) {
    if (count <= 0) return _rl_crypto_push_bytes(NULL, 0);
    uint8_t *buf = malloc((uint64_t)count);
    _rl_crypto_random(buf, (uint64_t)count);
    rl_array arr = _rl_crypto_push_bytes(buf, (uint64_t)count);
    free(buf);
    return arr;
}

rl_array rl_crypto_secure_token(int64_t count) {
    return rl_crypto_secure_random_bytes(count);
}

rl_string rl_crypto_secure_token_hex(int64_t count) {
    rl_array t = rl_crypto_secure_token(count);
    uint64_t n = 0;
    uint8_t *bytes = _rl_crypto_bytes(t, &n);
    char *out = malloc(n * 2 + 1);
    for (uint64_t i = 0; i < n; i++) {
        out[2 * i] = _RL_HEX[bytes[i] >> 4];
        out[2 * i + 1] = _RL_HEX[bytes[i] & 15];
    }
    out[n * 2] = '\0';
    free(bytes);
    // t owns int64 storage; element bytes were copied out above.
    free(t.data);
    rl_string s = { .data = out, .len = n * 2 };
    return s;
}

rl_string rl_crypto_secure_token_urlsafe(int64_t count) {
    rl_array t = rl_crypto_secure_token(count);
    uint64_t n = 0;
    uint8_t *bytes = _rl_crypto_bytes(t, &n);
    rl_string s = _rl_crypto_b64_encode(bytes, n, _RL_B64_URL, 0);
    free(bytes);
    free(t.data);
    return s;
}

// ---- crypto UUID ----

static rl_string _rl_crypto_uuid_str(const uint8_t id[16]) {
    char *out = malloc(37);
    snprintf(out, 37, "%02x%02x%02x%02x-%02x%02x-%02x%02x-%02x%02x-%02x%02x%02x%02x%02x%02x",
        id[0], id[1], id[2], id[3], id[4], id[5], id[6], id[7],
        id[8], id[9], id[10], id[11], id[12], id[13], id[14], id[15]);
    rl_string s = { .data = out, .len = 36 };
    return s;
}

rl_string rl_crypto_uuid_v4(void) {
    uint8_t id[16];
    _rl_crypto_random(id, 16);
    id[6] = (id[6] & 0x0f) | 0x40;
    id[8] = (id[8] & 0x3f) | 0x80;
    return _rl_crypto_uuid_str(id);
}

rl_string rl_crypto_uuid_v7(void) {
    struct timespec ts;
    clock_gettime(CLOCK_REALTIME, &ts);
    uint64_t ms = (uint64_t)ts.tv_sec * 1000 + (uint64_t)ts.tv_nsec / 1000000;
    uint8_t id[16];
    _rl_crypto_random(id, 16);
    id[0] = (uint8_t)(ms >> 40); id[1] = (uint8_t)(ms >> 32);
    id[2] = (uint8_t)(ms >> 24); id[3] = (uint8_t)(ms >> 16);
    id[4] = (uint8_t)(ms >> 8); id[5] = (uint8_t)ms;
    id[6] = (id[6] & 0x0f) | 0x70;
    id[8] = (id[8] & 0x3f) | 0x80;
    return _rl_crypto_uuid_str(id);
}

rl_result rl_crypto_uuid_parse(rl_string s) {
    const char *data = (s.data != NULL) ? s.data : "";
    // 8-4-4-4-12 hex with dashes; normalized output on success.
    if (s.len != 36 || data[8] != '-' || data[13] != '-' || data[18] != '-' || data[23] != '-') {
        return _rl_cli_err("uuid_parse: malformed UUID");
    }
    uint8_t id[16];
    unsigned pos = 0;
    for (uint64_t i = 0; i < 36; i++) {
        if (data[i] == '-') continue;
        int v = _rl_crypto_hex_val(data[i]);
        if (v < 0) return _rl_cli_err("uuid_parse: malformed UUID");
        if (pos % 2 == 0) id[pos / 2] = (uint8_t)(v << 4);
        else id[pos / 2] |= (uint8_t)v;
        pos++;
    }
    if (pos != 32) return _rl_cli_err("uuid_parse: malformed UUID");
    return rl_ok_str(_rl_crypto_uuid_str(id));
}

// ---- crypto passwords (argon2) ----
// Needs libargon2: rlt links -DRL_USE_ARGON2 -largon2 when generated code
// mentions rl_crypto_password. Hand-built binaries without the flag get a
// loud abort naming the missing flags. Params match the VM exactly
// (argon2id, m=19456, t=2, p=1, 16-byte salt, 32-byte hash) so hashes
// verify cross-backend.

#ifdef RL_USE_ARGON2
#include <argon2.h>
#endif

rl_string rl_crypto_password_hash(rl_string password) {
#ifdef RL_USE_ARGON2
    const char *pwd = (password.data != NULL) ? password.data : "";
    uint8_t salt[16];
    _rl_crypto_random(salt, 16);
    char encoded[256];
    int rc = argon2id_hash_encoded(2, 19456, 1, pwd, password.len, salt, 16, 32, encoded, sizeof(encoded));
    if (rc != ARGON2_OK) {
        fprintf(stderr, "error: password_hash: %s\n", argon2_error_message(rc));
        _rl_abort();
    }
    return _rl_crypto_push_str(encoded, strlen(encoded));
#else
    (void)password;
    fprintf(stderr, "error: password_hash needs -DRL_USE_ARGON2 -largon2\n");
    _rl_abort();
    return _rl_crypto_push_str("", 0);
#endif
}

bool rl_crypto_password_verify(rl_string password, rl_string hash) {
#ifdef RL_USE_ARGON2
    const char *pwd = (password.data != NULL) ? password.data : "";
    char *h = malloc(hash.len + 1);
    if (hash.len > 0 && hash.data != NULL) memcpy(h, hash.data, hash.len);
    h[hash.len] = '\0';
    int rc = argon2id_verify(h, pwd, password.len);
    free(h);
    return rc == ARGON2_OK;
#else
    (void)password;
    (void)hash;
    fprintf(stderr, "error: password_verify needs -DRL_USE_ARGON2 -largon2\n");
    _rl_abort();
    return 0;
#endif
}

// ---- cli progress ----
// Byte-identical format to the VM: \r{label} [{#24}] {pct:3}%, \n at 100%.

rl_result rl_cli_progress_bar(int64_t current, int64_t total, rl_string label) {
    double frac = 0.0;
    if (total > 0) {
        frac = (double)(current < 0 ? 0 : current) / (double)total;
        if (frac < 0.0) frac = 0.0;
        if (frac > 1.0) frac = 1.0;
    }
    int filled = (int)(frac * 24.0 + 0.5);
    char bar[25];
    for (int i = 0; i < 24; i++) bar[i] = (i < filled) ? '#' : '-';
    bar[24] = '\0';
    char *lname = _rl_cli_cstr(label);
    fprintf(stderr, "\r%s [%s] %3lld%%", lname, bar, (long long)(frac * 100.0 + 0.5));
    if (frac >= 1.0) fprintf(stderr, "\n");
    fflush(stderr);
    free(lname);
    return rl_ok_null();
}

rl_result rl_cli_spinner_tick(int64_t frame) {
    static const char frames[4] = { '|', '/', '-', '\\' };
    long long idx = frame % 4;
    if (idx < 0) idx += 4;
    fprintf(stderr, "\r%c", frames[idx]);
    fflush(stderr);
    return rl_ok_null();
}

// ---- time ----

// Format a Unix timestamp with a strftime-style `pattern`.
// Format a Unix timestamp with a strftime-style pattern, or an error.
rl_result rl_time_format_time(int64_t timestamp, rl_string pattern) {
    if (pattern.data == NULL) return rl_err(-1);
    time_t t = (time_t)timestamp;
    struct tm *tm = gmtime(&t);
    if (!tm) return rl_err(-1);
    char buf[256];
    char pat[pattern.len + 1];
    memcpy(pat, pattern.data, pattern.len);
    pat[pattern.len] = '\0';
    if (strftime(buf, sizeof(buf), pat, tm) == 0) return rl_err(-1);
    uint64_t len = strlen(buf);
    char *out = malloc(len + 1);
    memcpy(out, buf, len + 1);
    rl_string result = { .data = out, .len = len, .rc = 1 };
    return rl_ok_str(result);
}

// Format as `YYYY-MM-DD` in local time, or an error.
rl_result rl_time_format_date_str(int64_t timestamp) {
    time_t t = (time_t)timestamp;
    struct tm *tm = gmtime(&t);
    if (!tm) return rl_err(-1);
    char buf[64];
    if (strftime(buf, sizeof(buf), "%Y-%m-%d", tm) == 0) return rl_err(-1);
    uint64_t len = strlen(buf);
    char *out = malloc(len + 1);
    memcpy(out, buf, len + 1);
    rl_string result = { .data = out, .len = len, .rc = 1 };
    return rl_ok_str(result);
}

// Format as `HH:MM:SS` in local time, or an error.
rl_result rl_time_format_time_str(int64_t timestamp) {
    time_t t = (time_t)timestamp;
    struct tm *tm = gmtime(&t);
    if (!tm) return rl_err(-1);
    char buf[64];
    if (strftime(buf, sizeof(buf), "%H:%M:%S", tm) == 0) return rl_err(-1);
    uint64_t len = strlen(buf);
    char *out = malloc(len + 1);
    memcpy(out, buf, len + 1);
    rl_string result = { .data = out, .len = len, .rc = 1 };
    return rl_ok_str(result);
}

// Split into `[year, month, day, hour, min, sec]` components.
// Split into `[year, month, day, hour, min, sec]`, or an error.
rl_result rl_time_parts(int64_t timestamp) {
    time_t t = (time_t)timestamp;
    struct tm *tm = gmtime(&t);
    if (!tm) return rl_err(-1);
    int64_t parts[6] = {
        tm->tm_year + 1900,
        tm->tm_mon + 1,
        tm->tm_mday,
        tm->tm_hour,
        tm->tm_min,
        tm->tm_sec
    };
    return rl_ok_arr(rl_arr_from_vals(parts, 6, sizeof(int64_t)));
}

// ---- io ----

// Read the whole file as one string / one array element per line; the result is an error when the file cannot be read.
rl_result rl_io_read_file(rl_string path) {
    char buf[path.len + 1];
    memcpy(buf, path.data, path.len);
    buf[path.len] = '\0';
    FILE *f = fopen(buf, "rb");
    if (!f) {
        return rl_err_msg(rl_str_literal("failed to open file for reading", 31));
    }
    fseek(f, 0, SEEK_END);
    long size = ftell(f);
    fseek(f, 0, SEEK_SET);
    if (size <= 0) {
        fclose(f);
        return rl_ok_str(rl_str_literal("", 0));
    }
    char *out = malloc((uint64_t)size);
    size_t n = fread(out, 1, (uint64_t)size, f);
    fclose(f);
    rl_string result = { .data = out, .len = (uint64_t)n, .rc = 1 };
    return rl_ok_str(result);
}

// Read the whole file as one string / one array element per line; the result is an error when the file cannot be read.
rl_result rl_io_read_lines(rl_string path) {
    rl_result content_r = rl_io_read_file(path);
    if (!content_r.is_ok) return content_r;
    rl_string nl = { .data = "\n", .len = 1, .rc = 1 };
    rl_array lines = rl_str_split(content_r.data.str, nl);
    return rl_ok_arr(lines);
}

// Read one whitespace-separated token / int / float from stdin.
rl_string rl_io_read(void) {
    uint64_t cap = 256;
    char *buf = malloc(cap);
    uint64_t len = 0;
    int c;
    while ((c = fgetc(stdin)) != EOF && c != '\n') {
        if (len >= cap) {
            cap *= 2;
            buf = realloc(buf, cap);
        }
        buf[len++] = (char)c;
    }
    buf[len] = '\0';
    rl_string result = { .data = buf, .len = len, .rc = 1 };
    return result;
}

// Read one whitespace-separated token / int / float from stdin.
int64_t rl_io_read_int(void) {
    int64_t v = 0;
    scanf("%ld", &v);
    return v;
}

// Read one whitespace-separated token / int / float from stdin.
double rl_io_read_float(void) {
    double v = 0.0;
    scanf("%lf", &v);
    return v;
}

// Overwrite / append `content`; the result is an error on failure.
rl_result rl_io_write_file(rl_string path, rl_string content) {
    char buf[path.len + 1];
    memcpy(buf, path.data, path.len);
    buf[path.len] = '\0';
    FILE *f = fopen(buf, "wb");
    if (!f) return rl_err_msg(rl_str_literal("failed to open file for writing", 30));
    fwrite(content.data, 1, content.len, f);
    fclose(f);
    return rl_ok_null();
}

// Overwrite / append `content`; the result is an error on failure.
rl_result rl_io_append_file(rl_string path, rl_string content) {
    char buf[path.len + 1];
    memcpy(buf, path.data, path.len);
    buf[path.len] = '\0';
    FILE *f = fopen(buf, "ab");
    if (!f) return rl_err_msg(rl_str_literal("failed to open file for appending", 32));
    fwrite(content.data, 1, content.len, f);
    fclose(f);
    return rl_ok_null();
}

// Delete the file at `path`; ok null on success, or an error.
rl_result rl_io_delete_file(rl_string path) {
    if (path.data == NULL) return rl_err(-1);
    char buf[path.len + 1];
    memcpy(buf, path.data, path.len);
    buf[path.len] = '\0';
    if (remove(buf) != 0) return rl_err(-1);
    return rl_ok_null();
}

// True when stdin is a terminal (used for cursor hide/show only).
bool rl_io_isatty(void) {
    return isatty(STDIN_FILENO) != 0;
}

// Write to stderr without / with a trailing newline.
void rl_io_eprint(rl_string msg) {
    fprintf(stderr, "%.*s", (int)msg.len, msg.data);
}

// Write to stderr without / with a trailing newline.
void rl_io_eprintln(rl_string msg) {
    fprintf(stderr, "%.*s\n", (int)msg.len, msg.data);
}

// Read the whole file as an array of byte values (error when unreadable).
rl_result rl_io_read_bytes(rl_string path) {
    char buf[path.len + 1];
    memcpy(buf, path.data, path.len);
    buf[path.len] = '\0';
    FILE *f = fopen(buf, "rb");
    if (!f) return rl_err_msg(rl_str_literal("failed to open file for reading", 31));
    fseek(f, 0, SEEK_END);
    long size = ftell(f);
    fseek(f, 0, SEEK_SET);
    if (size <= 0) {
        fclose(f);
        rl_array empty = { .data = NULL, .len = 0, .cap = 0, .elem_size = sizeof(int64_t), .type_tag = RL_TAG_I64 };
        return rl_ok_arr(empty);
    }
    char *raw = malloc((uint64_t)size);
    size_t n = fread(raw, 1, (uint64_t)size, f);
    fclose(f);
    int64_t *bytes = malloc(n * sizeof(int64_t));
    for (uint64_t i = 0; i < n; i++) {
        bytes[i] = (int64_t)(unsigned char)raw[i];
    }
    free(raw);
    rl_array result = { .data = bytes, .len = n, .cap = n, .elem_size = sizeof(int64_t), .type_tag = RL_TAG_I64 };
    return rl_ok_arr(result);
}

// Read all of stdin until EOF; ok with the text, or an error.
rl_result rl_io_read_all_stdin(void) {
    size_t cap = 4096, len = 0;
    char *buf = malloc(cap + 1);
    size_t n;
    while ((n = fread(buf + len, 1, cap - len, stdin)) > 0) {
        len += n;
        if (len == cap) {
            cap *= 2;
            buf = realloc(buf, cap + 1);
        }
    }
    if (ferror(stdin)) {
        free(buf);
        char *msg = malloc(64);
        int m = snprintf(msg, 64, "read_all_stdin: %s", strerror(errno));
        return rl_err_msg((rl_string){ .data = msg, .len = (uint64_t)m, .rc = 1 });
    }
    buf[len] = '\0';
    rl_string out = { .data = buf, .len = (uint64_t)len, .rc = 1 };
    return rl_ok_str(out);
}

// Decode a byte array as UTF-8; ok with the string, or an error.
rl_result rl_io_decode_utf8(rl_array bytes) {
    uint64_t n = bytes.len;
    unsigned char *raw = malloc(n > 0 ? n : 1);
    for (uint64_t i = 0; i < n; i++) {
        unsigned v;
        if (bytes.elem_size == 1) v = ((unsigned char *)bytes.data)[i];
        else if (bytes.data == NULL) v = 0;
        else v = (unsigned char)(((int64_t *)bytes.data)[i]);
        raw[i] = (unsigned char)v;
    }
    uint64_t i = 0;
    while (i < n) {
        unsigned char c = raw[i];
        if (c < 0x80) { i++; continue; }
        uint64_t need = 0;
        uint32_t cp = 0;
        if ((c & 0xE0) == 0xC0) { need = 2; cp = c & 0x1F; }
        else if ((c & 0xF0) == 0xE0) { need = 3; cp = c & 0x0F; }
        else if ((c & 0xF8) == 0xF0) { need = 4; cp = c & 0x07; }
        else {
            char *msg = malloc(96);
            int m = snprintf(msg, 96, "decode_utf8: invalid UTF-8 at byte invalid utf-8 sequence of 1 bytes from index %llu", (unsigned long long)i);
            free(raw);
            return rl_err_msg((rl_string){ .data = msg, .len = (uint64_t)m, .rc = 1 });
        }
        if (i + need > n) {
            char *msg = malloc(96);
            int m = snprintf(msg, 96, "decode_utf8: invalid UTF-8 at byte incomplete utf-8 byte sequence from index %llu", (unsigned long long)i);
            free(raw);
            return rl_err_msg((rl_string){ .data = msg, .len = (uint64_t)m, .rc = 1 });
        }
        bool good = true;
        for (uint64_t j = 1; j < need; j++) {
            unsigned char d = raw[i + j];
            if ((d & 0xC0) != 0x80) { good = false; break; }
            cp = (cp << 6) | (d & 0x3F);
        }
        if (!good) {
            char *msg = malloc(96);
            int m = snprintf(msg, 96, "decode_utf8: invalid UTF-8 at byte invalid utf-8 sequence of %llu bytes from index %llu", (unsigned long long)need, (unsigned long long)i);
            free(raw);
            return rl_err_msg((rl_string){ .data = msg, .len = (uint64_t)m, .rc = 1 });
        }
        bool overlong = (need == 2 && cp < 0x80) || (need == 3 && cp < 0x800) || (need == 4 && cp < 0x10000);
        bool surrogate = (cp >= 0xD800 && cp <= 0xDFFF);
        bool too_big = (cp > 0x10FFFF);
        if (overlong || surrogate || too_big) {
            char *msg = malloc(96);
            int m = snprintf(msg, 96, "decode_utf8: invalid UTF-8 at byte invalid utf-8 sequence of %llu bytes from index %llu", (unsigned long long)need, (unsigned long long)i);
            free(raw);
            return rl_err_msg((rl_string){ .data = msg, .len = (uint64_t)m, .rc = 1 });
        }
        i += need;
    }
    rl_string out = { .data = (char *)raw, .len = n, .rc = 1 };
    return rl_ok_str(out);
}

// Encode a string as UTF-8 bytes; bare array of byte values.
rl_array rl_io_encode_utf8(rl_string s) {
    if (s.data == NULL || s.len == 0) {
        rl_array empty = { .data = NULL, .len = 0, .cap = 0, .elem_size = sizeof(int64_t), .type_tag = RL_TAG_I64 };
        return empty;
    }
    int64_t *buf = malloc(s.len * sizeof(int64_t));
    for (uint64_t i = 0; i < s.len; i++) {
        buf[i] = (int64_t)(unsigned char)s.data[i];
    }
    rl_array out = { .data = buf, .len = s.len, .cap = s.len, .elem_size = sizeof(int64_t), .type_tag = RL_TAG_I64 };
    return out;
}

// ---- types ----

// Format an int as decimal / binary (`0b...`) / hex (`0x...`) / octal.
// `to_string` over a result payload: int/float/bool/char/string,
// err otherwise.
rl_result rl_types_to_string(rl_result x) {
    if (!x.is_ok) return x;
    char buf[32];
    int len = 0;
    switch (x.tag) {
        case RL_TAG_I64: len = snprintf(buf, sizeof(buf), "%ld", (long)x.data.i64); break;
        case RL_TAG_F64: len = snprintf(buf, sizeof(buf), "%g", x.data.f64); break;
        case RL_TAG_BOOL: len = snprintf(buf, sizeof(buf), "%s", x.data.boolean ? "true" : "false"); break;
        case RL_TAG_CHAR: {
            uint32_t code = (uint32_t)x.data.i64;
            if (code < 0x80) { buf[0] = (char)code; buf[1] = '\0'; len = 1; }
            else if (code < 0x800) {
                buf[0] = (char)(0xC0 | (code >> 6)); buf[1] = (char)(0x80 | (code & 0x3F)); buf[2] = '\0'; len = 2;
            } else if (code < 0x10000) {
                buf[0] = (char)(0xE0 | (code >> 12)); buf[1] = (char)(0x80 | ((code >> 6) & 0x3F)); buf[2] = (char)(0x80 | (code & 0x3F)); buf[3] = '\0'; len = 3;
            } else {
                buf[0] = (char)(0xF0 | (code >> 18)); buf[1] = (char)(0x80 | ((code >> 12) & 0x3F)); buf[2] = (char)(0x80 | ((code >> 6) & 0x3F)); buf[3] = (char)(0x80 | (code & 0x3F)); buf[4] = '\0'; len = 4;
            }
            break;
        }
        case RL_TAG_STR: return rl_ok_str(x.data.str);
        default: {
            const char *name = _rl_tag_name(x.tag);
            char *msg = malloc(48);
            int n = snprintf(msg, 48, "cannot parse \"%s\" as string", name);
            return rl_err_msg((rl_string){ .data = msg, .len = (uint64_t)n, .rc = 1 });
        }
    }
    char *out = malloc(len + 1);
    memcpy(out, buf, len + 1);
    rl_string result = { .data = out, .len = (uint64_t)len, .rc = 1 };
    return rl_ok_str(result);
}

// Render an unsigned value in base 2/8/16 (no prefix, `-` for negatives).
static rl_string _rl_format_radix(uint64_t uv, bool negative, unsigned base) {
    const char *digits = "0123456789abcdef";
    char buf[66];
    int i = 65;
    buf[i] = '\0';
    if (uv == 0) buf[--i] = '0';
    while (uv > 0) { buf[--i] = digits[uv % base]; uv /= base; }
    if (negative) buf[--i] = '-';
    uint64_t len = 65 - (uint64_t)i;
    char *out = malloc(len + 1);
    memcpy(out, buf + i, len + 1);
    rl_string result = { .data = out, .len = len, .rc = 1 };
    return result;
}

// `to_bin` over a result payload: int/byte/bool/char/string, err otherwise.
rl_result rl_types_to_bin(rl_result x) {
    if (!x.is_ok) return x;
    switch (x.tag) {
        case RL_TAG_I64: {
            int64_t v = x.data.i64;
            return rl_ok_str(_rl_format_radix(v < 0 ? (uint64_t)(-(v + 1)) + 1 : (uint64_t)v, v < 0, 2));
        }
        case RL_TAG_BOOL: return rl_ok_str(rl_str_literal(x.data.boolean ? "1" : "0", 1));
        case RL_TAG_CHAR: return rl_ok_str(_rl_format_radix((uint64_t)(uint32_t)x.data.i64, false, 2));
        case RL_TAG_STR: {
            uint64_t total = 0;
            for (uint64_t i = 0; i < x.data.str.len; i++) {
                unsigned char b = (unsigned char)x.data.str.data[i];
                total += b == 0 ? 1 : 8 - __builtin_clz((unsigned)b);
            }
            char *out = malloc(total + 1);
            uint64_t pos = 0;
            for (uint64_t i = 0; i < x.data.str.len; i++) {
                unsigned char b = (unsigned char)x.data.str.data[i];
                if (b == 0) { out[pos++] = '0'; continue; }
                int bits = 8 - __builtin_clz((unsigned)b);
                for (int k = bits - 1; k >= 0; k--) out[pos++] = ((b >> k) & 1) ? '1' : '0';
            }
            out[pos] = '\0';
            return rl_ok_str((rl_string){ .data = out, .len = pos, .rc = 1 });
        }
        default: {
            const char *name = _rl_tag_name(x.tag);
            char *msg = malloc(48);
            int n = snprintf(msg, 48, "cannot parse \"%s\" as binary", name);
            return rl_err_msg((rl_string){ .data = msg, .len = (uint64_t)n, .rc = 1 });
        }
    }
}

// `to_hex` over a result payload: int/byte/char/string, err otherwise.
rl_result rl_types_to_hex(rl_result x) {
    if (!x.is_ok) return x;
    switch (x.tag) {
        case RL_TAG_I64: {
            int64_t v = x.data.i64;
            return rl_ok_str(_rl_format_radix(v < 0 ? (uint64_t)(-(v + 1)) + 1 : (uint64_t)v, v < 0, 16));
        }
        case RL_TAG_CHAR: return rl_ok_str(_rl_format_radix((uint64_t)(uint32_t)x.data.i64, false, 16));
        case RL_TAG_STR: {
            char *out = malloc(x.data.str.len * 2 + 1);
            const char *digits = "0123456789abcdef";
            for (uint64_t i = 0; i < x.data.str.len; i++) {
                unsigned char b = (unsigned char)x.data.str.data[i];
                out[2 * i] = digits[b >> 4];
                out[2 * i + 1] = digits[b & 0xF];
            }
            out[x.data.str.len * 2] = '\0';
            return rl_ok_str((rl_string){ .data = out, .len = x.data.str.len * 2, .rc = 1 });
        }
        default: {
            const char *name = _rl_tag_name(x.tag);
            char *msg = malloc(52);
            int n = snprintf(msg, 52, "cannot parse \"%s\" as hexadecimal", name);
            return rl_err_msg((rl_string){ .data = msg, .len = (uint64_t)n, .rc = 1 });
        }
    }
}

// Format an int as decimal / binary (`0b...`) / hex (`0x...`) / octal.
// `to_oct` over a result payload: int/byte/char/string, err otherwise.
rl_result rl_types_to_oct(rl_result x) {
    if (!x.is_ok) return x;
    switch (x.tag) {
        case RL_TAG_I64: {
            int64_t v = x.data.i64;
            return rl_ok_str(_rl_format_radix(v < 0 ? (uint64_t)(-(v + 1)) + 1 : (uint64_t)v, v < 0, 8));
        }
        case RL_TAG_CHAR: return rl_ok_str(_rl_format_radix((uint64_t)(uint32_t)x.data.i64, false, 8));
        case RL_TAG_STR: {
            uint64_t total = 0;
            for (uint64_t i = 0; i < x.data.str.len; i++) {
                unsigned char b = (unsigned char)x.data.str.data[i];
                total += b < 8 ? 1 : (b < 64 ? 2 : 3);
            }
            char *out = malloc(total + 1);
            uint64_t pos = 0;
            for (uint64_t i = 0; i < x.data.str.len; i++) {
                unsigned char b = (unsigned char)x.data.str.data[i];
                char tmp[4];
                int n = snprintf(tmp, sizeof(tmp), "%o", b);
                memcpy(out + pos, tmp, n);
                pos += n;
            }
            out[pos] = '\0';
            return rl_ok_str((rl_string){ .data = out, .len = pos, .rc = 1 });
        }
        default: {
            const char *name = _rl_tag_name(x.tag);
            char *msg = malloc(48);
            int n = snprintf(msg, 48, "cannot parse \"%s\" as octal", name);
            return rl_err_msg((rl_string){ .data = msg, .len = (uint64_t)n, .rc = 1 });
        }
    }
}

// Turn an error result into a panic; wrap an int as a byte / char value.
rl_result rl_types_error_unwrap(rl_result x) {
    if (x.is_ok) {
        return rl_err_msg(rl_str_literal("error_unwrap: expected error, got ok", 36));
    }
    return x;
}

// Turn an error result into a panic; wrap an int as a byte / char value.
rl_result rl_types_to_byte(rl_result x) {
    switch (x.tag) {
        case RL_TAG_I64: return rl_ok_i64((int64_t)(unsigned char)x.data.i64);
        case RL_TAG_F64: return rl_ok_i64((int64_t)(unsigned char)(int64_t)x.data.f64);
        case RL_TAG_BOOL: return rl_ok_i64(x.data.boolean ? 1 : 0);
        case RL_TAG_CHAR: return rl_ok_i64((int64_t)(unsigned char)x.data.i64);
        case RL_TAG_STR: {
            uint64_t tlen = 0;
            char *t = _rl_trim_copy(x.data.str, &tlen);
            unsigned long v = 0;
            bool ok = false;
            if (tlen > 0 && (strncmp(t, "0x", 2) == 0 || strncmp(t, "0X", 2) == 0)) {
                char *end = NULL;
                errno = 0;
                v = strtoul(t + 2, &end, 16);
                ok = errno == 0 && end && *end == '\0' && end != t + 2 && v <= 0xFF;
            } else if (tlen > 0) {
                char *end = NULL;
                errno = 0;
                v = strtoul(t, &end, 10);
                ok = errno == 0 && end && *end == '\0' && end != t && v <= 0xFF;
            }
            rl_result r;
            if (ok) {
                r = rl_ok_i64((int64_t)(unsigned char)v);
            } else {
                char *msg = malloc(tlen + 34);
                int n = snprintf(msg, tlen + 34, "cannot parse \"%s\" as byte", t);
                r = rl_err_msg((rl_string){ .data = msg, .len = (uint64_t)n, .rc = 1 });
            }
            free(t);
            return r;
        }
        default: return rl_err_msg(rl_str_literal("cannot convert to byte", 22));
    }
}

// Turn an error result into a panic; wrap an int as a byte / char value.
rl_result rl_types_to_char(rl_result x) {
    if (!x.is_ok) return x;
    switch (x.tag) {
        case RL_TAG_CHAR: return x;
        case RL_TAG_I64: {
            uint32_t code = (uint32_t)x.data.i64;
            if (code <= 0x10FFFF && !(code >= 0xD800 && code <= 0xDFFF)) {
                rl_result r = { .is_ok = true, .tag = RL_TAG_CHAR, .err_code = 0 };
                r.data.i64 = (int64_t)code;
                return r;
            }
            char *msg = malloc(64);
            int n = snprintf(msg, 64, "%ld is not a valid unicode codepoint", (long)x.data.i64);
            return rl_err_msg((rl_string){ .data = msg, .len = (uint64_t)n, .rc = 1 });
        }
        case RL_TAG_STR: {
            uint32_t code = 0;
            uint64_t used = 0;
            if (_rl_utf8_decode(x.data.str.data, x.data.str.len, &code, &used)
                && used == x.data.str.len) {
                rl_result r = { .is_ok = true, .tag = RL_TAG_CHAR, .err_code = 0 };
                r.data.i64 = (int64_t)code;
                return r;
            }
            return rl_err_msg(rl_str_literal("string must be exactly one character", 39));
        }
        default: {
            const char *name = _rl_tag_name(x.tag);
            char *msg = malloc(64);
            int n = snprintf(msg, 64, "cannot parse \"%s\" as character", name);
            return rl_err_msg((rl_string){ .data = msg, .len = (uint64_t)n, .rc = 1 });
        }
    }
}

// Tag name for conversion error messages.
static const char *_rl_tag_name(enum rl_type_tag tag) {
    switch (tag) {
        case RL_TAG_NULL: return "null";
        case RL_TAG_I64: return "int";
        case RL_TAG_F64: return "float";
        case RL_TAG_BOOL: return "bool";
        case RL_TAG_CHAR: return "char";
        case RL_TAG_STR: return "string";
        case RL_TAG_ARR: return "array";
        case RL_TAG_MAP: return "map";
        case RL_TAG_SET: return "set";
        case RL_TAG_CLOSURE: return "closure";
        default: return "unknown";
    }
}

// Trim ASCII whitespace; returns a fresh NUL-terminated copy with its
// length (without the terminator).
static char *_rl_trim_copy(rl_string s, uint64_t *out_len) {
    uint64_t start = 0;
    while (start < s.len && (s.data[start] == ' ' || s.data[start] == '\t'
        || s.data[start] == '\n' || s.data[start] == '\r')) start++;
    uint64_t end = s.len;
    while (end > start && (s.data[end - 1] == ' ' || s.data[end - 1] == '\t'
        || s.data[end - 1] == '\n' || s.data[end - 1] == '\r')) end--;
    uint64_t len = end - start;
    char *out = malloc(len + 1);
    memcpy(out, s.data + start, len);
    out[len] = '\0';
    if (out_len) *out_len = len;
    return out;
}

// Decode one UTF-8 sequence; writes the codepoint and bytes consumed,
// false on invalid input.
static bool _rl_utf8_decode(const char *s, uint64_t len, uint32_t *code, uint64_t *used) {
    if (len == 0) return false;
    unsigned char c = (unsigned char)s[0];
    if (c < 0x80) { *code = c; *used = 1; return true; }
    uint32_t cp;
    uint64_t need;
    if ((c & 0xE0) == 0xC0) { cp = c & 0x1F; need = 2; }
    else if ((c & 0xF0) == 0xE0) { cp = c & 0x0F; need = 3; }
    else if ((c & 0xF8) == 0xF0) { cp = c & 0x07; need = 4; }
    else return false;
    if (len < need) return false;
    for (uint64_t i = 1; i < need; i++) {
        unsigned char d = (unsigned char)s[i];
        if ((d & 0xC0) != 0x80) return false;
        cp = (cp << 6) | (d & 0x3F);
    }
    *code = cp;
    *used = need;
    return true;
}

// `to_int` over a result payload: int/float/bool/char/string, err otherwise.
rl_result rl_to_int(rl_result x) {
    if (!x.is_ok) return x;
    switch (x.tag) {
        case RL_TAG_I64: return x;
        case RL_TAG_F64: return rl_ok_i64((int64_t)x.data.f64);
        case RL_TAG_BOOL: return rl_ok_i64(x.data.boolean ? 1 : 0);
        case RL_TAG_CHAR: return rl_ok_i64(x.data.i64);
        case RL_TAG_STR: {
            uint64_t tlen = 0;
            char *t = _rl_trim_copy(x.data.str, &tlen);
            int64_t v = 0;
            bool ok = false;
            if (tlen > 0 && (strncmp(t, "0x", 2) == 0 || strncmp(t, "0X", 2) == 0)) {
                char *end = NULL;
                errno = 0;
                long long parsed = strtoll(t + 2, &end, 16);
                if (errno == 0 && end && *end == '\0' && end != t + 2
                    && parsed >= INT64_MIN && parsed <= INT64_MAX) {
                    v = (int64_t)parsed;
                    ok = true;
                }
            } else if (tlen > 0) {
                char *end = NULL;
                errno = 0;
                long long parsed = strtoll(t, &end, 10);
                if (errno == 0 && end && *end == '\0' && end != t
                    && parsed >= INT64_MIN && parsed <= INT64_MAX) {
                    v = (int64_t)parsed;
                    ok = true;
                }
            }
            rl_result r;
            if (ok) {
                r = rl_ok_i64(v);
            } else {
                char *msg = malloc(tlen + 32);
                int n = snprintf(msg, tlen + 32, "cannot parse \"%s\" as int", t);
                r = rl_err_msg((rl_string){ .data = msg, .len = (uint64_t)n, .rc = 1 });
            }
            free(t);
            return r;
        }
        default: {
            const char *name = _rl_tag_name(x.tag);
            char *msg = malloc(48);
            int n = snprintf(msg, 48, "cannot parse \"%s\" as int", name);
            return rl_err_msg((rl_string){ .data = msg, .len = (uint64_t)n, .rc = 1 });
        }
    }
}

// `to_float` over a result payload: float/int/bool/string, err otherwise.
rl_result rl_to_float(rl_result x) {
    if (!x.is_ok) return x;
    switch (x.tag) {
        case RL_TAG_F64: return x;
        case RL_TAG_I64: return rl_ok_f64((double)x.data.i64);
        case RL_TAG_BOOL: return rl_ok_f64(x.data.boolean ? 1.0 : 0.0);
        case RL_TAG_STR: {
            uint64_t tlen = 0;
            char *t = _rl_trim_copy(x.data.str, &tlen);
            rl_result r;
            if (tlen > 0) {
                char *end = NULL;
                double parsed = strtod(t, &end);
                if (end && *end == '\0' && end != t) {
                    r = rl_ok_f64(parsed);
                    free(t);
                    return r;
                }
            }
            char *msg = malloc(tlen + 34);
            int n = snprintf(msg, tlen + 34, "cannot parse \"%s\" as float", t);
            r = rl_err_msg((rl_string){ .data = msg, .len = (uint64_t)n, .rc = 1 });
            free(t);
            return r;
        }
        default: {
            const char *name = _rl_tag_name(x.tag);
            char *msg = malloc(48);
            int n = snprintf(msg, 48, "cannot parse \"%s\" as float", name);
            return rl_err_msg((rl_string){ .data = msg, .len = (uint64_t)n, .rc = 1 });
        }
    }
}

// `to_bool` over a result payload: bool/int/float/null/string, err otherwise.
rl_result rl_to_bool(rl_result x) {
    if (!x.is_ok) return x;
    switch (x.tag) {
        case RL_TAG_BOOL: return x;
        case RL_TAG_I64: return rl_ok_bool(x.data.i64 != 0);
        case RL_TAG_F64: return rl_ok_bool(x.data.f64 != 0.0);
        case RL_TAG_NULL: return rl_ok_bool(false);
        case RL_TAG_STR: {
            uint64_t tlen = 0;
            char *t = _rl_trim_copy(x.data.str, &tlen);
            bool v = !(tlen == 0 || strcmp(t, "false") == 0 || strcmp(t, "0") == 0);
            free(t);
            return rl_ok_bool(v);
        }
        default: {
            const char *name = _rl_tag_name(x.tag);
            char *msg = malloc(48);
            int n = snprintf(msg, 48, "cannot parse \"%s\" as bool", name);
            return rl_err_msg((rl_string){ .data = msg, .len = (uint64_t)n, .rc = 1 });
        }
    }
}

// ---- random ----

static int rl_rand_initialized = 0;

// Seed the C RNG once on first use.
static void rl_rand_ensure_init(void) {
    if (!rl_rand_initialized) {
        srand((unsigned int)time(NULL));
        rl_rand_initialized = 1;
    }
}

// Reseed the C RNG (RL rand_seed); later values are deterministic.
void rl_rand_seed(int64_t seed) {
    srand((unsigned int)seed);
    rl_rand_initialized = 1;
}

// Unseeded pseudo-random values from the C library RNG. Full-range non-negative int / float in [0, 1).
int64_t rl_rand_int(void) {
    rl_rand_ensure_init();
    return (int64_t)rand();
}

// Unseeded pseudo-random values from the C library RNG. Full-range non-negative int / float in [0, 1).
double rl_rand_float(void) {
    rl_rand_ensure_init();
    return (double)rand() / (double)RAND_MAX;
}

// Fair coin flip / flip that is true with probability `weight`.
bool rl_rand_bool(void) {
    rl_rand_ensure_init();
    return rand() % 2 == 0;
}

// Fair coin flip / flip that is true with probability `weight`.
bool rl_rand_bool_weighted(double weight) {
    rl_rand_ensure_init();
    return rl_rand_float() < weight;
}

// Random printable ASCII char / byte in [0, 255].
char rl_rand_char(void) {
    rl_rand_ensure_init();
    return (char)('a' + rand() % 26);
}

// Random printable ASCII char / byte in [0, 255].
int64_t rl_rand_byte(void) {
    rl_rand_ensure_init();
    return (int64_t)(rand() % 256);
}

// Int in [min, max] / float in [min, max).
int64_t rl_rand_int_range(int64_t min, int64_t max) {
    rl_rand_ensure_init();
    if (min >= max) return min;
    return min + (int64_t)(rand() % (uint64_t)(max - min));
}

// Int in [min, max] / float in [min, max).
double rl_rand_float_range(double min, double max) {
    rl_rand_ensure_init();
    return min + (max - min) * rl_rand_float();
}

// Die roll in [1, sides] / int in [0, stop) / stepped range value.
int64_t rl_rand_dice(int64_t sides) {
    rl_rand_ensure_init();
    if (sides <= 0) return 0;
    return 1 + (int64_t)(rand() % (uint64_t)sides);
}

// Die roll in [1, sides] / int in [0, stop) / stepped range value.
int64_t rl_rand_range(int64_t stop) {
    rl_rand_ensure_init();
    if (stop <= 0) return 0;
    return (int64_t)(rand() % (uint64_t)stop);
}

// Die roll in [1, sides] / int in [0, stop) / stepped range value.
int64_t rl_rand_range_step(int64_t start, int64_t stop, int64_t step) {
    rl_rand_ensure_init();
    if (step == 0 || (start < stop && step < 0) || (start > stop && step > 0)) return start;
    uint64_t range;
    if (step > 0) {
        range = (uint64_t)((stop - start + step - 1) / step);
    } else {
        range = (uint64_t)((start - stop - step - 1) / (-step));
    }
    if (range == 0) return start;
    return start + (int64_t)((uint64_t)(rand() % (int)range) * (uint64_t)step);
}

// Random alphanumeric string of `count` chars.
rl_string rl_rand_string(int64_t count) {
    rl_rand_ensure_init();
    if (count <= 0) {
        rl_string result = { .data = "", .len = 0, .rc = 1 };
        return result;
    }
    char *buf = malloc((uint64_t)count + 1);
    for (int64_t i = 0; i < count; i++) {
        buf[i] = (char)('a' + rand() % 26);
    }
    buf[count] = '\0';
    rl_string result = { .data = buf, .len = (uint64_t)count, .rc = 1 };
    return result;
}

// Array of `count` die rolls in [1, sides].
rl_result rl_rand_dices(int64_t count, int64_t sides) {
    rl_rand_ensure_init();
    if (count <= 0) return rl_err_msg(rl_str_literal("count should be 1 or higher", 27));
    if (sides <= 0) return rl_err_msg(rl_str_literal("sides should be 1 or higher", 27));
    int64_t *buf = malloc(count * sizeof(int64_t));
    for (int64_t i = 0; i < count; i++) {
        buf[i] = 1 + rand() % (int)sides;
    }
    rl_array result = { .data = buf, .len = (uint64_t)count, .cap = (uint64_t)count, .elem_size = sizeof(int64_t), .type_tag = RL_TAG_I64 };
    return rl_ok_arr(result);
}

// Array of `count` random bytes.
rl_result rl_rand_bytes(int64_t count) {
    rl_rand_ensure_init();
    if (count <= 0) return rl_err_msg(rl_str_literal("count cannot be less than zero", 29));
    int64_t *buf = malloc(count * sizeof(int64_t));
    for (int64_t i = 0; i < count; i++) {
        buf[i] = (int64_t)(unsigned char)(rand() % 256);
    }
    rl_array result = { .data = buf, .len = (uint64_t)count, .cap = (uint64_t)count, .elem_size = sizeof(int64_t), .type_tag = RL_TAG_I64 };
    return rl_ok_arr(result);
}

// One uniform pick from `arr` (error when empty).
rl_result rl_rand_choice(rl_array arr) {
    if (arr.len == 0) return rl_err_msg(rl_str_literal("array is empty", 14));
    rl_rand_ensure_init();
    uint64_t idx = (uint64_t)(rand() % (int)arr.len);
    int64_t *elems = (int64_t *)arr.data;
    return rl_ok_i64(elems[idx]);
}

// `count` picks with replacement / without replacement.
rl_result rl_rand_choices(rl_array arr, int64_t count) {
    if (arr.len == 0) return rl_err_msg(rl_str_literal("array is empty", 14));
    if (count <= 0) return rl_err_msg(rl_str_literal("count should be 1 or higher", 27));
    rl_rand_ensure_init();
    int64_t *buf = malloc(count * sizeof(int64_t));
    int64_t *elems = (int64_t *)arr.data;
    for (int64_t i = 0; i < count; i++) {
        buf[i] = elems[rand() % (int)arr.len];
    }
    rl_array result = { .data = buf, .len = (uint64_t)count, .cap = (uint64_t)count, .elem_size = sizeof(int64_t), .type_tag = arr.type_tag };
    return rl_ok_arr(result);
}

// `count` picks with replacement / without replacement.
rl_result rl_rand_sample(rl_array arr, int64_t count) {
    if (arr.len == 0) return rl_err_msg(rl_str_literal("array is empty", 14));
    if (count <= 0) return rl_err_msg(rl_str_literal("count should be 1 or higher", 27));
    if ((uint64_t)count > arr.len) return rl_err_msg(rl_str_literal("count larger than array", 23));
    rl_rand_ensure_init();
    int64_t *indices = malloc(arr.len * sizeof(int64_t));
    for (uint64_t i = 0; i < arr.len; i++) indices[i] = (int64_t)i;
    for (uint64_t i = arr.len - 1; i > 0; i--) {
        uint64_t j = (uint64_t)(rand() % ((int)i + 1));
        int64_t tmp = indices[i]; indices[i] = indices[j]; indices[j] = tmp;
    }
    int64_t *elems = (int64_t *)arr.data;
    int64_t *buf = malloc(count * sizeof(int64_t));
    for (int64_t i = 0; i < count; i++) {
        buf[i] = elems[indices[i]];
    }
    free(indices);
    rl_array result = { .data = buf, .len = (uint64_t)count, .cap = (uint64_t)count, .elem_size = sizeof(int64_t), .type_tag = arr.type_tag };
    return rl_ok_arr(result);
}

// Shuffled copy of `arr`.
rl_result rl_rand_shuffle(rl_array arr) {
    if (arr.len == 0) return rl_err_msg(rl_str_literal("array is empty", 14));
    rl_rand_ensure_init();
    int64_t *buf = malloc(arr.len * sizeof(int64_t));
    memcpy(buf, arr.data, arr.len * sizeof(int64_t));
    for (uint64_t i = arr.len - 1; i > 0; i--) {
        uint64_t j = (uint64_t)(rand() % ((int)i + 1));
        int64_t tmp = buf[i]; buf[i] = buf[j]; buf[j] = tmp;
    }
    rl_array result = { .data = buf, .len = arr.len, .cap = arr.cap, .elem_size = sizeof(int64_t), .type_tag = arr.type_tag };
    return rl_ok_arr(result);
}

// ---- collections (rl_string key wrappers) ----

// Add / remove / membership test, each reporting success as a result.
rl_result rl_set_add_s(rl_set *s, rl_value value) {
    rl_set_add(s, value);
    return rl_ok_i64(1);
}

// Add / remove / membership test, each reporting success as a result.
rl_result rl_set_remove_s(rl_set *s, rl_value value) {
    rl_set_remove(s, value);
    return rl_ok_i64(1);
}

// Add / remove / membership test, each reporting success as a result.
rl_result rl_set_contains_s(rl_set s, rl_value value) {
    return rl_ok_bool(rl_set_contains(s, value));
}

// Copy all elements into a fresh array.
rl_array rl_set_to_array(rl_set s) {
    rl_value *buf = malloc(s.len * sizeof(rl_value));
    memcpy(buf, s.data, s.len * sizeof(rl_value));
    rl_array arr = { .data = buf, .len = s.len, .cap = s.len, .elem_size = sizeof(rl_value), .type_tag = RL_TAG_I64 };
    return arr;
}

// Membership test / removal returning a result; lookup returning the value or an error when the key is missing.
rl_result rl_map_contains_s(rl_map m, rl_string key) {
    char buf[key.len + 1];
    memcpy(buf, key.data, key.len);
    buf[key.len] = '\0';
    return rl_ok_bool(rl_map_contains(m, buf));
}

// Membership test / removal returning a result; lookup returning the value or an error when the key is missing.
rl_result rl_map_remove_s(rl_map m, rl_string key) {
    char buf[key.len + 1];
    memcpy(buf, key.data, key.len);
    buf[key.len] = '\0';
    rl_map_remove(&m, buf);
    return rl_ok(m);
}

// Membership test / removal returning a result; lookup returning the value or an error when the key is missing.
rl_result rl_map_get_s(rl_map m, rl_string key) {
    char buf[key.len + 1];
    memcpy(buf, key.data, key.len);
    buf[key.len] = '\0';
    if (!rl_map_contains(m, buf)) {
        return rl_err_msg(rl_str_literal("key not found in map", 20));
    }
    rl_value v = rl_map_get(m, buf);
    switch (v.tag) {
        case RL_VTAG_NULL: return rl_ok_null();
        case RL_VTAG_I64: return rl_ok_i64(v.data.i64);
        case RL_VTAG_F64: return rl_ok_f64(v.data.f64);
        case RL_VTAG_BOOL: return rl_ok_bool(v.data.boolean);
        case RL_VTAG_STR: return rl_ok_str(v.data.str);
        case RL_VTAG_ARR: return rl_ok_arr(v.data.arr);
        case RL_VTAG_CLOSURE: {
            rl_result r = { .is_ok = true, .tag = RL_TAG_CLOSURE, .data.closure = v.data.closure, .err_code = 0 };
            return r;
        }
        default: return rl_ok_i64(v.data.i64);
    }
}

// Fresh arrays holding copies of all keys / all values.
rl_array rl_map_keys_s(rl_map m) {
    int64_t *buf = malloc(m.len * sizeof(int64_t));
    for (uint64_t i = 0; i < m.len; i++) {
        uint64_t len = strlen(m.entries[i].key);
        char *s = malloc(len + 1);
        memcpy(s, m.entries[i].key, len + 1);
        buf[i] = (int64_t)(uintptr_t)s;
    }
    return rl_arr_from_vals(buf, m.len, sizeof(int64_t));
}

// Fresh arrays holding copies of all keys / all values.
rl_array rl_map_values_s(rl_map m) {
    rl_value *buf = malloc(m.len * sizeof(rl_value));
    for (uint64_t i = 0; i < m.len; i++) {
        buf[i] = m.entries[i].value;
    }
    rl_array arr = { .data = buf, .len = m.len, .cap = m.len, .elem_size = sizeof(rl_value), .type_tag = RL_TAG_I64 };
    return arr;
}

// New map holding `b` layered over `a` (`b` wins on conflicts).
rl_map rl_map_merge_s(rl_map a, rl_map b) {
    rl_map result;
    result.len = a.len;
    result.cap = a.cap ? a.cap : 8;
    if (result.cap < a.len + b.len) result.cap = (a.len + b.len) * 2;
    result.entries = malloc(result.cap * sizeof(rl_map_entry));
    memcpy(result.entries, a.entries, a.len * sizeof(rl_map_entry));
    for (uint64_t i = 0; i < b.len; i++) {
        bool found = false;
        for (uint64_t j = 0; j < result.len; j++) {
            if (strcmp(result.entries[j].key, b.entries[i].key) == 0) {
                result.entries[j].value = b.entries[i].value;
                found = true;
                break;
            }
        }
        if (!found) {
            if (result.len >= result.cap) {
                result.cap *= 2;
                result.entries = realloc(result.entries, result.cap * sizeof(rl_map_entry));
            }
            result.entries[result.len++] = b.entries[i];
        }
    }
    return result;
}

// Array of single-entry maps, one per key.
rl_array rl_map_to_array_s(rl_map m) {
    rl_value *buf = malloc(m.len * sizeof(rl_value));
    for (uint64_t i = 0; i < m.len; i++) {
        buf[i] = m.entries[i].value;
    }
    rl_array arr = { .data = buf, .len = m.len, .cap = m.len, .elem_size = sizeof(rl_value), .type_tag = RL_TAG_I64 };
    return arr;
}

// ---- set algebra (batch 2) ----

// Forward declarations for array helpers defined further below.
static rl_result _rl_arr_elem_as_result(rl_array a, uint64_t idx, int32_t tag);
static int64_t _rl_arr_read_i64(rl_array a, uint64_t i);

// Union: every element of `a` plus those of `b` not already present.
rl_set rl_set_union(rl_set a, rl_set b) {
    rl_set out = rl_set_new();
    for (uint64_t i = 0; i < a.len; i++) rl_set_add(&out, a.data[i]);
    for (uint64_t i = 0; i < b.len; i++) {
        if (!rl_set_contains(out, b.data[i])) rl_set_add(&out, b.data[i]);
    }
    return out;
}

// Intersection: elements present in both sets.
rl_set rl_set_intersection(rl_set a, rl_set b) {
    rl_set out = rl_set_new();
    for (uint64_t i = 0; i < a.len; i++) {
        if (rl_set_contains(b, a.data[i])) rl_set_add(&out, a.data[i]);
    }
    return out;
}

// Difference: elements of `a` not present in `b`.
rl_set rl_set_difference(rl_set a, rl_set b) {
    rl_set out = rl_set_new();
    for (uint64_t i = 0; i < a.len; i++) {
        if (!rl_set_contains(b, a.data[i])) rl_set_add(&out, a.data[i]);
    }
    return out;
}

// Symmetric difference: elements in exactly one of the sets.
rl_set rl_set_symmetric_difference(rl_set a, rl_set b) {
    rl_set out = rl_set_new();
    for (uint64_t i = 0; i < a.len; i++) {
        if (!rl_set_contains(b, a.data[i])) rl_set_add(&out, a.data[i]);
    }
    for (uint64_t i = 0; i < b.len; i++) {
        if (!rl_set_contains(a, b.data[i])) rl_set_add(&out, b.data[i]);
    }
    return out;
}

// True when every element of `a` is also in `b`.
rl_result rl_set_is_subset(rl_set a, rl_set b) {
    for (uint64_t i = 0; i < a.len; i++) {
        if (!rl_set_contains(b, a.data[i])) return rl_ok_bool(false);
    }
    return rl_ok_bool(true);
}

// True when every element of `b` is also in `a`.
rl_result rl_set_is_superset(rl_set a, rl_set b) {
    return rl_set_is_subset(b, a);
}

// Convert a boxed map/set element into a result payload.
static rl_result _rl_value_to_result(rl_value v) {
    switch (v.tag) {
        case RL_VTAG_NULL: return rl_ok_null();
        case RL_VTAG_I64: return rl_ok_i64(v.data.i64);
        case RL_VTAG_F64: return rl_ok_f64(v.data.f64);
        case RL_VTAG_BOOL: return rl_ok_bool(v.data.boolean);
        case RL_VTAG_CHAR: {
            rl_result r = { .is_ok = true, .tag = RL_TAG_CHAR, .err_code = 0 };
            r.data.i64 = v.data.i64;
            return r;
        }
        case RL_VTAG_STR: return rl_ok_str(v.data.str);
        case RL_VTAG_ARR: return rl_ok_arr(v.data.arr);
        case RL_VTAG_MAP:
            if (v.data.map) return rl_ok_map(*v.data.map);
            return rl_ok_null();
        case RL_VTAG_SET:
            if (v.data.set) return rl_ok_set(*v.data.set);
            return rl_ok_null();
        case RL_VTAG_CLOSURE: {
            rl_result r = { .is_ok = true, .tag = RL_TAG_CLOSURE, .err_code = 0 };
            r.data.closure = v.data.closure;
            return r;
        }
        default: return rl_ok_i64(v.data.i64);
    }
}

// Lookup with a boxed default; ok with the value or the default.
rl_result rl_map_get_or_s(rl_map m, rl_string key, rl_value def) {
    if (key.data == NULL) return rl_err(-1);
    char *buf = malloc(key.len + 1);
    if (key.len > 0) memcpy(buf, key.data, key.len);
    buf[key.len] = '\0';
    rl_result out;
    if (rl_map_contains(m, buf)) {
        rl_value v = rl_map_get(m, buf);
        out = _rl_value_to_result(v);
    } else {
        out = _rl_value_to_result(def);
    }
    free(buf);
    return out;
}

// Lookup with insert; stores the default when missing, ok with the value.
rl_result rl_map_get_or_insert_s(rl_map *m, rl_string key, rl_value def) {
    if (key.data == NULL) return rl_err(-1);
    char *buf = malloc(key.len + 1);
    if (key.len > 0) memcpy(buf, key.data, key.len);
    buf[key.len] = '\0';
    rl_result out;
    if (rl_map_contains(*m, buf)) {
        rl_value v = rl_map_get(*m, buf);
        out = _rl_value_to_result(v);
    } else {
        rl_map_set(m, buf, def);
        out = _rl_value_to_result(def);
    }
    free(buf);
    return out;
}

// Legacy int-default variants for dynamically typed defaults.
rl_result rl_map_get_or(rl_map m, rl_string key, int64_t def) {
    rl_value v = { .tag = RL_VTAG_I64, .data.i64 = def };
    return rl_map_get_or_s(m, key, v);
}

// Legacy int-default variant for dynamically typed defaults.
rl_result rl_map_get_or_insert(rl_map *m, rl_string key, int64_t def) {
    rl_value v = { .tag = RL_VTAG_I64, .data.i64 = def };
    return rl_map_get_or_insert_s(m, key, v);
}

// Sift one int element up toward the root (min-heap).
static void _rl_heap_sift_up_i64(char *buf, uint64_t idx, uint64_t es) {
    uint64_t i = idx;
    while (i > 0) {
        uint64_t parent = (i - 1) / 2;
        int64_t a = 0;
        int64_t b = 0;
        memcpy(&a, buf + i * es, es < sizeof(int64_t) ? es : sizeof(int64_t));
        memcpy(&b, buf + parent * es, es < sizeof(int64_t) ? es : sizeof(int64_t));
        if (a < b) {
            char tmp[8];
            memcpy(tmp, buf + i * es, es);
            memcpy(buf + i * es, buf + parent * es, es);
            memcpy(buf + parent * es, tmp, es);
            i = parent;
        } else {
            break;
        }
    }
}

// Sift one float element up toward the root (min-heap).
static void _rl_heap_sift_up_f64(char *buf, uint64_t idx, uint64_t es) {
    uint64_t i = idx;
    while (i > 0) {
        uint64_t parent = (i - 1) / 2;
        double a = 0;
        double b = 0;
        memcpy(&a, buf + i * es, sizeof(double));
        memcpy(&b, buf + parent * es, sizeof(double));
        if (a < b) {
            char tmp[8];
            memcpy(tmp, buf + i * es, es);
            memcpy(buf + i * es, buf + parent * es, es);
            memcpy(buf + parent * es, tmp, es);
            i = parent;
        } else {
            break;
        }
    }
}

// Sift the root down (min-heap) for int elements.
static void _rl_heap_sift_down_i64(char *buf, uint64_t len, uint64_t es) {
    uint64_t i = 0;
    for (;;) {
        uint64_t left = 2 * i + 1;
        uint64_t right = 2 * i + 2;
        uint64_t smallest = i;
        int64_t cur = 0;
        memcpy(&cur, buf + smallest * es, es < sizeof(int64_t) ? es : sizeof(int64_t));
        if (left < len) {
            int64_t lv = 0;
            memcpy(&lv, buf + left * es, es < sizeof(int64_t) ? es : sizeof(int64_t));
            if (lv < cur) {
                smallest = left;
                cur = lv;
            }
        }
        if (right < len) {
            int64_t rv = 0;
            memcpy(&rv, buf + right * es, es < sizeof(int64_t) ? es : sizeof(int64_t));
            if (rv < cur) smallest = right;
        }
        if (smallest != i) {
            char tmp[8];
            memcpy(tmp, buf + i * es, es);
            memcpy(buf + i * es, buf + smallest * es, es);
            memcpy(buf + smallest * es, tmp, es);
            i = smallest;
        } else {
            break;
        }
    }
}

// Sift the root down (min-heap) for float elements.
static void _rl_heap_sift_down_f64(char *buf, uint64_t len, uint64_t es) {
    uint64_t i = 0;
    for (;;) {
        uint64_t left = 2 * i + 1;
        uint64_t right = 2 * i + 2;
        uint64_t smallest = i;
        double cur = 0;
        memcpy(&cur, buf + smallest * es, sizeof(double));
        if (left < len) {
            double lv = 0;
            memcpy(&lv, buf + left * es, sizeof(double));
            if (lv < cur) {
                smallest = left;
                cur = lv;
            }
        }
        if (right < len) {
            double rv = 0;
            memcpy(&rv, buf + right * es, sizeof(double));
            if (rv < cur) smallest = right;
        }
        if (smallest != i) {
            char tmp[8];
            memcpy(tmp, buf + i * es, es);
            memcpy(buf + i * es, buf + smallest * es, es);
            memcpy(buf + smallest * es, tmp, es);
            i = smallest;
        } else {
            break;
        }
    }
}

// Push an int onto the heap; only int and float heaps sift.
rl_result rl_heap_push(rl_array a, int64_t v) {
    uint64_t es = a.elem_size ? (uint64_t)a.elem_size : sizeof(int64_t);
    uint64_t new_len = a.len + 1;
    char *buf = malloc(new_len * es);
    if (a.data && a.len > 0) memcpy(buf, a.data, a.len * es);
    memset(buf + a.len * es, 0, es);
    memcpy(buf + a.len * es, &v, es < sizeof(int64_t) ? es : sizeof(int64_t));
    if (a.type_tag == RL_TAG_F64 && es == sizeof(double)) {
        double fv = (double)v;
        memcpy(buf + a.len * es, &fv, sizeof(double));
        _rl_heap_sift_up_f64(buf, a.len, es);
    } else if (a.type_tag == RL_TAG_I64) {
        _rl_heap_sift_up_i64(buf, a.len, es);
    }
    rl_array out;
    out.data = buf;
    out.len = new_len;
    out.cap = new_len;
    out.elem_size = (int32_t)es;
    out.type_tag = a.type_tag;
    return rl_ok_arr(out);
}

// Push a boxed value onto the heap, preserving width like push_v.
rl_result rl_heap_push_v(rl_array a, rl_value v) {
    rl_result pushed = rl_arr_push_v(a, v);
    if (!pushed.is_ok) return pushed;
    rl_array out = pushed.data.arr;
    uint64_t es = out.elem_size ? (uint64_t)out.elem_size : sizeof(int64_t);
    if (out.len == 0) return pushed;
    if (out.type_tag == RL_TAG_F64 && es == sizeof(double)) {
        _rl_heap_sift_up_f64((char *)out.data, out.len - 1, es);
    } else if (out.type_tag == RL_TAG_I64) {
        _rl_heap_sift_up_i64((char *)out.data, out.len - 1, es);
    }
    pushed.data.arr = out;
    return pushed;
}

// Pop the root; ok with the remaining heap, error when empty.
rl_result rl_heap_pop(rl_array a) {
    if (a.len == 0) return rl_make_err(-1, "heap_pop: called on empty array");
    uint64_t es = a.elem_size ? (uint64_t)a.elem_size : sizeof(int64_t);
    uint64_t new_len = a.len - 1;
    if (new_len == 0) {
        rl_array out;
        out.data = NULL;
        out.len = 0;
        out.cap = 0;
        out.elem_size = (int32_t)es;
        out.type_tag = a.type_tag;
        return rl_ok_arr(out);
    }
    char *buf = malloc(new_len * es);
    memcpy(buf, a.data, new_len * es);
    memcpy(buf, (char *)a.data + (a.len - 1) * es, es);
    if (a.type_tag == RL_TAG_F64 && es == sizeof(double)) {
        _rl_heap_sift_down_f64(buf, new_len, es);
    } else if (a.type_tag == RL_TAG_I64) {
        _rl_heap_sift_down_i64(buf, new_len, es);
    }
    rl_array out;
    out.data = buf;
    out.len = new_len;
    out.cap = new_len;
    out.elem_size = (int32_t)es;
    out.type_tag = a.type_tag;
    return rl_ok_arr(out);
}

// Peek at the root without removing it; error when empty.
rl_result rl_heap_peek(rl_array a) {
    if (a.len == 0) return rl_make_err(-1, "heap_peek: called on empty array");
    uint64_t es = a.elem_size ? (uint64_t)a.elem_size : sizeof(int64_t);
    int64_t v = 0;
    memcpy(&v, a.data, es < sizeof(int64_t) ? es : sizeof(int64_t));
    return rl_ok_i64(v);
}

// Peek with an explicit payload tag for non-int heaps.
rl_result rl_heap_peek_t(rl_array a, int32_t tag) {
    if (a.len == 0) return rl_make_err(-1, "heap_peek: called on empty array");
    return _rl_arr_elem_as_result(a, 0, tag);
}

// Push an int to the front of the deque.
rl_result rl_deque_push_front(rl_array a, int64_t v) {
    return rl_arr_insert(a, 0, v);
}

// Push a boxed value to the front, preserving width like insert_v.
rl_result rl_deque_push_front_v(rl_array a, rl_value v) {
    return rl_arr_insert_v(a, 0, v);
}

// Drop the front element; error when empty.
rl_result rl_deque_pop_front(rl_array a) {
    if (a.len == 0) return rl_make_err(-1, "deque_pop_front: called on empty array");
    return rl_arr_remove(a, 0);
}

// Leftmost insertion point for `v` in a sorted int array.
rl_result rl_bisect_left(rl_array a, int64_t v) {
    uint64_t lo = 0;
    uint64_t hi = a.len;
    while (lo < hi) {
        uint64_t mid = lo + (hi - lo) / 2;
        int64_t mv = _rl_arr_read_i64(a, mid);
        if (mv < v) lo = mid + 1;
        else hi = mid;
    }
    return rl_ok_i64((int64_t)lo);
}

// Rightmost insertion point for `v` in a sorted int array.
rl_result rl_bisect_right(rl_array a, int64_t v) {
    uint64_t lo = 0;
    uint64_t hi = a.len;
    while (lo < hi) {
        uint64_t mid = lo + (hi - lo) / 2;
        int64_t mv = _rl_arr_read_i64(a, mid);
        if (mv <= v) lo = mid + 1;
        else hi = mid;
    }
    return rl_ok_i64((int64_t)lo);
}

// Copy with `v` inserted at its sorted position (after equals).
rl_result rl_sorted_insert(rl_array a, int64_t v) {
    uint64_t es = a.elem_size ? (uint64_t)a.elem_size : sizeof(int64_t);
    uint64_t pos = a.len;
    for (uint64_t i = 0; i < a.len; i++) {
        if (_rl_arr_read_i64(a, i) > v) {
            pos = i;
            break;
        }
    }
    uint64_t new_len = a.len + 1;
    char *buf = malloc(new_len * es);
    char *src = (char *)a.data;
    if (pos > 0 && src) memcpy(buf, src, pos * es);
    memset(buf + pos * es, 0, es);
    memcpy(buf + pos * es, &v, es < sizeof(int64_t) ? es : sizeof(int64_t));
    if (src && pos < a.len) memcpy(buf + (pos + 1) * es, src + pos * es, (a.len - pos) * es);
    rl_array out;
    out.data = buf;
    out.len = new_len;
    out.cap = new_len;
    out.elem_size = (int32_t)es;
    out.type_tag = a.type_tag;
    return rl_ok_arr(out);
}

// ---- array (generic) ----

// Append v / drop and return the last element.
rl_result rl_arr_push(rl_array a, int64_t v) {
    uint64_t new_len = a.len + 1;
    int64_t *buf = malloc(new_len * sizeof(int64_t));
    if (a.data) memcpy(buf, a.data, a.len * sizeof(int64_t));
    buf[a.len] = v;
    return rl_ok_arr(rl_arr_from_vals(buf, new_len, sizeof(int64_t)));
}

// Append a boxed value to an array of any element type; ok with the new
// array, or an error when the value does not fit the element width.
rl_result rl_arr_push_v(rl_array a, rl_value v) {
    uint64_t es = a.elem_size ? (uint64_t)a.elem_size : sizeof(int64_t);
    uint64_t new_len = a.len + 1;
    char *buf = malloc(new_len * es);
    if (a.data && a.len > 0) memcpy(buf, a.data, a.len * es);
    char *slot = buf + a.len * es;
    memset(slot, 0, es);
    switch (v.tag) {
        case RL_VTAG_I64:
            if (es == 1 || es == 2 || es == 4 || es == 8) {
                memcpy(slot, &v.data.i64, es);
                break;
            }
            free(buf);
            return rl_err(-1);
        case RL_VTAG_F64:
            if (es == sizeof(double)) { memcpy(slot, &v.data.f64, es); break; }
            if (es == sizeof(float)) { float f = (float)v.data.f64; memcpy(slot, &f, es); break; }
            free(buf);
            return rl_err(-1);
        case RL_VTAG_BOOL:
            if (es == 1 || es == 2 || es == 4 || es == 8) {
                uint64_t one = v.data.boolean ? 1 : 0;
                memcpy(slot, &one, es);
                break;
            }
            free(buf);
            return rl_err(-1);
        case RL_VTAG_STR:
            if (es == sizeof(rl_string)) { memcpy(slot, &v.data.str, es); break; }
            free(buf);
            return rl_err(-1);
        case RL_VTAG_CHAR:
            if (es == 1 || es == 2 || es == 4 || es == 8) {
                uint64_t code = (uint64_t)v.data.i64;
                memcpy(slot, &code, es);
                break;
            }
            free(buf);
            return rl_err(-1);
        default:
            free(buf);
            return rl_err(-1);
    }
    rl_array out;
    out.data = buf;
    out.len = new_len;
    out.cap = new_len;
    out.elem_size = (int32_t)es;
    out.type_tag = a.type_tag;
    return rl_ok_arr(out);
}

// Drop the last element; ok with the shortened array, or an error.
rl_result rl_arr_pop(rl_array a) {
    if (a.len == 0) return rl_make_err(-1, "pop from empty array");
    uint64_t es = a.elem_size ? (uint64_t)a.elem_size : sizeof(int64_t);
    uint64_t new_len = a.len - 1;
    char *buf = NULL;
    if (new_len > 0) {
        buf = malloc(new_len * es);
        memcpy(buf, a.data, new_len * es);
    }
    rl_array out;
    out.data = buf;
    out.len = new_len;
    out.cap = new_len;
    out.elem_size = (int32_t)es;
    out.type_tag = a.type_tag;
    return rl_ok_arr(out);
}

// Wrap the stored element at `idx` as a result payload by tag.
static rl_result _rl_arr_elem_as_result(rl_array a, uint64_t idx, int32_t tag) {
    char *slot = (char *)a.data + idx * (a.elem_size ? (uint64_t)a.elem_size : sizeof(int64_t));
    switch (tag) {
        case RL_TAG_F64: {
            double v = 0;
            memcpy(&v, slot, sizeof(double));
            return rl_ok_f64(v);
        }
        case RL_TAG_BOOL: return rl_ok_bool(*(uint8_t *)slot != 0);
        case RL_TAG_STR: {
            rl_string v;
            memcpy(&v, slot, sizeof(rl_string));
            return rl_ok_str(v);
        }
        case RL_TAG_CHAR: {
            rl_result r = { .is_ok = true, .tag = RL_TAG_CHAR, .err_code = 0 };
            r.data.i64 = (int64_t)(*(uint8_t *)slot);
            return r;
        }
        default: {
            uint64_t es = a.elem_size ? (uint64_t)a.elem_size : sizeof(int64_t);
            int64_t v = 0;
            memcpy(&v, slot, es < sizeof(int64_t) ? es : sizeof(int64_t));
            return rl_ok_i64(v);
        }
    }
}

// First element by tag, or an error when empty.
rl_result rl_arr_first_t(rl_array a, int32_t tag) {
    if (a.len == 0) return rl_make_err(-1, "first of empty array");
    return _rl_arr_elem_as_result(a, 0, tag);
}

// Last element by tag, or an error when empty.
rl_result rl_arr_last_t(rl_array a, int32_t tag) {
    if (a.len == 0) return rl_make_err(-1, "last of empty array");
    return _rl_arr_elem_as_result(a, a.len - 1, tag);
}

// Compare a stored element against a boxed value.
static bool _rl_arr_elem_eq(rl_array a, uint64_t idx, rl_value v) {
    uint64_t es = a.elem_size ? (uint64_t)a.elem_size : sizeof(int64_t);
    char *slot = (char *)a.data + idx * es;
    switch (v.tag) {
        case RL_VTAG_I64: {
            int64_t stored = 0;
            memcpy(&stored, slot, es < sizeof(int64_t) ? es : sizeof(int64_t));
            return stored == v.data.i64;
        }
        case RL_VTAG_F64: {
            if (es != sizeof(double)) return false;
            double stored = 0;
            memcpy(&stored, slot, sizeof(double));
            return stored == v.data.f64;
        }
        case RL_VTAG_BOOL:
            return es == 1 && *(uint8_t *)slot == (v.data.boolean ? 1 : 0);
        case RL_VTAG_CHAR:
            return es == 1 && *(uint8_t *)slot == (uint8_t)v.data.i64;
        case RL_VTAG_STR: {
            if (es != sizeof(rl_string)) return false;
            rl_string stored;
            memcpy(&stored, slot, sizeof(rl_string));
            return stored.len == v.data.str.len
                && memcmp(stored.data, v.data.str.data, stored.len) == 0;
        }
        default: return false;
    }
}

// Membership test over any element type.
rl_result rl_arr_contains_v(rl_array a, rl_value v) {
    for (uint64_t i = 0; i < a.len; i++) {
        if (_rl_arr_elem_eq(a, i, v)) return rl_ok_bool(true);
    }
    return rl_ok_bool(false);
}

// First index over any element type, or -1 when absent.
rl_result rl_arr_index_of_v(rl_array a, rl_value v) {
    for (uint64_t i = 0; i < a.len; i++) {
        if (_rl_arr_elem_eq(a, i, v)) return rl_ok_i64((int64_t)i);
    }
    return rl_ok_i64(-1);
}

// Insert `v` at `idx` / drop the element at `idx`.
// Insert a boxed value at `idx` for arrays of any element type.
rl_result rl_arr_insert_v(rl_array a, int64_t idx, rl_value v) {
    uint64_t es = a.elem_size ? (uint64_t)a.elem_size : sizeof(int64_t);
    if (idx < 0) idx = 0;
    if ((uint64_t)idx > a.len) idx = (int64_t)a.len;
    uint64_t new_len = a.len + 1;
    char *buf = malloc(new_len * es);
    char *src = (char *)a.data;
    if (idx > 0 && src) memcpy(buf, src, (uint64_t)idx * es);
    char *slot = buf + idx * es;
    memset(slot, 0, es);
    switch (v.tag) {
        case RL_VTAG_I64: memcpy(slot, &v.data.i64, es < 8 ? es : 8); break;
        case RL_VTAG_F64:
            if (es == sizeof(double)) memcpy(slot, &v.data.f64, es);
            else { int64_t i = (int64_t)v.data.f64; memcpy(slot, &i, es < 8 ? es : 8); }
            break;
        case RL_VTAG_BOOL: { uint64_t one = v.data.boolean ? 1 : 0; memcpy(slot, &one, es < 8 ? es : 8); break; }
        case RL_VTAG_STR:
            if (es == sizeof(rl_string)) memcpy(slot, &v.data.str, es);
            else { free(buf); return rl_err(-1); }
            break;
        case RL_VTAG_CHAR: { uint64_t code = (uint64_t)v.data.i64; memcpy(slot, &code, es < 8 ? es : 8); break; }
        default: free(buf); return rl_err(-1);
    }
    if (src && (uint64_t)idx < a.len) memcpy(buf + (idx + 1) * es, src + idx * es, (a.len - (uint64_t)idx) * es);
    rl_array out;
    out.data = buf;
    out.len = new_len;
    out.cap = new_len;
    out.elem_size = (int32_t)es;
    out.type_tag = a.type_tag;
    return rl_ok_arr(out);
}

rl_result rl_arr_insert(rl_array a, int64_t idx, int64_t v) {
    if (idx < 0) idx = 0;
    if ((uint64_t)idx > a.len) idx = (int64_t)a.len;
    uint64_t new_len = a.len + 1;
    int64_t *buf = malloc(new_len * sizeof(int64_t));
    int64_t *src = (int64_t *)a.data;
    if (idx > 0 && src) memcpy(buf, src, (uint64_t)idx * sizeof(int64_t));
    buf[idx] = v;
    if (src && (uint64_t)idx < a.len) memcpy(buf + idx + 1, src + idx, (a.len - (uint64_t)idx) * sizeof(int64_t));
    return rl_ok_arr(rl_arr_from_vals(buf, new_len, sizeof(int64_t)));
}

// Insert `v` at `idx` / drop the element at `idx`.
rl_result rl_arr_remove(rl_array a, int64_t idx) {
    if (idx < 0 || (uint64_t)idx >= a.len) return rl_make_err(-1, "index out of bounds");
    uint64_t es = a.elem_size ? (uint64_t)a.elem_size : sizeof(int64_t);
    char *src = (char *)a.data;
    uint64_t new_len = a.len - 1;
    char *buf = malloc(new_len * es);
    if (idx > 0) memcpy(buf, src, (uint64_t)idx * es);
    if ((uint64_t)idx < a.len - 1) memcpy(buf + idx * es, src + (idx + 1) * es, (a.len - (uint64_t)idx - 1) * es);
    rl_array out;
    out.data = buf;
    out.len = new_len;
    out.cap = new_len;
    out.elem_size = (int32_t)es;
    out.type_tag = a.type_tag;
    return rl_ok_arr(out);
}

// Reversed / concatenated copies (inputs unchanged).
rl_array rl_arr_reverse(rl_array a) {
    uint64_t es = a.elem_size ? (uint64_t)a.elem_size : sizeof(int64_t);
    char *buf = malloc(a.len * es);
    for (uint64_t i = 0; i < a.len; i++) {
        memcpy(buf + i * es, (char *)a.data + (a.len - 1 - i) * es, es);
    }
    rl_array out;
    out.data = buf;
    out.len = a.len;
    out.cap = a.len;
    out.elem_size = (int32_t)es;
    out.type_tag = a.type_tag;
    return out;
}

// Reversed / concatenated copies (inputs unchanged).
rl_array rl_arr_concat(rl_array a, rl_array b) {
    uint64_t es = a.elem_size ? (uint64_t)a.elem_size : sizeof(int64_t);
    uint64_t new_len = a.len + b.len;
    char *buf = malloc(new_len * es);
    if (a.data) memcpy(buf, a.data, a.len * es);
    if (b.data) {
        uint64_t bes = b.elem_size ? (uint64_t)b.elem_size : sizeof(int64_t);
        uint64_t copy_es = bes < es ? bes : es;
        for (uint64_t i = 0; i < b.len; i++) {
            memset(buf + (a.len + i) * es, 0, es);
            memcpy(buf + (a.len + i) * es, (char *)b.data + i * bes, copy_es);
        }
    }
    rl_array out;
    out.data = buf;
    out.len = new_len;
    out.cap = new_len;
    out.elem_size = (int32_t)es;
    out.type_tag = a.type_tag;
    return out;
}

// First / last element, or an error when empty.
rl_result rl_arr_first(rl_array a) {
    if (a.len == 0) return rl_make_err(-1, "first of empty array");
    return rl_ok_i64(((int64_t *)a.data)[0]);
}

// First / last element, or an error when empty.
rl_result rl_arr_last(rl_array a) {
    if (a.len == 0) return rl_make_err(-1, "last of empty array");
    return rl_ok_i64(((int64_t *)a.data)[a.len - 1]);
}

// Element equality by tag: strings compare contents, everything else
// compares raw bytes (ints, floats and bools have no padding).
static bool _rl_arr_elems_eq(uint64_t es, int32_t tag, const char *x, const char *y) {
    if (tag == RL_TAG_STR) {
        rl_string a;
        rl_string b;
        memcpy(&a, x, sizeof(rl_string));
        memcpy(&b, y, sizeof(rl_string));
        return a.len == b.len && memcmp(a.data, b.data, a.len) == 0;
    }
    return memcmp(x, y, es) == 0;
}

// Copy with duplicates removed, keeping first-seen order.
rl_array rl_arr_unique_t(rl_array a, int32_t tag) {
    uint64_t es = a.elem_size ? (uint64_t)a.elem_size : sizeof(int64_t);
    char *buf = malloc(a.len * es);
    uint64_t w = 0;
    for (uint64_t i = 0; i < a.len; i++) {
        bool found = false;
        for (uint64_t j = 0; j < w; j++) {
            if (_rl_arr_elems_eq(es, tag, buf + j * es, (char *)a.data + i * es)) {
                found = true;
                break;
            }
        }
        if (!found) {
            memcpy(buf + w * es, (char *)a.data + i * es, es);
            w++;
        }
    }
    rl_array out;
    out.data = buf;
    out.len = w;
    out.cap = a.len;
    out.elem_size = (int32_t)es;
    out.type_tag = a.type_tag;
    return out;
}

// Copy of `[start, end)` with clamping.
rl_array rl_arr_slice(rl_array a, int64_t start, int64_t end) {
    uint64_t es = a.elem_size ? (uint64_t)a.elem_size : sizeof(int64_t);
    if (start < 0) start = 0;
    if (end > (int64_t)a.len) end = (int64_t)a.len;
    if (start >= end) {
        rl_array empty = { .data = NULL, .len = 0, .cap = 0, .elem_size = (int32_t)es, .type_tag = a.type_tag };
        return empty;
    }
    uint64_t len = (uint64_t)(end - start);
    char *buf = malloc(len * es);
    memcpy(buf, (char *)a.data + start * es, len * es);
    rl_array out;
    out.data = buf;
    out.len = len;
    out.cap = len;
    out.elem_size = (int32_t)es;
    out.type_tag = a.type_tag;
    return out;
}

// Membership test / first index (or -1) wrapped as results.
rl_result rl_arr_contains(rl_array a, int64_t v) {
    int64_t *src = (int64_t *)a.data;
    for (uint64_t i = 0; i < a.len; i++) {
        if (src[i] == v) return rl_ok_bool(true);
    }
    return rl_ok_bool(false);
}

// Membership test / first index (or -1) wrapped as results.
rl_result rl_arr_index_of(rl_array a, int64_t v) {
    int64_t *src = (int64_t *)a.data;
    for (uint64_t i = 0; i < a.len; i++) {
        if (src[i] == v) return rl_ok_i64((int64_t)i);
    }
    return rl_ok_i64(-1);
}

// Fresh array of `count` copies of `v`.
rl_array rl_arr_fill(int64_t v, int64_t count) {
    if (count <= 0) return rl_arr_from_vals(NULL, 0, sizeof(int64_t));
    int64_t *buf = malloc((uint64_t)count * sizeof(int64_t));
    for (int64_t i = 0; i < count; i++) buf[i] = v;
    return rl_arr_from_vals(buf, (uint64_t)count, sizeof(int64_t));
}

// Fresh array of `count` copies of a boxed value; element width follows
// the value tag.
rl_array rl_arr_fill_v(rl_value v, int64_t count) {
    uint64_t es = sizeof(int64_t);
    int32_t tag = RL_TAG_I64;
    switch (v.tag) {
        case RL_VTAG_F64: es = sizeof(double); tag = RL_TAG_F64; break;
        case RL_VTAG_BOOL: es = sizeof(bool); tag = RL_TAG_BOOL; break;
        case RL_VTAG_STR: es = sizeof(rl_string); tag = RL_TAG_STR; break;
        case RL_VTAG_ARR: es = sizeof(rl_array); tag = RL_TAG_ARR; break;
        default: break;
    }
    if (count <= 0) {
        rl_array empty = { .data = NULL, .len = 0, .cap = 0, .elem_size = (int32_t)es, .type_tag = tag };
        return empty;
    }
    char *buf = malloc((uint64_t)count * es);
    for (int64_t i = 0; i < count; i++) {
        char *slot = buf + i * es;
        memset(slot, 0, es);
        switch (v.tag) {
            case RL_VTAG_F64: memcpy(slot, &v.data.f64, es); break;
            case RL_VTAG_BOOL: *(uint8_t *)slot = v.data.boolean ? 1 : 0; break;
            case RL_VTAG_STR: memcpy(slot, &v.data.str, es); break;
            case RL_VTAG_ARR: memcpy(slot, &v.data.arr, es); break;
            default: memcpy(slot, &v.data.i64, es < 8 ? es : 8); break;
        }
    }
    rl_array out;
    out.data = buf;
    out.len = (uint64_t)count;
    out.cap = (uint64_t)count;
    out.elem_size = (int32_t)es;
    out.type_tag = tag;
    return out;
}

// Stepped integer sequence, or an error for a zero step.
rl_result rl_arr_range(int64_t start, int64_t end, int64_t step) {
    if (step == 0) {
        return rl_make_err(-1, "arr_range: step must be positive, got 0");
    }
    if (step < 0) {
        char msg[64];
        snprintf(msg, sizeof(msg), "arr_range: step must be positive, got %ld", (long)step);
        uint64_t len = strlen(msg);
        char *buf = malloc(len + 1);
        memcpy(buf, msg, len + 1);
        rl_string s = { .data = buf, .len = len, .rc = 1 };
        rl_result r = { .is_ok = false, .tag = RL_TAG_STR, .data.str = s, .err_code = -1 };
        return r;
    }
    uint64_t cap = 16;
    int64_t *buf = malloc(cap * sizeof(int64_t));
    uint64_t count = 0;
    for (int64_t i = start; i < end; i += step) {
        if (count >= cap) { cap *= 2; buf = realloc(buf, cap * sizeof(int64_t)); }
        buf[count++] = i;
    }
    return rl_ok_arr(rl_arr_from_vals(buf, count, sizeof(int64_t)));
}

// Sum / product / max / min, erroring on empty input.
// Read one element as int64 (narrower widths widen).
static int64_t _rl_arr_read_i64(rl_array a, uint64_t i) {
    uint64_t es = a.elem_size ? (uint64_t)a.elem_size : sizeof(int64_t);
    int64_t v = 0;
    if (a.data) memcpy(&v, (char *)a.data + i * es, es < sizeof(int64_t) ? es : sizeof(int64_t));
    return v;
}

// Read one element as float64.
static double _rl_arr_read_f64(rl_array a, uint64_t i) {
    uint64_t es = a.elem_size ? (uint64_t)a.elem_size : sizeof(int64_t);
    double v = 0;
    if (a.data) memcpy(&v, (char *)a.data + i * es, es < sizeof(double) ? es : sizeof(double));
    return v;
}

rl_result rl_arr_sum_t(rl_array a, int32_t tag) {
    if (tag == RL_TAG_F64) {
        double sum = 0;
        for (uint64_t i = 0; i < a.len; i++) sum += _rl_arr_read_f64(a, i);
        return rl_ok_f64(sum);
    }
    int64_t sum = 0;
    for (uint64_t i = 0; i < a.len; i++) sum += _rl_arr_read_i64(a, i);
    return rl_ok_i64(sum);
}

// Sum / product / max / min, erroring on empty input.
rl_result rl_arr_product_t(rl_array a, int32_t tag) {
    if (tag == RL_TAG_F64) {
        double prod = 1;
        for (uint64_t i = 0; i < a.len; i++) prod *= _rl_arr_read_f64(a, i);
        return rl_ok_f64(prod);
    }
    int64_t prod = 1;
    for (uint64_t i = 0; i < a.len; i++) prod *= _rl_arr_read_i64(a, i);
    return rl_ok_i64(prod);
}

// Sum / product / max / min, erroring on empty input.
rl_result rl_arr_max_t(rl_array a, int32_t tag) {
    if (a.len == 0) return rl_make_err(-1, "max of empty array");
    if (tag == RL_TAG_F64) {
        double max = _rl_arr_read_f64(a, 0);
        for (uint64_t i = 1; i < a.len; i++) {
            double v = _rl_arr_read_f64(a, i);
            if (v > max) max = v;
        }
        return rl_ok_f64(max);
    }
    int64_t max = _rl_arr_read_i64(a, 0);
    for (uint64_t i = 1; i < a.len; i++) {
        int64_t v = _rl_arr_read_i64(a, i);
        if (v > max) max = v;
    }
    return rl_ok_i64(max);
}

// Sum / product / max / min, erroring on empty input.
rl_result rl_arr_min_t(rl_array a, int32_t tag) {
    if (a.len == 0) return rl_make_err(-1, "min of empty array");
    if (tag == RL_TAG_F64) {
        double min = _rl_arr_read_f64(a, 0);
        for (uint64_t i = 1; i < a.len; i++) {
            double v = _rl_arr_read_f64(a, i);
            if (v < min) min = v;
        }
        return rl_ok_f64(min);
    }
    int64_t min = _rl_arr_read_i64(a, 0);
    for (uint64_t i = 1; i < a.len; i++) {
        int64_t v = _rl_arr_read_i64(a, i);
        if (v < min) min = v;
    }
    return rl_ok_i64(min);
}

// Compare two int64 elements for qsort (ascending).
static int rl_arr_cmp_i64(const void *a, const void *b) {
    int64_t va = *(const int64_t *)a;
    int64_t vb = *(const int64_t *)b;
    return (va > vb) - (va < vb);
}

// Compare two float64 elements for qsort (ascending).
static int rl_arr_cmp_f64(const void *a, const void *b) {
    double va = *(const double *)a;
    double vb = *(const double *)b;
    return (va > vb) - (va < vb);
}

// Compare two string elements for qsort (ascending).
static int rl_arr_cmp_str(const void *a, const void *b) {
    rl_string sa = *(const rl_string *)a;
    rl_string sb = *(const rl_string *)b;
    uint64_t min_len = sa.len < sb.len ? sa.len : sb.len;
    int cmp = memcmp(sa.data, sb.data, min_len);
    if (cmp != 0) return cmp;
    return (sa.len > sb.len) - (sa.len < sb.len);
}

// Sorted copy (input unchanged); element kind by tag.
rl_array rl_arr_sort_t(rl_array a, int32_t tag) {
    uint64_t es = a.elem_size ? (uint64_t)a.elem_size : sizeof(int64_t);
    if (a.len == 0) {
        rl_array empty = { .data = NULL, .len = 0, .cap = 0, .elem_size = (int32_t)es, .type_tag = tag };
        return empty;
    }
    if (tag == RL_TAG_F64 && es == sizeof(double)) {
        double *buf = malloc(a.len * sizeof(double));
        if (a.data) memcpy(buf, a.data, a.len * sizeof(double));
        qsort(buf, a.len, sizeof(double), rl_arr_cmp_f64);
        rl_array result = { .data = buf, .len = a.len, .cap = a.len, .elem_size = sizeof(double), .type_tag = RL_TAG_F64 };
        return result;
    }
    if (tag == RL_TAG_STR && es == sizeof(rl_string)) {
        rl_string *buf = malloc(a.len * sizeof(rl_string));
        if (a.data) memcpy(buf, a.data, a.len * sizeof(rl_string));
        qsort(buf, a.len, sizeof(rl_string), rl_arr_cmp_str);
        rl_array result = { .data = buf, .len = a.len, .cap = a.len, .elem_size = sizeof(rl_string), .type_tag = RL_TAG_STR };
        return result;
    }
    int64_t *buf = malloc(a.len * sizeof(int64_t));
    for (uint64_t i = 0; i < a.len; i++) buf[i] = _rl_arr_read_i64(a, i);
    qsort(buf, a.len, sizeof(int64_t), rl_arr_cmp_i64);
    return rl_arr_from_vals(buf, a.len, sizeof(int64_t));
}

// One-level flatten of nested arrays; element width follows the first
// inner array.
rl_array rl_arr_flatten(rl_array a) {
    uint64_t outer_es = a.elem_size ? (uint64_t)a.elem_size : sizeof(rl_array);
    uint64_t cap = 16;
    uint64_t es = sizeof(int64_t);
    int32_t tag = RL_TAG_I64;
    // Size pass: find the inner width first.
    for (uint64_t i = 0; i < a.len; i++) {
        rl_array inner;
        memset(&inner, 0, sizeof(inner));
        memcpy(&inner, (char *)a.data + i * outer_es,
            outer_es < sizeof(inner) ? outer_es : sizeof(inner));
        if (inner.elem_size) {
            es = (uint64_t)inner.elem_size;
            tag = inner.type_tag;
            break;
        }
    }
    char *buf = malloc(cap * es);
    uint64_t count = 0;
    uint64_t total = 0;
    for (uint64_t i = 0; i < a.len; i++) {
        rl_array inner;
        memset(&inner, 0, sizeof(inner));
        memcpy(&inner, (char *)a.data + i * outer_es,
            outer_es < sizeof(inner) ? outer_es : sizeof(inner));
        total += inner.len;
    }
    if (total > cap) {
        cap = total;
        buf = realloc(buf, cap * es);
    }
    for (uint64_t i = 0; i < a.len; i++) {
        rl_array inner;
        memset(&inner, 0, sizeof(inner));
        memcpy(&inner, (char *)a.data + i * outer_es,
            outer_es < sizeof(inner) ? outer_es : sizeof(inner));
        uint64_t inner_es = inner.elem_size ? (uint64_t)inner.elem_size : es;
        if (inner_es != es) continue;
        for (uint64_t j = 0; j < inner.len; j++) {
            memcpy(buf + count * es, (char *)inner.data + j * inner_es, es);
            count++;
        }
    }
    rl_array out;
    out.data = buf;
    out.len = count;
    out.cap = cap;
    out.elem_size = (int32_t)es;
    out.type_tag = tag;
    return out;
}

// ---- array batch 2: chunk / windows / swap / cycle_take ----

// Split into chunks of `size`; the last chunk may be smaller.
rl_result rl_arr_chunk(rl_array a, int64_t size) {
    if (size <= 0) {
        int n = snprintf(NULL, 0, "arr_chunk: size must be positive, got %ld", (long)size);
        char *msg = malloc((uint64_t)n + 1);
        snprintf(msg, (uint64_t)n + 1, "arr_chunk: size must be positive, got %ld", (long)size);
        return rl_err_msg((rl_string){ .data = msg, .len = (uint64_t)n, .rc = 1 });
    }
    uint64_t es = a.elem_size ? (uint64_t)a.elem_size : sizeof(int64_t);
    uint64_t sz = (uint64_t)size;
    if (a.len == 0) {
        rl_array out = { .data = NULL, .len = 0, .cap = 0, .elem_size = sizeof(rl_array), .type_tag = RL_TAG_ARR };
        return rl_ok_arr(out);
    }
    uint64_t chunk_n = (a.len + sz - 1) / sz;
    rl_array *buf = malloc(chunk_n * sizeof(rl_array));
    for (uint64_t c = 0; c < chunk_n; c++) {
        uint64_t start = c * sz;
        uint64_t len = a.len - start < sz ? a.len - start : sz;
        char *cbuf = malloc(len * es);
        memcpy(cbuf, (char *)a.data + start * es, len * es);
        buf[c].data = cbuf;
        buf[c].len = len;
        buf[c].cap = len;
        buf[c].elem_size = (int32_t)es;
        buf[c].type_tag = a.type_tag;
    }
    rl_array out;
    out.data = buf;
    out.len = chunk_n;
    out.cap = chunk_n;
    out.elem_size = sizeof(rl_array);
    out.type_tag = RL_TAG_ARR;
    return rl_ok_arr(out);
}

// Sliding windows of `size`; error when size exceeds the length.
rl_result rl_arr_windows(rl_array a, int64_t size) {
    if (size <= 0) {
        int n = snprintf(NULL, 0, "arr_windows: size must be positive, got %ld", (long)size);
        char *msg = malloc((uint64_t)n + 1);
        snprintf(msg, (uint64_t)n + 1, "arr_windows: size must be positive, got %ld", (long)size);
        return rl_err_msg((rl_string){ .data = msg, .len = (uint64_t)n, .rc = 1 });
    }
    uint64_t sz = (uint64_t)size;
    if (sz > a.len) {
        int n = snprintf(NULL, 0, "arr_windows: size %ld exceeds array length %llu", (long)size, (unsigned long long)a.len);
        char *msg = malloc((uint64_t)n + 1);
        snprintf(msg, (uint64_t)n + 1, "arr_windows: size %ld exceeds array length %llu", (long)size, (unsigned long long)a.len);
        return rl_err_msg((rl_string){ .data = msg, .len = (uint64_t)n, .rc = 1 });
    }
    uint64_t es = a.elem_size ? (uint64_t)a.elem_size : sizeof(int64_t);
    uint64_t win_n = a.len - sz + 1;
    if (win_n == 0) {
        rl_array out = { .data = NULL, .len = 0, .cap = 0, .elem_size = sizeof(rl_array), .type_tag = RL_TAG_ARR };
        return rl_ok_arr(out);
    }
    rl_array *buf = malloc(win_n * sizeof(rl_array));
    for (uint64_t w = 0; w < win_n; w++) {
        char *cbuf = malloc(sz * es);
        memcpy(cbuf, (char *)a.data + w * es, sz * es);
        buf[w].data = cbuf;
        buf[w].len = sz;
        buf[w].cap = sz;
        buf[w].elem_size = (int32_t)es;
        buf[w].type_tag = a.type_tag;
    }
    rl_array out;
    out.data = buf;
    out.len = win_n;
    out.cap = win_n;
    out.elem_size = sizeof(rl_array);
    out.type_tag = RL_TAG_ARR;
    return rl_ok_arr(out);
}

// Copy with elements `i` and `j` exchanged.
rl_result rl_arr_swap(rl_array a, int64_t i, int64_t j) {
    if (i < 0 || (uint64_t)i >= a.len || j < 0 || (uint64_t)j >= a.len) {
        int n = snprintf(NULL, 0, "arr_swap: index out of bounds: %ld or %ld (len %llu)", (long)i, (long)j, (unsigned long long)a.len);
        char *msg = malloc((uint64_t)n + 1);
        snprintf(msg, (uint64_t)n + 1, "arr_swap: index out of bounds: %ld or %ld (len %llu)", (long)i, (long)j, (unsigned long long)a.len);
        return rl_err_msg((rl_string){ .data = msg, .len = (uint64_t)n, .rc = 1 });
    }
    uint64_t es = a.elem_size ? (uint64_t)a.elem_size : sizeof(int64_t);
    char *buf = malloc(a.len * es);
    if (a.len > 0 && a.data) memcpy(buf, a.data, a.len * es);
    if (i != j) {
        char *tmp = malloc(es);
        memcpy(tmp, buf + i * es, es);
        memcpy(buf + i * es, buf + j * es, es);
        memcpy(buf + j * es, tmp, es);
        free(tmp);
    }
    rl_array out;
    out.data = buf;
    out.len = a.len;
    out.cap = a.len;
    out.elem_size = (int32_t)es;
    out.type_tag = a.type_tag;
    return rl_ok_arr(out);
}

// First `n` elements of the endlessly repeated array.
rl_result rl_arr_cycle_take(rl_array a, int64_t n) {
    if (n < 0) {
        int m = snprintf(NULL, 0, "arr_cycle_take: n must be non-negative, got %ld", (long)n);
        char *msg = malloc((uint64_t)m + 1);
        snprintf(msg, (uint64_t)m + 1, "arr_cycle_take: n must be non-negative, got %ld", (long)n);
        return rl_err_msg((rl_string){ .data = msg, .len = (uint64_t)m, .rc = 1 });
    }
    uint64_t es = a.elem_size ? (uint64_t)a.elem_size : sizeof(int64_t);
    if (a.len == 0 || n == 0) {
        rl_array out = { .data = NULL, .len = 0, .cap = 0, .elem_size = (int32_t)es, .type_tag = a.type_tag };
        return rl_ok_arr(out);
    }
    uint64_t count = (uint64_t)n;
    char *buf = malloc(count * es);
    for (uint64_t k = 0; k < count; k++) {
        memcpy(buf + k * es, (char *)a.data + (k % a.len) * es, es);
    }
    rl_array out;
    out.data = buf;
    out.len = count;
    out.cap = count;
    out.elem_size = (int32_t)es;
    out.type_tag = a.type_tag;
    return rl_ok_arr(out);
}

// Write a boxed fill value into a tuple field of `es` bytes.
static void _rl_write_fill(char *slot, uint64_t es, rl_value fill) {
    memset(slot, 0, es);
    switch (fill.tag) {
        case RL_VTAG_I64:
            memcpy(slot, &fill.data.i64, es < 8 ? es : 8);
            break;
        case RL_VTAG_F64:
            if (es == sizeof(double)) memcpy(slot, &fill.data.f64, es);
            else {
                int64_t iv = (int64_t)fill.data.f64;
                memcpy(slot, &iv, es < 8 ? es : 8);
            }
            break;
        case RL_VTAG_BOOL: {
            uint64_t one = fill.data.boolean ? 1 : 0;
            memcpy(slot, &one, es < 8 ? es : 8);
            break;
        }
        case RL_VTAG_STR:
            if (es == sizeof(rl_string)) memcpy(slot, &fill.data.str, es);
            break;
        case RL_VTAG_CHAR: {
            uint64_t code = (uint64_t)fill.data.i64;
            memcpy(slot, &code, es < 8 ? es : 8);
            break;
        }
        case RL_VTAG_ARR:
            if (es == sizeof(rl_array)) memcpy(slot, &fill.data.arr, es);
            break;
        default:
            break;
    }
}

// Pairwise tuples padded with `fill` up to the longer length.
rl_result rl_arr_zip_longest_t(rl_array a, rl_array b, rl_value fill, uint64_t es_tuple, uint64_t off_b, uint64_t es_a, uint64_t es_b) {
    uint64_t max_len = a.len > b.len ? a.len : b.len;
    if (max_len == 0) {
        rl_array out = { .data = NULL, .len = 0, .cap = 0, .elem_size = (int32_t)es_tuple, .type_tag = RL_TAG_I64 };
        return rl_ok_arr(out);
    }
    char *buf = malloc(max_len * es_tuple);
    for (uint64_t i = 0; i < max_len; i++) {
        char *slot = buf + i * es_tuple;
        memset(slot, 0, es_tuple);
        if (i < a.len && a.data) memcpy(slot, (char *)a.data + i * es_a, es_a);
        else _rl_write_fill(slot, es_a, fill);
        if (i < b.len && b.data) memcpy(slot + off_b, (char *)b.data + i * es_b, es_b);
        else _rl_write_fill(slot + off_b, es_b, fill);
    }
    rl_array out;
    out.data = buf;
    out.len = max_len;
    out.cap = max_len;
    out.elem_size = (int32_t)es_tuple;
    out.type_tag = RL_TAG_I64;
    return rl_ok_arr(out);
}

// Legacy int-fill variant for dynamically typed fills.
rl_result rl_arr_zip_longest(rl_array a, rl_array b, int64_t fill, uint64_t es_tuple, uint64_t off_b, uint64_t es_a, uint64_t es_b) {
    rl_value v = { .tag = RL_VTAG_I64, .data.i64 = fill };
    return rl_arr_zip_longest_t(a, b, v, es_tuple, off_b, es_a, es_b);
}

// ---- closure-consuming array functions ----

// Box one array element as an `rl_result` argument by payload tag.
// Unknown tags fall back to the legacy int64 read.
static rl_result _rl_box_array_elem(rl_array arr, uint64_t i, int32_t tag) {
    uint64_t es = arr.elem_size ? (uint64_t)arr.elem_size : sizeof(int64_t);
    char *slot = arr.data ? (char *)arr.data + i * es : NULL;
    switch (tag) {
        case RL_TAG_F64: {
            double v = 0;
            if (slot) memcpy(&v, slot, es < sizeof(double) ? es : sizeof(double));
            return rl_ok_f64(v);
        }
        case RL_TAG_BOOL:
            return rl_ok_bool(slot && *(uint8_t *)slot != 0);
        case RL_TAG_STR: {
            rl_string v = { .data = NULL, .len = 0, .rc = 0 };
            if (slot) memcpy(&v, slot, sizeof(rl_string));
            return rl_ok_str(v);
        }
        case RL_TAG_ARR: {
            rl_array v = { .data = NULL, .len = 0, .cap = 0, .elem_size = 0, .type_tag = 0 };
            if (slot) memcpy(&v, slot, sizeof(rl_array));
            return rl_ok_arr(v);
        }
        case RL_TAG_MAP: {
            rl_map v = { .entries = NULL, .len = 0, .cap = 0 };
            if (slot) memcpy(&v, slot, sizeof(rl_map));
            return rl_ok_map(v);
        }
        case RL_TAG_SET: {
            rl_set v = { .data = NULL, .len = 0, .cap = 0 };
            if (slot) memcpy(&v, slot, sizeof(rl_set));
            return rl_ok_set(v);
        }
        case RL_TAG_CHAR: {
            int64_t v = 0;
            if (slot) memcpy(&v, slot, es < sizeof(int64_t) ? es : sizeof(int64_t));
            rl_result r = { .is_ok = true, .tag = RL_TAG_CHAR, .err_code = 0 };
            r.data.i64 = v;
            return r;
        }
        case RL_TAG_CLOSURE: {
            rl_closure v;
            memset(&v, 0, sizeof(v));
            if (slot) memcpy(&v, slot, sizeof(rl_closure));
            return rl_ok_closure(v);
        }
        default: {
            int64_t v = 0;
            if (slot) memcpy(&v, slot, es < sizeof(int64_t) ? es : sizeof(int64_t));
            return rl_ok_i64(v);
        }
    }
}

// Predicate truth: bool, nonzero int, or null-as-false like the VM's checks.
static bool _rl_pred_truth(rl_result v) {
    if (!v.is_ok) return false;
    if (v.tag == RL_TAG_BOOL) return v.data.boolean;
    if (v.tag == RL_TAG_I64) return v.data.i64 != 0;
    if (v.tag == RL_TAG_NULL) return false;
    return false;
}

// Keep elements where pred returns true; callback errors propagate.
rl_result rl_arr_filter_closure(rl_array arr, rl_closure pred, int32_t tag) {
    uint64_t es = arr.elem_size ? (uint64_t)arr.elem_size : sizeof(int64_t);
    uint64_t cap = 16;
    char *buf = malloc(cap * es);
    uint64_t count = 0;
    for (uint64_t i = 0; i < arr.len; i++) {
        rl_result arg = _rl_box_array_elem(arr, i, tag);
        rl_result keep = rl_closure_call(pred, &arg, 1);
        if (!keep.is_ok) { free(buf); return keep; }
        if (_rl_pred_truth(keep)) {
            if (count >= cap) { cap *= 2; buf = realloc(buf, cap * es); }
            memcpy(buf + count * es, (char *)arr.data + i * es, es);
            count++;
        }
    }
    rl_array out;
    out.data = buf;
    out.len = count;
    out.cap = cap;
    out.elem_size = (int32_t)es;
    out.type_tag = arr.type_tag;
    return rl_ok_arr(out);
}

// Apply `fn` to every element, collecting the outputs. Outputs must be
// homogeneous; callback errors and mixed outputs propagate as errors.
rl_result rl_arr_map_closure(rl_array arr, rl_closure fn, int32_t tag) {
    uint64_t cap = 16;
    rl_result *outs = malloc(cap * sizeof(rl_result));
    uint64_t count = 0;
    for (uint64_t i = 0; i < arr.len; i++) {
        rl_result arg = _rl_box_array_elem(arr, i, tag);
        rl_result mapped = rl_closure_call(fn, &arg, 1);
        if (!mapped.is_ok) {
            free(outs);
            return mapped;
        }
        if (count >= cap) { cap *= 2; outs = realloc(outs, cap * sizeof(rl_result)); }
        outs[count++] = mapped;
    }
    if (count == 0) {
        free(outs);
        rl_array empty = { .data = NULL, .len = 0, .cap = 0, .elem_size = sizeof(int64_t), .type_tag = RL_TAG_I64 };
        return rl_ok_arr(empty);
    }
    int32_t out_tag = outs[0].tag;
    for (uint64_t i = 1; i < count; i++) {
        if (outs[i].tag != out_tag) {
            free(outs);
            return rl_err(-1);
        }
    }
    uint64_t es;
    switch (out_tag) {
        case RL_TAG_F64: es = sizeof(double); break;
        case RL_TAG_BOOL: es = sizeof(bool); break;
        case RL_TAG_STR: es = sizeof(rl_string); break;
        case RL_TAG_ARR: es = sizeof(rl_array); break;
        case RL_TAG_MAP: es = sizeof(rl_map); break;
        case RL_TAG_SET: es = sizeof(rl_set); break;
        case RL_TAG_CHAR: es = sizeof(int64_t); break;
        default: es = sizeof(int64_t); break;
    }
    char *buf = malloc(count * es);
    for (uint64_t i = 0; i < count; i++) {
        char *slot = buf + i * es;
        switch (out_tag) {
            case RL_TAG_F64: memcpy(slot, &outs[i].data.f64, es); break;
            case RL_TAG_BOOL: { uint8_t b = outs[i].data.boolean ? 1 : 0; memcpy(slot, &b, es); break; }
            case RL_TAG_STR: memcpy(slot, &outs[i].data.str, es); break;
            case RL_TAG_ARR: memcpy(slot, &outs[i].data.arr, es); break;
            case RL_TAG_MAP: memcpy(slot, &outs[i].data.map, es); break;
            case RL_TAG_SET: memcpy(slot, &outs[i].data.set, es); break;
            case RL_TAG_CHAR: memcpy(slot, &outs[i].data.i64, es); break;
            default: memcpy(slot, &outs[i].data.i64, es); break;
        }
    }
    free(outs);
    rl_array out;
    out.data = buf;
    out.len = count;
    out.cap = count;
    out.elem_size = (int32_t)es;
    out.type_tag = out_tag;
    return rl_ok_arr(out);
}

// First element where `pred` returns true (error when none matches).
// First element where `pred` holds (null when none); errors propagate.
rl_result rl_arr_find_closure(rl_array arr, rl_closure pred, int32_t tag) {
    for (uint64_t i = 0; i < arr.len; i++) {
        rl_result arg = _rl_box_array_elem(arr, i, tag);
        rl_result found = rl_closure_call(pred, &arg, 1);
        if (!found.is_ok) return found;
        if (_rl_pred_truth(found)) return arg;
    }
    return rl_ok_null();
}

// Left fold starting from `init`; callback errors propagate.
rl_result rl_arr_reduce_closure(rl_array arr, rl_closure fn, rl_result init, int32_t tag) {
    rl_result acc = init;
    for (uint64_t i = 0; i < arr.len; i++) {
        rl_result args[2] = { acc, _rl_box_array_elem(arr, i, tag) };
        acc = rl_closure_call(fn, args, 2);
        if (!acc.is_ok) return acc;
    }
    return acc;
}

// Index of the first match, or -1 when absent; errors propagate.
rl_result rl_arr_find_index_closure(rl_array arr, rl_closure pred, int32_t tag) {
    for (uint64_t i = 0; i < arr.len; i++) {
        rl_result arg = _rl_box_array_elem(arr, i, tag);
        rl_result found = rl_closure_call(pred, &arg, 1);
        if (!found.is_ok) return found;
        if (_rl_pred_truth(found)) return rl_ok_i64((int64_t)i);
    }
    return rl_ok_i64((int64_t)-1);
}

// True when `pred` holds for all elements; errors propagate.
rl_result rl_arr_all_closure(rl_array arr, rl_closure pred, int32_t tag) {
    for (uint64_t i = 0; i < arr.len; i++) {
        rl_result arg = _rl_box_array_elem(arr, i, tag);
        rl_result ok = rl_closure_call(pred, &arg, 1);
        if (!ok.is_ok) return ok;
        if (!_rl_pred_truth(ok)) return rl_ok_bool(false);
    }
    return rl_ok_bool(true);
}

// True when `pred` holds for at least one element; errors propagate.
rl_result rl_arr_any_closure(rl_array arr, rl_closure pred, int32_t tag) {
    for (uint64_t i = 0; i < arr.len; i++) {
        rl_result arg = _rl_box_array_elem(arr, i, tag);
        rl_result ok = rl_closure_call(pred, &arg, 1);
        if (!ok.is_ok) return ok;
        if (_rl_pred_truth(ok)) return rl_ok_bool(true);
    }
    return rl_ok_bool(false);
}

// Run `fn` for side effects; ok null unless a call errors.
rl_result rl_arr_for_each_closure(rl_array arr, rl_closure fn, int32_t tag) {
    for (uint64_t i = 0; i < arr.len; i++) {
        rl_result arg = _rl_box_array_elem(arr, i, tag);
        rl_result r = rl_closure_call(fn, &arg, 1);
        if (!r.is_ok) return r;
    }
    return rl_ok_null();
}

// Map, then concatenate one level of the resulting arrays. Inner arrays
// must share one element width; callback errors propagate.
rl_result rl_arr_flat_map_closure(rl_array arr, rl_closure fn, int32_t tag) {
    uint64_t cap = 16;
    uint64_t es = 0;
    char *buf = malloc(cap * sizeof(int64_t));
    uint64_t count = 0;
    for (uint64_t i = 0; i < arr.len; i++) {
        rl_result arg = _rl_box_array_elem(arr, i, tag);
        rl_result mapped = rl_closure_call(fn, &arg, 1);
        if (!mapped.is_ok) { free(buf); return mapped; }
        if (mapped.tag != RL_TAG_ARR) {
            free(buf);
            return rl_err(-1);
        }
        rl_array inner = mapped.data.arr;
        uint64_t inner_es = inner.elem_size ? (uint64_t)inner.elem_size : sizeof(int64_t);
        if (es == 0) {
            es = inner_es;
            free(buf);
            cap = inner.len > 16 ? inner.len : 16;
            buf = malloc(cap * es);
        } else if (inner_es != es) {
            free(buf);
            return rl_err(-1);
        }
        for (uint64_t j = 0; j < inner.len; j++) {
            if (count >= cap) { cap *= 2; buf = realloc(buf, cap * es); }
            memcpy(buf + count * es, (char *)inner.data + j * inner_es, es);
            count++;
        }
    }
    if (es == 0) es = sizeof(int64_t);
    rl_array out;
    out.data = buf;
    out.len = count;
    out.cap = cap;
    out.elem_size = (int32_t)es;
    out.type_tag = arr.type_tag;
    return rl_ok_arr(out);
}

// Sort using `cmp(a, b)` returning negative / zero / positive.
// Element width is preserved; callback errors propagate.
rl_result rl_arr_sort_by_closure(rl_array arr, rl_closure cmp, int32_t tag) {
    uint64_t es = arr.elem_size ? (uint64_t)arr.elem_size : sizeof(int64_t);
    uint64_t len = arr.len;
    char *buf = malloc(len * es);
    if (len > 0 && arr.data) memcpy(buf, arr.data, len * es);

    // Simple insertion sort using the comparator closure
    char *key = malloc(es);
    for (uint64_t i = 1; i < len; i++) {
        memcpy(key, buf + i * es, es);
        int64_t j = (int64_t)i - 1;
        while (j >= 0) {
            rl_result args[2] = {
                _rl_box_array_elem((rl_array){ .data = buf + j * es, .len = 1, .cap = 1, .elem_size = (int32_t)es, .type_tag = arr.type_tag }, 0, tag),
                _rl_box_array_elem((rl_array){ .data = key, .len = 1, .cap = 1, .elem_size = (int32_t)es, .type_tag = arr.type_tag }, 0, tag),
            };
            rl_result cmp_result = rl_closure_call(cmp, args, 2);
            if (!cmp_result.is_ok) { free(key); free(buf); return cmp_result; }
            // If cmp(a, b) > 0, swap (ascending order)
            if (cmp_result.tag == RL_TAG_I64 && cmp_result.data.i64 > 0) {
                memcpy(buf + (j + 1) * es, buf + j * es, es);
                j--;
            } else {
                break;
            }
        }
        memcpy(buf + (j + 1) * es, key, es);
    }
    free(key);
    rl_array out;
    out.data = buf;
    out.len = len;
    out.cap = len;
    out.elem_size = (int32_t)es;
    out.type_tag = arr.type_tag;
    return rl_ok_arr(out);
}

// Split into matching and rest by predicate; ok with the pair.
rl_result rl_arr_partition_closure(rl_array arr, rl_closure pred, int32_t tag) {
    uint64_t es = arr.elem_size ? (uint64_t)arr.elem_size : sizeof(int64_t);
    uint64_t cap_m = 16;
    uint64_t cap_r = 16;
    char *buf_m = malloc(cap_m * es);
    char *buf_r = malloc(cap_r * es);
    uint64_t count_m = 0;
    uint64_t count_r = 0;
    for (uint64_t i = 0; i < arr.len; i++) {
        rl_result arg = _rl_box_array_elem(arr, i, tag);
        rl_result keep = rl_closure_call(pred, &arg, 1);
        if (!keep.is_ok) {
            free(buf_m);
            free(buf_r);
            return keep;
        }
        if (_rl_pred_truth(keep)) {
            if (count_m >= cap_m) {
                cap_m *= 2;
                buf_m = realloc(buf_m, cap_m * es);
            }
            memcpy(buf_m + count_m * es, (char *)arr.data + i * es, es);
            count_m++;
        } else {
            if (count_r >= cap_r) {
                cap_r *= 2;
                buf_r = realloc(buf_r, cap_r * es);
            }
            memcpy(buf_r + count_r * es, (char *)arr.data + i * es, es);
            count_r++;
        }
    }
    rl_array left;
    left.data = count_m > 0 ? buf_m : NULL;
    if (count_m == 0) free(buf_m);
    left.len = count_m;
    left.cap = count_m > 0 ? cap_m : 0;
    left.elem_size = (int32_t)es;
    left.type_tag = arr.type_tag;
    rl_array right;
    right.data = count_r > 0 ? buf_r : NULL;
    if (count_r == 0) free(buf_r);
    right.len = count_r;
    right.cap = count_r > 0 ? cap_r : 0;
    right.elem_size = (int32_t)es;
    right.type_tag = arr.type_tag;
    rl_array *pair = malloc(2 * sizeof(rl_array));
    pair[0] = left;
    pair[1] = right;
    rl_array out;
    out.data = pair;
    out.len = 2;
    out.cap = 2;
    out.elem_size = sizeof(rl_array);
    out.type_tag = RL_TAG_ARR;
    return rl_ok_arr(out);
}

// Compare two mapped keys: ints, then floats, then strings, else equal.
static int _rl_key_cmp(rl_result a, rl_result b) {
    if (a.tag == RL_TAG_I64 && b.tag == RL_TAG_I64) {
        if (a.data.i64 < b.data.i64) return -1;
        if (a.data.i64 > b.data.i64) return 1;
        return 0;
    }
    if (a.tag == RL_TAG_F64 && b.tag == RL_TAG_F64) {
        if (a.data.f64 < b.data.f64) return -1;
        if (a.data.f64 > b.data.f64) return 1;
        return 0;
    }
    if (a.tag == RL_TAG_STR && b.tag == RL_TAG_STR) {
        uint64_t min_len = a.data.str.len < b.data.str.len ? a.data.str.len : b.data.str.len;
        int cmp = 0;
        if (min_len > 0) cmp = memcmp(a.data.str.data, b.data.str.data, min_len);
        if (cmp != 0) return cmp < 0 ? -1 : 1;
        if (a.data.str.len < b.data.str.len) return -1;
        if (a.data.str.len > b.data.str.len) return 1;
        return 0;
    }
    return 0;
}

// Element with the greatest mapped key; callback errors propagate.
rl_result rl_arr_max_by_closure(rl_array arr, rl_closure fn, int32_t tag) {
    if (arr.len == 0) return rl_make_err(-1, "arr_max_by: called on empty array");
    rl_result first = _rl_box_array_elem(arr, 0, tag);
    rl_result best_key = rl_closure_call(fn, &first, 1);
    if (!best_key.is_ok) return best_key;
    uint64_t best = 0;
    for (uint64_t i = 1; i < arr.len; i++) {
        rl_result arg = _rl_box_array_elem(arr, i, tag);
        rl_result key = rl_closure_call(fn, &arg, 1);
        if (!key.is_ok) return key;
        if (_rl_key_cmp(best_key, key) < 0) {
            best = i;
            best_key = key;
        }
    }
    return _rl_box_array_elem(arr, best, tag);
}

// Element with the smallest mapped key; callback errors propagate.
rl_result rl_arr_min_by_closure(rl_array arr, rl_closure fn, int32_t tag) {
    if (arr.len == 0) return rl_make_err(-1, "arr_min_by: called on empty array");
    rl_result first = _rl_box_array_elem(arr, 0, tag);
    rl_result best_key = rl_closure_call(fn, &first, 1);
    if (!best_key.is_ok) return best_key;
    uint64_t best = 0;
    for (uint64_t i = 1; i < arr.len; i++) {
        rl_result arg = _rl_box_array_elem(arr, i, tag);
        rl_result key = rl_closure_call(fn, &arg, 1);
        if (!key.is_ok) return key;
        if (_rl_key_cmp(best_key, key) > 0) {
            best = i;
            best_key = key;
        }
    }
    return _rl_box_array_elem(arr, best, tag);
}

// ---- closure-consuming result functions ----
// Apply closures to result payloads, passing the other case through.
// Apply `fn` to an ok result (passes errors through as-is).
rl_result rl_result_map_closure(rl_result val, rl_closure fn) {
    if (!val.is_ok) return val;
    return rl_closure_call(fn, &val, 1);
}

// Apply `fn` to the payload of an error result, wrapping the return as
// the new error (passes ok values through as-is).
rl_result rl_result_map_err_closure(rl_result val, rl_closure fn) {
    if (val.is_ok) return val;
    rl_result arg;
    if (val.tag == RL_TAG_STR) {
        arg = rl_ok_str(val.data.str);
    } else if (val.err_code != 0) {
        arg = rl_ok_i64(val.err_code);
    } else {
        arg = rl_ok_i64(val.data.i64);
    }
    rl_result mapped = rl_closure_call(fn, &arg, 1);
    if (!mapped.is_ok) return mapped;
    rl_result err = { .is_ok = false, .tag = mapped.tag, .err_code = 0 };
    err.data = mapped.data;
    return err;
}

// ---- closure-consuming debug ----
// Benchmark helper that times closure runs.
// Run `fn` `iterations` times; returns elapsed milliseconds.
rl_result rl_bench_closure(rl_closure fn, int64_t iterations) {
    struct timespec start, end;
    clock_gettime(CLOCK_MONOTONIC, &start);
    for (int64_t i = 0; i < iterations; i++) {
        rl_closure_call(fn, NULL, 0);
    }
    clock_gettime(CLOCK_MONOTONIC, &end);
    double elapsed = (end.tv_sec - start.tv_sec) + (end.tv_nsec - start.tv_nsec) / 1e9;
    return rl_ok_f64(elapsed / (double)iterations);
}

// ---- terminal (ANSI escape codes) ----

#include <sys/ioctl.h>
#include <sys/select.h>
#include <unistd.h>
#include <stdio.h>

// Saved TTY settings while raw mode is active.
static struct termios _rl_saved_termios;
static bool _rl_term_raw = false;

// Raw mode plus alternate screen, mirroring crossterm: errors when stdin
// has no TTY settings to take over.
rl_result rl_term_enter(void) {
    if (tcgetattr(STDIN_FILENO, &_rl_saved_termios) != 0) return rl_err(-1);
    struct termios raw = _rl_saved_termios;
    raw.c_iflag &= (unsigned int)(~(BRKINT | ICRNL | INPCK | ISTRIP | IXON));
    raw.c_oflag &= (unsigned int)(~OPOST);
    raw.c_cflag |= (CS8);
    raw.c_lflag &= (unsigned int)(~(ECHO | ICANON | IEXTEN | ISIG));
    raw.c_cc[VMIN] = 1;
    raw.c_cc[VTIME] = 0;
    // NOW, not FLUSH: pending typeahead must survive the switch.
    if (tcsetattr(STDIN_FILENO, TCSANOW, &raw) != 0) return rl_err(-1);
    _rl_term_raw = true;
    printf("\x1b[?1049h");
    return rl_ok_null();
}

// Flush, restore cooked mode, show the cursor, leave the screen.
rl_result rl_term_leave(void) {
    fflush(stdout);
    fflush(stderr);
    if (_rl_term_raw) {
        tcsetattr(STDIN_FILENO, TCSANOW, &_rl_saved_termios);
        _rl_term_raw = false;
    }
    printf("\x1b[?25h\x1b[?1049l");
    fflush(stdout);
    return rl_ok_null();
}

// Clear the whole screen / the current line.
rl_result rl_term_clear(void) {
    // Clear only: no cursor homing, matching crossterm Clear(All).
    printf("\x1b[2J");
    return rl_ok_null();
}

// Clear the whole screen / the current line.
rl_result rl_term_clear_line(void) {
    printf("\x1b[2K");
    return rl_ok_null();
}

// Move the cursor: absolute position, column only, row only, relative steps, or N lines down / up to column 0.
// Reject negative coordinates with the VM's error shape
// (`<name> must be >= 0, got <v>`); use at the top of cursor/size fns.
#define _RL_TERM_NONNEG(v, name) do { if ((v) < 0) { char *_m = malloc(64); int _n = snprintf(_m, 64, "%s must be >= 0, got %ld", name, (long)(v)); return rl_err_msg((rl_string){ .data = _m, .len = (uint64_t)_n, .rc = 1 }); } } while (0)

rl_result rl_term_move(int64_t col, int64_t row) {
    _RL_TERM_NONNEG(col, "x");
    _RL_TERM_NONNEG(row, "y");
    printf("\x1b[%ld;%ldH", (long)row + 1, (long)col + 1);
    return rl_ok_null();
}

// Move the cursor: absolute position, column only, row only, relative steps, or N lines down / up to column 0.
rl_result rl_term_move_to_col(int64_t col) {
    _RL_TERM_NONNEG(col, "col");
    printf("\x1b[%ldG", (long)col + 1);
    return rl_ok_null();
}

// Move the cursor: absolute position, column only, row only, relative steps, or N lines down / up to column 0.
rl_result rl_term_move_to_row(int64_t row) {
    _RL_TERM_NONNEG(row, "row");
    printf("\x1b[%ld;d", (long)row + 1);
    return rl_ok_null();
}

// Move the cursor: absolute position, column only, row only, relative steps, or N lines down / up to column 0.
rl_result rl_term_move_up(int64_t n) {
    _RL_TERM_NONNEG(n, "n");
    printf("\x1b[%ldA", (long)n);
    return rl_ok_null();
}

// Move the cursor: absolute position, column only, row only, relative steps, or N lines down / up to column 0.
rl_result rl_term_move_down(int64_t n) {
    _RL_TERM_NONNEG(n, "n");
    printf("\x1b[%ldB", (long)n);
    return rl_ok_null();
}

// Move the cursor: absolute position, column only, row only, relative steps, or N lines down / up to column 0.
rl_result rl_term_move_left(int64_t n) {
    _RL_TERM_NONNEG(n, "n");
    printf("\x1b[%ldD", (long)n);
    return rl_ok_null();
}

// Move the cursor: absolute position, column only, row only, relative steps, or N lines down / up to column 0.
rl_result rl_term_move_right(int64_t n) {
    _RL_TERM_NONNEG(n, "n");
    printf("\x1b[%ldC", (long)n);
    return rl_ok_null();
}

// Move the cursor: absolute position, column only, row only, relative steps, or N lines down / up to column 0.
rl_result rl_term_next_line(int64_t n) {
    _RL_TERM_NONNEG(n, "n");
    printf("\x1b[%ldE", (long)n);
    return rl_ok_null();
}

// Move the cursor: absolute position, column only, row only, relative steps, or N lines down / up to column 0.
rl_result rl_term_prev_line(int64_t n) {
    _RL_TERM_NONNEG(n, "n");
    printf("\x1b[%ldF", (long)n);
    return rl_ok_null();
}

// Save / restore the cursor position; hide / show the cursor.
rl_result rl_term_save_cursor(void) {
    printf("\x1b[s");
    return rl_ok_null();
}

// Save / restore the cursor position; hide / show the cursor.
rl_result rl_term_restore_cursor(void) {
    printf("\x1b[u");
    return rl_ok_null();
}

// Save / restore the cursor position; hide / show the cursor.
rl_result rl_term_hide_cursor(void) {
    printf("\x1b[?25l");
    return rl_ok_null();
}

// Save / restore the cursor position; hide / show the cursor.
rl_result rl_term_show_cursor(void) {
    printf("\x1b[?25h");
    return rl_ok_null();
}

// Read the terminal size into `out_cols` / `out_rows`.
rl_result rl_term_get_size(int64_t *out_cols, int64_t *out_rows) {
    struct winsize ws;
    if (ioctl(STDOUT_FILENO, TIOCGWINSZ, &ws) == 0) {
        *out_cols = ws.ws_col;
        *out_rows = ws.ws_row;
    } else {
        *out_cols = 80;
        *out_rows = 24;
    }
    return rl_ok_null();
}

// Request a terminal size (may be ignored by the emulator).
rl_result rl_term_set_size(int64_t cols, int64_t rows) {
    _RL_TERM_NONNEG(cols, "cols");
    _RL_TERM_NONNEG(rows, "rows");
    printf("\x1b[8;%ld;%ldt", (long)rows, (long)cols);
    return rl_ok_null();
}

// Set the window title to the single char `ch`.
// Set the window title to `title`.
rl_result rl_term_set_title(rl_string title) {
    if (title.data == NULL) return rl_err(-1);
    printf("\x1b]0;%.*s\x07", (int)title.len, title.data);
    return rl_ok_null();
}

// Scroll the viewport up / down by `n` lines.
rl_result rl_term_scroll_up(int64_t n) {
    _RL_TERM_NONNEG(n, "n");
    printf("\x1b[%ldS", (long)n);
    return rl_ok_null();
}

// Scroll the viewport up / down by `n` lines.
rl_result rl_term_scroll_down(int64_t n) {
    _RL_TERM_NONNEG(n, "n");
    printf("\x1b[%ldT", (long)n);
    return rl_ok_null();
}

// Flush pending output.
rl_result rl_term_flush(void) {
    fflush(stdout);
    return rl_ok_null();
}

// Truecolor foreground / background, or reset to the default pair.
rl_result rl_term_set_fg(int64_t r, int64_t g, int64_t b) {
    printf("\x1b[38;2;%ld;%ld;%ldm", (long)r, (long)g, (long)b);
    return rl_ok_null();
}

// Truecolor foreground / background, or reset to the default pair.
rl_result rl_term_set_bg(int64_t r, int64_t g, int64_t b) {
    printf("\x1b[48;2;%ld;%ld;%ldm", (long)r, (long)g, (long)b);
    return rl_ok_null();
}

// Truecolor foreground / background, or reset to the default pair.
rl_result rl_term_reset_color(void) {
    printf("\x1b[0m");
    return rl_ok_null();
}

// Map color name to ANSI code, or -1 when unknown.
static int _term_named_color(rl_string name) {
    if (name.len == 3 && memcmp(name.data, "red", 3) == 0) return 31;
    if (name.len == 5 && memcmp(name.data, "green", 5) == 0) return 32;
    if (name.len == 6 && memcmp(name.data, "yellow", 6) == 0) return 33;
    if (name.len == 4 && memcmp(name.data, "blue", 4) == 0) return 34;
    if (name.len == 7 && memcmp(name.data, "magenta", 7) == 0) return 35;
    if (name.len == 4 && memcmp(name.data, "cyan", 4) == 0) return 36;
    if (name.len == 5 && memcmp(name.data, "white", 5) == 0) return 37;
    if (name.len == 5 && memcmp(name.data, "black", 5) == 0) return 30;
    if (name.len == 10 && memcmp(name.data, "dark_black", 10) == 0) return 90;
    if (name.len == 8 && memcmp(name.data, "dark_red", 8) == 0) return 91;
    if (name.len == 10 && memcmp(name.data, "dark_green", 10) == 0) return 92;
    if (name.len == 11 && memcmp(name.data, "dark_yellow", 11) == 0) return 93;
    if (name.len == 9 && memcmp(name.data, "dark_blue", 9) == 0) return 94;
    if (name.len == 12 && memcmp(name.data, "dark_magenta", 12) == 0) return 95;
    if (name.len == 9 && memcmp(name.data, "dark_cyan", 9) == 0) return 96;
    if (name.len == 4 && memcmp(name.data, "grey", 4) == 0) return 90;
    return -1;
}

// Named-color ("red", "blue", ...) foreground / background.
rl_result rl_term_fg(rl_string name) {
    int code = _term_named_color(name);
    if (code < 0) {
        return rl_err_msg(rl_str_literal("term_fg(): unknown color", 24));
    }
    printf("\x1b[%dm", code);
    return rl_ok_null();
}

// Named-color ("red", "blue", ...) foreground / background.
rl_result rl_term_bg(rl_string name) {
    int code = _term_named_color(name);
    if (code < 0) {
        return rl_err_msg(rl_str_literal("term_bg(): unknown color", 24));
    }
    printf("\x1b[%dm", code + 10);
    return rl_ok_null();
}

// Text attributes; `reset_attr` clears them all.
rl_result rl_term_bold(void) { printf("\x1b[1m"); return rl_ok_null(); }
// Text attributes; `reset_attr` clears them all.
rl_result rl_term_dim(void) { printf("\x1b[2m"); return rl_ok_null(); }
// Text attributes; `reset_attr` clears them all.
rl_result rl_term_italic(void) { printf("\x1b[3m"); return rl_ok_null(); }
// Text attributes; `reset_attr` clears them all.
rl_result rl_term_underline(void) { printf("\x1b[4m"); return rl_ok_null(); }
// Text attributes; `reset_attr` clears them all.
rl_result rl_term_blink(void) { printf("\x1b[5m"); return rl_ok_null(); }
// Text attributes; `reset_attr` clears them all.
rl_result rl_term_reverse(void) { printf("\x1b[7m"); return rl_ok_null(); }
// Text attributes; `reset_attr` clears them all.
rl_result rl_term_crossed_out(void) { printf("\x1b[9m"); return rl_ok_null(); }
// Text attributes; `reset_attr` clears them all.
rl_result rl_term_reset_attr(void) { printf("\x1b[0m"); return rl_ok_null(); }

// Line wrapping on / off.
rl_result rl_term_enable_wrap(void) { printf("\x1b[?7h"); return rl_ok_null(); }
// Line wrapping on / off.
rl_result rl_term_disable_wrap(void) { printf("\x1b[?7l"); return rl_ok_null(); }

// Synchronized-output markers to avoid flicker during redraws.
rl_result rl_term_begin_sync(void) { printf("\x1b[?2026h"); return rl_ok_null(); }
// Synchronized-output markers to avoid flicker during redraws.
rl_result rl_term_end_sync(void) { printf("\x1b[?2026l"); return rl_ok_null(); }

// Mouse-event reporting on / off.
rl_result rl_term_enable_mouse(void) { printf("\x1b[?1003h\x1b[?1006h"); return rl_ok_null(); }
// Mouse-event reporting on / off.
rl_result rl_term_disable_mouse(void) { printf("\x1b[?1003l\x1b[?1006l"); return rl_ok_null(); }

// Print a value without moving to a new line (shared rendering with
// `format`: bare values print raw, results print decorated).
void rl_term_print_inline(rl_fmt_arg arg) {
    char *s = NULL;
    uint64_t n = 0;
    _rl_fmt_arg_to_str(arg, &s, &n);
    if (n > 0) fwrite(s, 1, n, stdout);
    free(s);
}

// Read one key press as an array of key codes (blocks).
// Non-blocking read of one byte into `out`; false on timeout / EOF.
static bool _rl_term_try_read(uint8_t *out, long micros) {
    fd_set fds;
    struct timeval tv = { .tv_sec = micros / 1000000, .tv_usec = micros % 1000000 };
    FD_ZERO(&fds);
    FD_SET(STDIN_FILENO, &fds);
    if (select(STDIN_FILENO + 1, &fds, NULL, NULL, &tv) <= 0) return false;
    return read(STDIN_FILENO, out, 1) == 1;
}

// Wrap a C string as an owned rl_string.
static rl_string _rl_term_key_str(const char *s) {
    uint64_t len = strlen(s);
    char *heap = malloc(len + 1);
    memcpy(heap, s, len + 1);
    rl_string r = { .data = heap, .len = len, .rc = 1 };
    return r;
}

// Single-element key array.
static rl_result _rl_term_key_one(rl_string name) {
    rl_string *sarr = malloc(sizeof(rl_string));
    sarr[0] = name;
    rl_array arr = { .data = sarr, .len = 1, .cap = 1, .elem_size = sizeof(rl_string), .type_tag = RL_TAG_STR };
    return rl_ok_arr(arr);
}

// Read one key press as an array of key-code strings (blocks); ok with
// the array, or an error on EOF. Names mirror crossterm: `Char:x`,
// `Ctrl:x`, `Enter`, `Esc`, `Backspace`, arrows, `PageUp`, function keys.
rl_result rl_term_read_key(void) {
    // Flush pending output first so the frame being waited on is visible.
    fflush(stdout);
    char buf[32];
    uint64_t total = 0;
    // blocking read of one byte
    uint8_t c;
    if (read(STDIN_FILENO, &c, 1) != 1) return rl_err(-1);
    // Single-byte keys and control codes.
    if (c != 27) {
        if (c == '\r' || c == '\n') return _rl_term_key_one(_rl_term_key_str("Enter"));
        if (c == '\t') return _rl_term_key_one(_rl_term_key_str("Tab"));
        if (c == 127) return _rl_term_key_one(_rl_term_key_str("Backspace"));
        if (c == 0) return _rl_term_key_one(_rl_term_key_str("Null"));
        if (c < 0x20) {
            char name[8];
            snprintf(name, sizeof(name), "Ctrl:%c", (char)('a' + c - 1));
            return _rl_term_key_one(_rl_term_key_str(name));
        }
        if (c < 0x80) {
            char name[8];
            snprintf(name, sizeof(name), "Char:%c", (char)c);
            return _rl_term_key_one(_rl_term_key_str(name));
        }
        // UTF-8 sequence: read continuation bytes, decode the char.
        buf[total++] = (char)c;
        uint64_t need = 0;
        if ((c & 0xE0) == 0xC0) need = 2;
        else if ((c & 0xF0) == 0xE0) need = 3;
        else if ((c & 0xF8) == 0xF0) need = 4;
        else return _rl_term_key_one(_rl_term_key_str("Unknown"));
        while (total < need) {
            uint8_t next;
            if (!_rl_term_try_read(&next, 100000)) break;
            buf[total++] = (char)next;
        }
        uint32_t code = 0;
        uint64_t used = 0;
        if (_rl_utf8_decode(buf, total, &code, &used) && used == total) {
            char tmp[8];
            uint64_t tlen = 0;
            if (code < 0x80) { tmp[0] = (char)code; tlen = 1; }
            else if (code < 0x800) {
                tmp[0] = (char)(0xC0 | (code >> 6)); tmp[1] = (char)(0x80 | (code & 0x3F)); tlen = 2;
            } else if (code < 0x10000) {
                tmp[0] = (char)(0xE0 | (code >> 12)); tmp[1] = (char)(0x80 | ((code >> 6) & 0x3F)); tmp[2] = (char)(0x80 | (code & 0x3F)); tlen = 3;
            } else {
                tmp[0] = (char)(0xF0 | (code >> 18)); tmp[1] = (char)(0x80 | ((code >> 12) & 0x3F)); tmp[2] = (char)(0x80 | ((code >> 6) & 0x3F)); tmp[3] = (char)(0x80 | (code & 0x3F)); tlen = 4;
            }
            char *name = malloc(5 + tlen + 1);
            memcpy(name, "Char:", 5);
            memcpy(name + 5, tmp, tlen);
            name[5 + tlen] = '\0';
            rl_string s = { .data = name, .len = 5 + tlen, .rc = 1 };
            return _rl_term_key_one(s);
        }
        return _rl_term_key_one(_rl_term_key_str("Unknown"));
    }
    // Escape: lone ESC, Alt+key, or a CSI/SS3 sequence.
    uint8_t n1;
    if (!_rl_term_try_read(&n1, 100000)) {
        return _rl_term_key_one(_rl_term_key_str("Esc"));
    }
    if (n1 == '[') {
        uint8_t n2;
        if (!_rl_term_try_read(&n2, 50000)) {
            return _rl_term_key_one(_rl_term_key_str("Esc"));
        }
        switch (n2) {
            case 'A': return _rl_term_key_one(_rl_term_key_str("Up"));
            case 'B': return _rl_term_key_one(_rl_term_key_str("Down"));
            case 'C': return _rl_term_key_one(_rl_term_key_str("Right"));
            case 'D': return _rl_term_key_one(_rl_term_key_str("Left"));
            case 'H': return _rl_term_key_one(_rl_term_key_str("Home"));
            case 'F': return _rl_term_key_one(_rl_term_key_str("End"));
            case 'Z': return _rl_term_key_one(_rl_term_key_str("BackTab"));
            case 'I': return _rl_term_key_one(_rl_term_key_str("FocusGained"));
            case 'M': {
                // Old X10 mouse report: ESC [ M Cb Cx Cy.
                uint8_t cb = 0, cx = 0, cy = 0;
                if (!_rl_term_try_read(&cb, 50000)) break;
                if (!_rl_term_try_read(&cx, 50000)) break;
                if (!_rl_term_try_read(&cy, 50000)) break;
                const char *kind = "MouseUnknown";
                if (cb == 32 + 0) kind = "MouseLeft";
                else if (cb == 32 + 1) kind = "MouseMiddle";
                else if (cb == 32 + 2) kind = "MouseRight";
                else if ((cb & 64) != 0) kind = "MouseUp";
                char col[16], row[16];
                snprintf(col, sizeof(col), "%u", (unsigned)(cx - 32));
                snprintf(row, sizeof(row), "%u", (unsigned)(cy - 32));
                rl_string *sarr = malloc(3 * sizeof(rl_string));
                sarr[0] = _rl_term_key_str(kind);
                sarr[1] = _rl_term_key_str(col);
                sarr[2] = _rl_term_key_str(row);
                rl_array arr = { .data = sarr, .len = 3, .cap = 3, .elem_size = sizeof(rl_string), .type_tag = RL_TAG_STR };
                return rl_ok_arr(arr);
            }
            default: break;
        }
        if (n2 == '<') {
            // SGR mouse report: ESC [ < Cb ; Cx ; Cy M/m.
            char params[64];
            uint64_t plen = 0;
            uint8_t ch = 0;
            bool press = true;
            while (plen < sizeof(params) - 1) {
                if (!_rl_term_try_read(&ch, 50000)) break;
                if (ch == 'M' || ch == 'm') { press = (ch == 'M'); break; }
                params[plen++] = (char)ch;
            }
            params[plen] = '\0';
            unsigned cb = 0, cx = 0, cy = 0;
            sscanf(params, "%u;%u;%u", &cb, &cx, &cy);
            const char *kind = "MouseUnknown";
            if (!press) kind = "MouseUp";
            else if (cb == 0) kind = "MouseLeft";
            else if (cb == 1) kind = "MouseMiddle";
            else if (cb == 2) kind = "MouseRight";
            else if (cb == 64) kind = "ScrollUp";
            else if (cb == 65) kind = "ScrollDown";
            else if ((cb & 32) != 0) kind = "MouseDrag";
            else if ((cb & 64) != 0) kind = "MouseMove";
            char col[16], row[16];
            snprintf(col, sizeof(col), "%u", cx);
            snprintf(row, sizeof(row), "%u", cy);
            rl_string *sarr = malloc(3 * sizeof(rl_string));
            sarr[0] = _rl_term_key_str(kind);
            sarr[1] = _rl_term_key_str(col);
            sarr[2] = _rl_term_key_str(row);
            rl_array arr = { .data = sarr, .len = 3, .cap = 3, .elem_size = sizeof(rl_string), .type_tag = RL_TAG_STR };
            return rl_ok_arr(arr);
        }
        if ((n2 >= '0' && n2 <= '9')) {
            // Numeric CSI: collect until the final byte.
            char params[32];
            uint64_t plen = 0;
            params[plen++] = (char)n2;
            uint8_t fin = 0;
            while (plen < sizeof(params) - 1) {
                if (!_rl_term_try_read(&fin, 50000)) break;
                if ((fin >= '0' && fin <= '9') || fin == ';') { params[plen++] = (char)fin; continue; }
                break;
            }
            params[plen] = '\0';
            const char *kind = "Unknown";
            if (fin == '~') {
                unsigned code = 0;
                sscanf(params, "%u", &code);
                switch (code) {
                    case 1: case 7: kind = "Home"; break;
                    case 2: kind = "Insert"; break;
                    case 3: kind = "Delete"; break;
                    case 4: case 8: kind = "End"; break;
                    case 5: kind = "PageUp"; break;
                    case 6: kind = "PageDown"; break;
                    case 11: kind = "F1"; break;
                    case 12: kind = "F2"; break;
                    case 13: kind = "F3"; break;
                    case 14: kind = "F4"; break;
                    case 15: kind = "F5"; break;
                    case 17: kind = "F6"; break;
                    case 18: kind = "F7"; break;
                    case 19: kind = "F8"; break;
                    case 20: kind = "F9"; break;
                    case 21: kind = "F10"; break;
                    case 23: kind = "F11"; break;
                    case 24: kind = "F12"; break;
                    default: break;
                }
            } else if (fin == 'R') {
                // Cursor position report: ESC [ row ; col R (ignore).
                return rl_term_read_key();
            } else if (fin == 'M' || fin == 'm') {
                kind = "MouseUnknown";
            }
            return _rl_term_key_one(_rl_term_key_str(kind));
        }
        return _rl_term_key_one(_rl_term_key_str("Unknown"));
    }
    if (n1 == 'O') {
        uint8_t n2;
        if (!_rl_term_try_read(&n2, 50000)) {
            return _rl_term_key_one(_rl_term_key_str("FocusLost"));
        }
        switch (n2) {
            case 'P': return _rl_term_key_one(_rl_term_key_str("F1"));
            case 'Q': return _rl_term_key_one(_rl_term_key_str("F2"));
            case 'R': return _rl_term_key_one(_rl_term_key_str("F3"));
            case 'S': return _rl_term_key_one(_rl_term_key_str("F4"));
            default: break;
        }
        return _rl_term_key_one(_rl_term_key_str("Unknown"));
    }
    // Alt+key: crossterm reports the plain char.
    if (n1 >= 0x20 && n1 < 0x7F) {
        char name[8];
        snprintf(name, sizeof(name), "Char:%c", (char)n1);
        return _rl_term_key_one(_rl_term_key_str(name));
    }
    return _rl_term_key_one(_rl_term_key_str("Unknown"));
}

// Ok with `[cols, rows]`, or an error when the size is unknown.
rl_result rl_term_size(void) {
    int64_t cols = 0, rows = 0;
    rl_result r = rl_term_get_size(&cols, &rows);
    if (!r.is_ok) return r;
    int64_t vals[2] = { cols, rows };
    return rl_ok_arr(rl_arr_from_vals(vals, 2, sizeof(int64_t)));
}

// Query cursor via DSR; ok with [x, y] or an error on timeout/non-tty.
rl_result rl_term_get_cursor_pos(void) {
    if (!isatty(STDIN_FILENO)) {
        char *msg = malloc(64);
        int m = snprintf(msg, 64, "term_get_cursor_pos(): not a tty");
        return rl_err_msg((rl_string){ .data = msg, .len = (uint64_t)m, .rc = 1 });
    }
    fflush(stdout);
    fputs("\x1b[6n", stdout);
    fflush(stdout);
    uint8_t c = 0;
    if (!_rl_term_try_read(&c, 100000) || c != 27) {
        char *msg = malloc(64);
        int m = snprintf(msg, 64, "term_get_cursor_pos(): timeout");
        return rl_err_msg((rl_string){ .data = msg, .len = (uint64_t)m, .rc = 1 });
    }
    if (!_rl_term_try_read(&c, 100000) || c != '[') {
        char *msg = malloc(64);
        int m = snprintf(msg, 64, "term_get_cursor_pos(): timeout");
        return rl_err_msg((rl_string){ .data = msg, .len = (uint64_t)m, .rc = 1 });
    }
    long row = 0, col = 0;
    bool have_digit = false;
    while (1) {
        if (!_rl_term_try_read(&c, 100000)) {
            char *msg = malloc(64);
            int m = snprintf(msg, 64, "term_get_cursor_pos(): timeout");
            return rl_err_msg((rl_string){ .data = msg, .len = (uint64_t)m, .rc = 1 });
        }
        if (c >= '0' && c <= '9') {
            row = row * 10 + (c - '0');
            have_digit = true;
        } else if (c == ';' && have_digit) {
            break;
        } else {
            char *msg = malloc(64);
            int m = snprintf(msg, 64, "term_get_cursor_pos(): timeout");
            return rl_err_msg((rl_string){ .data = msg, .len = (uint64_t)m, .rc = 1 });
        }
    }
    have_digit = false;
    while (1) {
        if (!_rl_term_try_read(&c, 100000)) {
            char *msg = malloc(64);
            int m = snprintf(msg, 64, "term_get_cursor_pos(): timeout");
            return rl_err_msg((rl_string){ .data = msg, .len = (uint64_t)m, .rc = 1 });
        }
        if (c >= '0' && c <= '9') {
            col = col * 10 + (c - '0');
            have_digit = true;
        } else if (c == 'R' && have_digit) {
            break;
        } else {
            char *msg = malloc(64);
            int m = snprintf(msg, 64, "term_get_cursor_pos(): timeout");
            return rl_err_msg((rl_string){ .data = msg, .len = (uint64_t)m, .rc = 1 });
        }
    }
    int64_t vals[2] = { (int64_t)col, (int64_t)row };
    return rl_ok_arr(rl_arr_from_vals(vals, 2, sizeof(int64_t)));
}

// True when input arrives within `ms` milliseconds.
rl_result rl_term_poll(int64_t ms) {
    fflush(stdout);
    fd_set fds;
    struct timeval tv = { .tv_sec = (long)(ms / 1000), .tv_usec = (long)((ms % 1000) * 1000) };
    FD_ZERO(&fds);
    FD_SET(STDIN_FILENO, &fds);
    return rl_ok_bool(select(STDIN_FILENO + 1, &fds, NULL, NULL, &tv) > 0);
}

// ---- result unwrap (with error checking) ----

// Checked unwrap used by RL `unwrap`: aborts with a message when `r`
// is an error instead of silently reading a dead union member.
int64_t rl_result_unwrap_i64(rl_result r) {
    if (!r.is_ok) {
        fprintf(stderr, "error: unwrap called on err value\n");
        _rl_abort();
    }
    return r.data.i64;
}

// Checked unwrap used by RL `unwrap`: aborts with a message when `r`
// is an error instead of silently reading a dead union member.
double rl_result_unwrap_f64(rl_result r) {
    if (!r.is_ok) {
        fprintf(stderr, "error: unwrap called on err value\n");
        _rl_abort();
    }
    return r.data.f64;
}

// Checked unwrap used by RL `unwrap`: aborts with a message when `r`
// is an error instead of silently reading a dead union member.
bool rl_result_unwrap_bool(rl_result r) {
    if (!r.is_ok) {
        fprintf(stderr, "error: unwrap called on err value\n");
        _rl_abort();
    }
    return r.data.boolean;
}

// Checked unwrap used by RL `unwrap`: aborts with a message when `r`
// is an error instead of silently reading a dead union member.
rl_string rl_result_unwrap_str(rl_result r) {
    if (!r.is_ok) {
        fprintf(stderr, "error: unwrap called on err value\n");
        _rl_abort();
    }
    return r.data.str;
}

// Checked unwrap of an array payload; aborts on error like the rest.
rl_array rl_result_unwrap_arr(rl_result r) {
    if (!r.is_ok) {
        fprintf(stderr, "error: unwrap called on err value\n");
        _rl_abort();
    }
    return r.data.arr;
}

// Checked unwrap of a map payload; aborts on error like the rest.
rl_map rl_result_unwrap_map(rl_result r) {
    if (!r.is_ok) {
        fprintf(stderr, "error: unwrap called on err value\n");
        _rl_abort();
    }
    return r.data.map;
}

// Checked unwrap of a set payload; aborts on error like the rest.
rl_set rl_result_unwrap_set(rl_result r) {
    if (!r.is_ok) {
        fprintf(stderr, "error: unwrap called on err value\n");
        _rl_abort();
    }
    return r.data.set;
}

// Checked unwrap of a boxed closure payload; aborts on error like the rest.
rl_closure rl_result_unwrap_closure(rl_result r) {
    if (!r.is_ok) {
        fprintf(stderr, "error: unwrap called on err value\n");
        _rl_abort();
    }
    return *r.data.closure;
}

// Abort for result_unwrap_err called on an ok value, echoing the value
// like the VM's message does.
static void _rl_unwrap_err_abort(rl_result v) {
    char *s = NULL;
    uint64_t n = 0;
    _rl_fmt_arg_to_str((rl_fmt_arg){ v, false }, &s, &n);
    fprintf(stderr, "error: result_unwrap_err: called on %.*s\n", (int)n, s ? s : "");
    free(s);
                _rl_abort();
}

// Unwrap an error payload; aborts when given an ok value.
rl_string rl_result_unwrap_err_str(rl_result r) {
    if (r.is_ok) _rl_unwrap_err_abort(r);
    return r.data.str;
}

// Unwrap an error payload; aborts when given an ok value.
int64_t rl_result_unwrap_err_i64(rl_result r) {
    if (r.is_ok) _rl_unwrap_err_abort(r);
    return r.data.i64;
}

// Unwrap an error payload; aborts when given an ok value.
double rl_result_unwrap_err_f64(rl_result r) {
    if (r.is_ok) _rl_unwrap_err_abort(r);
    return r.data.f64;
}

// Unwrap an error payload; aborts when given an ok value.
bool rl_result_unwrap_err_bool(rl_result r) {
    if (r.is_ok) _rl_unwrap_err_abort(r);
    return r.data.boolean;
}

// Length of a string/array/map/set result payload, or an error for
// anything else. Backs dynamically-typed `len` calls.
rl_result rl_len_result(rl_result v) {
    if (!v.is_ok) return v;
    switch (v.tag) {
        case RL_TAG_STR: return rl_ok_i64((int64_t)v.data.str.len);
        case RL_TAG_ARR: return rl_ok_i64((int64_t)v.data.arr.len);
        case RL_TAG_MAP: return rl_ok_i64((int64_t)rl_map_len(v.data.map));
        case RL_TAG_SET: return rl_ok_i64((int64_t)rl_set_len(v.data.set));
        default: return rl_err(-1);
    }
}

// ---- math ----

// Absolute value / power over int and float payloads; non-numeric input yields an RL error.
rl_result rl_math_abs(rl_result x) {
    switch (x.tag) {
        case RL_TAG_F64: return rl_ok_f64(fabs(x.data.f64));
        default: return rl_ok_i64(llabs(x.data.i64));
    }
}

// Absolute value / power over int and float payloads; non-numeric input yields an RL error.
rl_result rl_math_pow(rl_result base, rl_result exp) {
    if (base.tag == RL_TAG_I64 && exp.tag == RL_TAG_I64) {
        int64_t b = base.data.i64;
        int64_t e = exp.data.i64;
        if (e < 0) {
            return rl_ok_f64(pow((double)b, (double)e));
        }
        int64_t result = 1;
        while (e > 0) {
            if (e & 1) result *= b;
            e >>= 1;
            b *= b;
        }
        return rl_ok_i64(result);
    }
    double b_val = (base.tag == RL_TAG_F64) ? base.data.f64 : (double)base.data.i64;
    double e_val = (exp.tag == RL_TAG_F64) ? exp.data.f64 : (double)exp.data.i64;
    return rl_ok_f64(pow(b_val, e_val));
}

// ---- type checks ----

// Return ok bool true when payload tag is bool.
rl_result rl_is_bool(rl_result x) { return rl_ok_bool(x.tag == RL_TAG_BOOL); }
// Return ok bool true when payload tag is int.
rl_result rl_is_int(rl_result x) { return rl_ok_bool(x.tag == RL_TAG_I64); }
// Return ok bool true when payload tag is float.
rl_result rl_is_float(rl_result x) { return rl_ok_bool(x.tag == RL_TAG_F64); }
// Return ok bool true when payload tag is string.
rl_result rl_is_string(rl_result x) { return rl_ok_bool(x.tag == RL_TAG_STR); }
// Return ok bool true when payload tag is null.
rl_result rl_is_null(rl_result x) { return rl_ok_bool(x.tag == RL_TAG_NULL); }
// Return ok bool true when payload tag is char.
rl_result rl_is_char(rl_result x) { return rl_ok_bool(x.tag == RL_TAG_CHAR); }
// Return ok bool true when payload tag is int (byte).
rl_result rl_is_byte(rl_result x) { return rl_ok_bool(x.tag == RL_TAG_I64); }
// Return ok bool true when result is an error.
rl_result rl_is_error(rl_result x) { return rl_ok_bool(!x.is_ok); }
// Return ok bool true when payload tag is array.
rl_result rl_is_array(rl_result x) { return rl_ok_bool(x.tag == RL_TAG_ARR); }
// Return ok bool true when payload tag is map.
rl_result rl_is_map(rl_result x) { return rl_ok_bool(x.tag == RL_TAG_MAP); }
// Return ok bool true when payload tag is set.
rl_result rl_is_set(rl_result x) { return rl_ok_bool(x.tag == RL_TAG_SET); }
// Tuples never travel boxed; always false at runtime (literals fold).
rl_result rl_is_tuple(rl_result x) { (void)x; return rl_ok_bool(false); }
// Return ok bool true when payload is a closure.
rl_result rl_is_function(rl_result x) { return rl_ok_bool(x.tag == RL_TAG_CLOSURE); }
// Narrow ints erase to I64 at runtime; literals fold statically.
rl_result rl_is_uint(rl_result x) { return rl_ok_bool(x.tag == RL_TAG_I64); }
// Narrow ints erase to I64 at runtime; literals fold statically.
rl_result rl_is_sbyte(rl_result x) { return rl_ok_bool(x.tag == RL_TAG_I64); }
// Narrow ints erase to I64 at runtime; literals fold statically.
rl_result rl_is_bsbyte(rl_result x) { return rl_ok_bool(x.tag == RL_TAG_I64); }
// Narrow ints erase to I64 at runtime; literals fold statically.
rl_result rl_is_bbyte(rl_result x) { return rl_ok_bool(x.tag == RL_TAG_I64); }
// Narrow ints erase to I64 at runtime; literals fold statically.
rl_result rl_is_sint(rl_result x) { return rl_ok_bool(x.tag == RL_TAG_I64); }
// Narrow ints erase to I64 at runtime; literals fold statically.
rl_result rl_is_suint(rl_result x) { return rl_ok_bool(x.tag == RL_TAG_I64); }
// Small float erases to F64 at runtime; literals fold statically.
rl_result rl_is_sfloat(rl_result x) { return rl_ok_bool(x.tag == RL_TAG_F64); }

// Handle kind range check used by the is_*_handle testers below.
static bool _rl_id_in_range(int64_t id, int64_t base, int count) {
    return id >= base && id < base + (int64_t)count;
}

// Abort instead of returning (RL never type).
rl_never rl_never_fn(void) {
    fprintf(stderr, "error: reached unreachable code\n");
                _rl_abort();
}

// ---- std::c (FFI) ----

#define RL_C_MAX_HANDLES 256

static struct { void *handle; } rl_c_handles[RL_C_MAX_HANDLES];
static int rl_c_handle_count = 0;

// Allocate a tagged handle id for a dlopen pointer.
static rl_result rl_c_new_handle(void *h) {
    if (rl_c_handle_count >= RL_C_MAX_HANDLES) {
        return rl_err_msg(rl_str_literal("c: too many open handles", 24));
    }
    int idx = rl_c_handle_count++;
    rl_c_handles[idx].handle = h;
    return rl_ok_i64(RL_HANDLE_C_BASE + (int64_t)idx);
}

// Decode a tagged id to a table index, or -1 when out of range.
static int64_t _rl_c_idx(int64_t tagged) {
    int64_t idx = tagged - RL_HANDLE_C_BASE;
    if (idx < 0 || idx >= rl_c_handle_count) return -1;
    return idx;
}

// Look up dlopen pointer by handle id (NULL when unknown).
static void *rl_c_get_handle(rl_result r) {
    if (r.tag != RL_TAG_I64) return NULL;
    int64_t idx = _rl_c_idx(r.data.i64);
    if (idx < 0) return NULL;
    return rl_c_handles[idx].handle;
}

// Copy RL string into NUL-terminated C string (caller owns).
static char *rl_string_to_cstr(rl_string s) {
    char *buf = (char *)malloc(s.len + 1);
    memcpy(buf, s.data, s.len);
    buf[s.len] = '\0';
    return buf;
}

// Compile C `source` to a cached shared object; result holds its path.
rl_result rl_c_compile(rl_string source) {
    const char *tmpdir = getenv("TMPDIR");
    if (!tmpdir) tmpdir = "/tmp";
    char cache_dir[512];
    snprintf(cache_dir, sizeof(cache_dir), "%s/rl_std_c_cache", tmpdir);
    mkdir(cache_dir, 0755);

    unsigned long hash = 5381;
    for (uint64_t i = 0; i < source.len; i++) {
        hash = ((hash << 5) + hash) + (unsigned char)source.data[i];
    }

    char src_path[600], out_path[600];
    snprintf(src_path, sizeof(src_path), "%s/%016lx.c", cache_dir, hash);
    snprintf(out_path, sizeof(out_path), "%s/%016lx.so", cache_dir, hash);

    FILE *f = fopen(src_path, "w");
    if (!f) return rl_err_msg(rl_str_literal("c: failed to write source", 25));
    fwrite(source.data, 1, source.len, f);
    fclose(f);

    const char *cc = getenv("CC");
    if (!cc) cc = "cc";
    char cmd[700];
    snprintf(cmd, sizeof(cmd), "%s -shared -fPIC -O2 -o %s %s 2>&1", cc, out_path, src_path);
    int rc = system(cmd);
    if (rc != 0) {
        return rl_err_msg(rl_str_literal("c: compile failed", 17));
    }

    void *handle = dlopen(out_path, RTLD_NOW);
    if (!handle) {
        return rl_err_msg(rl_str_literal("c: dlopen failed", 16));
    }
    return rl_c_new_handle(handle);
}

// Load a shared object; result holds its handle id.
rl_result rl_c_load(rl_string path) {
    char *cpath = rl_string_to_cstr(path);
    void *handle = dlopen(cpath, RTLD_NOW);
    free(cpath);
    if (!handle) {
        return rl_err_msg(rl_str_literal("c: dlopen failed", 16));
    }
    return rl_c_new_handle(handle);
}

// True (as a result) when the handle exports `fn_name`.
rl_result rl_c_has_symbol(int64_t handle_id, rl_string fn_name) {
    void *h = NULL;
    int64_t idx = _rl_c_idx(handle_id);
    if (idx >= 0) {
        h = rl_c_handles[idx].handle;
    }
    if (!h) return rl_err_msg(rl_str_literal("c: invalid handle", 17));
    char *name = rl_string_to_cstr(fn_name);
    void *sym = dlsym(h, name);
    free(name);
    return rl_ok_bool(sym != NULL);
}

// Unload the handle (no-op for unknown ids).
rl_result rl_c_close(int64_t handle_id) {
    void *h = NULL;
    int64_t idx = _rl_c_idx(handle_id);
    if (idx >= 0) {
        h = rl_c_handles[idx].handle;
    }
    if (!h) return rl_err_msg(rl_str_literal("c: invalid handle", 17));
    dlclose(h);
    if (idx >= 0) {
        rl_c_handles[idx].handle = NULL;
    }
    return rl_ok_null();
}

// Drop all cached compile artifacts.
rl_result rl_c_clear_cache(void) {
    const char *tmpdir = getenv("TMPDIR");
    if (!tmpdir) tmpdir = "/tmp";
    char cache_dir[512];
    snprintf(cache_dir, sizeof(cache_dir), "%s/rl_std_c_cache", tmpdir);
    char cmd[600];
    snprintf(cmd, sizeof(cmd), "rm -rf %s", cache_dir);
    system(cmd);
    return rl_ok_null();
}

#ifdef RL_USE_LIBFFI
#include <ffi.h>
#endif

// Call `fn_name` with `argc` boxed args described by `arg_types` ("i64",
// "f64", "str", ...), converting to `ret_type` on return.
rl_result rl_c_call(int64_t handle_id, rl_string fn_name, int64_t argc, void **argv, const char **arg_types, rl_string ret_type) {
    void *h = NULL;
    int64_t cidx = _rl_c_idx(handle_id);
    if (cidx >= 0) {
        h = rl_c_handles[cidx].handle;
    }
    if (!h) return rl_err_msg(rl_str_literal("c::call: invalid handle", 23));

    char *name = rl_string_to_cstr(fn_name);
    void *sym = dlsym(h, name);
    free(name);
    if (!sym) return rl_err_msg(rl_str_literal("c::call: symbol not found", 25));

#ifndef RL_USE_LIBFFI
    (void)argc; (void)argv; (void)arg_types; (void)ret_type;
    return rl_err_msg(rl_str_literal("c::call: requires libffi (add -DRL_USE_LIBFFI -lffi to compile flags)", 68));
#else
    ffi_type **ffi_arg_types = argc ? malloc(sizeof(ffi_type *) * argc) : NULL;
    void **ffi_values = argc ? malloc(sizeof(void *) * argc) : NULL;
    int64_t *i64_slots = argc ? malloc(sizeof(int64_t) * argc) : NULL;
    double *f64_slots = argc ? malloc(sizeof(double) * argc) : NULL;

    for (int64_t i = 0; i < argc; i++) {
        const char *type_str = arg_types[i];
        if (strncmp(type_str, "i32", 3) == 0) {
            ffi_arg_types[i] = &ffi_type_sint32;
            i64_slots[i] = (int64_t)(int32_t)(intptr_t)argv[i];
            ffi_values[i] = &i64_slots[i];
        } else if (strncmp(type_str, "i64", 3) == 0 || strncmp(type_str, "bool", 4) == 0) {
            ffi_arg_types[i] = &ffi_type_sint64;
            i64_slots[i] = (int64_t)(intptr_t)argv[i];
            ffi_values[i] = &i64_slots[i];
        } else if (strncmp(type_str, "f32", 3) == 0) {
            ffi_arg_types[i] = &ffi_type_float;
            float v = *(float *)&argv[i];
            f64_slots[i] = v;
            ffi_values[i] = &f64_slots[i];
        } else if (strncmp(type_str, "f64", 3) == 0) {
            ffi_arg_types[i] = &ffi_type_double;
            f64_slots[i] = *(double *)&argv[i];
            ffi_values[i] = &f64_slots[i];
        } else if (strncmp(type_str, "u8", 2) == 0 || strncmp(type_str, "char", 4) == 0) {
            ffi_arg_types[i] = &ffi_type_uint8;
            i64_slots[i] = (uint8_t)(intptr_t)argv[i];
            ffi_values[i] = &i64_slots[i];
        } else if (strncmp(type_str, "i16", 3) == 0) {
            ffi_arg_types[i] = &ffi_type_sint16;
            i64_slots[i] = (int16_t)(intptr_t)argv[i];
            ffi_values[i] = &i64_slots[i];
        } else if (strncmp(type_str, "str:", 4) == 0 || strncmp(type_str, "string", 6) == 0) {
            ffi_arg_types[i] = &ffi_type_pointer;
            ffi_values[i] = &argv[i];
        } else {
            ffi_arg_types[i] = &ffi_type_sint64;
            i64_slots[i] = (int64_t)(intptr_t)argv[i];
            ffi_values[i] = &i64_slots[i];
        }
    }

    ffi_type *ffi_ret = &ffi_type_void;
    const char *rt = ret_type.data;
    if (strncmp(rt, "void", 4) == 0) ffi_ret = &ffi_type_void;
    else if (strncmp(rt, "i32", 3) == 0) ffi_ret = &ffi_type_sint32;
    else if (strncmp(rt, "i64", 3) == 0) ffi_ret = &ffi_type_sint64;
    else if (strncmp(rt, "f32", 3) == 0) ffi_ret = &ffi_type_float;
    else if (strncmp(rt, "f64", 3) == 0) ffi_ret = &ffi_type_double;
    else if (strncmp(rt, "bool", 4) == 0 || strncmp(rt, "u8", 2) == 0) ffi_ret = &ffi_type_uint8;
    else if (strncmp(rt, "i16", 3) == 0) ffi_ret = &ffi_type_sint16;
    else ffi_ret = &ffi_type_pointer;

    ffi_cif cif;
    ffi_status status = ffi_prep_cif(&cif, FFI_DEFAULT_ABI, argc, ffi_ret, ffi_arg_types);
    if (status != FFI_OK) {
        free(ffi_arg_types); free(ffi_values); free(i64_slots); free(f64_slots);
        return rl_err_msg(rl_str_literal("c::call: ffi_prep_cif failed", 28));
    }

    union { int64_t i; double f; int32_t i32; uint8_t u8; int16_t i16; void *ptr; } retbuf;
    ffi_call(&cif, FFI_FN(sym), &retbuf, ffi_values);

    free(ffi_arg_types); free(ffi_values); free(i64_slots); free(f64_slots);

    if (strncmp(rt, "void", 4) == 0) return rl_ok_null();
    else if (strncmp(rt, "i32", 3) == 0) return rl_ok_i64(retbuf.i32);
    else if (strncmp(rt, "i64", 3) == 0) return rl_ok_i64(retbuf.i);
    else if (strncmp(rt, "f32", 3) == 0) return rl_ok_f64((double)retbuf.f);
    else if (strncmp(rt, "f64", 3) == 0) return rl_ok_f64(retbuf.f);
    else if (strncmp(rt, "bool", 4) == 0 || strncmp(rt, "u8", 2) == 0) return rl_ok_i64(retbuf.u8);
    else if (strncmp(rt, "i16", 3) == 0) return rl_ok_i64(retbuf.i16);
    else if (strncmp(rt, "str", 3) == 0 || strncmp(rt, "string", 6) == 0) {
        const char *s = (const char *)retbuf.ptr;
        if (!s) return rl_ok_str(rl_str_literal("", 0));
        uint64_t len = strlen(s);
        char *buf = malloc(len + 1);
        memcpy(buf, s, len + 1);
        return rl_ok_str((rl_string){ .data = buf, .len = len, .rc = 1 });
    }
    return rl_ok_i64(retbuf.i);
#endif
}

// ---- std::net (TCP/UDP) ----

#include <sys/socket.h>
#include <netinet/in.h>
#include <netinet/tcp.h>
#include <arpa/inet.h>
#include <netdb.h>
#include <fcntl.h>
#include <poll.h>

#define RL_NET_MAX_HANDLES 256
#define RL_NET_BUF_SIZE 65536

enum rl_net_handle_kind { RL_NET_TCP_LISTENER, RL_NET_TCP_STREAM, RL_NET_UDP_SOCKET };

static struct {
    enum rl_net_handle_kind kind;
    int fd;
} rl_net_handles[RL_NET_MAX_HANDLES];
static int rl_net_handle_count = 0;

// Allocate a tagged socket handle id for fd and kind.
static rl_result rl_net_new_handle(int fd, enum rl_net_handle_kind kind) {
    if (rl_net_handle_count >= RL_NET_MAX_HANDLES) {
        return rl_err(-1);
    }
    int idx = rl_net_handle_count++;
    rl_net_handles[idx].kind = kind;
    rl_net_handles[idx].fd = fd;
    return rl_ok_i64(RL_HANDLE_NET_BASE + (int64_t)idx);
}

// Decode a tagged id to a table index, or -1 when out of range.
static int64_t _rl_net_idx(int64_t tagged) {
    int64_t idx = tagged - RL_HANDLE_NET_BASE;
    if (idx < 0 || idx >= rl_net_handle_count) return -1;
    return idx;
}

// Look up fd by handle id, or -1 when kind mismatches.
static int rl_net_get_fd(int64_t handle_id, enum rl_net_handle_kind expected) {
    int64_t idx = _rl_net_idx(handle_id);
    if (idx < 0) return -1;
    if (rl_net_handles[idx].kind != expected) return -1;
    return rl_net_handles[idx].fd;
}

// Parse ip:port text into sockaddr_in (0 on success).
static int rl_net_resolve_addr(const char *addr_str, struct sockaddr_in *out) {
    struct addrinfo hints = {0}, *res;
    hints.ai_family = AF_INET;
    hints.ai_socktype = SOCK_STREAM;
    int rc = getaddrinfo(addr_str, NULL, &hints, &res);
    if (rc != 0) return -1;
    memcpy(out, res->ai_addr, sizeof(struct sockaddr_in));
    freeaddrinfo(res);
    return 0;
}

// Bind and listen; result holds the listener handle id.
rl_result rl_net_tcp_listen(rl_string address) {
    if (address.data == NULL) return rl_err(-1);
    char addr_buf[256];
    int len = address.len < 255 ? (int)address.len : 255;
    memcpy(addr_buf, address.data, len);
    addr_buf[len] = '\0';

    char *colon = strrchr(addr_buf, ':');
    if (!colon) return rl_err(-1);
    *colon = '\0';
    int port = atoi(colon + 1);

    struct sockaddr_in addr = {0};
    addr.sin_family = AF_INET;
    addr.sin_port = htons(port);
    if (inet_pton(AF_INET, addr_buf, &addr.sin_addr) != 1) {
        struct addrinfo hints = {0}, *res;
        hints.ai_family = AF_INET;
        hints.ai_socktype = SOCK_STREAM;
        if (getaddrinfo(addr_buf, NULL, &hints, &res) == 0) {
            memcpy(&addr, res->ai_addr, sizeof(struct sockaddr_in));
            freeaddrinfo(res);
        } else {
            return rl_err(-1);
        }
    }

    int fd = socket(AF_INET, SOCK_STREAM, 0);
    if (fd < 0) return rl_err(-1);

    int opt = 1;
    setsockopt(fd, SOL_SOCKET, SO_REUSEADDR, &opt, sizeof(opt));

    if (bind(fd, (struct sockaddr *)&addr, sizeof(addr)) < 0) { close(fd); return rl_err(-1); }
    if (listen(fd, 128) < 0) { close(fd); return rl_err(-1); }

    return rl_net_new_handle(fd, RL_NET_TCP_LISTENER);
}

// Accept one client; result holds the connection handle id.
rl_result rl_net_tcp_accept(int64_t handle_id) {
    int fd = rl_net_get_fd(handle_id, RL_NET_TCP_LISTENER);
    if (fd < 0) return rl_err(-1);

    int client = accept(fd, NULL, NULL);
    if (client < 0) return rl_err(-1);

    return rl_net_new_handle(client, RL_NET_TCP_STREAM);
}

// Connect; result holds the connection handle id.
rl_result rl_net_tcp_connect(rl_string address) {
    if (address.data == NULL) return rl_err(-1);
    char addr_buf[256];
    int len = address.len < 255 ? (int)address.len : 255;
    memcpy(addr_buf, address.data, len);
    addr_buf[len] = '\0';

    char *colon = strrchr(addr_buf, ':');
    if (!colon) return rl_err(-1);
    *colon = '\0';
    int port = atoi(colon + 1);

    struct sockaddr_in addr = {0};
    addr.sin_family = AF_INET;
    addr.sin_port = htons(port);
    if (inet_pton(AF_INET, addr_buf, &addr.sin_addr) != 1) {
        struct addrinfo hints = {0}, *res;
        hints.ai_family = AF_INET;
        hints.ai_socktype = SOCK_STREAM;
        if (getaddrinfo(addr_buf, NULL, &hints, &res) == 0) {
            memcpy(&addr, res->ai_addr, sizeof(struct sockaddr_in));
            freeaddrinfo(res);
        } else {
            return rl_err(-1);
        }
    }

    int fd = socket(AF_INET, SOCK_STREAM, 0);
    if (fd < 0) return rl_err(-1);

    if (connect(fd, (struct sockaddr *)&addr, sizeof(addr)) < 0) { close(fd); return rl_err(-1); }

    return rl_net_new_handle(fd, RL_NET_TCP_STREAM);
}

// Read up to `max_bytes`; result holds the bytes as a string.
rl_result rl_net_tcp_read(int64_t handle_id, int64_t max_bytes) {
    int fd = rl_net_get_fd(handle_id, RL_NET_TCP_STREAM);
    if (fd < 0) return rl_err(-1);

    int buf_size = max_bytes > 0 ? (int)max_bytes : RL_NET_BUF_SIZE;
    char *buf = malloc(buf_size);
    ssize_t n = read(fd, buf, buf_size);
    if (n < 0) { free(buf); return rl_err(-1); }
    if (n == 0) { free(buf); return rl_ok_str(rl_str_literal("", 0)); }

    return rl_ok_str(rl_str_literal(buf, n));
}

// Write all of `data`; result holds the byte count.
rl_result rl_net_tcp_write(int64_t handle_id, rl_string data) {
    int fd = rl_net_get_fd(handle_id, RL_NET_TCP_STREAM);
    if (fd < 0) return rl_err(-1);
    if (data.data == NULL) return rl_err(-1);

    ssize_t n = write(fd, data.data, data.len);
    if (n < 0) return rl_err(-1);

    return rl_ok_i64(n);
}

// Remote / local `"ip:port"` of the connection.
rl_result rl_net_tcp_peer_addr(int64_t handle_id) {
    int fd = rl_net_get_fd(handle_id, RL_NET_TCP_STREAM);
    if (fd < 0) return rl_err(-1);

    struct sockaddr_in addr;
    socklen_t len = sizeof(addr);
    if (getpeername(fd, (struct sockaddr *)&addr, &len) < 0) return rl_err(-1);

    char buf[64];
    snprintf(buf, sizeof(buf), "%s:%d", inet_ntoa(addr.sin_addr), ntohs(addr.sin_port));
    uint64_t slen = strlen(buf);
    char *dup = malloc(slen + 1);
    memcpy(dup, buf, slen + 1);
    return rl_ok_str(rl_str_literal(dup, slen));
}

// Remote / local `"ip:port"` of the connection.
rl_result rl_net_tcp_local_addr(int64_t handle_id) {
    int fd = rl_net_get_fd(handle_id, RL_NET_TCP_STREAM);
    if (fd < 0) return rl_err(-1);

    struct sockaddr_in addr;
    socklen_t len = sizeof(addr);
    if (getsockname(fd, (struct sockaddr *)&addr, &len) < 0) return rl_err(-1);

    char buf[64];
    snprintf(buf, sizeof(buf), "%s:%d", inet_ntoa(addr.sin_addr), ntohs(addr.sin_port));
    uint64_t slen = strlen(buf);
    char *dup = malloc(slen + 1);
    memcpy(dup, buf, slen + 1);
    return rl_ok_str(rl_str_literal(dup, slen));
}

// Read/write timeout in milliseconds (0 disables).
rl_result rl_net_tcp_set_timeout(int64_t handle_id, int64_t millis) {
    int fd = rl_net_get_fd(handle_id, RL_NET_TCP_STREAM);
    if (fd < 0) return rl_err(-1);

    struct timeval tv;
    tv.tv_sec = millis / 1000;
    tv.tv_usec = (millis % 1000) * 1000;

    if (setsockopt(fd, SOL_SOCKET, SO_RCVTIMEO, &tv, sizeof(tv)) < 0) return rl_err(-1);
    if (setsockopt(fd, SOL_SOCKET, SO_SNDTIMEO, &tv, sizeof(tv)) < 0) return rl_err(-1);

    return rl_ok_null();
}

// Toggle non-blocking mode.
rl_result rl_net_tcp_set_nonblocking(int64_t handle_id, bool flag) {
    int fd = rl_net_get_fd(handle_id, RL_NET_TCP_STREAM);
    if (fd < 0) return rl_err(-1);

    int flags = fcntl(fd, F_GETFL, 0);
    if (flags < 0) return rl_err(-1);

    if (flag) flags |= O_NONBLOCK;
    else flags &= ~O_NONBLOCK;

    if (fcntl(fd, F_SETFL, flags) < 0) return rl_err(-1);

    return rl_ok_null();
}

// Half-close the read side, write side, or both ("r" / "w" / "rw").
rl_result rl_net_tcp_shutdown(int64_t handle_id, rl_string mode) {
    int fd = rl_net_get_fd(handle_id, RL_NET_TCP_STREAM);
    if (fd < 0) return rl_err(-1);

    int how;
    if (mode.len == 4 && memcmp(mode.data, "read", 4) == 0) how = SHUT_RD;
    else if (mode.len == 5 && memcmp(mode.data, "write", 5) == 0) how = SHUT_WR;
    else if (mode.len == 4 && memcmp(mode.data, "both", 4) == 0) how = SHUT_RDWR;
    else return rl_err(-1);

    if (shutdown(fd, how) < 0) return rl_err(-1);

    return rl_ok_null();
}

// Close the socket.
rl_result rl_net_tcp_close(int64_t handle_id) {
    int64_t idx = _rl_net_idx(handle_id);
    if (idx < 0) return rl_err(-1);

    enum rl_net_handle_kind kind = rl_net_handles[idx].kind;
    if (kind != RL_NET_TCP_LISTENER && kind != RL_NET_TCP_STREAM) return rl_err(-1);

    close(rl_net_handles[idx].fd);
    rl_net_handles[idx].fd = -1;

    return rl_ok_null();
}

// Bind a UDP socket; result holds its handle id.
rl_result rl_net_udp_bind(rl_string address) {
    if (address.data == NULL) return rl_err(-1);
    char addr_buf[256];
    int len = address.len < 255 ? (int)address.len : 255;
    memcpy(addr_buf, address.data, len);
    addr_buf[len] = '\0';

    char *colon = strrchr(addr_buf, ':');
    if (!colon) return rl_err(-1);
    *colon = '\0';
    int port = atoi(colon + 1);

    struct sockaddr_in addr = {0};
    addr.sin_family = AF_INET;
    addr.sin_port = htons(port);
    addr.sin_addr.s_addr = INADDR_ANY;

    int fd = socket(AF_INET, SOCK_DGRAM, 0);
    if (fd < 0) return rl_err(-1);

    int opt = 1;
    setsockopt(fd, SOL_SOCKET, SO_REUSEADDR, &opt, sizeof(opt));

    if (bind(fd, (struct sockaddr *)&addr, sizeof(addr)) < 0) { close(fd); return rl_err(-1); }

    return rl_net_new_handle(fd, RL_NET_UDP_SOCKET);
}

// Fix a default peer for `send` (does not handshake).
rl_result rl_net_udp_connect(int64_t handle_id, rl_string address) {
    int fd = rl_net_get_fd(handle_id, RL_NET_UDP_SOCKET);
    if (fd < 0) return rl_err(-1);
    if (address.data == NULL) return rl_err(-1);

    char addr_buf[256];
    int len = address.len < 255 ? (int)address.len : 255;
    memcpy(addr_buf, address.data, len);
    addr_buf[len] = '\0';

    char *colon = strrchr(addr_buf, ':');
    if (!colon) return rl_err(-1);
    *colon = '\0';
    int port = atoi(colon + 1);

    struct sockaddr_in addr = {0};
    addr.sin_family = AF_INET;
    addr.sin_port = htons(port);
    if (inet_pton(AF_INET, addr_buf, &addr.sin_addr) != 1) return rl_err(-1);

    if (connect(fd, (struct sockaddr *)&addr, sizeof(addr)) < 0) return rl_err(-1);

    return rl_ok_null();
}

// Send to the default peer / to an explicit address; result holds bytes sent.
rl_result rl_net_udp_send(int64_t handle_id, rl_string data) {
    int fd = rl_net_get_fd(handle_id, RL_NET_UDP_SOCKET);
    if (fd < 0) return rl_err(-1);
    if (data.data == NULL) return rl_err(-1);

    ssize_t n = send(fd, data.data, data.len, 0);
    if (n < 0) return rl_err(-1);

    return rl_ok_i64(n);
}

// Send to the default peer / to an explicit address; result holds bytes sent.
rl_result rl_net_udp_send_to(int64_t handle_id, rl_string data, rl_string address) {
    int fd = rl_net_get_fd(handle_id, RL_NET_UDP_SOCKET);
    if (fd < 0) return rl_err(-1);
    if (data.data == NULL) return rl_err(-1);
    if (address.data == NULL) return rl_err(-1);

    char addr_buf[256];
    int len = address.len < 255 ? (int)address.len : 255;
    memcpy(addr_buf, address.data, len);
    addr_buf[len] = '\0';

    char *colon = strrchr(addr_buf, ':');
    if (!colon) return rl_err(-1);
    *colon = '\0';
    int port = atoi(colon + 1);

    struct sockaddr_in addr = {0};
    addr.sin_family = AF_INET;
    addr.sin_port = htons(port);
    if (inet_pton(AF_INET, addr_buf, &addr.sin_addr) != 1) return rl_err(-1);

    ssize_t n = sendto(fd, data.data, data.len, 0, (struct sockaddr *)&addr, sizeof(addr));
    if (n < 0) return rl_err(-1);

    return rl_ok_i64(n);
}

// Canonical 2-tuple layouts used for single-tuple results. These match
// the program generated `rl_tuple_2` structs field for field:
// (int, string) for HTTP responses, (string, string) for UDP sender
// pairs and HTTP header pairs.
typedef struct { int64_t field_0; rl_string field_1; } _rl_tuple_is;
typedef struct { rl_string field_0; rl_string field_1; } _rl_tuple_ss;

// Wrap one (int, string) tuple as a single element array result.
// Uses I64 tag like rl_arr_zip_t so generic array printing does not
// mistake the bytes for nested arrays.
static rl_result _rl_ok_tuple_is(int64_t f0, char *bdata, uint64_t blen) {
    _rl_tuple_is *slot = malloc(sizeof(_rl_tuple_is));
    slot->field_0 = f0;
    slot->field_1 = (rl_string){ .data = bdata, .len = blen, .rc = 1 };
    rl_array out;
    out.data = slot;
    out.len = 1;
    out.cap = 1;
    out.elem_size = (int32_t)sizeof(_rl_tuple_is);
    out.type_tag = RL_TAG_I64;
    return rl_ok_arr(out);
}

// Wrap one (string, string) tuple as a single element array result.
static rl_result _rl_ok_tuple_ss(char *adata, uint64_t alen, char *bdata, uint64_t blen) {
    _rl_tuple_ss *slot = malloc(sizeof(_rl_tuple_ss));
    slot->field_0 = (rl_string){ .data = adata, .len = alen, .rc = 1 };
    slot->field_1 = (rl_string){ .data = bdata, .len = blen, .rc = 1 };
    rl_array out;
    out.data = slot;
    out.len = 1;
    out.cap = 1;
    out.elem_size = (int32_t)sizeof(_rl_tuple_ss);
    out.type_tag = RL_TAG_I64;
    return rl_ok_arr(out);
}

// Receive one datagram / datagram plus sender address as a 2-tuple.
rl_result rl_net_udp_recv(int64_t handle_id, int64_t max_bytes) {
    int fd = rl_net_get_fd(handle_id, RL_NET_UDP_SOCKET);
    if (fd < 0) return rl_err(-1);

    int buf_size = max_bytes > 0 ? (int)max_bytes : RL_NET_BUF_SIZE;
    char *buf = malloc(buf_size);
    ssize_t n = recv(fd, buf, buf_size, 0);
    if (n < 0) { free(buf); return rl_err(-1); }

    return rl_ok_str(rl_str_literal(buf, n));
}

// Receive one datagram plus sender address as a 2-tuple
// (data string, sender "ip:port" string).
rl_result rl_net_udp_recv_from(int64_t handle_id, int64_t max_bytes) {
    int fd = rl_net_get_fd(handle_id, RL_NET_UDP_SOCKET);
    if (fd < 0) return rl_err(-1);

    int buf_size = max_bytes > 0 ? (int)max_bytes : RL_NET_BUF_SIZE;
    char *buf = malloc(buf_size + 1);
    struct sockaddr_in sender;
    socklen_t sender_len = sizeof(sender);

    ssize_t n = recvfrom(fd, buf, buf_size, 0, (struct sockaddr *)&sender, &sender_len);
    if (n < 0) { free(buf); return rl_err(-1); }
    if (n == 0) {
        char *empty = malloc(1);
        empty[0] = '\0';
        char addr_str[64];
        snprintf(addr_str, sizeof(addr_str), "%s:%d", inet_ntoa(sender.sin_addr), ntohs(sender.sin_port));
        uint64_t addr_len = strlen(addr_str);
        char *addr_dup = malloc(addr_len + 1);
        memcpy(addr_dup, addr_str, addr_len + 1);
        free(buf);
        return _rl_ok_tuple_ss(empty, 0, addr_dup, addr_len);
    }

    char *data = malloc((uint64_t)n + 1);
    memcpy(data, buf, (uint64_t)n);
    data[n] = '\0';
    free(buf);

    char addr_str[64];
    snprintf(addr_str, sizeof(addr_str), "%s:%d", inet_ntoa(sender.sin_addr), ntohs(sender.sin_port));
    uint64_t addr_len = strlen(addr_str);
    char *addr_dup = malloc(addr_len + 1);
    memcpy(addr_dup, addr_str, addr_len + 1);

    return _rl_ok_tuple_ss(data, (uint64_t)n, addr_dup, addr_len);
}

// Close the socket.
rl_result rl_net_udp_close(int64_t handle_id) {
    int64_t idx = _rl_net_idx(handle_id);
    if (idx < 0) return rl_err(-1);
    if (rl_net_handles[idx].kind != RL_NET_UDP_SOCKET) return rl_err(-1);

    close(rl_net_handles[idx].fd);
    rl_net_handles[idx].fd = -1;

    return rl_ok_null();
}

// DNS lookup of `"host:port"`; result holds an array of `"ip"` strings.
rl_result rl_net_resolve(rl_string host_port) {
    if (host_port.data == NULL) return rl_err(-1);
    char buf[256];
    int len = host_port.len < 255 ? (int)host_port.len : 255;
    memcpy(buf, host_port.data, len);
    buf[len] = '\0';

    // Split "host:port" like the VM's to_socket_addrs does; the port is
    // only used to validate the shape, the output is IP strings.
    char *colon = strrchr(buf, ':');
    char *host = buf;
    char *port = NULL;
    if (colon) {
        *colon = '\0';
        host = buf;
        port = colon + 1;
        if (*host == '\0' || *port == '\0') return rl_err(-1);
    }

    struct addrinfo hints = {0}, *res;
    hints.ai_family = AF_INET;
    hints.ai_socktype = SOCK_STREAM;

    int rc = getaddrinfo(host, port, &hints, &res);
    if (rc != 0) return rl_err(-1);

    int count = 0;
    for (struct addrinfo *p = res; p != NULL; p = p->ai_next) count++;
    if (count == 0) {
        freeaddrinfo(res);
        rl_array empty = { .data = NULL, .len = 0, .cap = 0,
            .elem_size = sizeof(rl_string), .type_tag = RL_TAG_STR };
        return rl_ok_arr(empty);
    }

    rl_string *items = malloc((uint64_t)count * sizeof(rl_string));
    int i = 0;
    for (struct addrinfo *p = res; p != NULL; p = p->ai_next) {
        struct sockaddr_in *addr = (struct sockaddr_in *)p->ai_addr;
        char ip[64];
        inet_ntop(AF_INET, &addr->sin_addr, ip, sizeof(ip));
        uint64_t ip_len = strlen(ip);
        char *ip_dup = malloc(ip_len + 1);
        memcpy(ip_dup, ip, ip_len + 1);
        items[i++] = (rl_string){ .data = ip_dup, .len = ip_len, .rc = 1 };
    }

    freeaddrinfo(res);
    rl_array out;
    out.data = items;
    out.len = (uint64_t)count;
    out.cap = (uint64_t)count;
    out.elem_size = (int32_t)sizeof(rl_string);
    out.type_tag = RL_TAG_STR;
    return rl_ok_arr(out);
}

// ---- std::http (server + client via POSIX sockets + libcurl) ----

#ifdef RL_USE_CURL
#include <curl/curl.h>
#endif

#include <sys/socket.h>
#include <netinet/in.h>
#include <arpa/inet.h>
#include <poll.h>

#define RL_HTTP_MAX_HANDLES 256
#define RL_HTTP_BUF_SIZE 65536

enum rl_http_handle_kind { RL_HTTP_SERVER, RL_HTTP_REQUEST };

struct rl_http_request_data {
    char method[16];
    char url[2048];
    char headers_raw[RL_HTTP_BUF_SIZE];
    char *body;
    int64_t body_len;
    int client_fd;
};

static struct {
    enum rl_http_handle_kind kind;
    union {
        int server_fd;
        struct rl_http_request_data *request;
    } data;
} rl_http_handles[RL_HTTP_MAX_HANDLES];
static int rl_http_handle_count = 0;

// Allocate a tagged HTTP handle id for a table entry.
static rl_result rl_http_new_handle(void *ptr, enum rl_http_handle_kind kind) {
    if (rl_http_handle_count >= RL_HTTP_MAX_HANDLES) return rl_err(-1);
    int idx = rl_http_handle_count++;
    rl_http_handles[idx].kind = kind;
    if (kind == RL_HTTP_SERVER) {
        rl_http_handles[idx].data.server_fd = *(int *)ptr;
    } else {
        rl_http_handles[idx].data.request = (struct rl_http_request_data *)ptr;
    }
    return rl_ok_i64(RL_HANDLE_HTTP_BASE + (int64_t)idx);
}

// Decode a tagged id to a table index, or -1 when out of range.
static int64_t _rl_http_idx(int64_t tagged) {
    int64_t idx = tagged - RL_HANDLE_HTTP_BASE;
    if (idx < 0 || idx >= rl_http_handle_count) return -1;
    return idx;
}

// minimal HTTP/1.1 server: bind, listen, accept, parse request, return handle

// Start listening on addr; result holds the server id.
rl_result rl_http_server_start(rl_string addr) {
    if (addr.data == NULL) return rl_err(-1);
    char buf[256];
    int len = addr.len < 255 ? (int)addr.len : 255;
    memcpy(buf, addr.data, len);
    buf[len] = '\0';

    char *colon = strrchr(buf, ':');
    if (!colon) return rl_err(-1);
    *colon = '\0';
    int port = atoi(colon + 1);

    struct sockaddr_in sa = {0};
    sa.sin_family = AF_INET;
    sa.sin_port = htons(port);
    sa.sin_addr.s_addr = INADDR_ANY;

    int fd = socket(AF_INET, SOCK_STREAM, 0);
    if (fd < 0) return rl_err(-1);

    int opt = 1;
    setsockopt(fd, SOL_SOCKET, SO_REUSEADDR, &opt, sizeof(opt));

    if (bind(fd, (struct sockaddr *)&sa, sizeof(sa)) < 0) { close(fd); return rl_err(-1); }
    if (listen(fd, 128) < 0) { close(fd); return rl_err(-1); }

    return rl_http_new_handle(&fd, RL_HTTP_SERVER);
}

// Read one CRLF-terminated line from fd.
static int rl_http_read_line(int fd, char *buf, int max) {
    int n = 0;
    while (n < max - 1) {
        char c;
        if (read(fd, &c, 1) <= 0) break;
        buf[n++] = c;
        if (c == '\n') break;
    }
    buf[n] = '\0';
    return n;
}

// Block for the next request / poll without blocking (error when none); result holds the request id.
rl_result rl_http_server_recv(int64_t handle_id) {
    int64_t hidx = _rl_http_idx(handle_id);
    if (hidx < 0) return rl_err(-1);
    if (rl_http_handles[hidx].kind != RL_HTTP_SERVER) return rl_err(-1);

    int server_fd = rl_http_handles[hidx].data.server_fd;
    struct sockaddr_in client_addr;
    socklen_t addr_len = sizeof(client_addr);
    int client_fd = accept(server_fd, (struct sockaddr *)&client_addr, &addr_len);
    if (client_fd < 0) return rl_err(-1);

    struct rl_http_request_data *req = calloc(1, sizeof(struct rl_http_request_data));
    req->client_fd = client_fd;

    // parse request line: "METHOD /path HTTP/1.1\r\n"
    char line[2048];
    int n = rl_http_read_line(client_fd, line, sizeof(line));
    if (n <= 0) { close(client_fd); free(req); return rl_err(-1); }

    // strip \r\n
    char *cr = strchr(line, '\r'); if (cr) *cr = '\0';
    char *nl = strchr(line, '\n'); if (nl) *nl = '\0';

    char *sp1 = strchr(line, ' ');
    char *sp2 = sp1 ? strchr(sp1 + 1, ' ') : NULL;
    if (!sp1 || !sp2) { close(client_fd); free(req); return rl_err(-1); }

    *sp1 = '\0'; *sp2 = '\0';
    strncpy(req->method, line, sizeof(req->method) - 1);
    strncpy(req->url, sp1 + 1, sizeof(req->url) - 1);

    // read headers into raw buffer
    int hdr_pos = 0;
    int content_length = 0;
    for (;;) {
        n = rl_http_read_line(client_fd, line, sizeof(line));
        if (n <= 0) break;
        cr = strchr(line, '\r'); if (cr) *cr = '\0';
        nl = strchr(line, '\n'); if (nl) *nl = '\0';
        if (strlen(line) == 0) break; // empty line = end of headers

        if (hdr_pos + (int)strlen(line) + 2 < (int)sizeof(req->headers_raw)) {
            memcpy(req->headers_raw + hdr_pos, line, strlen(line));
            hdr_pos += strlen(line);
            req->headers_raw[hdr_pos++] = '\n';
            req->headers_raw[hdr_pos] = '\0';
        }

        // extract Content-Length
        if (strncasecmp(line, "Content-Length:", 15) == 0) {
            content_length = atoi(line + 15);
        }
    }

    // read body
    if (content_length > 0 && content_length < RL_HTTP_BUF_SIZE) {
        req->body = malloc(content_length + 1);
        int total = 0;
        while (total < content_length) {
            int r = read(client_fd, req->body + total, content_length - total);
            if (r <= 0) break;
            total += r;
        }
        req->body[total] = '\0';
        req->body_len = total;
    } else {
        req->body = strdup("");
        req->body_len = 0;
    }

    return rl_http_new_handle(req, RL_HTTP_REQUEST);
}

// Block for the next request / poll without blocking (error when none); result holds the request id.
rl_result rl_http_server_try_recv(int64_t handle_id) {
    int64_t hidx = _rl_http_idx(handle_id);
    if (hidx < 0) return rl_err(-1);
    if (rl_http_handles[hidx].kind != RL_HTTP_SERVER) return rl_err(-1);

    int server_fd = rl_http_handles[hidx].data.server_fd;

    struct pollfd pfd = { .fd = server_fd, .events = POLLIN };
    int ret = poll(&pfd, 1, 0);
    if (ret <= 0) return rl_ok_null();

    return rl_http_server_recv(handle_id);
}

// Stop the server and drop pending requests.
rl_result rl_http_server_stop(int64_t handle_id) {
    int64_t hidx = _rl_http_idx(handle_id);
    if (hidx < 0) return rl_err(-1);
    if (rl_http_handles[hidx].kind != RL_HTTP_SERVER) return rl_err(-1);

    close(rl_http_handles[hidx].data.server_fd);
    rl_http_handles[hidx].data.server_fd = -1;
    return rl_ok_null();
}

// Method ("GET", ...) / path+query / one header / full body of a request.
rl_result rl_http_request_method(int64_t handle_id) {
    int64_t hidx = _rl_http_idx(handle_id);
    if (hidx < 0) return rl_err(-1);
    if (rl_http_handles[hidx].kind != RL_HTTP_REQUEST) return rl_err(-1);

    struct rl_http_request_data *req = rl_http_handles[hidx].data.request;
    uint64_t slen = strlen(req->method);
    char *dup = malloc(slen + 1);
    memcpy(dup, req->method, slen + 1);
    return rl_ok_str(rl_str_literal(dup, slen));
}

// Method ("GET", ...) / path+query / one header / full body of a request.
rl_result rl_http_request_url(int64_t handle_id) {
    int64_t hidx = _rl_http_idx(handle_id);
    if (hidx < 0) return rl_err(-1);
    if (rl_http_handles[hidx].kind != RL_HTTP_REQUEST) return rl_err(-1);

    struct rl_http_request_data *req = rl_http_handles[hidx].data.request;
    uint64_t slen = strlen(req->url);
    char *dup = malloc(slen + 1);
    memcpy(dup, req->url, slen + 1);
    return rl_ok_str(rl_str_literal(dup, slen));
}

// Method ("GET", ...) / path+query / one header / full body of a request.
rl_result rl_http_request_header(int64_t handle_id, rl_string name) {
    int64_t hidx = _rl_http_idx(handle_id);
    if (hidx < 0) return rl_err(-1);
    if (rl_http_handles[hidx].kind != RL_HTTP_REQUEST) return rl_err(-1);
    if (name.data == NULL) return rl_err(-1);

    struct rl_http_request_data *req = rl_http_handles[hidx].data.request;

    // search headers_raw for "Name: value"
    char needle[512];
    int nlen = name.len < 510 ? (int)name.len : 510;
    memcpy(needle, name.data, nlen);
    needle[nlen] = '\0';

    char *found = NULL;
    char *line = req->headers_raw;
    while (*line) {
        if (strncasecmp(line, needle, nlen) == 0 && line[nlen] == ':') {
            line += nlen + 1;
            while (*line == ' ') line++;
            char *end = strchr(line, '\n');
            int vlen = end ? (int)(end - line) : (int)strlen(line);
            char *val = malloc(vlen + 1);
            memcpy(val, line, vlen);
            val[vlen] = '\0';
            return rl_ok_str(rl_str_literal(val, vlen));
        }
        char *next = strchr(line, '\n');
        if (!next) break;
        line = next + 1;
    }

    return rl_err(-1);
}

// Method ("GET", ...) / path+query / one header / full body of a request.
rl_result rl_http_request_body(int64_t handle_id) {
    int64_t hidx = _rl_http_idx(handle_id);
    if (hidx < 0) return rl_err(-1);
    if (rl_http_handles[hidx].kind != RL_HTTP_REQUEST) return rl_err(-1);

    struct rl_http_request_data *req = rl_http_handles[hidx].data.request;
    uint64_t slen = req->body_len;
    char *dup = malloc(slen + 1);
    memcpy(dup, req->body, slen);
    dup[slen] = '\0';
    return rl_ok_str(rl_str_literal(dup, slen));
}

// Answer a request and close it; pass `has_content_type` 0 to omit.
rl_result rl_http_respond(int64_t handle_id, int64_t status, rl_string body, rl_string content_type, int has_content_type) {
    int64_t hidx = _rl_http_idx(handle_id);
    if (hidx < 0) return rl_err(-1);
    if (rl_http_handles[hidx].kind != RL_HTTP_REQUEST) return rl_err(-1);
    if (status < 100 || status > 599) return rl_err(-1);
    if (body.data == NULL) return rl_err(-1);
    if (has_content_type && content_type.data == NULL) return rl_err(-1);

    struct rl_http_request_data *req = rl_http_handles[hidx].data.request;
    int fd = req->client_fd;

    const char *status_text = "OK";
    if (status == 201) status_text = "Created";
    else if (status == 404) status_text = "Not Found";
    else if (status == 500) status_text = "Internal Server Error";
    else if (status == 400) status_text = "Bad Request";
    else if (status == 403) status_text = "Forbidden";
    else if (status == 204) status_text = "No Content";
    else if (status == 301) status_text = "Moved Permanently";
    else if (status == 302) status_text = "Found";

    char header_buf[RL_HTTP_BUF_SIZE];
    int hlen = snprintf(header_buf, sizeof(header_buf),
        "HTTP/1.1 %ld %s\r\n"
        "Content-Length: %ld\r\n",
        (long)status, status_text, (long)body.len);

    if (has_content_type && content_type.len > 0) {
        hlen += snprintf(header_buf + hlen, sizeof(header_buf) - hlen,
            "Content-Type: %.*s\r\n", (int)content_type.len, content_type.data);
    } else {
        hlen += snprintf(header_buf + hlen, sizeof(header_buf) - hlen,
            "Content-Type: text/plain\r\n");
    }

    hlen += snprintf(header_buf + hlen, sizeof(header_buf) - hlen, "\r\n");

    write(fd, header_buf, hlen);
    if (body.len > 0) write(fd, body.data, body.len);
    close(fd);

    // clean up request handle
    free(req->body);
    free(req);
    rl_http_handles[hidx].kind = RL_HTTP_SERVER; // mark as consumed
    rl_http_handles[hidx].data.server_fd = -1;

    return rl_ok_null();
}

// ---- client (libcurl) ----

#ifdef RL_USE_CURL

struct rl_http_curl_buf {
    char *data;
    size_t len;
    size_t cap;
};

// Append curl response bytes to a growing buffer.
static size_t rl_http_curl_write_cb(void *ptr, size_t size, size_t nmemb, void *userdata) {
    struct rl_http_curl_buf *buf = (struct rl_http_curl_buf *)userdata;
    size_t new_len = buf->len + size * nmemb;
    if (new_len > buf->cap) {
        buf->cap = new_len * 2;
        buf->data = realloc(buf->data, buf->cap);
    }
    memcpy(buf->data + buf->len, ptr, size * nmemb);
    buf->len = new_len;
    return size * nmemb;
}

// Run a curl request and wrap (status, body) as a 2-tuple result.
static rl_result rl_http_curl_perform(CURL *curl) {
    struct rl_http_curl_buf resp = {0};
    resp.cap = 4096;
    resp.data = malloc(resp.cap);

    long status = 0;
    curl_easy_setopt(curl, CURLOPT_WRITEFUNCTION, rl_http_curl_write_cb);
    curl_easy_setopt(curl, CURLOPT_WRITEDATA, &resp);

    CURLcode res = curl_easy_perform(curl);
    if (res != CURLE_OK) {
        free(resp.data);
        return rl_err(-1);
    }
    curl_easy_getinfo(curl, CURLINFO_RESPONSE_CODE, &status);

    // body is exactly what curl wrote
    size_t body_len = resp.len;
    char *body_copy = malloc(body_len + 1);
    if (body_len > 0) memcpy(body_copy, resp.data, body_len);
    body_copy[body_len] = '\0';

    rl_result out = _rl_ok_tuple_is((int64_t)status, body_copy, (uint64_t)body_len);
    free(resp.data);
    return out;
}

// GET shorthand; result holds a 2-tuple (status int, body string).
rl_result rl_http_get(rl_string url) {
    if (url.data == NULL) return rl_err(-1);
    char url_buf[2048];
    int ulen = url.len < 2047 ? (int)url.len : 2047;
    memcpy(url_buf, url.data, ulen);
    url_buf[ulen] = '\0';

    CURL *curl = curl_easy_init();
    if (!curl) return rl_err(-1);

    curl_easy_setopt(curl, CURLOPT_URL, url_buf);
    curl_easy_setopt(curl, CURLOPT_FOLLOWLOCATION, 1L);
    curl_easy_setopt(curl, CURLOPT_TIMEOUT, 30L);

    rl_result result = rl_http_curl_perform(curl);
    curl_easy_cleanup(curl);
    return result;
}

// POST shorthand; result holds a 2-tuple (status int, body string).
rl_result rl_http_post(rl_string url, rl_string body, rl_string content_type, int has_content_type) {
    if (url.data == NULL) return rl_err(-1);
    if (body.data == NULL) return rl_err(-1);
    if (has_content_type && content_type.data == NULL) return rl_err(-1);
    char url_buf[2048];
    int ulen = url.len < 2047 ? (int)url.len : 2047;
    memcpy(url_buf, url.data, ulen);
    url_buf[ulen] = '\0';

    CURL *curl = curl_easy_init();
    if (!curl) return rl_err(-1);

    curl_easy_setopt(curl, CURLOPT_URL, url_buf);
    curl_easy_setopt(curl, CURLOPT_POST, 1L);
    curl_easy_setopt(curl, CURLOPT_POSTFIELDS, body.data);
    curl_easy_setopt(curl, CURLOPT_POSTFIELDSIZE, (long)body.len);
    curl_easy_setopt(curl, CURLOPT_FOLLOWLOCATION, 1L);
    curl_easy_setopt(curl, CURLOPT_TIMEOUT, 30L);

    char ct_buf[512];
    if (has_content_type && content_type.len > 0) {
        int ctlen = content_type.len < 510 ? (int)content_type.len : 510;
        memcpy(ct_buf, content_type.data, ctlen);
        ct_buf[ctlen] = '\0';
    } else {
        strcpy(ct_buf, "text/plain");
    }
    struct curl_slist *headers = NULL;
    char hdr[600];
    snprintf(hdr, sizeof(hdr), "Content-Type: %s", ct_buf);
    headers = curl_slist_append(headers, hdr);
    curl_easy_setopt(curl, CURLOPT_HTTPHEADER, headers);

    rl_result result = rl_http_curl_perform(curl);
    curl_slist_free_all(headers);
    curl_easy_cleanup(curl);
    return result;
}

// Full client request; result holds a 2-tuple (status int, body string).
// `headers` is an array of (name string, value string) 2-tuples.
rl_result rl_http_request(rl_string method, rl_string url, rl_string body, int has_body, rl_array headers, int has_headers) {
    if (method.data == NULL) return rl_err(-1);
    if (url.data == NULL) return rl_err(-1);
    if (has_body && body.data == NULL) return rl_err(-1);
    char url_buf[2048];
    int ulen = url.len < 2047 ? (int)url.len : 2047;
    memcpy(url_buf, url.data, ulen);
    url_buf[ulen] = '\0';

    char method_buf[16];
    int mlen = method.len < 15 ? (int)method.len : 15;
    memcpy(method_buf, method.data, mlen);
    method_buf[mlen] = '\0';

    CURL *curl = curl_easy_init();
    if (!curl) return rl_err(-1);

    curl_easy_setopt(curl, CURLOPT_URL, url_buf);
    curl_easy_setopt(curl, CURLOPT_FOLLOWLOCATION, 1L);
    curl_easy_setopt(curl, CURLOPT_TIMEOUT, 30L);

    // set custom method
    curl_easy_setopt(curl, CURLOPT_CUSTOMREQUEST, method_buf);

    // keep body bytes alive until after perform
    char *body_buf = NULL;
    if (has_body && body.len > 0) {
        body_buf = malloc(body.len);
        memcpy(body_buf, body.data, body.len);
        curl_easy_setopt(curl, CURLOPT_POSTFIELDS, body_buf);
        curl_easy_setopt(curl, CURLOPT_POSTFIELDSIZE, (long)body.len);
    } else if (has_body) {
        curl_easy_setopt(curl, CURLOPT_POSTFIELDS, "");
        curl_easy_setopt(curl, CURLOPT_POSTFIELDSIZE, 0L);
    }

    struct curl_slist *hdr_list = NULL;
    if (has_headers && headers.data != NULL && headers.len > 0) {
        if (headers.elem_size != (int32_t)sizeof(_rl_tuple_ss)) {
            if (body_buf) free(body_buf);
            curl_easy_cleanup(curl);
            return rl_err(-1);
        }
        _rl_tuple_ss *pairs = (_rl_tuple_ss *)headers.data;
        for (uint64_t i = 0; i < headers.len; i++) {
            rl_string k = pairs[i].field_0;
            rl_string v = pairs[i].field_1;
            if (k.data == NULL || v.data == NULL) {
                if (body_buf) free(body_buf);
                curl_slist_free_all(hdr_list);
                curl_easy_cleanup(curl);
                return rl_err(-1);
            }
            char *line = malloc(k.len + v.len + 4);
            memcpy(line, k.data, k.len);
            line[k.len] = ':';
            line[k.len + 1] = ' ';
            memcpy(line + k.len + 2, v.data, v.len);
            line[k.len + 2 + v.len] = '\0';
            hdr_list = curl_slist_append(hdr_list, line);
            free(line);
        }
        if (hdr_list) curl_easy_setopt(curl, CURLOPT_HTTPHEADER, hdr_list);
    }

    rl_result result = rl_http_curl_perform(curl);
    if (hdr_list) curl_slist_free_all(hdr_list);
    if (body_buf) free(body_buf);
    curl_easy_cleanup(curl);
    return result;
}

#else

// stubs when libcurl is not available
// Stub GET shorthand; returns an error when libcurl is unavailable.
rl_result rl_http_get(rl_string url) { (void)url; return rl_err(-1); }
// Stub POST shorthand; returns an error when libcurl is unavailable.
rl_result rl_http_post(rl_string url, rl_string body, rl_string ct, int h) { (void)url; (void)body; (void)ct; (void)h; return rl_err(-1); }
// Stub full request; returns an error when libcurl is unavailable.
rl_result rl_http_request(rl_string m, rl_string u, rl_string b, int hb, rl_array h, int hh) { (void)m; (void)u; (void)b; (void)hb; (void)h; (void)hh; return rl_err(-1); }

#endif

// ---- std::audio (playback via miniaudio) ----
#ifdef RL_USE_AUDIO

#include <stdarg.h>

#define RL_AUDIO_MAX_HANDLES 256

static struct {
    bool used;
    ma_sound *sound;
    float base_volume;
    bool paused;
} rl_audio_handles[RL_AUDIO_MAX_HANDLES];
static int rl_audio_handle_count = 0;
// Master volume applied to every sound, set by set_master_volume.
static float rl_audio_master_volume = 1.0f;
// Selected output device name (NULL means the system default).
static char *rl_audio_output_device = NULL;
static bool rl_audio_output_has_device = false;
static ma_device_id rl_audio_output_device_id;
// Device the shared engine currently runs on (NULL means default).
static char *rl_audio_engine_device = NULL;
static ma_engine rl_audio_engine;
static bool rl_audio_engine_ready = false;
static ma_result rl_audio_engine_status = MA_SUCCESS;

// Format an audio error message into an owned RL error result.
static rl_result _rl_audio_err(const char *fmt, ...) {
    char buf[1024];
    va_list ap;
    va_start(ap, fmt);
    int n = vsnprintf(buf, sizeof(buf), fmt, ap);
    va_end(ap);
    if (n < 0) return rl_err(-1);
    if (n >= (int)sizeof(buf)) n = (int)sizeof(buf) - 1;
    char *msg = malloc((uint64_t)n + 1);
    memcpy(msg, buf, (uint64_t)n + 1);
    return rl_err_msg((rl_string){ .data = msg, .len = (uint64_t)n, .rc = 1 });
}

// Copy an RL string into a NUL-terminated buffer. False when the RL
// string is null or does not fit; RL strings are never NUL-terminated
// so this copy is required before any C file or device API.
static bool _rl_audio_cstr(rl_string s, char *buf, uint64_t cap) {
    if (s.data == NULL || s.len + 1 > cap) return false;
    memcpy(buf, s.data, s.len);
    buf[s.len] = '\0';
    return true;
}

// True when the path opens for reading (errno reports why not).
static bool _rl_audio_probe_file(const char *cpath) {
    FILE *f = fopen(cpath, "rb");
    if (f == NULL) return false;
    fclose(f);
    return true;
}

// Start the shared engine on the selected output device when needed.
// Reuses the running engine; when the selection changed it restarts the
// engine only while no sound is live (live sounds keep their device,
// mirroring the VM where existing sinks stay on the old device).
static bool _rl_audio_ensure_engine(void) {
    const char *want = rl_audio_output_device;
    bool same = (want == NULL && rl_audio_engine_device == NULL)
        || (want != NULL && rl_audio_engine_device != NULL
            && strcmp(want, rl_audio_engine_device) == 0);
    if (rl_audio_engine_ready && same) {
        rl_audio_engine_status = MA_SUCCESS;
        return true;
    }
    if (rl_audio_engine_ready) {
        int live = 0;
        for (int i = 0; i < rl_audio_handle_count; i++) {
            if (rl_audio_handles[i].used) live++;
        }
        if (live > 0) {
            rl_audio_engine_status = MA_SUCCESS;
            return true;
        }
        ma_engine_uninit(&rl_audio_engine);
        rl_audio_engine_ready = false;
        free(rl_audio_engine_device);
        rl_audio_engine_device = NULL;
    }
    ma_engine_config config = ma_engine_config_init();
    if (rl_audio_output_has_device) {
        config.pPlaybackDeviceID = &rl_audio_output_device_id;
    }
    rl_audio_engine_status = ma_engine_init(&config, &rl_audio_engine);
    if (rl_audio_engine_status != MA_SUCCESS) return false;
    rl_audio_engine_ready = true;
    if (want != NULL) {
        rl_audio_engine_device = malloc(strlen(want) + 1);
        strcpy(rl_audio_engine_device, want);
    }
    return true;
}

// Allocate a tagged audio handle id for a live sound.
static rl_result _rl_audio_new_handle(ma_sound *sound, float base_volume) {
    if (rl_audio_handle_count >= RL_AUDIO_MAX_HANDLES) {
        return _rl_audio_err("audio: too many open handles");
    }
    int idx = rl_audio_handle_count++;
    rl_audio_handles[idx].used = true;
    rl_audio_handles[idx].sound = sound;
    rl_audio_handles[idx].base_volume = base_volume;
    rl_audio_handles[idx].paused = false;
    return rl_ok_i64(RL_HANDLE_AUDIO_BASE + (int64_t)idx);
}

// Live sound for a tagged id, or NULL when unknown or stopped. Never
// crashes on stale ids; callers turn NULL into an RL unknown-handle error.
static ma_sound *_rl_audio_get(int64_t tagged, int *out_idx) {
    int64_t idx = tagged - RL_HANDLE_AUDIO_BASE;
    if (idx < 0 || idx >= rl_audio_handle_count) return NULL;
    if (!rl_audio_handles[idx].used || rl_audio_handles[idx].sound == NULL) return NULL;
    if (out_idx != NULL) *out_idx = (int)idx;
    return rl_audio_handles[idx].sound;
}

// Play a file to completion; ok null on success, or an RL error.
rl_result rl_audio_play_file(rl_string path) {
    char cpath[4096];
    if (!_rl_audio_cstr(path, cpath, sizeof(cpath))) {
        return _rl_audio_err("play_file: invalid path");
    }
    if (!_rl_audio_probe_file(cpath)) {
        int e = errno;
        return _rl_audio_err("play_file(\"%s\"): %s (os error %d)", cpath, strerror(e), e);
    }
    if (!_rl_audio_ensure_engine()) {
        return _rl_audio_err("play_file: %s", ma_result_description(rl_audio_engine_status));
    }
    ma_sound sound;
    ma_result r = ma_sound_init_from_file(&rl_audio_engine, cpath, 0, NULL, NULL, &sound);
    if (r != MA_SUCCESS) {
        return _rl_audio_err("play_file(\"%s\"): %s", cpath, ma_result_description(r));
    }
    ma_sound_start(&sound);
    while (!ma_sound_at_end(&sound)) {
        ma_sleep(5);
    }
    ma_sound_uninit(&sound);
    return rl_ok_null();
}

// Start async playback; ok with the sound handle id, or an RL error.
rl_result rl_audio_play_file_async(rl_string path) {
    char cpath[4096];
    if (!_rl_audio_cstr(path, cpath, sizeof(cpath))) {
        return _rl_audio_err("play_file_async: invalid path");
    }
    if (!_rl_audio_probe_file(cpath)) {
        int e = errno;
        return _rl_audio_err("play_file_async(\"%s\"): %s (os error %d)", cpath, strerror(e), e);
    }
    if (!_rl_audio_ensure_engine()) {
        return _rl_audio_err("play_file_async: %s", ma_result_description(rl_audio_engine_status));
    }
    ma_sound *sound = malloc(sizeof(ma_sound));
    ma_result r = ma_sound_init_from_file(&rl_audio_engine, cpath, 0, NULL, NULL, sound);
    if (r != MA_SUCCESS) {
        free(sound);
        return _rl_audio_err("play_file_async(\"%s\"): %s", cpath, ma_result_description(r));
    }
    ma_sound_set_volume(sound, rl_audio_master_volume);
    ma_sound_start(sound);
    rl_result h = _rl_audio_new_handle(sound, 1.0f);
    if (!h.is_ok) {
        ma_sound_uninit(sound);
        free(sound);
    }
    return h;
}

// Play a sine tone and block until done; ok null, or an RL error.
rl_result rl_audio_beep(double freq, int64_t duration_ms) {
    if (!_rl_audio_ensure_engine()) {
        return _rl_audio_err("beep: %s", ma_result_description(rl_audio_engine_status));
    }
    if (duration_ms < 0) duration_ms = 0;
    ma_waveform_config wcfg = ma_waveform_config_init(
        ma_format_f32, 1, ma_engine_get_sample_rate(&rl_audio_engine),
        ma_waveform_type_sine, 0.5, freq);
    ma_waveform wave;
    ma_result r = ma_waveform_init(&wcfg, &wave);
    if (r != MA_SUCCESS) {
        return _rl_audio_err("beep: %s", ma_result_description(r));
    }
    ma_sound sound;
    r = ma_sound_init_from_data_source(&rl_audio_engine, &wave, 0, NULL, &sound);
    if (r != MA_SUCCESS) {
        ma_waveform_uninit(&wave);
        return _rl_audio_err("beep: %s", ma_result_description(r));
    }
    ma_sound_start(&sound);
    ma_sleep((ma_uint32)duration_ms);
    ma_sound_stop(&sound);
    ma_sound_uninit(&sound);
    ma_waveform_uninit(&wave);
    return rl_ok_null();
}

// Pause a live sound; ok null, or an unknown-handle RL error.
rl_result rl_audio_sound_pause(int64_t handle_id) {
    int idx = -1;
    ma_sound *s = _rl_audio_get(handle_id, &idx);
    if (s == NULL) {
        return _rl_audio_err("sound_pause: unknown handle %ld", (long)handle_id);
    }
    ma_sound_stop(s);
    rl_audio_handles[idx].paused = true;
    return rl_ok_null();
}

// Resume a live sound; ok null, or an unknown-handle RL error.
rl_result rl_audio_sound_resume(int64_t handle_id) {
    int idx = -1;
    ma_sound *s = _rl_audio_get(handle_id, &idx);
    if (s == NULL) {
        return _rl_audio_err("sound_resume: unknown handle %ld", (long)handle_id);
    }
    ma_sound_start(s);
    rl_audio_handles[idx].paused = false;
    return rl_ok_null();
}

// Stop a sound and release its handle; ok null, or an unknown-handle error.
rl_result rl_audio_sound_stop(int64_t handle_id) {
    int idx = -1;
    ma_sound *s = _rl_audio_get(handle_id, &idx);
    if (s == NULL) {
        return _rl_audio_err("sound_stop: unknown handle %ld", (long)handle_id);
    }
    ma_sound_stop(s);
    ma_sound_uninit(s);
    free(s);
    rl_audio_handles[idx].sound = NULL;
    rl_audio_handles[idx].used = false;
    return rl_ok_null();
}

// True when the sound is paused; ok bool, or an unknown-handle error.
rl_result rl_audio_sound_is_paused(int64_t handle_id) {
    int idx = -1;
    ma_sound *s = _rl_audio_get(handle_id, &idx);
    if (s == NULL) {
        return _rl_audio_err("sound_is_paused: unknown handle %ld", (long)handle_id);
    }
    return rl_ok_bool(rl_audio_handles[idx].paused);
}

// Per-sound base volume (scaled by the master volume); ok null.
rl_result rl_audio_sound_set_volume(int64_t handle_id, double volume) {
    int idx = -1;
    ma_sound *s = _rl_audio_get(handle_id, &idx);
    if (s == NULL) {
        return _rl_audio_err("sound_set_volume: unknown handle %ld", (long)handle_id);
    }
    float base = (float)volume;
    rl_audio_handles[idx].base_volume = base;
    ma_sound_set_volume(s, base * rl_audio_master_volume);
    return rl_ok_null();
}

// Per-sound base volume; ok float, or an unknown-handle RL error.
rl_result rl_audio_sound_get_volume(int64_t handle_id) {
    int idx = -1;
    ma_sound *s = _rl_audio_get(handle_id, &idx);
    if (s == NULL) {
        return _rl_audio_err("sound_get_volume: unknown handle %ld", (long)handle_id);
    }
    return rl_ok_f64((double)rl_audio_handles[idx].base_volume);
}

// Playback speed multiplier; ok null, or an unknown-handle RL error.
rl_result rl_audio_sound_set_speed(int64_t handle_id, double speed) {
    ma_sound *s = _rl_audio_get(handle_id, NULL);
    if (s == NULL) {
        return _rl_audio_err("sound_set_speed: unknown handle %ld", (long)handle_id);
    }
    ma_sound_set_pitch(s, (float)speed);
    return rl_ok_null();
}

// Seek to position_ms from the start; ok null, or an RL error.
rl_result rl_audio_sound_seek(int64_t handle_id, int64_t position_ms) {
    ma_sound *s = _rl_audio_get(handle_id, NULL);
    if (s == NULL) {
        return _rl_audio_err("sound_seek: unknown handle %ld", (long)handle_id);
    }
    if (position_ms < 0) position_ms = 0;
    ma_format fmt = ma_format_unknown;
    ma_uint32 channels = 0;
    ma_uint32 srate = 0;
    ma_sound_get_data_format(s, &fmt, &channels, &srate, NULL, 0);
    if (srate == 0) srate = ma_engine_get_sample_rate(&rl_audio_engine);
    ma_uint64 frame = (ma_uint64)((double)position_ms * (double)srate / 1000.0);
    ma_result r = ma_sound_seek_to_pcm_frame(s, frame);
    if (r != MA_SUCCESS) {
        return _rl_audio_err("sound_seek: %s", ma_result_description(r));
    }
    (void)fmt;
    (void)channels;
    return rl_ok_null();
}

// True when playback reached the end; ok bool, or an unknown-handle error.
rl_result rl_audio_sound_is_finished(int64_t handle_id) {
    ma_sound *s = _rl_audio_get(handle_id, NULL);
    if (s == NULL) {
        return _rl_audio_err("sound_is_finished: unknown handle %ld", (long)handle_id);
    }
    return rl_ok_bool(ma_sound_at_end(s));
}

// Block until playback reaches the end; ok null, or an unknown-handle error.
rl_result rl_audio_sound_wait(int64_t handle_id) {
    ma_sound *s = _rl_audio_get(handle_id, NULL);
    if (s == NULL) {
        return _rl_audio_err("sound_wait: unknown handle %ld", (long)handle_id);
    }
    while (!ma_sound_at_end(s)) {
        ma_sleep(5);
    }
    return rl_ok_null();
}

// List playback device names; ok with a string array, or an RL error.
rl_result rl_audio_list_output_devices(void) {
    ma_context context;
    ma_result r = ma_context_init(NULL, 0, NULL, &context);
    if (r != MA_SUCCESS) {
        return _rl_audio_err("list_output_devices: %s", ma_result_description(r));
    }
    ma_device_info *infos = NULL;
    ma_uint32 count = 0;
    r = ma_context_get_devices(&context, &infos, &count, NULL, NULL);
    if (r != MA_SUCCESS) {
        ma_context_uninit(&context);
        return _rl_audio_err("list_output_devices: %s", ma_result_description(r));
    }
    if (count == 0) {
        ma_context_uninit(&context);
        rl_array empty = { .data = NULL, .len = 0, .cap = 0,
            .elem_size = (int32_t)sizeof(rl_string), .type_tag = RL_TAG_STR };
        return rl_ok_arr(empty);
    }
    rl_string *items = malloc((uint64_t)count * sizeof(rl_string));
    for (ma_uint32 i = 0; i < count; i++) {
        uint64_t n = strlen(infos[i].name);
        char *dup = malloc(n + 1);
        memcpy(dup, infos[i].name, n + 1);
        items[i] = (rl_string){ .data = dup, .len = n, .rc = 1 };
    }
    ma_context_uninit(&context);
    rl_array out;
    out.data = items;
    out.len = count;
    out.cap = count;
    out.elem_size = (int32_t)sizeof(rl_string);
    out.type_tag = RL_TAG_STR;
    return rl_ok_arr(out);
}

// Select the device used for future playback; ok null, or an RL error
// when the name is unknown.
rl_result rl_audio_set_output_device(rl_string name) {
    char nbuf[512];
    if (!_rl_audio_cstr(name, nbuf, sizeof(nbuf))) {
        return _rl_audio_err("set_output_device: invalid device name");
    }
    ma_context context;
    ma_result r = ma_context_init(NULL, 0, NULL, &context);
    if (r != MA_SUCCESS) {
        return _rl_audio_err("set_output_device: %s", ma_result_description(r));
    }
    ma_device_info *infos = NULL;
    ma_uint32 count = 0;
    r = ma_context_get_devices(&context, &infos, &count, NULL, NULL);
    if (r != MA_SUCCESS) {
        ma_context_uninit(&context);
        return _rl_audio_err("set_output_device: %s", ma_result_description(r));
    }
    bool found = false;
    ma_device_id picked;
    memset(&picked, 0, sizeof(picked));
    for (ma_uint32 i = 0; i < count; i++) {
        if (strcmp(infos[i].name, nbuf) == 0) {
            picked = infos[i].id;
            found = true;
            break;
        }
    }
    ma_context_uninit(&context);
    if (!found) {
        return _rl_audio_err("set_output_device: output device \"%s\" not found", nbuf);
    }
    free(rl_audio_output_device);
    rl_audio_output_device = malloc(strlen(nbuf) + 1);
    strcpy(rl_audio_output_device, nbuf);
    rl_audio_output_device_id = picked;
    rl_audio_output_has_device = true;
    return rl_ok_null();
}

// Set the master volume and rescale every live sound; ok null.
rl_result rl_audio_set_master_volume(double volume) {
    float master = (float)volume;
    rl_audio_master_volume = master;
    for (int i = 0; i < rl_audio_handle_count; i++) {
        if (rl_audio_handles[i].used && rl_audio_handles[i].sound != NULL) {
            ma_sound_set_volume(rl_audio_handles[i].sound,
                rl_audio_handles[i].base_volume * master);
        }
    }
    return rl_ok_null();
}

// Decode just enough metadata for duration and channel info. True on
// success; false keeps the caller returning a decode RL error.
static bool _rl_audio_probe_meta(const char *cpath, ma_format *fmt,
    ma_uint32 *channels, ma_uint32 *srate, int64_t *ms, ma_result *rc) {
    ma_decoder decoder;
    ma_result r = ma_decoder_init_file(cpath, NULL, &decoder);
    if (r != MA_SUCCESS) {
        if (rc != NULL) *rc = r;
        return false;
    }
    ma_uint64 frames = 0;
    if (ma_decoder_get_length_in_pcm_frames(&decoder, &frames) != MA_SUCCESS) {
        frames = 0;
    }
    ma_uint32 rate = decoder.outputSampleRate;
    ma_uint32 ch = decoder.outputChannels;
    ma_format format = decoder.outputFormat;
    ma_decoder_uninit(&decoder);
    if (channels != NULL) *channels = ch;
    if (srate != NULL) *srate = rate;
    if (fmt != NULL) *fmt = format;
    if (ms != NULL) {
        *ms = (rate == 0) ? 0 : (int64_t)(((double)frames / (double)rate) * 1000.0);
    }
    return true;
}

// Short codec/container name in the style of the VM metadata probe
// ("pcm_s16le", "mp3", "flac", "vorbis"); "unknown" when unrecognized.
static void _rl_audio_format_name(const char *cpath, ma_format fmt, char *out, uint64_t cap) {
    char magic[12];
    uint64_t mlen = 0;
    FILE *f = fopen(cpath, "rb");
    if (f != NULL) {
        mlen = fread(magic, 1, sizeof(magic), f);
        fclose(f);
    }
    const char *name = "unknown";
    if (mlen >= 12 && memcmp(magic, "RIFF", 4) == 0 && memcmp(magic + 8, "WAVE", 4) == 0) {
        switch (fmt) {
            case ma_format_u8: name = "pcm_u8"; break;
            case ma_format_s16: name = "pcm_s16le"; break;
            case ma_format_s24: name = "pcm_s24le"; break;
            case ma_format_s32: name = "pcm_s32le"; break;
            case ma_format_f32: name = "pcm_f32le"; break;
            default: name = "pcm_s16le"; break;
        }
    } else if (mlen >= 4 && memcmp(magic, "fLaC", 4) == 0) {
        name = "flac";
    } else if (mlen >= 4 && (memcmp(magic, "OggS", 4) == 0)) {
        name = "vorbis";
    } else if (mlen >= 3 && (memcmp(magic, "ID3", 3) == 0
        || ((unsigned char)magic[0] == 0xFF && ((unsigned char)magic[1] & 0xE0) == 0xE0))) {
        name = "mp3";
    } else {
        const char *dot = strrchr(cpath, '.');
        const char *ext = (dot != NULL) ? dot + 1 : "";
        char lower[16];
        uint64_t i = 0;
        while (ext[i] != '\0' && i + 1 < sizeof(lower)) {
            char c = ext[i];
            lower[i] = (c >= 'A' && c <= 'Z') ? (char)(c + 32) : c;
            i++;
        }
        lower[i] = '\0';
        if (strcmp(lower, "mp3") == 0) name = "mp3";
        else if (strcmp(lower, "flac") == 0) name = "flac";
        else if (strcmp(lower, "ogg") == 0 || strcmp(lower, "oga") == 0) name = "vorbis";
        else if (strcmp(lower, "opus") == 0) name = "opus";
        else if (strcmp(lower, "wav") == 0) name = "pcm_s16le";
        else if (strcmp(lower, "m4a") == 0 || strcmp(lower, "mp4") == 0) name = "aac";
    }
    uint64_t n = strlen(name);
    if (n + 1 > cap) n = cap - 1;
    memcpy(out, name, n);
    out[n] = '\0';
}

// File duration in milliseconds; ok int, or an RL error.
rl_result rl_audio_duration(rl_string path) {
    char cpath[4096];
    if (!_rl_audio_cstr(path, cpath, sizeof(cpath))) {
        return _rl_audio_err("audio_duration: invalid path");
    }
    if (!_rl_audio_probe_file(cpath)) {
        int e = errno;
        return _rl_audio_err("audio_duration(\"%s\"): %s (os error %d)", cpath, strerror(e), e);
    }
    ma_result r = MA_SUCCESS;
    int64_t ms = 0;
    if (!_rl_audio_probe_meta(cpath, NULL, NULL, NULL, &ms, &r)) {
        return _rl_audio_err("audio_duration(\"%s\"): %s", cpath, ma_result_description(r));
    }
    return rl_ok_i64(ms);
}

// Canonical 4-tuple layout for audio metadata results, matching the
// program generated rl_tuple_4 struct field for field.
typedef struct { int64_t field_0; int64_t field_1; int64_t field_2; rl_string field_3; } _rl_tuple_iiis;

// Wrap (channels, sample rate, duration ms, format) as a single element
// array result, like the other tuple shaped results.
static rl_result _rl_ok_tuple_iiis(int64_t c0, int64_t c1, int64_t c2, char *sdata, uint64_t slen) {
    _rl_tuple_iiis *slot = malloc(sizeof(_rl_tuple_iiis));
    slot->field_0 = c0;
    slot->field_1 = c1;
    slot->field_2 = c2;
    slot->field_3 = (rl_string){ .data = sdata, .len = slen, .rc = 1 };
    rl_array out;
    out.data = slot;
    out.len = 1;
    out.cap = 1;
    out.elem_size = (int32_t)sizeof(_rl_tuple_iiis);
    out.type_tag = RL_TAG_I64;
    return rl_ok_arr(out);
}

// File metadata as (channels, sample rate, duration ms, format name);
// ok with the tuple, or an RL error.
rl_result rl_audio_file_info(rl_string path) {
    char cpath[4096];
    if (!_rl_audio_cstr(path, cpath, sizeof(cpath))) {
        return _rl_audio_err("audio_file_info: invalid path");
    }
    if (!_rl_audio_probe_file(cpath)) {
        int e = errno;
        return _rl_audio_err("audio_file_info(\"%s\"): %s (os error %d)", cpath, strerror(e), e);
    }
    ma_format fmt = ma_format_unknown;
    ma_uint32 ch = 0;
    ma_uint32 rate = 0;
    int64_t ms = 0;
    ma_result r = MA_SUCCESS;
    if (!_rl_audio_probe_meta(cpath, &fmt, &ch, &rate, &ms, &r)) {
        return _rl_audio_err("audio_file_info(\"%s\"): %s", cpath, ma_result_description(r));
    }
    char fname[32];
    _rl_audio_format_name(cpath, fmt, fname, sizeof(fname));
    uint64_t flen = strlen(fname);
    char *fdup = malloc(flen + 1);
    memcpy(fdup, fname, flen + 1);
    return _rl_ok_tuple_iiis((int64_t)ch, (int64_t)rate, ms, fdup, flen);
}

#else

// Stubs when miniaudio is not compiled in (programs not using audio).
rl_result rl_audio_play_file(rl_string path) { (void)path; return rl_err(-1); }
rl_result rl_audio_play_file_async(rl_string path) { (void)path; return rl_err(-1); }
rl_result rl_audio_beep(double freq, int64_t duration_ms) { (void)freq; (void)duration_ms; return rl_err(-1); }
rl_result rl_audio_sound_pause(int64_t handle_id) { (void)handle_id; return rl_err(-1); }
rl_result rl_audio_sound_resume(int64_t handle_id) { (void)handle_id; return rl_err(-1); }
rl_result rl_audio_sound_stop(int64_t handle_id) { (void)handle_id; return rl_err(-1); }
rl_result rl_audio_sound_is_paused(int64_t handle_id) { (void)handle_id; return rl_err(-1); }
rl_result rl_audio_sound_set_volume(int64_t handle_id, double volume) { (void)handle_id; (void)volume; return rl_err(-1); }
rl_result rl_audio_sound_get_volume(int64_t handle_id) { (void)handle_id; return rl_err(-1); }
rl_result rl_audio_sound_set_speed(int64_t handle_id, double speed) { (void)handle_id; (void)speed; return rl_err(-1); }
rl_result rl_audio_sound_seek(int64_t handle_id, int64_t position_ms) { (void)handle_id; (void)position_ms; return rl_err(-1); }
rl_result rl_audio_sound_is_finished(int64_t handle_id) { (void)handle_id; return rl_err(-1); }
rl_result rl_audio_sound_wait(int64_t handle_id) { (void)handle_id; return rl_err(-1); }
rl_result rl_audio_list_output_devices(void) { return rl_err(-1); }
rl_result rl_audio_set_output_device(rl_string name) { (void)name; return rl_err(-1); }
rl_result rl_audio_set_master_volume(double volume) { (void)volume; return rl_err(-1); }
rl_result rl_audio_duration(rl_string path) { (void)path; return rl_err(-1); }
rl_result rl_audio_file_info(rl_string path) { (void)path; return rl_err(-1); }

#endif

// ---- handle kind testers ----

// True only for ok I64 ids ever issued by that domain. Errors, other
// tags, bare small ints and other domains are false. Gui has no C
// backend and always returns false.
rl_result rl_is_c_handle(rl_result x) {
    if (!x.is_ok || x.tag != RL_TAG_I64) return rl_ok_bool(false);
    return rl_ok_bool(_rl_id_in_range(x.data.i64, RL_HANDLE_C_BASE, rl_c_handle_count));
}
rl_result rl_is_net_handle(rl_result x) {
    if (!x.is_ok || x.tag != RL_TAG_I64) return rl_ok_bool(false);
    return rl_ok_bool(_rl_id_in_range(x.data.i64, RL_HANDLE_NET_BASE, rl_net_handle_count));
}
rl_result rl_is_http_handle(rl_result x) {
    if (!x.is_ok || x.tag != RL_TAG_I64) return rl_ok_bool(false);
    return rl_ok_bool(_rl_id_in_range(x.data.i64, RL_HANDLE_HTTP_BASE, rl_http_handle_count));
}
rl_result rl_is_audio_handle(rl_result x) {
#ifdef RL_USE_AUDIO
    if (!x.is_ok || x.tag != RL_TAG_I64) return rl_ok_bool(false);
    return rl_ok_bool(_rl_id_in_range(x.data.i64, RL_HANDLE_AUDIO_BASE, rl_audio_handle_count));
#else
    (void)x;
    return rl_ok_bool(false);
#endif
}
rl_result rl_is_gui_handle(rl_result x) {
    (void)x;
    return rl_ok_bool(false);
}
rl_result rl_is_file_handle(rl_result x) {
    if (!x.is_ok || x.tag != RL_TAG_I64) return rl_ok_bool(false);
    return rl_ok_bool(_rl_id_in_range(x.data.i64, RL_HANDLE_FILE_BASE, _rl_fs_handle_count));
}
