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

// ---- byte buffers ---------------------------------------------------------------
// Incremental accumulation without the O(n^2) of repeated `arr_push` /
// `__str_concat`: amortized-O(1) push and bulk append behind an integer
// handle. The primitive floor for an RL-written renderer, lexer, and
// C-emitter. Misuse aborts (bad handle, out-of-bounds), like the rest
// of `core::`.

/// Per-runtime access to the `core` buffer table. Implemented by `VmRuntime`.
pub trait BufStore: Runtime {
    fn buf_insert(cx: &mut Self::Cx, buf: Vec<u8>) -> u64;
    fn buf_get(cx: &Self::Cx, id: u64) -> Option<&Vec<u8>>;
    fn buf_get_mut(cx: &mut Self::Cx, id: u64) -> Option<&mut Vec<u8>>;
    fn buf_remove(cx: &mut Self::Cx, id: u64) -> Option<Vec<u8>>;
}

/// Outcome of a finished worker thread: the displayed final value on
/// success, or the displayed error on failure. Only strings cross the
/// thread boundary; values never do (`Vm` is `!Send` by design).
pub type JobOutcome = Result<String, String>;

/// One message on a worker's channel: streamed progress, or the final
/// outcome (after which the worker's sender is gone).
pub enum ThreadMsg {
    Progress(String),
    Done(JobOutcome),
}

/// One poll of a worker thread: unknown id, nothing new, a progress
/// message, or the finished outcome (which reaps the job).
pub enum ThreadPoll {
    Unknown,
    Pending,
    Message(String),
    Done(JobOutcome),
}

/// Per-runtime access to the worker-thread table. Implemented by
/// `VmRuntime`: each worker owns a fresh `Vm` on its own pooled thread,
/// so no `Send` bounds leak into the VM itself.
pub trait ThreadStore: BufStore {
    /// Queue source on the shared pool. Err on nested spawn (workers
    /// cannot spawn) or a saturated pool; the id is only valid on success.
    fn thread_spawn(cx: &mut Self::Cx, source: String) -> Result<u64, String>;
    fn thread_poll(cx: &mut Self::Cx, id: u64) -> ThreadPoll;
    /// Sends a progress message from inside a worker. Returns false on a
    /// main `Vm` (which has no worker channel).
    fn thread_emit(cx: &mut Self::Cx, text: String) -> bool;
    /// True when `cx` is a worker Vm (has a worker channel).
    fn thread_is_worker(cx: &Self::Cx) -> bool;
}
/// Extracts a `Buffer` handle id from a value, or returns a type error.
#[cfg(feature = "impls")]
pub fn extract_buffer<R: Runtime>(v: &R::Value, name: &str) -> Result<u64, String> {
    match R::as_handle(v, rl_ast::statements::HandleKind::Buffer) {
        Some(id) => Ok(id),
        None => Err(format!(
            "{}: expected a buffer handle, got {}",
            name,
            R::type_name(v)
        )),
    }
}

#[cfg(feature = "impls")]
fn buf_lookup<R: BufStore>(
    cx: &mut R::Cx,
    handle: &R::Value,
    name: &str,
    span: R::Span,
) -> Result<u64, Error> {
    match extract_buffer::<R>(handle, name) {
        Ok(id) => match R::buf_get(cx, id) {
            Some(_) => Ok(id),
            None => Err(R::error(
                cx,
                format!("{}: invalid buffer handle", name),
                span,
            )),
        },
        Err(e) => Err(R::error(cx, e, span)),
    }
}

#[native_fn(module = "core", bound = "BufStore", sig( -> handle(Buffer)))]
pub fn __buf_new<R: BufStore>(cx: &mut R::Cx) -> R::Value {
    let id = R::buf_insert(cx, Vec::new());
    R::make_handle(rl_ast::statements::HandleKind::Buffer, id)
}

#[native_fn(module = "core", bound = "BufStore", sig(handle(Buffer) -> int))]
pub fn __buf_len<R: BufStore>(
    cx: &mut R::Cx,
    handle: R::Value,
    span: R::Span,
) -> Result<i64, Error> {
    let id = buf_lookup::<R>(cx, &handle, "__buf_len", span)?;
    Ok(R::buf_get(cx, id).map(|b| b.len() as i64).unwrap_or(0))
}

#[native_fn(module = "core", bound = "BufStore", sig(handle(Buffer), byte -> null))]
pub fn __buf_push_byte<R: BufStore>(
    cx: &mut R::Cx,
    handle: R::Value,
    byte: u8,
    span: R::Span,
) -> Result<R::Value, Error> {
    let id = buf_lookup::<R>(cx, &handle, "__buf_push_byte", span)?;
    if let Some(buf) = R::buf_get_mut(cx, id) {
        buf.push(byte);
    }
    Ok(R::null())
}

#[native_fn(module = "core", bound = "BufStore", sig(handle(Buffer), int -> byte))]
pub fn __buf_get_byte<R: BufStore>(
    cx: &mut R::Cx,
    handle: R::Value,
    idx: i64,
    span: R::Span,
) -> Result<u8, Error> {
    let id = buf_lookup::<R>(cx, &handle, "__buf_get_byte", span)?;
    let buf = R::buf_get(cx, id).ok_or_else(|| {
        R::error(cx, "__buf_get_byte: invalid buffer handle".to_string(), span)
    })?;
    if idx < 0 || (idx as usize) >= buf.len() {
        return Err(R::error(
            cx,
            format!(
                "__buf_get_byte: index {idx} out of bounds (len {})",
                buf.len()
            ),
            span,
        ));
    }
    Ok(buf[idx as usize])
}

#[native_fn(module = "core", bound = "BufStore", sig(handle(Buffer), int, byte -> null))]
pub fn __buf_set_byte<R: BufStore>(
    cx: &mut R::Cx,
    handle: R::Value,
    idx: i64,
    byte: u8,
    span: R::Span,
) -> Result<R::Value, Error> {
    let id = buf_lookup::<R>(cx, &handle, "__buf_set_byte", span)?;
    let len = R::buf_get(cx, id).map(|b| b.len()).unwrap_or(0);
    if idx < 0 || (idx as usize) >= len {
        return Err(R::error(
            cx,
            format!("__buf_set_byte: index {idx} out of bounds (len {len})"),
            span,
        ));
    }
    if let Some(buf) = R::buf_get_mut(cx, id) {
        buf[idx as usize] = byte;
    }
    Ok(R::null())
}

#[native_fn(module = "core", bound = "BufStore", sig(handle(Buffer), string -> null))]
pub fn __buf_append<R: BufStore>(
    cx: &mut R::Cx,
    handle: R::Value,
    text: String,
    span: R::Span,
) -> Result<R::Value, Error> {
    let id = buf_lookup::<R>(cx, &handle, "__buf_append", span)?;
    if let Some(buf) = R::buf_get_mut(cx, id) {
        buf.extend_from_slice(text.as_bytes());
    }
    Ok(R::null())
}

#[native_fn(module = "core", bound = "BufStore", sig(handle(Buffer), int, int -> string))]
pub fn __buf_slice<R: BufStore>(
    cx: &mut R::Cx,
    handle: R::Value,
    start: i64,
    end: i64,
    span: R::Span,
) -> Result<String, Error> {
    let id = buf_lookup::<R>(cx, &handle, "__buf_slice", span)?;
    let buf = R::buf_get(cx, id).ok_or_else(|| {
        R::error(cx, "__buf_slice: invalid buffer handle".to_string(), span)
    })?;
    let len = buf.len() as i64;
    if start < 0 || end < start || end > len {
        return Err(R::error(
            cx,
            format!("__buf_slice: bad range {start}..{end} for len {len}"),
            span,
        ));
    }
    // Byte slicing must not split a UTF-8 codepoint.
    let is_continuation = |b: u8| b & 0xc0 == 0x80;
    if (start > 0 && start < len && is_continuation(buf[start as usize]))
        || (end > 0 && end < len && is_continuation(buf[end as usize]))
    {
        return Err(R::error(
            cx,
            "__buf_slice: range splits a UTF-8 codepoint".to_string(),
            span,
        ));
    }
    Ok(String::from_utf8_lossy(&buf[start as usize..end as usize]).into_owned())
}

#[native_fn(module = "core", bound = "BufStore", sig(handle(Buffer) -> null))]
pub fn __buf_clear<R: BufStore>(
    cx: &mut R::Cx,
    handle: R::Value,
    span: R::Span,
) -> Result<R::Value, Error> {
    let id = buf_lookup::<R>(cx, &handle, "__buf_clear", span)?;
    if let Some(buf) = R::buf_get_mut(cx, id) {
        buf.clear();
    }
    Ok(R::null())
}

#[native_fn(module = "core", bound = "BufStore", sig(handle(Buffer) -> string))]
pub fn __buf_to_string<R: BufStore>(
    cx: &mut R::Cx,
    handle: R::Value,
    span: R::Span,
) -> Result<String, Error> {
    let id = buf_lookup::<R>(cx, &handle, "__buf_to_string", span)?;
    Ok(R::buf_get(cx, id)
        .map(|b| String::from_utf8_lossy(b).into_owned())
        .unwrap_or_default())
}

#[native_fn(module = "core", bound = "BufStore", sig(handle(Buffer) -> null))]
pub fn __buf_free<R: BufStore>(
    cx: &mut R::Cx,
    handle: R::Value,
    span: R::Span,
) -> Result<R::Value, Error> {
    let id = buf_lookup::<R>(cx, &handle, "__buf_free", span)?;
    match R::buf_remove(cx, id) {
        Some(_) => Ok(R::null()),
        None => Err(R::error(
            cx,
            "__buf_free: invalid buffer handle".to_string(),
            span,
        )),
    }
}

#[native_fn(module = "core", bound = "BufStore", sig(handle(Buffer) -> int))]
pub fn __buf_addr<R: BufStore>(
    cx: &mut R::Cx,
    handle: R::Value,
    span: R::Span,
) -> Result<i64, Error> {
    let id = buf_lookup::<R>(cx, &handle, "__buf_addr", span)?;
    Ok(R::buf_get(cx, id)
        .map(|b| {
            if b.is_empty() {
                0
            } else {
                b.as_ptr() as i64
            }
        })
        .unwrap_or(0))
}

#[native_fn(module = "core", bound = "BufStore", sig(handle(Buffer), int -> null))]
pub fn __buf_resize<R: BufStore>(
    cx: &mut R::Cx,
    handle: R::Value,
    size: i64,
    span: R::Span,
) -> Result<R::Value, Error> {
    let id = buf_lookup::<R>(cx, &handle, "__buf_resize", span)?;
    if size < 0 {
        return Err(R::error(
            cx,
            format!("__buf_resize: negative size {size}"),
            span,
        ));
    }
    if let Some(buf) = R::buf_get_mut(cx, id) {
        buf.resize(size as usize, 0);
    }
    Ok(R::null())
}

// ---- worker threads ---------------------------------------------------------------
// Task parallelism without shared memory: `__spawn` runs RL source on a
// fresh `Vm` on a new OS thread; only strings cross the boundary. Poll
// protocol: `err("pending")` while running, `ok(text)` with the displayed
// final value when done, `err(text)` when the worker failed. Unknown ids
// abort, like bad buffer handles.

#[native_fn(module = "core", bound = "ThreadStore", sig(string -> int))]
pub fn __spawn<R: ThreadStore>(
    cx: &mut R::Cx,
    source: String,
    span: R::Span,
) -> Result<i64, Error> {
    match R::thread_spawn(cx, source) {
        Ok(id) => Ok(id as i64),
        Err(e) => Err(R::error(cx, e, span)),
    }
}

#[native_fn(module = "core", bound = "ThreadStore", sig(string -> null))]
pub fn __emit<R: ThreadStore>(cx: &mut R::Cx, text: String) -> R::Value {
    if R::thread_emit(cx, text) {
        R::null()
    } else {
        R::err(R::from_string("__emit: only a worker thread can emit".to_string()))
    }
}

#[native_fn(module = "core", bound = "ThreadStore", sig(int -> result[string]))]
pub fn __poll<R: ThreadStore>(cx: &mut R::Cx, job: i64) -> R::Value {
    if job < 0 {
        return R::err(R::from_string(format!("__poll: invalid job {job}")));
    }
    match R::thread_poll(cx, job as u64) {
        ThreadPoll::Unknown => R::err(R::from_string(format!("__poll: unknown job {job}"))),
        ThreadPoll::Pending => R::err(R::from_string("pending".to_string())),
        ThreadPoll::Message(text) => R::ok(R::from_string(text)),
        ThreadPoll::Done(Ok(text)) => R::ok(R::from_string(text)),
        ThreadPoll::Done(Err(text)) => R::err(R::from_string(text)),
    }
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

// ---- result assertion (trust, but verify by aborting) ------------------------
// "i am sure this is ok/err, give me its value": no static questions
// beyond the `result[T]` shape, abort loudly on the wrong variant or a
// non-result. Mirrors `Index`/`m[k]`, not `map_get`.

#[native_fn(module = "core", sig(result[T] -> T))]
pub fn __result_ok_value<R: Runtime>(
    cx: &mut R::Cx,
    value: R::Value,
    span: R::Span,
) -> Result<R::Value, Error> {
    if let Some(inner) = R::as_ok_inner(&value) {
        Ok(inner)
    } else if let Some(inner) = R::as_err_inner(&value) {
        Err(R::error(
            cx,
            format!(
                "__result_ok_value: called on err({})",
                R::display(&inner)
            ),
            span,
        ))
    } else {
        Err(R::error(
            cx,
            format!(
                "__result_ok_value: expected result, got {}",
                R::type_name(&value)
            ),
            span,
        ))
    }
}

#[native_fn(module = "core", untyped)]
pub fn __result_err_value<R: Runtime>(
    cx: &mut R::Cx,
    value: R::Value,
    span: R::Span,
) -> Result<R::Value, Error> {
    // untyped like `result_unwrap_err`: the `sig` language binds `T` to
    // the ok payload, but here `T` is the err payload, so the static
    // type stays dynamic (Unknown propagates silently).
    if let Some(inner) = R::as_err_inner(&value) {
        Ok(inner)
    } else if let Some(inner) = R::as_ok_inner(&value) {
        Err(R::error(
            cx,
            format!("__result_err_value: called on ok({})", R::display(&inner)),
            span,
        ))
    } else {
        Err(R::error(
            cx,
            format!(
                "__result_err_value: expected result, got {}",
                R::type_name(&value)
            ),
            span,
        ))
    }
}

// ---- module registration --------------------------------------------------

rl_std_core::native_module!("core";
    bound: ThreadStore;
    funcs: [
        __arr_new, __arr_push, __arr_get, __arr_set, __arr_remove, __arr_len,
        __map_new, __map_get, __map_set, __map_remove, __map_has, __map_keys, __map_len,
        __set_new, __set_add, __set_has, __set_remove, __set_len,
        __str_len, __str_get_byte, __str_slice, __str_concat,
        __buf_new, __buf_len, __buf_push_byte, __buf_get_byte,
        __buf_set_byte, __buf_append, __buf_slice, __buf_clear,
        __buf_to_string, __buf_free, __buf_addr, __buf_resize,
        __spawn, __emit, __poll,
        __syscall6,
        __abort, __type_of,
        __result_ok_value, __result_err_value,
    ],
);
