//! `core::` - compiler intrinsics for self-hosting.
//!
//! The primitive floor under an RL-written stdlib: only what RL cannot
//! express (container construction/access, type query, abort). Numeric
//! conversions are deliberately absent (the `as` operator owns them).
//! Everything else - sorting, formatting, parsing, hashing - is ordinary
//! RL code built on top of these.
//!
//! Primitive semantics abort loudly on misuse (missing keys,
//! out-of-bounds indexes) instead of returning `err`: RL code builds
//! `result`-returning wrappers where it wants soft errors. This mirrors
//! `Index`/`m[k]`, not `map_get`.
//!
//! ```rl
//! get __map_new, __map_set, __map_get from core
//!
//! dec m = __map_new()
//! __map_set(m, "a", 1)
//! dec int x = __map_get(m, "a")
//! ```

#[cfg(feature = "impls")]
use rl_std_core::Runtime;
use rl_std_macros::native_fn;
#[cfg(feature = "impls")]
use rl_ast::statements::TypeAnnotation;
#[cfg(feature = "impls")]
use rl_utils::errors::Error;

// ---- arrays ---------------------------------------------------------------

#[native_fn(module = "core", sig( -> array[T]))]
pub fn __arr_new<R: Runtime>() -> R::Value {
    R::array(Vec::new(), TypeAnnotation::Infer)
}

#[native_fn(module = "core", sig(array[T], T -> array[T]))]
pub fn __arr_push<R: Runtime>(
    cx: &mut R::Cx,
    arr: R::Value,
    val: R::Value,
    span: R::Span,
) -> Result<R::Value, Error> {
    let Some((items, elem)) = R::as_array(&arr) else {
        return Err(R::error(
            cx,
            format!("__arr_push: expects an array, got {}", R::type_name(&arr)),
            span,
        ));
    };
    // Same element discipline as the annotated type: pushing a foreign
    // value aborts instead of silently heterogenizing the array.
    if !R::types_compatible(&R::value_type(&val), &elem) {
        return Err(R::error(
            cx,
            "__arr_push: value type mismatches array element type".to_string(),
            span,
        ));
    }
    let mut out: Vec<R::Value> = items.to_vec();
    out.push(val);
    Ok(R::array(out, elem))
}

#[native_fn(module = "core", sig(array[T], int -> T))]
pub fn __arr_get<R: Runtime>(
    cx: &mut R::Cx,
    arr: R::Value,
    idx: R::Value,
    span: R::Span,
) -> Result<R::Value, Error> {
    let Some(idx) = R::as_i64(&idx) else {
        return Err(R::error(
            cx,
            format!(
                "__arr_get: index must be int, got {}",
                R::type_name(&idx)
            ),
            span,
        ));
    };
    let Some((items, _)) = R::as_array(&arr) else {
        return Err(R::error(
            cx,
            format!("__arr_get: expects an array, got {}", R::type_name(&arr)),
            span,
        ));
    };
    if idx < 0 || (idx as usize) >= items.len() {
        return Err(R::error(
            cx,
            format!("__arr_get: index {idx} out of bounds (len {})", items.len()),
            span,
        ));
    }
    Ok(items[idx as usize].clone())
}

#[native_fn(module = "core", sig(array[T], int, T -> array[T]))]
pub fn __arr_set<R: Runtime>(
    cx: &mut R::Cx,
    arr: R::Value,
    idx: R::Value,
    val: R::Value,
    span: R::Span,
) -> Result<R::Value, Error> {
    let Some(idx) = R::as_i64(&idx) else {
        return Err(R::error(
            cx,
            format!(
                "__arr_set: index must be int, got {}",
                R::type_name(&idx)
            ),
            span,
        ));
    };
    let Some((items, elem)) = R::as_array(&arr) else {
        return Err(R::error(
            cx,
            format!("__arr_set: expects an array, got {}", R::type_name(&arr)),
            span,
        ));
    };
    if idx < 0 || (idx as usize) >= items.len() {
        return Err(R::error(
            cx,
            format!("__arr_set: index {idx} out of bounds (len {})", items.len()),
            span,
        ));
    }
    let mut out: Vec<R::Value> = items.to_vec();
    out[idx as usize] = val;
    Ok(R::array(out, elem))
}

// ---- maps -------------------------------------------------------------------

#[native_fn(module = "core", sig( -> map[string, T]))]
pub fn __map_new<R: Runtime>() -> R::Value {
    R::map(Vec::new(), TypeAnnotation::String, TypeAnnotation::Infer)
}

#[native_fn(module = "core", sig(map[K, V], K -> V))]
pub fn __map_get<R: Runtime>(
    cx: &mut R::Cx,
    map: R::Value,
    key: R::Value,
    span: R::Span,
) -> Result<R::Value, Error> {
    match R::map_get(&map, &key) {
        Some(Some(v)) => Ok(v),
        Some(None) => Err(R::error(
            cx,
            "__map_get: key not found in map".to_string(),
            span,
        )),
        None => Err(R::error(
            cx,
            format!("__map_get: expects a map, got {}", R::type_name(&map)),
            span,
        )),
    }
}

#[native_fn(module = "core", sig(map[K, V], K, V -> map[K, V]))]
pub fn __map_set<R: Runtime>(map: R::Value, key: R::Value, val: R::Value) -> R::Value {
    // In-place like map_merge: maps are shared, arrays copy. Matches the
    // runtime's existing container semantics.
    R::map_insert(&map, &key, &val);
    map
}

#[native_fn(module = "core", sig(map[K, V] -> array[K]))]
pub fn __map_keys<R: Runtime>(map: R::Value) -> R::Value {
    match R::as_map(&map) {
        Some((entries, key_ty, _)) => {
            let items: Vec<R::Value> = entries.into_iter().map(|(k, _)| k).collect();
            R::array(items, key_ty)
        }
        // Unreachable for checked code; empty rather than forged.
        None => R::array(Vec::new(), TypeAnnotation::Infer),
    }
}

// ---- sets ---------------------------------------------------------------------

#[native_fn(module = "core", sig( -> set[T]))]
pub fn __set_new<R: Runtime>() -> R::Value {
    R::set(Vec::new(), TypeAnnotation::Infer)
}

#[native_fn(module = "core", sig(set[T], T -> set[T]))]
pub fn __set_add<R: Runtime>(
    cx: &mut R::Cx,
    set: R::Value,
    val: R::Value,
    span: R::Span,
) -> Result<R::Value, Error> {
    match R::set_insert(&set, &val) {
        Some(_) => Ok(set),
        None => Err(R::error(
            cx,
            format!(
                "__set_add: expects a set and a hashable value, got {} and {}",
                R::type_name(&set),
                R::type_name(&val)
            ),
            span,
        )),
    }
}

#[native_fn(module = "core", sig(set[T], T -> bool))]
pub fn __set_has<R: Runtime>(
    cx: &mut R::Cx,
    set: R::Value,
    val: R::Value,
    span: R::Span,
) -> Result<R::Value, Error> {
    match R::set_contains(&set, &val) {
        Some(hit) => Ok(R::from_bool(hit)),
        None => Err(R::error(
            cx,
            format!(
                "__set_has: expects a set and a hashable value, got {} and {}",
                R::type_name(&set),
                R::type_name(&val)
            ),
            span,
        )),
    }
}

// ---- removes ----------------------------------------------------------------------
// Absence aborts, uniformly with reads: a missing key or element means a
// wrong key, not an idempotent delete. RL code that wants ensure-absent
// checks membership first (`__map_has` / `__set_has`).

#[native_fn(module = "core", sig(array[T], int -> array[T]))]
pub fn __arr_remove<R: Runtime>(
    cx: &mut R::Cx,
    arr: R::Value,
    idx: R::Value,
    span: R::Span,
) -> Result<R::Value, Error> {
    let Some(idx) = R::as_i64(&idx) else {
        return Err(R::error(
            cx,
            format!(
                "__arr_remove: index must be int, got {}",
                R::type_name(&idx)
            ),
            span,
        ));
    };
    let Some((items, elem)) = R::as_array(&arr) else {
        return Err(R::error(
            cx,
            format!("__arr_remove: expects an array, got {}", R::type_name(&arr)),
            span,
        ));
    };
    if idx < 0 || (idx as usize) >= items.len() {
        return Err(R::error(
            cx,
            format!(
                "__arr_remove: index {idx} out of bounds (len {})",
                items.len()
            ),
            span,
        ));
    }
    let mut out: Vec<R::Value> = items.to_vec();
    out.remove(idx as usize);
    Ok(R::array(out, elem))
}

#[native_fn(module = "core", sig(map[K, V], K -> map[K, V]))]
pub fn __map_remove<R: Runtime>(
    cx: &mut R::Cx,
    map: R::Value,
    key: R::Value,
    span: R::Span,
) -> Result<R::Value, Error> {
    match R::map_remove(&map, &key) {
        Some(Some(_)) => Ok(map),
        Some(None) => Err(R::error(
            cx,
            "__map_remove: key not found in map".to_string(),
            span,
        )),
        None => Err(R::error(
            cx,
            format!("__map_remove: expects a map, got {}", R::type_name(&map)),
            span,
        )),
    }
}

#[native_fn(module = "core", sig(map[K, V], K -> bool))]
pub fn __map_has<R: Runtime>(
    cx: &mut R::Cx,
    map: R::Value,
    key: R::Value,
    span: R::Span,
) -> Result<R::Value, Error> {
    match R::map_contains(&map, &key) {
        Some(hit) => Ok(R::from_bool(hit)),
        None => Err(R::error(
            cx,
            format!("__map_has: expects a map, got {}", R::type_name(&map)),
            span,
        )),
    }
}

#[native_fn(module = "core", sig(set[T], T -> set[T]))]
pub fn __set_remove<R: Runtime>(
    cx: &mut R::Cx,
    set: R::Value,
    val: R::Value,
    span: R::Span,
) -> Result<R::Value, Error> {
    match R::set_remove(&set, &val) {
        Some(true) => Ok(set),
        Some(false) => Err(R::error(
            cx,
            "__set_remove: value not in set".to_string(),
            span,
        )),
        None => Err(R::error(
            cx,
            format!(
                "__set_remove: expects a set and a hashable value, got {} and {}",
                R::type_name(&set),
                R::type_name(&val)
            ),
            span,
        )),
    }
}

// ---- lengths --------------------------------------------------------------------
// No length primitive existed anywhere; RL-written iteration needs these.

#[native_fn(module = "core", sig(array[T] -> int))]
pub fn __arr_len<R: Runtime>(
    cx: &mut R::Cx,
    arr: R::Value,
    span: R::Span,
) -> Result<i64, Error> {
    match R::as_array(&arr) {
        Some((items, _)) => Ok(items.len() as i64),
        None => Err(R::error(
            cx,
            format!("__arr_len: expects an array, got {}", R::type_name(&arr)),
            span,
        )),
    }
}

#[native_fn(module = "core", sig(map[K, V] -> int))]
pub fn __map_len<R: Runtime>(
    cx: &mut R::Cx,
    map: R::Value,
    span: R::Span,
) -> Result<i64, Error> {
    match R::map_len(&map) {
        Some(n) => Ok(n as i64),
        None => Err(R::error(
            cx,
            format!("__map_len: expects a map, got {}", R::type_name(&map)),
            span,
        )),
    }
}

#[native_fn(module = "core", sig(set[T] -> int))]
pub fn __set_len<R: Runtime>(
    cx: &mut R::Cx,
    set: R::Value,
    span: R::Span,
) -> Result<i64, Error> {
    match R::set_len(&set) {
        Some(n) => Ok(n as i64),
        None => Err(R::error(
            cx,
            format!("__set_len: expects a set, got {}", R::type_name(&set)),
            span,
        )),
    }
}

// ---- strings --------------------------------------------------------------------
// Byte-oriented (bytes, not chars): hashing, encodings and binary protocols
// index bytes. `==` already compares strings, so no equality intrinsic;
// `+` does not concat strings, so `__str_concat` stays.

#[native_fn(module = "core", sig(string -> int))]
pub fn __str_len(s: String) -> i64 {
    s.len() as i64
}

#[native_fn(module = "core", sig(string, int -> byte))]
pub fn __str_get_byte<R: Runtime>(
    cx: &mut R::Cx,
    s: String,
    idx: i64,
    span: R::Span,
) -> Result<u8, Error> {
    let bytes = s.as_bytes();
    if idx < 0 || (idx as usize) >= bytes.len() {
        return Err(R::error(
            cx,
            format!(
                "__str_get_byte: index {idx} out of bounds (len {})",
                bytes.len()
            ),
            span,
        ));
    }
    Ok(bytes[idx as usize])
}

#[native_fn(module = "core", sig(string, int, int -> string))]
pub fn __str_slice<R: Runtime>(
    cx: &mut R::Cx,
    s: String,
    start: i64,
    end: i64,
    span: R::Span,
) -> Result<String, Error> {
    let len = s.len() as i64;
    if start < 0 || end < start || end > len {
        return Err(R::error(
            cx,
            format!("__str_slice: bad range {start}..{end} for len {len}"),
            span,
        ));
    }
    // Byte slicing panics mid-codepoint; reject instead of crashing.
    match s.get(start as usize..end as usize) {
        Some(part) => Ok(part.to_owned()),
        None => Err(R::error(
            cx,
            "__str_slice: range splits a UTF-8 codepoint".to_string(),
            span,
        )),
    }
}

#[native_fn(module = "core", sig(string, string -> string))]
pub fn __str_concat(a: String, b: String) -> String {
    let mut out = String::with_capacity(a.len() + b.len());
    out.push_str(&a);
    out.push_str(&b);
    out
}

// ---- raw syscall (Linux only) -------------------------------------------------------
// Escape hatch for libc-free binaries and exotic ioctls. Portable code
// uses std::fs / std::io; raw numbers are Linux-only by construction.
// Returns the raw register: negative means -errno, interpreted by the caller.

#[native_fn(module = "core", sig(int, int, int, int, int, int, int -> int))]
pub fn __syscall6(nr: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: i64, a6: i64) -> i64 {
    #[cfg(target_os = "linux")]
    unsafe {
        libc::syscall(
            nr as libc::c_long,
            a1 as libc::c_long,
            a2 as libc::c_long,
            a3 as libc::c_long,
            a4 as libc::c_long,
            a5 as libc::c_long,
            a6 as libc::c_long,
        ) as i64
    }
    #[cfg(not(target_os = "linux"))]
    {
        let _ = (nr, a1, a2, a3, a4, a5, a6);
        panic!("__syscall6 is Linux-only")
    }
}

// ---- runtime queries ----------------------------------------------------------

#[native_fn(module = "core", sig(string -> T))]
pub fn __abort<R: Runtime>(
    cx: &mut R::Cx,
    msg: R::Value,
    span: R::Span,
) -> Result<R::Value, Error> {
    // `T` return unifies anywhere: abort never produces a value, so any
    // storage type accepts it. Always fails loud, never returns err.
    let text = R::as_str(&msg).unwrap_or("abort").to_owned();
    Err(R::error(cx, text, span))
}

#[native_fn(module = "core", sig(T -> string))]
pub fn __type_of<R: Runtime>(v: R::Value) -> String {
    R::type_name(&v).to_owned()
}

// ---- module registration --------------------------------------------------

rl_std_core::native_module!("core";
    funcs: [
        __arr_new, __arr_push, __arr_get, __arr_set, __arr_remove, __arr_len,
        __map_new, __map_get, __map_set, __map_remove, __map_has, __map_keys, __map_len,
        __set_new, __set_add, __set_has, __set_remove, __set_len,
        __str_len, __str_get_byte, __str_slice, __str_concat,
        __syscall6,
        __abort, __type_of,
    ],
);
