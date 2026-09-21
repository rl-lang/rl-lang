#define _GNU_SOURCE
#define _POSIX_C_SOURCE 200809L
#include "rl_runtime.h"


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

// Format result payload into a fresh string (caller owns via out).
static void _rl_result_to_str(rl_result v, char **out, uint64_t *out_len) {
    char buf[128];
    int n = 0;
    if (!v.is_ok) { n = snprintf(buf, sizeof(buf), "err(%d)", v.err_code); }
    else switch (v.tag) {
        case RL_TAG_NULL: n = snprintf(buf, sizeof(buf), "null"); break;
        case RL_TAG_I64: n = snprintf(buf, sizeof(buf), "%ld", (long)v.data.i64); break;
        case RL_TAG_F64: n = snprintf(buf, sizeof(buf), "%g", v.data.f64); break;
        case RL_TAG_BOOL: n = snprintf(buf, sizeof(buf), "%s", v.data.boolean ? "true" : "false"); break;
        case RL_TAG_CHAR: n = snprintf(buf, sizeof(buf), "%c", (char)(unsigned char)v.data.i64); break;
        case RL_TAG_STR: { char *d = malloc(v.data.str.len + 1); memcpy(d, v.data.str.data, v.data.str.len); d[v.data.str.len] = '\0'; *out = d; *out_len = v.data.str.len; return; }
        default: n = snprintf(buf, sizeof(buf), "<value>"); break;
    }
    char *dup = malloc(n + 1);
    memcpy(dup, buf, n + 1);
    *out = dup;
    *out_len = n;
}

// Concatenate argc string results (used for `+` on strings).
rl_string rl_str_concat_variadic(rl_result *args, uint64_t argc) {
    uint64_t total = 0;
    char **parts = malloc(argc * sizeof(char *));
    uint64_t *part_lens = malloc(argc * sizeof(uint64_t));
    for (uint64_t i = 0; i < argc; i++) {
        _rl_result_to_str(args[i], &parts[i], &part_lens[i]);
        total += part_lens[i];
    }
    char *buf = malloc(total + 1);
    uint64_t pos = 0;
    for (uint64_t i = 0; i < argc; i++) {
        memcpy(buf + pos, parts[i], part_lens[i]);
        pos += part_lens[i];
        if (parts[i] != args[i].data.str.data) free(parts[i]);
    }
    buf[total] = '\0';
    free(parts);
    free(part_lens);
    return (rl_string){ .data = buf, .len = total, .rc = 1 };
}

// Interpolate args into a `{}` template string (caller owns).
rl_result rl_str_format(rl_string tmpl, rl_result *args, uint64_t argc) {
    uint64_t total = 0;
    char **parts = malloc((argc + 1) * sizeof(char *));
    uint64_t *part_lens = malloc((argc + 1) * sizeof(uint64_t));
    uint64_t part_count = 0;
    uint64_t arg_idx = 0;
    uint64_t i = 0;
    while (i < tmpl.len) {
        if (tmpl.data[i] == '{' && i + 1 < tmpl.len && tmpl.data[i + 1] == '}') {
            if (arg_idx >= argc) {
                for (uint64_t j = 0; j < part_count; j++) if (parts[j] != tmpl.data + 0) free(parts[j]);
                free(parts); free(part_lens);
                return rl_err_msg(rl_str_literal("format: not enough arguments for placeholders", 47));
            }
            _rl_result_to_str(args[arg_idx], &parts[part_count], &part_lens[part_count]);
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
        for (uint64_t j = 0; j < part_count; j++) if (parts[j] != tmpl.data + 0) free(parts[j]);
        free(parts); free(part_lens);
        return rl_err_msg(rl_str_literal("format: too many arguments for placeholders", 42));
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
    return rl_ok_str((rl_string){ .data = buf, .len = total, .rc = 1 });
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
void rl_print_str(rl_string v) { printf("%.*s", (int)v.len, v.data); }
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
void rl_println_str(rl_string v) { printf("%.*s\n", (int)v.len, v.data); }
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

// ---- time ----

// Wall-clock time in milliseconds since the Unix epoch.
int64_t rl_time_now_ms(void) {
    struct timespec ts;
    clock_gettime(CLOCK_REALTIME, &ts);
    return (int64_t)ts.tv_sec * 1000 + ts.tv_nsec / 1000000;
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
        default: name = "unknown"; break;
    }
    uint64_t len = strlen(name);
    char *buf = malloc(len + 1);
    memcpy(buf, name, len + 1);
    rl_string result = { .data = buf, .len = len, .rc = 1 };
    return result;
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

// ---- fs ----

// Size of the file at path.
int64_t rl_fs_file_size(rl_string path) {
    char buf[path.len + 1];
    memcpy(buf, path.data, path.len);
    buf[path.len] = '\0';
    struct stat st;
    if (stat(buf, &st) != 0) return -1;
    return (int64_t)st.st_size;
}

// Last-modified time of the file at `path`.
int64_t rl_fs_file_modified(rl_string path) {
    char buf[path.len + 1];
    memcpy(buf, path.data, path.len);
    buf[path.len] = '\0';
    struct stat st;
    if (stat(buf, &st) != 0) return -1;
    return (int64_t)st.st_mtime;
}

// Copy `src` to `dst`; 0 on success, -1 on failure.
int64_t rl_fs_copy_file(rl_string src, rl_string dst) {
    char sbuf[src.len + 1];
    memcpy(sbuf, src.data, src.len);
    sbuf[src.len] = '\0';
    char dbuf[dst.len + 1];
    memcpy(dbuf, dst.data, dst.len);
    dbuf[dst.len] = '\0';
    FILE *fin = fopen(sbuf, "rb");
    if (!fin) return -1;
    FILE *fout = fopen(dbuf, "wb");
    if (!fout) { fclose(fin); return -1; }
    char chunk[8192];
    size_t n;
    while ((n = fread(chunk, 1, sizeof(chunk), fin)) > 0) {
        fwrite(chunk, 1, n, fout);
    }
    fclose(fin);
    fclose(fout);
    return 0;
}

// Create `path` plus missing parents; 0 on success, -1 on failure.
int64_t rl_fs_mkdir_all(rl_string path) {
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
    return mkdir(buf, 0755);
}

// Delete the directory tree at `path`; 0 on success, -1 on failure.
int64_t rl_fs_rmdir_all(rl_string path) {
    char buf[path.len + 1];
    memcpy(buf, path.data, path.len);
    buf[path.len] = '\0';
    return rmdir(buf);
}

// Names (not full paths) of entries in the directory at `path`.
rl_array rl_fs_list_dir(rl_string path) {
    char buf[path.len + 1];
    memcpy(buf, path.data, path.len);
    buf[path.len] = '\0';
    DIR *d = opendir(buf);
    if (!d) {
        rl_array arr = { .data = NULL, .len = 0, .cap = 0, .elem_size = sizeof(rl_string), .type_tag = RL_TAG_STR };
        return arr;
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
    return arr;
}

// Rename to `new_name` in the same directory; returns the new full path.
rl_string rl_fs_rename_file(rl_string path, rl_string new_name) {
    char pbuf[path.len + 1];
    memcpy(pbuf, path.data, path.len);
    pbuf[path.len] = '\0';
    char nbuf[new_name.len + 1];
    memcpy(nbuf, new_name.data, new_name.len);
    nbuf[new_name.len] = '\0';
    rename(pbuf, nbuf);
    return new_name;
}

// ---- process ----

// Current working directory of the process.
rl_string rl_process_cwd(void) {
    char buf[4096];
    if (getcwd(buf, sizeof(buf)) == NULL) {
        rl_string result = { .data = "", .len = 0, .rc = 1 };
        return result;
    }
    uint64_t len = strlen(buf);
    char *out = malloc(len + 1);
    memcpy(out, buf, len + 1);
    rl_string result = { .data = out, .len = len, .rc = 1 };
    return result;
}

// Change directory; 0 on success, -1 on failure.
int64_t rl_process_set_cwd(rl_string path) {
    char buf[path.len + 1];
    memcpy(buf, path.data, path.len);
    buf[path.len] = '\0';
    return chdir(buf);
}

// Run `cmd` through the shell and capture stdout (variants return
// the exit code or one array element per output line instead).
rl_string rl_process_exec(rl_string cmd) {
    char buf[cmd.len + 1];
    memcpy(buf, cmd.data, cmd.len);
    buf[cmd.len] = '\0';
    FILE *fp = popen(buf, "r");
    if (!fp) {
        rl_string result = { .data = "", .len = 0, .rc = 1 };
        return result;
    }
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
    return result;
}

// Run `cmd` through the shell and capture stdout (variants return
// the exit code or one array element per output line instead).
int64_t rl_process_exec_code(rl_string cmd) {
    char buf[cmd.len + 1];
    memcpy(buf, cmd.data, cmd.len);
    buf[cmd.len] = '\0';
    return (int64_t)system(buf);
}

// Run `cmd` through the shell and capture stdout (variants return
// the exit code or one array element per output line instead).
rl_array rl_process_exec_lines(rl_string cmd) {
    rl_string output = rl_process_exec(cmd);
    rl_string nl = { .data = "\n", .len = 1, .rc = 1 };
    return rl_str_split(output, nl);
}

// Same, but with `env` assignments (e.g. `"A=1 B=2"`) prepended.
rl_string rl_process_with_exec(rl_string env, rl_string cmd) {
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
int64_t rl_process_with_exec_code(rl_string env, rl_string cmd) {
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
rl_array rl_process_with_exec_lines(rl_string env, rl_string cmd) {
    uint64_t total = env.len + 1 + cmd.len;
    char buf[total + 1];
    memcpy(buf, env.data, env.len);
    buf[env.len] = ' ';
    memcpy(buf + env.len + 1, cmd.data, cmd.len);
    buf[total] = '\0';
    rl_string combined = { .data = buf, .len = total, .rc = 0 };
    return rl_process_exec_lines(combined);
}

static int _rl_stored_argc = 0;
static char **_rl_stored_argv = NULL;

// Snapshot argv at startup; generated `main` calls this first.
void rl_store_args(int argc, char **argv) {
    _rl_stored_argc = argc;
    _rl_stored_argv = argv;
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

// ---- time ----

// Format a Unix timestamp with a strftime-style `pattern`.
rl_string rl_time_format_time(int64_t timestamp, rl_string pattern) {
    time_t t = (time_t)timestamp;
    struct tm *tm = gmtime(&t);
    char buf[256];
    strftime(buf, sizeof(buf), pattern.data, tm);
    uint64_t len = strlen(buf);
    char *out = malloc(len + 1);
    memcpy(out, buf, len + 1);
    rl_string result = { .data = out, .len = len, .rc = 1 };
    return result;
}

// Format as `YYYY-MM-DD` / `HH:MM:SS` in local time.
rl_string rl_time_format_date_str(int64_t timestamp) {
    time_t t = (time_t)timestamp;
    struct tm *tm = gmtime(&t);
    char buf[64];
    strftime(buf, sizeof(buf), "%Y-%m-%d", tm);
    uint64_t len = strlen(buf);
    char *out = malloc(len + 1);
    memcpy(out, buf, len + 1);
    rl_string result = { .data = out, .len = len, .rc = 1 };
    return result;
}

// Format as `YYYY-MM-DD` / `HH:MM:SS` in local time.
rl_string rl_time_format_time_str(int64_t timestamp) {
    time_t t = (time_t)timestamp;
    struct tm *tm = gmtime(&t);
    char buf[64];
    strftime(buf, sizeof(buf), "%H:%M:%S", tm);
    uint64_t len = strlen(buf);
    char *out = malloc(len + 1);
    memcpy(out, buf, len + 1);
    rl_string result = { .data = out, .len = len, .rc = 1 };
    return result;
}

// Split into `[year, month, day, hour, min, sec]` components.
rl_array rl_time_parts(int64_t timestamp) {
    time_t t = (time_t)timestamp;
    struct tm *tm = gmtime(&t);
    int64_t parts[6] = {
        tm->tm_year + 1900,
        tm->tm_mon + 1,
        tm->tm_mday,
        tm->tm_hour,
        tm->tm_min,
        tm->tm_sec
    };
    return rl_arr_from_vals(parts, 6, sizeof(int64_t));
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

// Delete the file at `path`; 0 on success, -1 on failure.
int64_t rl_io_delete_file(rl_string path) {
    char buf[path.len + 1];
    memcpy(buf, path.data, path.len);
    buf[path.len] = '\0';
    return remove(buf);
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

// ---- types ----

// Format an int as decimal / binary (`0b...`) / hex (`0x...`) / octal.
rl_string rl_types_to_string(int64_t v) {
    char buf[32];
    int len = snprintf(buf, sizeof(buf), "%ld", (long)v);
    char *out = malloc(len + 1);
    memcpy(out, buf, len + 1);
    rl_string result = { .data = out, .len = (uint64_t)len, .rc = 1 };
    return result;
}

// Format an int as decimal / binary (`0b...`) / hex (`0x...`) / octal.
rl_string rl_types_to_bin(int64_t v) {
    if (v == 0) {
        rl_string result = { .data = "0", .len = 1, .rc = 1 };
        return result;
    }
    char buf[65];
    int i = 64;
    buf[i] = '\0';
    uint64_t uv = (uint64_t)v;
    while (uv > 0) {
        buf[--i] = (uv & 1) ? '1' : '0';
        uv >>= 1;
    }
    uint64_t len = 64 - (uint64_t)i;
    char *out = malloc(len + 1);
    memcpy(out, buf + i, len + 1);
    rl_string result = { .data = out, .len = len, .rc = 1 };
    return result;
}

// Format an int as decimal / binary (`0b...`) / hex (`0x...`) / octal.
rl_string rl_types_to_hex(int64_t v) {
    char buf[32];
    int len = snprintf(buf, sizeof(buf), "%lx", (unsigned long)v);
    char *out = malloc(len + 1);
    memcpy(out, buf, len + 1);
    rl_string result = { .data = out, .len = (uint64_t)len, .rc = 1 };
    return result;
}

// Format an int as decimal / binary (`0b...`) / hex (`0x...`) / octal.
rl_string rl_types_to_oct(int64_t v) {
    char buf[32];
    int len = snprintf(buf, sizeof(buf), "%lo", (unsigned long)v);
    char *out = malloc(len + 1);
    memcpy(out, buf, len + 1);
    rl_string result = { .data = out, .len = (uint64_t)len, .rc = 1 };
    return result;
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
            char buf[x.data.str.len + 1];
            memcpy(buf, x.data.str.data, x.data.str.len);
            buf[x.data.str.len] = '\0';
            char *end;
            long v = strtol(buf, &end, 0);
            return rl_ok_i64((int64_t)(unsigned char)v);
        }
        default: return rl_err_msg(rl_str_literal("cannot convert to byte", 22));
    }
}

// Turn an error result into a panic; wrap an int as a byte / char value.
rl_result rl_types_to_char(rl_result x) {
    switch (x.tag) {
        case RL_TAG_I64: return rl_ok_i64(x.data.i64);
        case RL_TAG_CHAR: return rl_ok_i64(x.data.i64);
        case RL_TAG_STR: {
            if (x.data.str.len > 0) {
                return rl_ok_i64((int64_t)(unsigned char)x.data.str.data[0]);
            }
            return rl_err_msg(rl_str_literal("cannot convert empty string to char", 34));
        }
        default: return rl_err_msg(rl_str_literal("cannot convert to char", 22));
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

// ---- array (generic) ----

// Append v / drop and return the last element.
rl_result rl_arr_push(rl_array a, int64_t v) {
    uint64_t new_len = a.len + 1;
    int64_t *buf = malloc(new_len * sizeof(int64_t));
    if (a.data) memcpy(buf, a.data, a.len * sizeof(int64_t));
    buf[a.len] = v;
    return rl_ok_arr(rl_arr_from_vals(buf, new_len, sizeof(int64_t)));
}

// Append v / drop and return the last element.
rl_result rl_arr_pop(rl_array a) {
    if (a.len == 0) return rl_make_err(-1, "pop from empty array");
    return rl_ok_i64(((int64_t *)a.data)[a.len - 1]);
}

// Insert `v` at `idx` / drop the element at `idx`.
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
    int64_t *src = (int64_t *)a.data;
    uint64_t new_len = a.len - 1;
    int64_t *buf = malloc(new_len * sizeof(int64_t));
    if (idx > 0) memcpy(buf, src, (uint64_t)idx * sizeof(int64_t));
    if ((uint64_t)idx < a.len - 1) memcpy(buf + idx, src + idx + 1, (a.len - (uint64_t)idx - 1) * sizeof(int64_t));
    return rl_ok_arr(rl_arr_from_vals(buf, new_len, sizeof(int64_t)));
}

// Reversed / concatenated copies (inputs unchanged).
rl_array rl_arr_reverse(rl_array a) {
    int64_t *src = (int64_t *)a.data;
    int64_t *buf = malloc(a.len * sizeof(int64_t));
    for (uint64_t i = 0; i < a.len; i++) {
        buf[i] = src[a.len - 1 - i];
    }
    return rl_arr_from_vals(buf, a.len, sizeof(int64_t));
}

// Reversed / concatenated copies (inputs unchanged).
rl_array rl_arr_concat(rl_array a, rl_array b) {
    uint64_t new_len = a.len + b.len;
    int64_t *buf = malloc(new_len * sizeof(int64_t));
    if (a.data) memcpy(buf, a.data, a.len * sizeof(int64_t));
    if (b.data) memcpy(buf + a.len, b.data, b.len * sizeof(int64_t));
    return rl_arr_from_vals(buf, new_len, sizeof(int64_t));
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

// Copy with duplicates removed, keeping first-seen order.
rl_array rl_arr_unique(rl_array a) {
    int64_t *src = (int64_t *)a.data;
    int64_t *buf = malloc(a.len * sizeof(int64_t));
    uint64_t w = 0;
    for (uint64_t i = 0; i < a.len; i++) {
        bool found = false;
        for (uint64_t j = 0; j < w; j++) {
            if (buf[j] == src[i]) { found = true; break; }
        }
        if (!found) buf[w++] = src[i];
    }
    return rl_arr_from_vals(buf, w, sizeof(int64_t));
}

// Copy of `[start, end)` with clamping.
rl_array rl_arr_slice(rl_array a, int64_t start, int64_t end) {
    if (start < 0) start = 0;
    if (end > (int64_t)a.len) end = (int64_t)a.len;
    if (start >= end) {
        return rl_arr_from_vals(NULL, 0, sizeof(int64_t));
    }
    uint64_t len = (uint64_t)(end - start);
    int64_t *buf = malloc(len * sizeof(int64_t));
    memcpy(buf, (int64_t *)a.data + start, len * sizeof(int64_t));
    return rl_arr_from_vals(buf, len, sizeof(int64_t));
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
rl_result rl_arr_sum(rl_array a) {
    int64_t *src = (int64_t *)a.data;
    int64_t sum = 0;
    for (uint64_t i = 0; i < a.len; i++) sum += src[i];
    return rl_ok_i64(sum);
}

// Sum / product / max / min, erroring on empty input.
rl_result rl_arr_product(rl_array a) {
    int64_t *src = (int64_t *)a.data;
    int64_t prod = 1;
    for (uint64_t i = 0; i < a.len; i++) prod *= src[i];
    return rl_ok_i64(prod);
}

// Sum / product / max / min, erroring on empty input.
rl_result rl_arr_max(rl_array a) {
    if (a.len == 0) return rl_make_err(-1, "max of empty array");
    int64_t *src = (int64_t *)a.data;
    int64_t max = src[0];
    for (uint64_t i = 1; i < a.len; i++) {
        if (src[i] > max) max = src[i];
    }
    return rl_ok_i64(max);
}

// Sum / product / max / min, erroring on empty input.
rl_result rl_arr_min(rl_array a) {
    if (a.len == 0) return rl_make_err(-1, "min of empty array");
    int64_t *src = (int64_t *)a.data;
    int64_t min = src[0];
    for (uint64_t i = 1; i < a.len; i++) {
        if (src[i] < min) min = src[i];
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

// Sorted copy (input unchanged).
rl_array rl_arr_sort(rl_array a) {
    if (a.len == 0) return a;
    switch (a.type_tag) {
        case RL_TAG_F64: {
            double *buf = malloc(a.len * sizeof(double));
            if (a.data) memcpy(buf, a.data, a.len * sizeof(double));
            qsort(buf, a.len, sizeof(double), rl_arr_cmp_f64);
            rl_array result = { .data = buf, .len = a.len, .cap = a.len, .elem_size = sizeof(double), .type_tag = RL_TAG_F64 };
            return result;
        }
        case RL_TAG_STR: {
            rl_string *buf = malloc(a.len * sizeof(rl_string));
            if (a.data) memcpy(buf, a.data, a.len * sizeof(rl_string));
            qsort(buf, a.len, sizeof(rl_string), rl_arr_cmp_str);
            rl_array result = { .data = buf, .len = a.len, .cap = a.len, .elem_size = sizeof(rl_string), .type_tag = RL_TAG_STR };
            return result;
        }
        default: {
            int64_t *buf = malloc(a.len * sizeof(int64_t));
            if (a.data) memcpy(buf, a.data, a.len * sizeof(int64_t));
            qsort(buf, a.len, sizeof(int64_t), rl_arr_cmp_i64);
            return rl_arr_from_vals(buf, a.len, sizeof(int64_t));
        }
    }
}

// One-level flatten of nested arrays.
rl_array rl_arr_flatten(rl_array a) {
    int64_t *buf = malloc(a.len * sizeof(int64_t));
    if (a.data) memcpy(buf, a.data, a.len * sizeof(int64_t));
    return rl_arr_from_vals(buf, a.len, sizeof(int64_t));
}

// ---- closure-consuming array functions ----

// Keep elements where pred returns true.
rl_result rl_arr_filter_closure(rl_array arr, rl_closure pred) {
    uint64_t cap = 16;
    int64_t *buf = malloc(cap * sizeof(int64_t));
    uint64_t count = 0;
    int64_t *elems = (int64_t *)arr.data;
    for (uint64_t i = 0; i < arr.len; i++) {
        rl_result arg = rl_ok_i64(elems[i]);
        rl_result keep = rl_closure_call(pred, &arg, 1);
        bool truth = false;
        if (keep.is_ok) {
            if (keep.tag == RL_TAG_BOOL) truth = keep.data.boolean;
            else if (keep.tag == RL_TAG_I64) truth = (keep.data.i64 != 0);
            else if (keep.tag == RL_TAG_NULL) truth = false;
        }
        if (truth) {
            if (count >= cap) { cap *= 2; buf = realloc(buf, cap * sizeof(int64_t)); }
            buf[count++] = elems[i];
        }
    }
    return rl_ok_arr(rl_arr_from_vals(buf, count, sizeof(int64_t)));
}

// Apply `fn` to every element, collecting the outputs.
rl_result rl_arr_map_closure(rl_array arr, rl_closure fn) {
    uint64_t cap = arr.len > 0 ? arr.len : 16;
    int64_t *buf = malloc(cap * sizeof(int64_t));
    uint64_t count = 0;
    int64_t *elems = (int64_t *)arr.data;
    for (uint64_t i = 0; i < arr.len; i++) {
        rl_result arg = rl_ok_i64(elems[i]);
        rl_result mapped = rl_closure_call(fn, &arg, 1);
        if (count >= cap) { cap *= 2; buf = realloc(buf, cap * sizeof(int64_t)); }
        if (mapped.is_ok) {
            switch (mapped.tag) {
                case RL_TAG_I64: buf[count++] = mapped.data.i64; break;
                case RL_TAG_F64: { double d = mapped.data.f64; int64_t v; memcpy(&v, &d, sizeof(v)); buf[count++] = v; break; }
                case RL_TAG_BOOL: buf[count++] = mapped.data.boolean ? 1 : 0; break;
                case RL_TAG_CHAR: buf[count++] = mapped.data.i64; break;
                default: buf[count++] = 0; break;
            }
        } else {
            buf[count++] = 0;
        }
    }
    return rl_ok_arr(rl_arr_from_vals(buf, count, sizeof(int64_t)));
}

// First element where `pred` returns true (error when none matches).
rl_result rl_arr_find_closure(rl_array arr, rl_closure pred) {
    int64_t *elems = (int64_t *)arr.data;
    for (uint64_t i = 0; i < arr.len; i++) {
        rl_result arg = rl_ok_i64(elems[i]);
        rl_result found = rl_closure_call(pred, &arg, 1);
        if (found.is_ok && found.tag == RL_TAG_BOOL && found.data.boolean) {
            return rl_ok_i64(elems[i]);
        }
    }
    return rl_err_msg(rl_str_literal("not found", 9));
}

// Left fold starting from `init`.
rl_result rl_arr_reduce_closure(rl_array arr, rl_closure fn, rl_result init) {
    int64_t *elems = (int64_t *)arr.data;
    rl_result acc = init;
    for (uint64_t i = 0; i < arr.len; i++) {
        rl_result args[2] = { acc, rl_ok_i64(elems[i]) };
        acc = rl_closure_call(fn, args, 2);
    }
    return acc;
}

// Index of the first match (error when none matches).
rl_result rl_arr_find_index_closure(rl_array arr, rl_closure pred) {
    int64_t *elems = (int64_t *)arr.data;
    for (uint64_t i = 0; i < arr.len; i++) {
        rl_result arg = rl_ok_i64(elems[i]);
        rl_result found = rl_closure_call(pred, &arg, 1);
        if (found.is_ok && ((found.tag == RL_TAG_BOOL && found.data.boolean) || (found.tag == RL_TAG_I64 && found.data.i64 != 0))) {
            return rl_ok_i64((int64_t)i);
        }
    }
    return rl_ok_i64((int64_t)-1);
}

// True when `pred` holds for all / for at least one element.
rl_result rl_arr_all_closure(rl_array arr, rl_closure pred) {
    int64_t *elems = (int64_t *)arr.data;
    for (uint64_t i = 0; i < arr.len; i++) {
        rl_result arg = rl_ok_i64(elems[i]);
        rl_result ok = rl_closure_call(pred, &arg, 1);
        bool truth = false;
        if (ok.is_ok) {
            if (ok.tag == RL_TAG_BOOL) truth = ok.data.boolean;
            else if (ok.tag == RL_TAG_I64) truth = (ok.data.i64 != 0);
        }
        if (!truth) return rl_ok_bool(false);
    }
    return rl_ok_bool(true);
}

// True when `pred` holds for all / for at least one element.
rl_result rl_arr_any_closure(rl_array arr, rl_closure pred) {
    int64_t *elems = (int64_t *)arr.data;
    for (uint64_t i = 0; i < arr.len; i++) {
        rl_result arg = rl_ok_i64(elems[i]);
        rl_result ok = rl_closure_call(pred, &arg, 1);
        bool truth = false;
        if (ok.is_ok) {
            if (ok.tag == RL_TAG_BOOL) truth = ok.data.boolean;
            else if (ok.tag == RL_TAG_I64) truth = (ok.data.i64 != 0);
        }
        if (truth) return rl_ok_bool(true);
    }
    return rl_ok_bool(false);
}

// Run `fn` for side effects; returns the input length.
rl_result rl_arr_for_each_closure(rl_array arr, rl_closure fn) {
    int64_t *elems = (int64_t *)arr.data;
    for (uint64_t i = 0; i < arr.len; i++) {
        rl_result arg = rl_ok_i64(elems[i]);
        rl_closure_call(fn, &arg, 1);
    }
    return rl_ok_null();
}

// Map, then concatenate one level of the resulting arrays.
rl_result rl_arr_flat_map_closure(rl_array arr, rl_closure fn) {
    uint64_t cap = 16;
    int64_t *buf = malloc(cap * sizeof(int64_t));
    uint64_t count = 0;
    int64_t *elems = (int64_t *)arr.data;
    for (uint64_t i = 0; i < arr.len; i++) {
        rl_result arg = rl_ok_i64(elems[i]);
        rl_result mapped = rl_closure_call(fn, &arg, 1);
        if (mapped.is_ok && mapped.tag == RL_TAG_ARR) {
            rl_array inner = mapped.data.arr;
            int64_t *inner_elems = (int64_t *)inner.data;
            for (uint64_t j = 0; j < inner.len; j++) {
                if (count >= cap) { cap *= 2; buf = realloc(buf, cap * sizeof(int64_t)); }
                buf[count++] = inner_elems[j];
            }
        }
    }
    return rl_ok_arr(rl_arr_from_vals(buf, count, sizeof(int64_t)));
}

// Sort using `cmp(a, b)` returning negative / zero / positive.
rl_result rl_arr_sort_by_closure(rl_array arr, rl_closure cmp) {
    // Copy the array data for sorting
    int64_t *elems = (int64_t *)arr.data;
    uint64_t len = arr.len;
    int64_t *buf = malloc(len * sizeof(int64_t));
    if (len > 0 && elems) memcpy(buf, elems, len * sizeof(int64_t));

    // Simple insertion sort using the comparator closure
    for (uint64_t i = 1; i < len; i++) {
        int64_t key = buf[i];
        int64_t j = (int64_t)i - 1;
        while (j >= 0) {
            rl_result args[2] = { rl_ok_i64(buf[j]), rl_ok_i64(key) };
            rl_result cmp_result = rl_closure_call(cmp, args, 2);
            // If cmp(a, b) > 0, swap (ascending order)
            if (cmp_result.is_ok && cmp_result.tag == RL_TAG_I64 && cmp_result.data.i64 > 0) {
                buf[j + 1] = buf[j];
                j--;
            } else {
                break;
            }
        }
        buf[j + 1] = key;
    }
    return rl_ok_arr(rl_arr_from_vals(buf, len, sizeof(int64_t)));
}

// ---- closure-consuming result functions ----
// Apply closures to result payloads, passing the other case through.
// Apply `fn` to the payload of an ok result (passes errors through).
rl_result rl_result_map_closure(rl_result val, rl_closure fn) {
    if (!val.is_ok) return val;
    switch (val.tag) {
        case RL_TAG_I64: {
            rl_result arg = rl_ok_i64(val.data.i64);
            return rl_closure_call(fn, &arg, 1);
        }
        case RL_TAG_F64: {
            rl_result arg = rl_ok_f64(val.data.f64);
            return rl_closure_call(fn, &arg, 1);
        }
        case RL_TAG_BOOL: {
            rl_result arg = rl_ok_bool(val.data.boolean);
            return rl_closure_call(fn, &arg, 1);
        }
        case RL_TAG_STR: {
            rl_result arg = rl_ok_str(val.data.str);
            return rl_closure_call(fn, &arg, 1);
        }
        case RL_TAG_ARR: {
            rl_result arg = rl_ok_arr(val.data.arr);
            return rl_closure_call(fn, &arg, 1);
        }
        default: {
            rl_result arg = rl_ok_null();
            return rl_closure_call(fn, &arg, 1);
        }
    }
}

// Apply `fn` to the code of an error result (passes ok values through).
rl_result rl_result_map_err_closure(rl_result val, rl_closure fn) {
    if (val.is_ok) return val;
    switch (val.tag) {
        case RL_TAG_I64: {
            rl_result arg = rl_ok_i64(val.data.i64);
            return rl_err(rl_unwrap_i64(rl_closure_call(fn, &arg, 1)));
        }
        case RL_TAG_STR: {
            rl_result arg = rl_ok_str(val.data.str);
            return rl_closure_call(fn, &arg, 1);
        }
        default: {
            rl_result arg = rl_ok_i64(val.err_code);
            return rl_err(rl_unwrap_i64(rl_closure_call(fn, &arg, 1)));
        }
    }
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

// Switch to / back from the alternate screen buffer.
rl_result rl_term_enter(void) {
    printf("\x1b[?1049h");
    return rl_ok_null();
}

// Switch to / back from the alternate screen buffer.
rl_result rl_term_leave(void) {
    printf("\x1b[?1049l");
    return rl_ok_null();
}

// Clear the whole screen / the current line.
rl_result rl_term_clear(void) {
    printf("\x1b[2J\x1b[H");
    return rl_ok_null();
}

// Clear the whole screen / the current line.
rl_result rl_term_clear_line(void) {
    printf("\x1b[2K");
    return rl_ok_null();
}

// Move the cursor: absolute position, column only, row only, relative steps, or N lines down / up to column 0.
rl_result rl_term_move(int64_t col, int64_t row) {
    printf("\x1b[%ld;%ldH", (long)row + 1, (long)col + 1);
    return rl_ok_null();
}

// Move the cursor: absolute position, column only, row only, relative steps, or N lines down / up to column 0.
rl_result rl_term_move_to_col(int64_t col) {
    printf("\x1b[%ldG", (long)col + 1);
    return rl_ok_null();
}

// Move the cursor: absolute position, column only, row only, relative steps, or N lines down / up to column 0.
rl_result rl_term_move_to_row(int64_t row) {
    printf("\x1b[%ld;d", (long)row + 1);
    return rl_ok_null();
}

// Move the cursor: absolute position, column only, row only, relative steps, or N lines down / up to column 0.
rl_result rl_term_move_up(int64_t n) {
    printf("\x1b[%ldA", (long)n);
    return rl_ok_null();
}

// Move the cursor: absolute position, column only, row only, relative steps, or N lines down / up to column 0.
rl_result rl_term_move_down(int64_t n) {
    printf("\x1b[%ldB", (long)n);
    return rl_ok_null();
}

// Move the cursor: absolute position, column only, row only, relative steps, or N lines down / up to column 0.
rl_result rl_term_move_left(int64_t n) {
    printf("\x1b[%ldD", (long)n);
    return rl_ok_null();
}

// Move the cursor: absolute position, column only, row only, relative steps, or N lines down / up to column 0.
rl_result rl_term_move_right(int64_t n) {
    printf("\x1b[%ldC", (long)n);
    return rl_ok_null();
}

// Move the cursor: absolute position, column only, row only, relative steps, or N lines down / up to column 0.
rl_result rl_term_next_line(int64_t n) {
    printf("\x1b[%ldE", (long)n);
    return rl_ok_null();
}

// Move the cursor: absolute position, column only, row only, relative steps, or N lines down / up to column 0.
rl_result rl_term_prev_line(int64_t n) {
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
    printf("\x1b[8;%ld;%ldt", (long)rows, (long)cols);
    return rl_ok_null();
}

// Set the window title to the single char `ch`.
rl_result rl_term_set_title(int64_t ch) {
    printf("\x1b]0;%c\x07", (char)(unsigned char)ch);
    return rl_ok_null();
}

// Scroll the viewport up / down by `n` lines.
rl_result rl_term_scroll_up(int64_t n) {
    printf("\x1b[%ldS", (long)n);
    return rl_ok_null();
}

// Scroll the viewport up / down by `n` lines.
rl_result rl_term_scroll_down(int64_t n) {
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

// Print a value without moving to a new line.
void rl_term_print_inline(rl_result v) {
    switch (v.tag) {
        case RL_TAG_I64: printf("%ld", (long)v.data.i64); break;
        case RL_TAG_F64: printf("%g", v.data.f64); break;
        case RL_TAG_BOOL: printf("%s", v.data.boolean ? "true" : "false"); break;
        case RL_TAG_CHAR: printf("%c", (char)(unsigned char)v.data.i64); break;
        case RL_TAG_STR: printf("%.*s", (int)v.data.str.len, v.data.str.data); break;
        case RL_TAG_NULL: printf("null"); break;
        default: printf("<value>"); break;
    }
}

// Read one key press as an array of key codes (blocks).
rl_array rl_term_read_key(void) {
    char buf[32];
    uint64_t total = 0;
    // blocking read of one byte
    uint8_t c;
    if (read(STDIN_FILENO, &c, 1) != 1) {
        rl_string empty = rl_str_literal("", 0);
        rl_array arr = { .data = NULL, .len = 0, .cap = 0, .elem_size = sizeof(rl_string), .type_tag = RL_TAG_STR };
        return arr;
    }
    buf[total++] = c;
    // if escape, read more
    if (c == 27) {
        uint8_t next;
        // read with timeout using select
        fd_set fds;
        struct timeval tv = { .tv_sec = 0, .tv_usec = 100000 };
        FD_ZERO(&fds);
        FD_SET(STDIN_FILENO, &fds);
        while (select(STDIN_FILENO + 1, &fds, NULL, NULL, &tv) > 0) {
            if (read(STDIN_FILENO, &next, 1) != 1) break;
            buf[total++] = next;
            if (total >= sizeof(buf) - 1) break;
            FD_ZERO(&fds);
            FD_SET(STDIN_FILENO, &fds);
            tv.tv_sec = 0;
            tv.tv_usec = 10000;
        }
    }
    rl_string s = rl_str_literal(buf, total);
    rl_string *sarr = malloc(sizeof(rl_string));
    sarr[0] = s;
    rl_array arr = { .data = sarr, .len = 1, .cap = 1, .elem_size = sizeof(rl_string), .type_tag = RL_TAG_STR };
    return arr;
}

// True when input arrives within `ms` milliseconds.
bool rl_term_poll(int64_t ms) {
    fd_set fds;
    struct timeval tv = { .tv_sec = (long)(ms / 1000), .tv_usec = (long)((ms % 1000) * 1000) };
    FD_ZERO(&fds);
    FD_SET(STDIN_FILENO, &fds);
    return select(STDIN_FILENO + 1, &fds, NULL, NULL, &tv) > 0;
}

// ---- result unwrap (with error checking) ----

// Checked unwrap used by RL `unwrap`: aborts with a message when `r`
// is an error instead of silently reading a dead union member.
int64_t rl_result_unwrap_i64(rl_result r) {
    if (!r.is_ok) {
        fprintf(stderr, "error: unwrap called on err value\n");
        abort();
    }
    return r.data.i64;
}

// Checked unwrap used by RL `unwrap`: aborts with a message when `r`
// is an error instead of silently reading a dead union member.
double rl_result_unwrap_f64(rl_result r) {
    if (!r.is_ok) {
        fprintf(stderr, "error: unwrap called on err value\n");
        abort();
    }
    return r.data.f64;
}

// Checked unwrap used by RL `unwrap`: aborts with a message when `r`
// is an error instead of silently reading a dead union member.
bool rl_result_unwrap_bool(rl_result r) {
    if (!r.is_ok) {
        fprintf(stderr, "error: unwrap called on err value\n");
        abort();
    }
    return r.data.boolean;
}

// Checked unwrap used by RL `unwrap`: aborts with a message when `r`
// is an error instead of silently reading a dead union member.
rl_string rl_result_unwrap_str(rl_result r) {
    if (!r.is_ok) {
        fprintf(stderr, "error: unwrap called on err value\n");
        abort();
    }
    return r.data.str;
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

// Abort instead of returning (RL never type).
rl_never rl_never_fn(void) {
    fprintf(stderr, "error: reached unreachable code\n");
    abort();
}

// ---- std::c (FFI) ----

#define RL_C_MAX_HANDLES 256

static struct { void *handle; } rl_c_handles[RL_C_MAX_HANDLES];
static int rl_c_handle_count = 0;

// Allocate a handle id for a dlopen pointer.
static rl_result rl_c_new_handle(void *h) {
    if (rl_c_handle_count >= RL_C_MAX_HANDLES) {
        return rl_err_msg(rl_str_literal("c: too many open handles", 24));
    }
    int id = rl_c_handle_count++;
    rl_c_handles[id].handle = h;
    return rl_ok_i64(id);
}

// Look up dlopen pointer by handle id (NULL when unknown).
static void *rl_c_get_handle(rl_result r) {
    if (r.tag != RL_TAG_I64) return NULL;
    int64_t id = r.data.i64;
    if (id < 0 || id >= rl_c_handle_count) return NULL;
    return rl_c_handles[id].handle;
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
    if (handle_id >= 0 && handle_id < rl_c_handle_count) {
        h = rl_c_handles[handle_id].handle;
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
    if (handle_id >= 0 && handle_id < rl_c_handle_count) {
        h = rl_c_handles[handle_id].handle;
    }
    if (!h) return rl_err_msg(rl_str_literal("c: invalid handle", 17));
    dlclose(h);
    if (handle_id >= 0 && handle_id < rl_c_handle_count) {
        rl_c_handles[handle_id].handle = NULL;
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
    if (handle_id >= 0 && handle_id < rl_c_handle_count) {
        h = rl_c_handles[handle_id].handle;
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

// Allocate a socket handle id for fd and kind.
static rl_result rl_net_new_handle(int fd, enum rl_net_handle_kind kind) {
    if (rl_net_handle_count >= RL_NET_MAX_HANDLES) {
        return rl_err(-1);
    }
    int id = rl_net_handle_count++;
    rl_net_handles[id].kind = kind;
    rl_net_handles[id].fd = fd;
    return rl_ok_i64(id);
}

// Look up fd by handle id, or -1 when kind mismatches.
static int rl_net_get_fd(int64_t handle_id, enum rl_net_handle_kind expected) {
    if (handle_id < 0 || handle_id >= rl_net_handle_count) return -1;
    if (rl_net_handles[handle_id].kind != expected) return -1;
    return rl_net_handles[handle_id].fd;
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
    if (handle_id < 0 || handle_id >= rl_net_handle_count) return rl_err(-1);

    enum rl_net_handle_kind kind = rl_net_handles[handle_id].kind;
    if (kind != RL_NET_TCP_LISTENER && kind != RL_NET_TCP_STREAM) return rl_err(-1);

    close(rl_net_handles[handle_id].fd);
    rl_net_handles[handle_id].fd = -1;

    return rl_ok_null();
}

// Bind a UDP socket; result holds its handle id.
rl_result rl_net_udp_bind(rl_string address) {
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

    ssize_t n = send(fd, data.data, data.len, 0);
    if (n < 0) return rl_err(-1);

    return rl_ok_i64(n);
}

// Send to the default peer / to an explicit address; result holds bytes sent.
rl_result rl_net_udp_send_to(int64_t handle_id, rl_string data, rl_string address) {
    int fd = rl_net_get_fd(handle_id, RL_NET_UDP_SOCKET);
    if (fd < 0) return rl_err(-1);

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

// Receive one datagram / datagram plus sender address as a two-map.
rl_result rl_net_udp_recv(int64_t handle_id, int64_t max_bytes) {
    int fd = rl_net_get_fd(handle_id, RL_NET_UDP_SOCKET);
    if (fd < 0) return rl_err(-1);

    int buf_size = max_bytes > 0 ? (int)max_bytes : RL_NET_BUF_SIZE;
    char *buf = malloc(buf_size);
    ssize_t n = recv(fd, buf, buf_size, 0);
    if (n < 0) { free(buf); return rl_err(-1); }

    return rl_ok_str(rl_str_literal(buf, n));
}

// Receive one datagram / datagram plus sender address as a two-map.
rl_result rl_net_udp_recv_from(int64_t handle_id, int64_t max_bytes) {
    int fd = rl_net_get_fd(handle_id, RL_NET_UDP_SOCKET);
    if (fd < 0) return rl_err(-1);

    int buf_size = max_bytes > 0 ? (int)max_bytes : RL_NET_BUF_SIZE;
    char *buf = malloc(buf_size);
    struct sockaddr_in sender;
    socklen_t sender_len = sizeof(sender);

    ssize_t n = recvfrom(fd, buf, buf_size, 0, (struct sockaddr *)&sender, &sender_len);
    if (n < 0) { free(buf); return rl_err(-1); }

    char *data = malloc(n);
    memcpy(data, buf, n);
    free(buf);

    char addr_str[64];
    snprintf(addr_str, sizeof(addr_str), "%s:%d", inet_ntoa(sender.sin_addr), ntohs(sender.sin_port));
    uint64_t addr_len = strlen(addr_str);
    char *addr_dup = malloc(addr_len + 1);
    memcpy(addr_dup, addr_str, addr_len + 1);

    // Return 2-element array: [data_string, sender_addr_string]
    // Arrays store element pointers as int64_t (intptr_t)
    int64_t ptrs[2];
    ptrs[0] = (int64_t)(intptr_t)data;
    ptrs[1] = (int64_t)(intptr_t)addr_dup;
    rl_array result_arr = rl_arr_from_vals(ptrs, 2, sizeof(int64_t));

    return rl_ok_arr(result_arr);
}

// Close the socket.
rl_result rl_net_udp_close(int64_t handle_id) {
    if (handle_id < 0 || handle_id >= rl_net_handle_count) return rl_err(-1);
    if (rl_net_handles[handle_id].kind != RL_NET_UDP_SOCKET) return rl_err(-1);

    close(rl_net_handles[handle_id].fd);
    rl_net_handles[handle_id].fd = -1;

    return rl_ok_null();
}

// DNS lookup of `"host:port"`; result holds an array of `"ip:port"` strings.
rl_result rl_net_resolve(rl_string host_port) {
    char buf[256];
    int len = host_port.len < 255 ? (int)host_port.len : 255;
    memcpy(buf, host_port.data, len);
    buf[len] = '\0';

    struct addrinfo hints = {0}, *res;
    hints.ai_family = AF_INET;
    hints.ai_socktype = SOCK_STREAM;

    int rc = getaddrinfo(buf, NULL, &hints, &res);
    if (rc != 0) return rl_err(-1);

    // count results first
    int count = 0;
    for (struct addrinfo *p = res; p != NULL; p = p->ai_next) count++;

    int64_t *ptrs = malloc(count * sizeof(int64_t));
    int i = 0;
    for (struct addrinfo *p = res; p != NULL; p = p->ai_next) {
        struct sockaddr_in *addr = (struct sockaddr_in *)p->ai_addr;
        char ip[64];
        inet_ntop(AF_INET, &addr->sin_addr, ip, sizeof(ip));
        uint64_t ip_len = strlen(ip);
        char *ip_dup = malloc(ip_len + 1);
        memcpy(ip_dup, ip, ip_len + 1);
        ptrs[i++] = (int64_t)(intptr_t)ip_dup;
    }

    freeaddrinfo(res);
    rl_array result_arr = rl_arr_from_vals(ptrs, count, sizeof(int64_t));
    free(ptrs);
    return rl_ok_arr(result_arr);
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

// Allocate an HTTP handle id for a table entry.
static rl_result rl_http_new_handle(void *ptr, enum rl_http_handle_kind kind) {
    if (rl_http_handle_count >= RL_HTTP_MAX_HANDLES) return rl_err(-1);
    int id = rl_http_handle_count++;
    rl_http_handles[id].kind = kind;
    if (kind == RL_HTTP_SERVER) {
        rl_http_handles[id].data.server_fd = *(int *)ptr;
    } else {
        rl_http_handles[id].data.request = (struct rl_http_request_data *)ptr;
    }
    return rl_ok_i64(id);
}

// minimal HTTP/1.1 server: bind, listen, accept, parse request, return handle

// Start listening on addr; result holds the server id.
rl_result rl_http_server_start(rl_string addr) {
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
    if (handle_id < 0 || handle_id >= rl_http_handle_count) return rl_err(-1);
    if (rl_http_handles[handle_id].kind != RL_HTTP_SERVER) return rl_err(-1);

    int server_fd = rl_http_handles[handle_id].data.server_fd;
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
    if (handle_id < 0 || handle_id >= rl_http_handle_count) return rl_err(-1);
    if (rl_http_handles[handle_id].kind != RL_HTTP_SERVER) return rl_err(-1);

    int server_fd = rl_http_handles[handle_id].data.server_fd;

    struct pollfd pfd = { .fd = server_fd, .events = POLLIN };
    int ret = poll(&pfd, 1, 0);
    if (ret <= 0) return rl_ok_null();

    return rl_http_server_recv(handle_id);
}

// Stop the server and drop pending requests.
rl_result rl_http_server_stop(int64_t handle_id) {
    if (handle_id < 0 || handle_id >= rl_http_handle_count) return rl_err(-1);
    if (rl_http_handles[handle_id].kind != RL_HTTP_SERVER) return rl_err(-1);

    close(rl_http_handles[handle_id].data.server_fd);
    rl_http_handles[handle_id].data.server_fd = -1;
    return rl_ok_null();
}

// Method ("GET", ...) / path+query / one header / full body of a request.
rl_result rl_http_request_method(int64_t handle_id) {
    if (handle_id < 0 || handle_id >= rl_http_handle_count) return rl_err(-1);
    if (rl_http_handles[handle_id].kind != RL_HTTP_REQUEST) return rl_err(-1);

    struct rl_http_request_data *req = rl_http_handles[handle_id].data.request;
    uint64_t slen = strlen(req->method);
    char *dup = malloc(slen + 1);
    memcpy(dup, req->method, slen + 1);
    return rl_ok_str(rl_str_literal(dup, slen));
}

// Method ("GET", ...) / path+query / one header / full body of a request.
rl_result rl_http_request_url(int64_t handle_id) {
    if (handle_id < 0 || handle_id >= rl_http_handle_count) return rl_err(-1);
    if (rl_http_handles[handle_id].kind != RL_HTTP_REQUEST) return rl_err(-1);

    struct rl_http_request_data *req = rl_http_handles[handle_id].data.request;
    uint64_t slen = strlen(req->url);
    char *dup = malloc(slen + 1);
    memcpy(dup, req->url, slen + 1);
    return rl_ok_str(rl_str_literal(dup, slen));
}

// Method ("GET", ...) / path+query / one header / full body of a request.
rl_result rl_http_request_header(int64_t handle_id, rl_string name) {
    if (handle_id < 0 || handle_id >= rl_http_handle_count) return rl_err(-1);
    if (rl_http_handles[handle_id].kind != RL_HTTP_REQUEST) return rl_err(-1);

    struct rl_http_request_data *req = rl_http_handles[handle_id].data.request;

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
    if (handle_id < 0 || handle_id >= rl_http_handle_count) return rl_err(-1);
    if (rl_http_handles[handle_id].kind != RL_HTTP_REQUEST) return rl_err(-1);

    struct rl_http_request_data *req = rl_http_handles[handle_id].data.request;
    uint64_t slen = req->body_len;
    char *dup = malloc(slen + 1);
    memcpy(dup, req->body, slen);
    dup[slen] = '\0';
    return rl_ok_str(rl_str_literal(dup, slen));
}

// Answer a request and close it; pass `has_content_type` 0 to omit.
rl_result rl_http_respond(int64_t handle_id, int64_t status, rl_string body, rl_string content_type, int has_content_type) {
    if (handle_id < 0 || handle_id >= rl_http_handle_count) return rl_err(-1);
    if (rl_http_handles[handle_id].kind != RL_HTTP_REQUEST) return rl_err(-1);

    struct rl_http_request_data *req = rl_http_handles[handle_id].data.request;
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
    rl_http_handles[handle_id].kind = RL_HTTP_SERVER; // mark as consumed
    rl_http_handles[handle_id].data.server_fd = -1;

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

// Run a curl request and wrap the body or error as a result.
static rl_result rl_http_curl_perform(CURL *curl) {
    struct rl_http_curl_buf resp = {0};
    resp.cap = 4096;
    resp.data = malloc(resp.cap);

    long status = 0;
    curl_easy_setopt(curl, CURLOPT_WRITEFUNCTION, rl_http_curl_write_cb);
    curl_easy_setopt(curl, CURLOPT_WRITEDATA, &resp);
    curl_easy_setopt(curl, CURLOPT_HEADERFUNCTION, rl_http_curl_write_cb);
    curl_easy_setopt(curl, CURLOPT_HEADERDATA, &resp);

    CURLcode res = curl_easy_perform(curl);
    if (res != CURLE_OK) {
        free(resp.data);
        return rl_err(-1);
    }
    curl_easy_getinfo(curl, CURLINFO_RESPONSE_CODE, &status);

    // find body after \r\n\r\n
    char *body_start = memmem(resp.data, resp.len, "\r\n\r\n", 4);
    size_t body_len;
    char *body_data;
    if (body_start) {
        body_data = body_start + 4;
        body_len = resp.len - (size_t)(body_data - resp.data);
    } else {
        body_data = resp.data;
        body_len = resp.len;
    }

    // copy body to stable memory
    char *body_copy = malloc(body_len + 1);
    memcpy(body_copy, body_data, body_len);
    body_copy[body_len] = '\0';

    // return tuple (status, body) as 2-element int64 array of pointers
    int64_t ptrs[2];
    ptrs[0] = (int64_t)(intptr_t)(int64_t)status;
    ptrs[1] = (int64_t)(intptr_t)body_copy;
    rl_array result_arr = rl_arr_from_vals(ptrs, 2, sizeof(int64_t));

    free(resp.data);
    return rl_ok_arr(result_arr);
}

// GET / POST shorthand; result holds the response body as a string.
rl_result rl_http_get(rl_string url) {
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

// GET / POST shorthand; result holds the response body as a string.
rl_result rl_http_post(rl_string url, rl_string body, rl_string content_type, int has_content_type) {
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

// Pass `has_body` / `has_headers` 0 to skip those parts.
rl_result rl_http_request(rl_string method, rl_string url, rl_string body, int has_body, rl_string headers_json, int has_headers) {
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

    CURL *ehandle = curl;
    curl_easy_setopt(curl, CURLOPT_URL, url_buf);
    curl_easy_setopt(curl, CURLOPT_FOLLOWLOCATION, 1L);
    curl_easy_setopt(curl, CURLOPT_TIMEOUT, 30L);

    // set custom method
    curl_easy_setopt(curl, CURLOPT_CUSTOMREQUEST, method_buf);

    if (has_body) {
        curl_easy_setopt(curl, CURLOPT_POSTFIELDS, body.data);
        curl_easy_setopt(curl, CURLOPT_POSTFIELDSIZE, (long)body.len);
    }

    // parse simple headers: each line is "Name: Value\n"
    struct curl_slist *hdr_list = NULL;
    if (has_headers && headers_json.len > 0) {
        char *hdr_buf = malloc(headers_json.len + 1);
        memcpy(hdr_buf, headers_json.data, headers_json.len);
        hdr_buf[headers_json.len] = '\0';

        char *line = strtok(hdr_buf, "\n");
        while (line) {
            while (*line == ' ') line++;
            if (*line) hdr_list = curl_slist_append(hdr_list, line);
            line = strtok(NULL, "\n");
        }
        free(hdr_buf);
        if (hdr_list) curl_easy_setopt(curl, CURLOPT_HTTPHEADER, hdr_list);
    }

    rl_result result = rl_http_curl_perform(curl);
    if (hdr_list) curl_slist_free_all(hdr_list);
    curl_easy_cleanup(curl);
    return result;
}

#else

// stubs when libcurl is not available
// Stub GET shorthand; returns an error when libcurl is unavailable.
rl_result rl_http_get(rl_string url) { (void)url; return rl_err(-1); }
// GET / POST shorthand; result holds the response body as a string.
rl_result rl_http_post(rl_string url, rl_string body, rl_string ct, int h) { (void)url; (void)body; (void)ct; (void)h; return rl_err(-1); }
// Pass `has_body` / `has_headers` 0 to skip those parts.
rl_result rl_http_request(rl_string m, rl_string u, rl_string b, int hb, rl_string h, int hh) { (void)m; (void)u; (void)b; (void)hb; (void)h; (void)hh; return rl_err(-1); }

#endif
