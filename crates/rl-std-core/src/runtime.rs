//! The [`Runtime`] abstraction: the single trait that lets every stdlib
//! function be written once (generic over `R: Runtime`) and monomorphized into
//! a thin function pointer for the bytecode VM.
//!
//! It carries three associated types:
//! - [`Runtime::Value`] - the runtime's value enum (`VmValue`),
//! - [`Runtime::Cx`] - the mutable context threaded through calls (`Vm`),
//! - [`Runtime::Span`] - the call-site span type: `()` on the VM (which
//!   re-anchors native errors after the fact via `Vm::annotate`).
//!
//! The compound-value constructors ([`Runtime::array`], [`Runtime::map`], ...)
//! all take a [`TypeAnnotation`]: the VM impl discards it. This lets one
//! stdlib source line stay valid without leaking asymmetry into the function
//! bodies.

use crate::rng::Xoshiro256;
use rl_ast::statements::{HandleKind, TypeAnnotation};
use rl_utils::errors::Error;

pub trait Runtime: Sized + 'static {
    /// The runtime's value enum (`VmValue`).
    type Value: Clone;
    /// The mutable context threaded through every native call (`Vm`).
    type Cx;
    /// The call-site span type: `()` on the VM.
    type Span: Copy;

    // ---- error construction (absorbs the Span asymmetry) -------------------

    /// Builds a runtime error. The VM discards `span` and relies on
    /// `Vm::annotate` re-anchoring at the native call site.
    fn error(cx: &Self::Cx, msg: impl Into<String>, span: Self::Span) -> Error;

    // ---- scalar extraction (borrowing / copying, never allocating) ---------

    fn as_i64(v: &Self::Value) -> Option<i64>;
    fn as_u64(v: &Self::Value) -> Option<u64>;
    fn as_i32(v: &Self::Value) -> Option<i32>;
    fn as_u32(v: &Self::Value) -> Option<u32>;
    fn as_i16(v: &Self::Value) -> Option<i16>;
    fn as_u16(v: &Self::Value) -> Option<u16>;
    fn as_i8(v: &Self::Value) -> Option<i8>;
    fn as_u8(v: &Self::Value) -> Option<u8>;
    fn as_f64(v: &Self::Value) -> Option<f64>;
    fn as_f32(v: &Self::Value) -> Option<f32>;
    fn as_bool(v: &Self::Value) -> Option<bool>;
    fn as_char(v: &Self::Value) -> Option<char>;
    fn as_str(v: &Self::Value) -> Option<&str>;

    /// A human-readable type name for error messages (`"int"`, `"string"`, ...).
    fn type_name(v: &Self::Value) -> &'static str;

    /// The value's `Display` rendering (used by `str::concat`/`format`/`join`).
    fn display(v: &Self::Value) -> String;

    /// Whether the value is a callable (user function, native, or closure).
    fn is_callable(v: &Self::Value) -> bool;

    // ---- scalar construction ----------------------------------------------

    fn from_i64(x: i64) -> Self::Value;
    fn from_u64(x: u64) -> Self::Value;
    fn from_i32(x: i32) -> Self::Value;
    fn from_u32(x: u32) -> Self::Value;
    fn from_i16(x: i16) -> Self::Value;
    fn from_u16(x: u16) -> Self::Value;
    fn from_i8(x: i8) -> Self::Value;
    fn from_u8(x: u8) -> Self::Value;
    fn from_f64(x: f64) -> Self::Value;
    fn from_f32(x: f32) -> Self::Value;
    fn from_bool(x: bool) -> Self::Value;
    fn from_char(x: char) -> Self::Value;
    fn from_string(x: String) -> Self::Value;
    fn null() -> Self::Value;

    // ---- result wrappers (language-level Ok/Err/Error values) --------------

    fn ok(v: Self::Value) -> Self::Value;
    fn err(v: Self::Value) -> Self::Value;
    fn error_value(v: Self::Value) -> Self::Value;

    // ---- compound values (the TypeAnnotation is ignored by the VM) --------

    fn array(items: Vec<Self::Value>, elem: TypeAnnotation) -> Self::Value;
    fn tuple(items: Vec<Self::Value>) -> Self::Value;
    fn map(
        entries: Vec<(Self::Value, Self::Value)>,
        key: TypeAnnotation,
        val: TypeAnnotation,
    ) -> Self::Value;
    fn set(items: Vec<Self::Value>, elem: TypeAnnotation) -> Self::Value;

    /// Deconstructs an array into `(items, element_type)`. The VM returns
    /// [`TypeAnnotation::Infer`] for the element type. Functions that do not
    /// care ignore the second field.
    fn as_array(v: &Self::Value) -> Option<(&[Self::Value], TypeAnnotation)>;

    /// The elements of a tuple value, if `v` is a tuple.
    fn as_tuple(v: &Self::Value) -> Option<&[Self::Value]>;

    /// The value wrapped inside an `Ok(..)`, if any.
    fn as_ok_inner(v: &Self::Value) -> Option<Self::Value>;
    /// The value wrapped inside an `Err(..)`, if any.
    fn as_err_inner(v: &Self::Value) -> Option<Self::Value>;
    /// The value wrapped inside an `error(..)`, if any.
    fn as_error_inner(v: &Self::Value) -> Option<Self::Value>;

    /// A set's elements (as values) and element type, if `v` is a set.
    fn as_set(v: &Self::Value) -> Option<(Vec<Self::Value>, TypeAnnotation)>;
    /// A map's `(key, value)` entries and key/value types, if `v` is a map.
    #[allow(clippy::type_complexity)]
    fn as_map(
        v: &Self::Value,
    ) -> Option<(
        Vec<(Self::Value, Self::Value)>,
        TypeAnnotation,
        TypeAnnotation,
    )>;
    /// Whether `v` can be used as a set element / map key (a hashable scalar).
    fn is_valid_key(v: &Self::Value) -> bool;
    /// Whether two values are equal as set elements / map keys.
    fn keys_equal(a: &Self::Value, b: &Self::Value) -> bool;

    // ---- in-place container access (O(1) hash ops, matching the old
    //      per-runtime implementations) ---------------------------------------
    //
    // The value-level `as_set`/`as_map` accessors above return owned copies;
    // they are only used where a function genuinely needs the whole container
    // (`set_to_array`, `map_to_array`, `map_keys`, `map_values`). These
    // accessors read/mutate the shared set/map in place and are used by the
    // hot container operations (`set_add`, `set_contains`, `set_len`,
    // `map_merge`, `map_get`, ...), so they stay O(1) hash operations instead
    // of copying and rebuilding the whole container.

    /// Inserts `item` into set `v`. `Some(true)` if newly inserted,
    /// `Some(false)` if already present, `None` if `v` is not a set or `item`
    /// is not a valid set element.
    fn set_insert(v: &Self::Value, item: &Self::Value) -> Option<bool>;
    /// Removes `item` from set `v`. `Some(true)` if it was present,
    /// `Some(false)` otherwise, `None` if `v` is not a set or `item` is not a
    /// valid set element.
    fn set_remove(v: &Self::Value, item: &Self::Value) -> Option<bool>;
    /// Whether set `v` contains `item`. `None` if `v` is not a set or `item`
    /// is not a valid set element.
    fn set_contains(v: &Self::Value, item: &Self::Value) -> Option<bool>;
    /// The element count of set `v`. `None` if `v` is not a set.
    fn set_len(v: &Self::Value) -> Option<usize>;
    /// The declared element type of set `v`. `None` if `v` is not a set.
    fn set_element_type(v: &Self::Value) -> Option<TypeAnnotation>;

    /// Inserts `(key, value)` into map `v`. Returns `false` if `v` is not a
    /// map (a call with an invalid `key` is silently ignored - callers guard
    /// with [`Runtime::is_valid_key`]).
    fn map_insert(v: &Self::Value, key: &Self::Value, value: &Self::Value) -> bool;
    /// The value stored under `key` in map `v`: `Some(Some(v))` if present,
    /// `Some(None)` if absent, `None` if `v` is not a map or `key` is not a
    /// valid map key.
    fn map_get(v: &Self::Value, key: &Self::Value) -> Option<Option<Self::Value>>;
    /// Removes `key` from map `v`: `Some(Some(v))` if a value was removed,
    /// `Some(None)` if absent, `None` if `v` is not a map or `key` is not a
    /// valid map key.
    fn map_remove(v: &Self::Value, key: &Self::Value) -> Option<Option<Self::Value>>;
    /// Whether map `v` contains `key`. `None` if `v` is not a map or `key` is
    /// not a valid map key.
    fn map_contains(v: &Self::Value, key: &Self::Value) -> Option<bool>;
    /// The entry count of map `v`. `None` if `v` is not a map.
    fn map_len(v: &Self::Value) -> Option<usize>;
    /// The declared key/value types of map `v`. `None` if `v` is not a map.
    fn map_key_value_types(v: &Self::Value) -> Option<(TypeAnnotation, TypeAnnotation)>;
    /// Clears map `v` in place. Returns `false` if `v` is not a map.
    fn map_clear(v: &Self::Value) -> bool;
    /// Visits each `(key, value)` entry of map `v` (owned copies, arbitrary
    /// order), invoking `f` for each. Returns `false` if `v` is not a map.
    fn map_for_each<F: FnMut(Self::Value, Self::Value)>(v: &Self::Value, f: F) -> bool;
    /// Whole-value structural equality (used by `debug::assert_eq`).
    fn values_equal(a: &Self::Value, b: &Self::Value) -> bool;

    /// The inferred type annotation of a value. The VM does not track element
    /// types and returns [`TypeAnnotation::Infer`].
    fn value_type(v: &Self::Value) -> TypeAnnotation;

    /// Whether a value of type `actual` may be stored where `expected` is
    /// required. The VM performs no such check and always returns `true`.
    fn types_compatible(actual: &TypeAnnotation, expected: &TypeAnnotation) -> bool;

    // ---- re-entrancy (higher-order functions: arr_map/filter/sort) ---------

    /// Invokes a callable value with `args`. Normalizes the VM's `call_value`
    /// signature.
    fn call_value(
        cx: &mut Self::Cx,
        callee: &Self::Value,
        args: &[Self::Value],
        span: Self::Span,
    ) -> Result<Self::Value, Error>;

    /// The declared return type of a callable, when the runtime tracks it
    /// (the VM returns `None`, so return-type checks in higher-order
    /// functions are simply skipped there).
    fn callable_return_type(v: &Self::Value) -> Option<TypeAnnotation>;

    // ---- shared context state ----------------------------------------------

    fn rng(cx: &mut Self::Cx) -> &mut Xoshiro256;
    fn output_buffer(cx: &mut Self::Cx) -> &mut Option<String>;

    /// The `std::test` registry (cases, grouping, results). Each runtime
    /// context owns one, so test runs stay isolated per `Vm`.
    fn test_state(cx: &mut Self::Cx) -> &mut crate::TestState<Self::Value>;

    /// The handle id inside `v`, if it is a `Handle` of the given `kind`.
    fn as_handle(v: &Self::Value, kind: HandleKind) -> Option<u64>;
    /// Builds an opaque resource handle value of the given `kind`.
    fn make_handle(kind: HandleKind, id: u64) -> Self::Value;

    /// How many leading `std::env::args()` entries to skip for `process::args`
    /// (the runtime's own argv prefix). Defaults to 1; the CLI can override it.
    fn user_args_offset(cx: &Self::Cx) -> usize;
}

/// Per-domain access to a runtime's handle table (net / c / http / audio /
/// gui). One trait per domain keeps a stdlib module (e.g. `net`) from pulling
/// in another domain's dependencies. `Domain` is a zero-sized marker type;
/// `Handle` is the (shared, `rl-std`-defined) resource enum for that domain.
pub trait HandleStore<Domain>: Runtime {
    type Handle;

    /// Inserts a handle and returns its fresh id.
    fn insert(cx: &mut Self::Cx, handle: Self::Handle) -> u64;
    /// Borrows a handle by id.
    fn get(cx: &Self::Cx, id: u64) -> Option<&Self::Handle>;
    /// Mutably borrows a handle by id.
    fn get_mut(cx: &mut Self::Cx, id: u64) -> Option<&mut Self::Handle>;
    /// Removes a handle by id, returning it if present.
    fn remove(cx: &mut Self::Cx, id: u64) -> Option<Self::Handle>;
    /// Builds a handle value (`Value::Handle { kind, id }`) for this domain.
    fn handle_value(id: u64) -> Self::Value;
}
