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
        __arr_new, __arr_push, __arr_get, __arr_set,
        __map_new, __map_get, __map_set, __map_keys,
        __set_new, __set_add, __set_has,
        __abort, __type_of,
    ],
);
