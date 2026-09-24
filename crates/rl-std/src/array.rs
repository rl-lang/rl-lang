//! `std::array` - array manipulation functions.
//!
//! All functions operate on arrays (`VmValue::Arr` / `Value::Values`) and return
//! a language error for non-array inputs. The higher-order functions (`arr_map`,
//! `arr_filter`, `arr_reduce`, ...) take a callback as a raw `R::Value` and
//! re-enter the runtime via [`Runtime::call_value`]; callback errors propagate
//! (the wrapper returns `Result<R::Value, Error>` and uses `?`).
//!
//! Ported once from the former per-runtime `stdlib/array/*.rs` copies. Because
//! most functions are element-type-generic they take the array as a raw
//! `R::Value` (with an explicit `sig(...)`) and round-trip the element type
//! through [`Runtime::as_array`] / [`Runtime::array`] so the interpreter keeps
//! its tracked `items_type` while the VM ignores it.
//!
//! A few interpreter-only refinements cannot be expressed through the shared
//! `Runtime` API and are therefore relaxed to the VM's (looser) behaviour:
//! `arr_push` / `arr_insert` drop the interpreter's element-type-compatibility
//! check (no `infer_type` / `types_compatible` in the trait); the numeric
//! reductions (`arr_sum`/`arr_max`/`arr_min`/`arr_product`) and `arr_sort`
//! inspect the actual element values (the VM path) rather than the tracked
//! `items_type`; and `arr_fill` / `arr_map` tag their output with
//! `TypeAnnotation::Infer` instead of a precisely inferred element type.

use rl_ast::statements::TypeAnnotation;
use rl_std_core::Runtime;
use rl_std_macros::native_fn;
use rl_utils::errors::Error;

// ---- structural (raw array in, `result[array]` out) -----------------------

#[native_fn(module = "array", sig(array[T], T -> result[array[T]]))]
pub fn arr_push<R: Runtime>(_cx: &mut R::Cx, array: R::Value, value: R::Value) -> R::Value {
    let Some((slice, elem)) = R::as_array(&array) else {
        return R::err(R::from_string(format!(
            "arr_push: accepts only arrays, found {}",
            R::type_name(&array)
        )));
    };
    let mut items = slice.to_vec();
    items.push(value);
    R::ok(R::array(items, elem))
}

#[native_fn(module = "array", sig(array[T] -> result[array[T]]))]
pub fn arr_pop<R: Runtime>(_cx: &mut R::Cx, array: R::Value) -> R::Value {
    let Some((slice, elem)) = R::as_array(&array) else {
        return R::err(R::from_string(format!(
            "arr_pop: accepts only arrays, found {}",
            R::type_name(&array)
        )));
    };
    if slice.is_empty() {
        return R::err(R::from_string("arr_pop: called on empty array".to_string()));
    }
    let mut items = slice.to_vec();
    items.pop();
    R::ok(R::array(items, elem))
}

#[native_fn(module = "array", sig(array[T], T, int -> result[array[T]]))]
pub fn arr_insert<R: Runtime>(
    _cx: &mut R::Cx,
    array: R::Value,
    value: R::Value,
    index: i64,
) -> R::Value {
    let Some((slice, elem)) = R::as_array(&array) else {
        return R::err(R::from_string(format!(
            "arr_insert: accepts only arrays and values, found {}",
            R::type_name(&array)
        )));
    };
    if index < 0 || index as usize > slice.len() {
        return R::err(R::from_string(format!(
            "arr_insert: index out of bounds: {}",
            index
        )));
    }
    let mut items = slice.to_vec();
    items.insert(index as usize, value);
    R::ok(R::array(items, elem))
}

#[native_fn(module = "array", sig(array[T], int -> result[array[T]]))]
pub fn arr_remove<R: Runtime>(_cx: &mut R::Cx, array: R::Value, index: i64) -> R::Value {
    let Some((slice, elem)) = R::as_array(&array) else {
        return R::err(R::from_string(format!(
            "arr_remove: accepts only arrays, found {}",
            R::type_name(&array)
        )));
    };
    if index < 0 || index as usize >= slice.len() {
        return R::err(R::from_string(format!(
            "arr_remove: index out of bounds: {}",
            index
        )));
    }
    let mut items = slice.to_vec();
    items.remove(index as usize);
    R::ok(R::array(items, elem))
}

#[native_fn(module = "array", sig(array[T] -> result[array[T]]))]
pub fn arr_reverse<R: Runtime>(_cx: &mut R::Cx, array: R::Value) -> R::Value {
    let Some((slice, elem)) = R::as_array(&array) else {
        return R::err(R::from_string(format!(
            "arr_reverse: accepts only arrays, found {}",
            R::type_name(&array)
        )));
    };
    let mut items = slice.to_vec();
    items.reverse();
    R::ok(R::array(items, elem))
}

#[native_fn(module = "array", sig(array[T], array[T] -> result[array[T]]))]
pub fn arr_concat<R: Runtime>(_cx: &mut R::Cx, array1: R::Value, array2: R::Value) -> R::Value {
    let (Some((slice1, elem1)), Some((slice2, elem2))) =
        (R::as_array(&array1), R::as_array(&array2))
    else {
        return R::err(R::from_string(
            "arr_concat: accepts only arrays".to_string(),
        ));
    };
    // The interpreter tracks element types and rejects a mismatch; on the VM
    // both types are `Infer`, so this check is naturally a no-op there.
    if elem1 != elem2 {
        return R::err(R::from_string(format!(
            "arr_concat: type mismatch: array type {:?}, cannot concat {:?}",
            elem1, elem2
        )));
    }
    let mut items = slice1.to_vec();
    items.extend(slice2.iter().cloned());
    R::ok(R::array(items, elem1))
}

#[native_fn(module = "array", sig(array[T] -> result[T]))]
pub fn arr_first<R: Runtime>(_cx: &mut R::Cx, array: R::Value) -> R::Value {
    let Some((slice, _)) = R::as_array(&array) else {
        return R::err(R::from_string(format!(
            "arr_first: accepts only arrays, found {}",
            R::type_name(&array)
        )));
    };
    match slice.first() {
        Some(v) => R::ok(v.clone()),
        None => R::err(R::from_string(
            "arr_first: called on empty array".to_string(),
        )),
    }
}

#[native_fn(module = "array", sig(array[T] -> result[T]))]
pub fn arr_last<R: Runtime>(_cx: &mut R::Cx, array: R::Value) -> R::Value {
    let Some((slice, _)) = R::as_array(&array) else {
        return R::err(R::from_string(format!(
            "arr_last: accepts only arrays, found {}",
            R::type_name(&array)
        )));
    };
    match slice.last() {
        Some(v) => R::ok(v.clone()),
        None => R::err(R::from_string(
            "arr_last: called on empty array".to_string(),
        )),
    }
}

#[native_fn(module = "array", sig(array[T] -> result[array[T]]))]
pub fn arr_unique<R: Runtime>(_cx: &mut R::Cx, array: R::Value) -> R::Value {
    let Some((slice, elem)) = R::as_array(&array) else {
        return R::err(R::from_string(format!(
            "arr_unique: accepts only arrays, found {}",
            R::type_name(&array)
        )));
    };
    let mut seen: Vec<R::Value> = Vec::new();
    for item in slice {
        if !seen.iter().any(|s| values_equal::<R>(s, item)) {
            seen.push(item.clone());
        }
    }
    R::ok(R::array(seen, elem))
}

#[native_fn(module = "array", sig(array[array[T]] -> result[array[T]]))]
pub fn arr_flatten<R: Runtime>(_cx: &mut R::Cx, array: R::Value) -> R::Value {
    let Some((slice, elem)) = R::as_array(&array) else {
        return R::err(R::from_string(format!(
            "arr_flatten: accepts only arrays, found {}",
            R::type_name(&array)
        )));
    };
    let mut out: Vec<R::Value> = Vec::new();
    for v in slice {
        match R::as_array(v) {
            Some((inner, _)) => out.extend(inner.iter().cloned()),
            None => out.push(v.clone()),
        }
    }
    R::ok(R::array(out, elem))
}

#[native_fn(module = "array", sig(array[T], int, int -> result[array[T]]))]
pub fn arr_slice<R: Runtime>(_cx: &mut R::Cx, array: R::Value, start: i64, end: i64) -> R::Value {
    let Some((slice, elem)) = R::as_array(&array) else {
        return R::err(R::from_string(format!(
            "arr_slice: accepts only arrays, found {}",
            R::type_name(&array)
        )));
    };
    let start = start as usize;
    let end = end as usize;
    if start > slice.len() || end > slice.len() {
        return R::err(R::from_string(format!(
            "arr_slice: index out of bounds: {}..{} (len {})",
            start,
            end,
            slice.len()
        )));
    }
    if start > end {
        return R::err(R::from_string(format!(
            "arr_slice: start {} is greater than end {}",
            start, end
        )));
    }
    R::ok(R::array(slice[start..end].to_vec(), elem))
}

// ---- queries (raw array in, `result[bool|int]` out) -----------------------

#[native_fn(module = "array", sig(array[T], T -> result[bool]))]
pub fn arr_contains<R: Runtime>(_cx: &mut R::Cx, array: R::Value, value: R::Value) -> R::Value {
    let Some((slice, _)) = R::as_array(&array) else {
        return R::err(R::from_string(format!(
            "arr_contains: accepts only arrays, found {}",
            R::type_name(&array)
        )));
    };
    R::ok(R::from_bool(
        slice.iter().any(|v| values_equal::<R>(v, &value)),
    ))
}

#[native_fn(module = "array", sig(array[T], T -> result[int]))]
pub fn arr_index_of<R: Runtime>(_cx: &mut R::Cx, array: R::Value, value: R::Value) -> R::Value {
    let Some((slice, _)) = R::as_array(&array) else {
        return R::err(R::from_string(format!(
            "arr_index_of: accepts only arrays, found {}",
            R::type_name(&array)
        )));
    };
    match slice
        .iter()
        .position(|item| values_equal::<R>(item, &value))
    {
        Some(pos) => R::ok(R::from_i64(pos as i64)),
        None => R::ok(R::from_i64(-1)),
    }
}

#[native_fn(module = "array", sig(array[T] -> result[int]))]
pub fn arr_count<R: Runtime>(_cx: &mut R::Cx, array: R::Value) -> R::Value {
    let Some((slice, _)) = R::as_array(&array) else {
        return R::err(R::from_string(format!(
            "arr_count: accepts only arrays, found {}",
            R::type_name(&array)
        )));
    };
    R::ok(R::from_i64(slice.len() as i64))
}

#[native_fn(module = "array", sig(array[T] -> result[bool]))]
pub fn arr_is_empty<R: Runtime>(_cx: &mut R::Cx, array: R::Value) -> R::Value {
    let Some((slice, _)) = R::as_array(&array) else {
        return R::err(R::from_string(format!(
            "arr_is_empty: accepts only arrays, found {}",
            R::type_name(&array)
        )));
    };
    R::ok(R::from_bool(slice.is_empty()))
}

// NOTE: `len` is intentionally NOT defined here. Its return convention diverges
// between the runtimes (the VM returns `result[int]`, the interpreter a bare
// `int`) and it needs tuple access, so each runtime registers its own legacy
// `len` on top of this module's handles. The shared signature tree registers
// its name as untyped so the checker can still resolve it in both runtimes.

// ---- builders -------------------------------------------------------------

// `arr_fill` returns a bare array (not a `result[...]`). The interpreter infers
// the element type from `value`; that helper is not in the shared API, so the
// output is tagged `Infer` (which the VM ignores anyway).
#[native_fn(module = "array", sig(T, int -> array[T]))]
pub fn arr_fill<R: Runtime>(_cx: &mut R::Cx, value: R::Value, count: i64) -> R::Value {
    let items = vec![value; count.max(0) as usize];
    R::array(items, TypeAnnotation::Infer)
}

#[native_fn(module = "array", sig(int, int, int -> result[array[int]]))]
pub fn arr_range<R: Runtime>(_cx: &mut R::Cx, start: i64, end: i64, step: i64) -> R::Value {
    if step <= 0 {
        return R::err(R::from_string(format!(
            "arr_range: step must be positive, got {}",
            step
        )));
    }
    let items: Vec<R::Value> = (start..end)
        .step_by(step as usize)
        .map(R::from_i64)
        .collect();
    R::ok(R::array(items, TypeAnnotation::Int))
}

// ---- numeric reductions (inspect actual element values) -------------------
//
// The VM inspects the element values; the interpreter branches on the tracked
// `items_type`. Value-inspection satisfies both: for a well-typed interpreter
// `int`/`float` array it yields identical results, and it is the only path the
// VM can take (its element type is `Infer`).

#[native_fn(
    module = "array",
    sig(array[int] -> result[int]),
    sig(array[float] -> result[float])
)]
pub fn arr_sum<R: Runtime>(_cx: &mut R::Cx, array: R::Value) -> R::Value {
    let Some((slice, _)) = R::as_array(&array) else {
        return R::err(R::from_string(format!(
            "arr_sum: accepts only arrays, found {}",
            R::type_name(&array)
        )));
    };
    if slice.iter().any(is_float::<R>) {
        let sum: f64 = slice.iter().filter_map(|v| R::as_f64(v)).sum();
        R::ok(R::from_f64(sum))
    } else {
        let sum: i64 = slice.iter().filter_map(|v| R::as_i64(v)).sum();
        R::ok(R::from_i64(sum))
    }
}

#[native_fn(
    module = "array",
    sig(array[int] -> result[int]),
    sig(array[float] -> result[float])
)]
pub fn arr_product<R: Runtime>(_cx: &mut R::Cx, array: R::Value) -> R::Value {
    let Some((slice, _)) = R::as_array(&array) else {
        return R::err(R::from_string(format!(
            "arr_product: accepts only arrays, found {}",
            R::type_name(&array)
        )));
    };
    if slice.iter().any(is_float::<R>) {
        let product: f64 = slice.iter().filter_map(|v| R::as_f64(v)).product();
        R::ok(R::from_f64(product))
    } else {
        let product: i64 = slice.iter().filter_map(|v| R::as_i64(v)).product();
        R::ok(R::from_i64(product))
    }
}

#[native_fn(
    module = "array",
    sig(array[int] -> result[int]),
    sig(array[float] -> result[float])
)]
pub fn arr_max<R: Runtime>(_cx: &mut R::Cx, array: R::Value) -> R::Value {
    let Some((slice, _)) = R::as_array(&array) else {
        return R::err(R::from_string(format!(
            "arr_max: accepts only arrays, found {}",
            R::type_name(&array)
        )));
    };
    if slice.iter().any(is_float::<R>) {
        match slice
            .iter()
            .filter_map(|v| R::as_f64(v))
            .max_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal))
        {
            Some(max) => R::ok(R::from_f64(max)),
            None => R::err(R::from_string("arr_max: called on empty array".to_string())),
        }
    } else {
        match slice.iter().filter_map(|v| R::as_i64(v)).max() {
            Some(max) => R::ok(R::from_i64(max)),
            None => R::err(R::from_string("arr_max: called on empty array".to_string())),
        }
    }
}

#[native_fn(
    module = "array",
    sig(array[int] -> result[int]),
    sig(array[float] -> result[float])
)]
pub fn arr_min<R: Runtime>(_cx: &mut R::Cx, array: R::Value) -> R::Value {
    let Some((slice, _)) = R::as_array(&array) else {
        return R::err(R::from_string(format!(
            "arr_min: accepts only arrays, found {}",
            R::type_name(&array)
        )));
    };
    if slice.iter().any(is_float::<R>) {
        match slice
            .iter()
            .filter_map(|v| R::as_f64(v))
            .min_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal))
        {
            Some(min) => R::ok(R::from_f64(min)),
            None => R::err(R::from_string("arr_min: called on empty array".to_string())),
        }
    } else {
        match slice.iter().filter_map(|v| R::as_i64(v)).min() {
            Some(min) => R::ok(R::from_i64(min)),
            None => R::err(R::from_string("arr_min: called on empty array".to_string())),
        }
    }
}

// ---- sorting (value-inspecting comparator) --------------------------------

#[native_fn(
    module = "array",
    sig(array[int] -> result[array[int]]),
    sig(array[float] -> result[array[float]])
)]
pub fn arr_sort<R: Runtime>(_cx: &mut R::Cx, array: R::Value) -> R::Value {
    let Some((slice, elem)) = R::as_array(&array) else {
        return R::err(R::from_string(format!(
            "arr_sort: accepts only arrays, found {}",
            R::type_name(&array)
        )));
    };
    // All ints (or nulls) -> integer sort.
    if slice
        .iter()
        .all(|v| R::as_i64(v).is_some() || is_null::<R>(v))
    {
        let mut items = slice.to_vec();
        items.sort_by(|a, b| match (R::as_i64(a), R::as_i64(b)) {
            (Some(x), Some(y)) => x.cmp(&y),
            _ => std::cmp::Ordering::Equal,
        });
        return R::ok(R::array(items, elem));
    }
    // All floats -> float sort.
    if slice.iter().all(|v| R::as_f64(v).is_some()) {
        let mut items = slice.to_vec();
        items.sort_by(|a, b| match (R::as_f64(a), R::as_f64(b)) {
            (Some(x), Some(y)) => x.partial_cmp(&y).unwrap_or(std::cmp::Ordering::Equal),
            _ => std::cmp::Ordering::Equal,
        });
        return R::ok(R::array(items, elem));
    }
    // All strings -> lexicographic sort.
    if slice.iter().all(|v| R::as_str(v).is_some()) {
        let mut items = slice.to_vec();
        items.sort_by(|a, b| match (R::as_str(a), R::as_str(b)) {
            (Some(x), Some(y)) => x.cmp(y),
            _ => std::cmp::Ordering::Equal,
        });
        return R::ok(R::array(items, elem));
    }
    R::err(R::from_string(
        "arr_sort: accepts only int, float, or string arrays".to_string(),
    ))
}

// ---- zip (raw arrays in, bare `array[tuple]` out) -------------------------

#[native_fn(module = "array", sig(array[T], array[U] -> array[tuple[T, U]]))]
pub fn arr_zip<R: Runtime>(
    cx: &mut R::Cx,
    array1: R::Value,
    array2: R::Value,
    span: R::Span,
) -> Result<R::Value, Error> {
    let (Some((slice1, elem1)), Some((slice2, elem2))) =
        (R::as_array(&array1), R::as_array(&array2))
    else {
        return Err(R::error(
            cx,
            format!(
                "arr_zip: expected two arrays, got {} and {}",
                R::type_name(&array1),
                R::type_name(&array2)
            ),
            span,
        ));
    };
    let items: Vec<R::Value> = slice1
        .iter()
        .zip(slice2.iter())
        .map(|(x, y)| R::tuple(vec![x.clone(), y.clone()]))
        .collect();
    let tuple_ty = TypeAnnotation::Tuple(std::rc::Rc::new(vec![elem1, elem2]));
    Ok(R::array(items, tuple_ty))
}

// ---- higher-order (callback via `call_value`; errors propagate with `?`) --

#[native_fn(module = "array", sig(array[T], callback(T -> U) -> result[array[U]]))]
pub fn arr_map<R: Runtime>(
    cx: &mut R::Cx,
    array: R::Value,
    f: R::Value,
    span: R::Span,
) -> Result<R::Value, Error> {
    let Some((slice, _)) = R::as_array(&array) else {
        return Ok(R::err(R::from_string(format!(
            "arr_map: accepts only arrays, found {}",
            R::type_name(&array)
        ))));
    };
    if !R::is_callable(&f) {
        return Ok(R::err(R::from_string(format!(
            "arr_map: expected function or lambda, found {}",
            R::type_name(&f)
        ))));
    }
    let mut out = Vec::with_capacity(slice.len());
    for item in slice {
        out.push(R::call_value(cx, &f, std::slice::from_ref(item), span)?);
    }
    // The interpreter infers the result element type from the first item; that
    // helper is not in the shared API, so the output element type is `Infer`
    // (the VM ignores it regardless).
    Ok(R::ok(R::array(out, TypeAnnotation::Infer)))
}

#[native_fn(module = "array", sig(array[T], callback(T -> bool) -> result[array[T]]))]
pub fn arr_filter<R: Runtime>(
    cx: &mut R::Cx,
    array: R::Value,
    f: R::Value,
    span: R::Span,
) -> Result<R::Value, Error> {
    let Some((slice, elem)) = R::as_array(&array) else {
        return Ok(R::err(R::from_string(format!(
            "arr_filter: accepts only arrays, found {}",
            R::type_name(&array)
        ))));
    };
    if !R::is_callable(&f) {
        return Ok(R::err(R::from_string(format!(
            "arr_filter: expected function or lambda, found {}",
            R::type_name(&f)
        ))));
    }
    if let Some(rt) = R::callable_return_type(&f)
        && rt != TypeAnnotation::Bool
    {
        return Ok(R::err(R::from_string(format!(
            "arr_filter: expected function or lambda with Bool return type, found {:?}",
            Some(rt)
        ))));
    }
    let mut out = Vec::new();
    for item in slice {
        let keep = R::call_value(cx, &f, std::slice::from_ref(item), span)?;
        if R::as_bool(&keep) == Some(true) {
            out.push(item.clone());
        }
    }
    Ok(R::ok(R::array(out, elem)))
}

#[native_fn(module = "array", sig(array[T], callback(T -> bool) -> result[T]))]
pub fn arr_find<R: Runtime>(
    cx: &mut R::Cx,
    array: R::Value,
    f: R::Value,
    span: R::Span,
) -> Result<R::Value, Error> {
    let Some((slice, _)) = R::as_array(&array) else {
        return Ok(R::err(R::from_string(format!(
            "arr_find: accepts only arrays, found {}",
            R::type_name(&array)
        ))));
    };
    if !R::is_callable(&f) {
        return Ok(R::err(R::from_string(format!(
            "arr_find: expected function or lambda, found {}",
            R::type_name(&f)
        ))));
    }
    if let Some(rt) = R::callable_return_type(&f)
        && rt != TypeAnnotation::Bool
    {
        return Ok(R::err(R::from_string(format!(
            "arr_find: expected function or lambda with Bool return type, found {:?}",
            Some(rt)
        ))));
    }
    for item in slice {
        let hit = R::call_value(cx, &f, std::slice::from_ref(item), span)?;
        if R::as_bool(&hit) == Some(true) {
            return Ok(R::ok(item.clone()));
        }
    }
    Ok(R::ok(R::null()))
}

#[native_fn(module = "array", sig(array[T], callback(T -> bool) -> result[int]))]
pub fn arr_find_index<R: Runtime>(
    cx: &mut R::Cx,
    array: R::Value,
    f: R::Value,
    span: R::Span,
) -> Result<R::Value, Error> {
    let Some((slice, _)) = R::as_array(&array) else {
        return Ok(R::err(R::from_string(format!(
            "arr_find_index: accepts only arrays, found {}",
            R::type_name(&array)
        ))));
    };
    if !R::is_callable(&f) {
        return Ok(R::err(R::from_string(format!(
            "arr_find_index: expected function or lambda, found {}",
            R::type_name(&f)
        ))));
    }
    if let Some(rt) = R::callable_return_type(&f)
        && rt != TypeAnnotation::Bool
    {
        return Ok(R::err(R::from_string(format!(
            "arr_find_index: expected function or lambda with Bool return type, found {:?}",
            Some(rt)
        ))));
    }
    for (i, item) in slice.iter().enumerate() {
        let hit = R::call_value(cx, &f, std::slice::from_ref(item), span)?;
        if R::as_bool(&hit) == Some(true) {
            return Ok(R::ok(R::from_i64(i as i64)));
        }
    }
    Ok(R::ok(R::from_i64(-1)))
}
#[native_fn(module = "array", sig(array[T], callback(T -> bool) -> result[bool]))]
pub fn arr_all<R: Runtime>(
    cx: &mut R::Cx,
    array: R::Value,
    f: R::Value,
    span: R::Span,
) -> Result<R::Value, Error> {
    let Some((slice, _)) = R::as_array(&array) else {
        return Ok(R::err(R::from_string(format!(
            "arr_all: accepts only arrays, found {}",
            R::type_name(&array)
        ))));
    };
    if !R::is_callable(&f) {
        return Ok(R::err(R::from_string(format!(
            "arr_all: expected function or lambda, found {}",
            R::type_name(&f)
        ))));
    }
    if let Some(rt) = R::callable_return_type(&f)
        && rt != TypeAnnotation::Bool
    {
        return Ok(R::err(R::from_string(format!(
            "arr_all: expected function or lambda with Bool return type, found {:?}",
            Some(rt)
        ))));
    }
    for item in slice {
        let v = R::call_value(cx, &f, std::slice::from_ref(item), span)?;
        if R::as_bool(&v) == Some(false) {
            return Ok(R::ok(R::from_bool(false)));
        }
    }
    Ok(R::ok(R::from_bool(true)))
}
#[native_fn(module = "array", sig(array[T], callback(T -> bool) -> result[bool]))]
pub fn arr_any<R: Runtime>(
    cx: &mut R::Cx,
    array: R::Value,
    f: R::Value,
    span: R::Span,
) -> Result<R::Value, Error> {
    let Some((slice, _)) = R::as_array(&array) else {
        return Ok(R::err(R::from_string(format!(
            "arr_any: accepts only arrays, found {}",
            R::type_name(&array)
        ))));
    };
    if !R::is_callable(&f) {
        return Ok(R::err(R::from_string(format!(
            "arr_any: expected function or lambda, found {}",
            R::type_name(&f)
        ))));
    }
    if let Some(rt) = R::callable_return_type(&f)
        && rt != TypeAnnotation::Bool
    {
        return Ok(R::err(R::from_string(format!(
            "arr_any: expected function or lambda with Bool return type, found {:?}",
            Some(rt)
        ))));
    }
    for item in slice {
        let v = R::call_value(cx, &f, std::slice::from_ref(item), span)?;
        if R::as_bool(&v) == Some(true) {
            return Ok(R::ok(R::from_bool(true)));
        }
    }
    Ok(R::ok(R::from_bool(false)))
}

#[native_fn(module = "array", sig(array[T], callback(T -> array[U]) -> result[array[U]]))]
pub fn arr_flat_map<R: Runtime>(
    cx: &mut R::Cx,
    array: R::Value,
    f: R::Value,
    span: R::Span,
) -> Result<R::Value, Error> {
    let Some((slice, elem)) = R::as_array(&array) else {
        return Ok(R::err(R::from_string(format!(
            "arr_flat_map: accepts only arrays, found {}",
            R::type_name(&array)
        ))));
    };
    if !R::is_callable(&f) {
        return Ok(R::err(R::from_string(format!(
            "arr_flat_map: expected function or lambda, found {}",
            R::type_name(&f)
        ))));
    }
    if let Some(rt) = R::callable_return_type(&f)
        && !matches!(rt, TypeAnnotation::Array(_))
    {
        return Ok(R::err(R::from_string(format!(
            "arr_flat_map: expected function or lambda with Array return type, found {:?}",
            Some(rt)
        ))));
    }
    let mut out = Vec::with_capacity(slice.len());
    for item in slice {
        let mapped = R::call_value(cx, &f, std::slice::from_ref(item), span)?;
        if let Some((inner, _)) = R::as_array(&mapped) {
            out.extend(inner.iter().cloned());
        }
    }
    // Mirrors the interpreter: the input array's element type is preserved
    // (`items_type`); the VM ignores it.
    Ok(R::ok(R::array(out, elem)))
}

#[native_fn(module = "array", sig(array[T], callback(T -> null) -> result[null]))]
pub fn arr_for_each<R: Runtime>(
    cx: &mut R::Cx,
    array: R::Value,
    f: R::Value,
    span: R::Span,
) -> Result<R::Value, Error> {
    let Some((slice, _)) = R::as_array(&array) else {
        return Ok(R::err(R::from_string(format!(
            "arr_for_each: accepts only arrays, found {}",
            R::type_name(&array)
        ))));
    };
    if !R::is_callable(&f) {
        return Ok(R::err(R::from_string(format!(
            "arr_for_each: expected function or lambda, found {}",
            R::type_name(&f)
        ))));
    }
    if let Some(rt) = R::callable_return_type(&f)
        && rt != TypeAnnotation::Null
    {
        return Ok(R::err(R::from_string(format!(
            "arr_for_each: expected function or lambda with no (or null) return type, found {:?}",
            Some(rt)
        ))));
    }
    for item in slice {
        R::call_value(cx, &f, std::slice::from_ref(item), span)?;
    }
    Ok(R::ok(R::null()))
}

#[native_fn(module = "array", sig(array[T], callback(U, T -> U), U -> result[U]))]
pub fn arr_reduce<R: Runtime>(
    cx: &mut R::Cx,
    array: R::Value,
    f: R::Value,
    initial: R::Value,
    span: R::Span,
) -> Result<R::Value, Error> {
    let Some((slice, _)) = R::as_array(&array) else {
        return Ok(R::err(R::from_string(format!(
            "arr_reduce: accepts only arrays, found {}",
            R::type_name(&array)
        ))));
    };
    if !R::is_callable(&f) {
        return Ok(R::err(R::from_string(format!(
            "arr_reduce: expected function or lambda, found {}",
            R::type_name(&f)
        ))));
    }
    let mut acc = initial;
    for item in slice {
        acc = R::call_value(cx, &f, &[acc, item.clone()], span)?;
    }
    Ok(R::ok(acc))
}

#[native_fn(module = "array", sig(array[T], callback(T, T -> int) -> result[array[T]]))]
pub fn arr_sort_by<R: Runtime>(
    cx: &mut R::Cx,
    array: R::Value,
    f: R::Value,
    span: R::Span,
) -> Result<R::Value, Error> {
    let Some((slice, elem)) = R::as_array(&array) else {
        return Ok(R::err(R::from_string(format!(
            "arr_sort_by: accepts only arrays, found {}",
            R::type_name(&array)
        ))));
    };
    if !R::is_callable(&f) {
        return Ok(R::err(R::from_string(format!(
            "arr_sort_by: expected function or lambda, found {}",
            R::type_name(&f)
        ))));
    }
    // Insertion sort so the comparator (which may re-enter the runtime) is
    // invoked exactly as in the original per-runtime implementations.
    let mut items = slice.to_vec();
    for i in 1..items.len() {
        let mut j = i;
        while j > 0 {
            let cmp = R::call_value(cx, &f, &[items[j - 1].clone(), items[j].clone()], span)?;
            match R::as_i64(&cmp) {
                Some(n) if n > 0 => {
                    items.swap(j - 1, j);
                    j -= 1;
                }
                Some(_) => break,
                None => {
                    return Ok(R::err(R::from_string(format!(
                        "arr_sort_by: comparator must return int (-1, 0, 1), found {}",
                        R::type_name(&cmp)
                    ))));
                }
            }
        }
    }
    Ok(R::ok(R::array(items, elem)))
}

// ---- batch / window / partition / swap / cycle ----------------------------

#[native_fn(module = "array", sig(array[T], int -> result[array[array[T]]]))]
pub fn arr_chunk<R: Runtime>(_cx: &mut R::Cx, array: R::Value, size: i64) -> R::Value {
    let Some((slice, elem)) = R::as_array(&array) else {
        return R::err(R::from_string(format!(
            "arr_chunk: accepts only arrays, found {}",
            R::type_name(&array)
        )));
    };
    if size <= 0 {
        return R::err(R::from_string(format!(
            "arr_chunk: size must be positive, got {}",
            size
        )));
    }
    let size = size as usize;
    let chunks: Vec<R::Value> = slice
        .chunks(size)
        .map(|c| R::array(c.to_vec(), elem.clone()))
        .collect();
    let inner = TypeAnnotation::Array(Box::new(elem));
    R::ok(R::array(chunks, inner))
}

#[native_fn(module = "array", sig(array[T], int -> result[array[array[T]]]))]
pub fn arr_windows<R: Runtime>(_cx: &mut R::Cx, array: R::Value, size: i64) -> R::Value {
    let Some((slice, elem)) = R::as_array(&array) else {
        return R::err(R::from_string(format!(
            "arr_windows: accepts only arrays, found {}",
            R::type_name(&array)
        )));
    };
    if size <= 0 {
        return R::err(R::from_string(format!(
            "arr_windows: size must be positive, got {}",
            size
        )));
    }
    let size = size as usize;
    if size > slice.len() {
        return R::err(R::from_string(format!(
            "arr_windows: size {} exceeds array length {}",
            size,
            slice.len()
        )));
    }
    let windows: Vec<R::Value> = slice
        .windows(size)
        .map(|w| R::array(w.to_vec(), elem.clone()))
        .collect();
    let inner = TypeAnnotation::Array(Box::new(elem));
    R::ok(R::array(windows, inner))
}

#[native_fn(module = "array", sig(array[T], int, int -> result[array[T]]))]
pub fn arr_swap<R: Runtime>(_cx: &mut R::Cx, array: R::Value, i: i64, j: i64) -> R::Value {
    let Some((slice, elem)) = R::as_array(&array) else {
        return R::err(R::from_string(format!(
            "arr_swap: accepts only arrays, found {}",
            R::type_name(&array)
        )));
    };
    if i < 0 || i as usize >= slice.len() || j < 0 || j as usize >= slice.len() {
        return R::err(R::from_string(format!(
            "arr_swap: index out of bounds: {} or {} (len {})",
            i,
            j,
            slice.len()
        )));
    }
    let mut items = slice.to_vec();
    items.swap(i as usize, j as usize);
    R::ok(R::array(items, elem))
}

#[native_fn(module = "array", sig(array[T], callback(T -> bool) -> result[array[array[T]]]))]
pub fn arr_partition<R: Runtime>(
    cx: &mut R::Cx,
    array: R::Value,
    f: R::Value,
    span: R::Span,
) -> Result<R::Value, Error> {
    let Some((slice, elem)) = R::as_array(&array) else {
        return Ok(R::err(R::from_string(format!(
            "arr_partition: accepts only arrays, found {}",
            R::type_name(&array)
        ))));
    };
    if !R::is_callable(&f) {
        return Ok(R::err(R::from_string(format!(
            "arr_partition: expected function or lambda, found {}",
            R::type_name(&f)
        ))));
    }
    let mut matching = Vec::new();
    let mut rest = Vec::new();
    for item in slice {
        let result = R::call_value(cx, &f, std::slice::from_ref(item), span)?;
        if R::as_bool(&result) == Some(true) {
            matching.push(item.clone());
        } else {
            rest.push(item.clone());
        }
    }
    let inner = TypeAnnotation::Array(Box::new(elem));
    Ok(R::ok(R::array(
        vec![
            R::array(matching, inner.clone()),
            R::array(rest, inner),
        ],
        TypeAnnotation::Infer,
    )))
}

#[native_fn(module = "array", sig(array[T], callback(T -> U) -> result[T]))]
pub fn arr_max_by<R: Runtime>(
    cx: &mut R::Cx,
    array: R::Value,
    f: R::Value,
    span: R::Span,
) -> Result<R::Value, Error> {
    let Some((slice, _)) = R::as_array(&array) else {
        return Ok(R::err(R::from_string(format!(
            "arr_max_by: accepts only arrays, found {}",
            R::type_name(&array)
        ))));
    };
    if !R::is_callable(&f) {
        return Ok(R::err(R::from_string(format!(
            "arr_max_by: expected function or lambda, found {}",
            R::type_name(&f)
        ))));
    }
    if slice.is_empty() {
        return Ok(R::err(R::from_string(
            "arr_max_by: called on empty array".to_string(),
        )));
    }
    let mut best = &slice[0];
    let mut best_key = R::call_value(cx, &f, std::slice::from_ref(best), span)?;
    for item in &slice[1..] {
        let key = R::call_value(cx, &f, std::slice::from_ref(item), span)?;
        let cmp = match (R::as_i64(&best_key), R::as_i64(&key)) {
            (Some(a), Some(b)) => a.cmp(&b),
            _ => match (R::as_f64(&best_key), R::as_f64(&key)) {
                (Some(a), Some(b)) => a.partial_cmp(&b).unwrap_or(std::cmp::Ordering::Equal),
                _ => match (R::as_str(&best_key), R::as_str(&key)) {
                    (Some(a), Some(b)) => a.cmp(b),
                    _ => std::cmp::Ordering::Equal,
                },
            },
        };
        if cmp == std::cmp::Ordering::Less {
            best = item;
            best_key = key;
        }
    }
    Ok(R::ok(best.clone()))
}

#[native_fn(module = "array", sig(array[T], callback(T -> U) -> result[T]))]
pub fn arr_min_by<R: Runtime>(
    cx: &mut R::Cx,
    array: R::Value,
    f: R::Value,
    span: R::Span,
) -> Result<R::Value, Error> {
    let Some((slice, _)) = R::as_array(&array) else {
        return Ok(R::err(R::from_string(format!(
            "arr_min_by: accepts only arrays, found {}",
            R::type_name(&array)
        ))));
    };
    if !R::is_callable(&f) {
        return Ok(R::err(R::from_string(format!(
            "arr_min_by: expected function or lambda, found {}",
            R::type_name(&f)
        ))));
    }
    if slice.is_empty() {
        return Ok(R::err(R::from_string(
            "arr_min_by: called on empty array".to_string(),
        )));
    }
    let mut best = &slice[0];
    let mut best_key = R::call_value(cx, &f, std::slice::from_ref(best), span)?;
    for item in &slice[1..] {
        let key = R::call_value(cx, &f, std::slice::from_ref(item), span)?;
        let cmp = match (R::as_i64(&best_key), R::as_i64(&key)) {
            (Some(a), Some(b)) => a.cmp(&b),
            _ => match (R::as_f64(&best_key), R::as_f64(&key)) {
                (Some(a), Some(b)) => a.partial_cmp(&b).unwrap_or(std::cmp::Ordering::Equal),
                _ => match (R::as_str(&best_key), R::as_str(&key)) {
                    (Some(a), Some(b)) => a.cmp(b),
                    _ => std::cmp::Ordering::Equal,
                },
            },
        };
        if cmp == std::cmp::Ordering::Greater {
            best = item;
            best_key = key;
        }
    }
    Ok(R::ok(best.clone()))
}

#[native_fn(module = "array", sig(array[T], array[T], T -> result[array[tuple[T, T]]]))]
pub fn arr_zip_longest<R: Runtime>(
    cx: &mut R::Cx,
    array1: R::Value,
    array2: R::Value,
    fill: R::Value,
    span: R::Span,
) -> Result<R::Value, Error> {
    let (Some((slice1, elem1)), Some((slice2, elem2))) =
        (R::as_array(&array1), R::as_array(&array2))
    else {
        return Err(R::error(
            cx,
            format!(
                "arr_zip_longest: expected two arrays, got {} and {}",
                R::type_name(&array1),
                R::type_name(&array2)
            ),
            span,
        ));
    };
    let max_len = slice1.len().max(slice2.len());
    let items: Vec<R::Value> = (0..max_len)
        .map(|i| {
            let a = if i < slice1.len() {
                slice1[i].clone()
            } else {
                fill.clone()
            };
            let b = if i < slice2.len() {
                slice2[i].clone()
            } else {
                fill.clone()
            };
            R::tuple(vec![a, b])
        })
        .collect();
    let tuple_ty = TypeAnnotation::Tuple(std::rc::Rc::new(vec![elem1, elem2]));
    Ok(R::ok(R::array(items, tuple_ty)))
}

#[native_fn(module = "array", sig(array[T], int -> result[array[T]]))]
pub fn arr_cycle_take<R: Runtime>(_cx: &mut R::Cx, array: R::Value, n: i64) -> R::Value {
    let Some((slice, elem)) = R::as_array(&array) else {
        return R::err(R::from_string(format!(
            "arr_cycle_take: accepts only arrays, found {}",
            R::type_name(&array)
        )));
    };
    if n < 0 {
        return R::err(R::from_string(format!(
            "arr_cycle_take: n must be non-negative, got {}",
            n
        )));
    }
    if slice.is_empty() {
        return R::ok(R::array(Vec::new(), elem));
    }
    let n = n as usize;
    let items: Vec<R::Value> = (0..n)
        .map(|i| slice[i % slice.len()].clone())
        .collect();
    R::ok(R::array(items, elem))
}

// ---- local helpers --------------------------------------------------------

/// Whether the value reads as a float but not as an int (i.e. a genuine float
/// element), used to decide the int-vs-float branch of the numeric reductions.
fn is_float<R: Runtime>(v: &R::Value) -> bool {
    R::as_i64(v).is_none() && R::as_f64(v).is_some()
}

fn is_null<R: Runtime>(v: &R::Value) -> bool {
    R::type_name(v) == "null"
}

/// Structural value equality expressed through the shared `Runtime` API (which
/// exposes no `PartialEq` on `R::Value`). Values of different kinds are never
/// equal (so a string `"1"` never matches an int `1`, matching the runtimes'
/// derived `PartialEq`); scalars are compared via their extractor, and any
/// remaining same-kind values (arrays, tuples, maps, ...) fall back to their
/// `Display` rendering.
fn values_equal<R: Runtime>(a: &R::Value, b: &R::Value) -> bool {
    let ta = R::type_name(a);
    if ta != R::type_name(b) {
        return false;
    }
    match ta {
        "bool" => R::as_bool(a) == R::as_bool(b),
        "char" => R::as_char(a) == R::as_char(b),
        "string" => R::as_str(a) == R::as_str(b),
        "int" => R::as_i64(a) == R::as_i64(b),
        "float" => R::as_f64(a) == R::as_f64(b),
        // Other numeric kinds (uint / byte / small / big variants) lack a
        // shared same-width extractor; their `Display` renders the exact value,
        // which is a sound equality for same-`type_name` integers/floats.
        // Compound values (array / tuple / map / ...) also fall back to
        // `Display`, matching how the runtimes render them structurally.
        _ => R::display(a) == R::display(b),
    }
}

rl_std_core::native_module!("array";
    funcs: [
        arr_push, arr_pop, arr_insert, arr_remove, arr_reverse, arr_concat,
        arr_first, arr_last, arr_max, arr_min, arr_sum, arr_product,
        arr_unique, arr_is_empty, arr_index_of, arr_count, arr_contains,
        arr_range, arr_flatten, arr_sort, arr_fill, arr_slice,
        arr_map, arr_filter, arr_all, arr_any, arr_find, arr_find_index,
        arr_reduce, arr_sort_by, arr_flat_map, arr_for_each, arr_zip,
        arr_chunk, arr_windows, arr_swap, arr_partition,
        arr_max_by, arr_min_by, arr_zip_longest, arr_cycle_take,
    ],
);
