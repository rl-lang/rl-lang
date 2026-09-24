//! `std::collections` - functions for working with `set[T]` and `map[K, V]`.
//!
//! The rl module name is `collections`; the Rust module is `collections`.
//! Ported once from the former per-runtime `stdlib/collections/*.rs` copies.
//!
//! These functions are value-polymorphic over the element/key/value types, so
//! they take raw `R::Value` arguments and use dedicated `Runtime` accessors to
//! read and rebuild sets/maps. Their explicit `sig(...)` overloads mirror
//! `rl-commons/src/stdlib_signatures/collections.rs` (`set[T]`, `map[K, V]`).
//!
//! Every function returns a language `result[T]` value (`ok(..)` / `err(..)`),
//! matching the old `vok!` / `verr!` bodies. The hot mutating/query operations
//! (`set_add`, `set_contains`, `set_len`, `map_merge`, `map_get`, ...) use the
//! `Runtime` in-place accessors (O(1) hash ops, mutating the shared set/map),
//! restoring the behaviour of the old per-runtime implementations. The
//! whole-container accessors (`as_set` / `as_map`) are only used where a
//! function genuinely needs every element (`set_to_array`, `map_to_array`,
//! `map_keys`, `map_values`).

use rl_ast::statements::TypeAnnotation;
use rl_std_core::Runtime;
use rl_std_macros::native_fn;
use std::rc::Rc;

// ---- sets -----------------------------------------------------------------

#[native_fn(module = "collections", sig(set[T], T -> result[set[T]]))]
pub fn set_add<R: Runtime>(set: R::Value, value: R::Value) -> R::Value {
    let Some(elem) = R::set_element_type(&set) else {
        return R::err(R::from_string(format!(
            "set_add: accepts only sets, found {}",
            R::type_name(&set)
        )));
    };
    // Interpreter-only element-type check (a no-op on the VM, whose sets carry
    // no tracked element type).
    let val_type = R::value_type(&value);
    if !R::types_compatible(&val_type, &elem) {
        return R::err(R::from_string(format!(
            "set_add: type mismatch: set expects {elem:?}, cannot add {val_type:?}"
        )));
    }
    if !R::is_valid_key(&value) {
        return R::err(R::from_string(format!(
            "set_add: cannot add {} to a set",
            R::type_name(&value)
        )));
    }
    R::set_insert(&set, &value);
    R::ok(set)
}

#[native_fn(module = "collections", sig(set[T], T -> result[set[T]]))]
pub fn set_remove<R: Runtime>(set: R::Value, value: R::Value) -> R::Value {
    if !R::is_valid_key(&value) {
        return R::err(R::from_string(format!(
            "set_remove: cannot remove {} from a set",
            R::type_name(&value)
        )));
    }
    match R::set_remove(&set, &value) {
        Some(_) => R::ok(set),
        None => R::err(R::from_string(format!(
            "set_remove: accepts only sets, found {}",
            R::type_name(&set)
        ))),
    }
}

#[native_fn(module = "collections", sig(set[T], T -> result[bool]))]
pub fn set_contains<R: Runtime>(set: R::Value, value: R::Value) -> R::Value {
    if !R::is_valid_key(&value) {
        return R::ok(R::from_bool(false));
    }
    match R::set_contains(&set, &value) {
        Some(found) => R::ok(R::from_bool(found)),
        None => R::err(R::from_string(format!(
            "set_contains: accepts only sets, found {}",
            R::type_name(&set)
        ))),
    }
}

#[native_fn(module = "collections", sig(set[T] -> result[int]))]
pub fn set_len<R: Runtime>(set: R::Value) -> R::Value {
    match R::set_len(&set) {
        Some(len) => R::ok(R::from_i64(len as i64)),
        None => R::err(R::from_string(format!(
            "set_len: accepts only sets, found {}",
            R::type_name(&set)
        ))),
    }
}

#[native_fn(module = "collections", sig(set[T] -> result[bool]))]
pub fn set_is_empty<R: Runtime>(set: R::Value) -> R::Value {
    match R::set_len(&set) {
        Some(len) => R::ok(R::from_bool(len == 0)),
        None => R::err(R::from_string(format!(
            "set_is_empty: accepts only sets, found {}",
            R::type_name(&set)
        ))),
    }
}

#[native_fn(module = "collections", sig(set[T] -> result[array[T]]))]
pub fn set_to_array<R: Runtime>(set: R::Value) -> R::Value {
    match R::as_set(&set) {
        Some((items, elem)) => R::ok(R::array(items, elem)),
        None => R::err(R::from_string(format!(
            "set_to_array: accepts only sets, found {}",
            R::type_name(&set)
        ))),
    }
}

// ---- maps -----------------------------------------------------------------

#[native_fn(module = "collections", sig(map[K, V], K -> result[bool]))]
pub fn map_contains<R: Runtime>(map: R::Value, key: R::Value) -> R::Value {
    if !R::is_valid_key(&key) {
        return R::ok(R::from_bool(false));
    }
    match R::map_contains(&map, &key) {
        Some(found) => R::ok(R::from_bool(found)),
        None => R::err(R::from_string(format!(
            "map_contains: accepts only maps, found {}",
            R::type_name(&map)
        ))),
    }
}

#[native_fn(module = "collections", sig(map[K, V], K -> result[map[K, V]]))]
pub fn map_remove<R: Runtime>(map: R::Value, key: R::Value) -> R::Value {
    if !R::is_valid_key(&key) {
        return R::err(R::from_string(format!(
            "map_remove: cannot remove {} from a map",
            R::type_name(&key)
        )));
    }
    match R::map_remove(&map, &key) {
        Some(_) => R::ok(map),
        None => R::err(R::from_string(format!(
            "map_remove: accepts only maps, found {}",
            R::type_name(&map)
        ))),
    }
}

#[native_fn(module = "collections", sig(map[K, V] -> result[int]))]
pub fn map_len<R: Runtime>(map: R::Value) -> R::Value {
    match R::map_len(&map) {
        Some(len) => R::ok(R::from_i64(len as i64)),
        None => R::err(R::from_string(format!(
            "map_len: accepts only maps, found {}",
            R::type_name(&map)
        ))),
    }
}

#[native_fn(module = "collections", sig(map[K, V] -> result[bool]))]
pub fn map_is_empty<R: Runtime>(map: R::Value) -> R::Value {
    match R::map_len(&map) {
        Some(len) => R::ok(R::from_bool(len == 0)),
        None => R::err(R::from_string(format!(
            "map_is_empty: accepts only maps, found {}",
            R::type_name(&map)
        ))),
    }
}

#[native_fn(module = "collections", sig(map[K, V] -> result[array[tuple[K, V]]]))]
pub fn map_to_array<R: Runtime>(map: R::Value) -> R::Value {
    match R::as_map(&map) {
        Some((entries, key_ty, val_ty)) => {
            let items: Vec<R::Value> = entries
                .into_iter()
                .map(|(k, v)| R::tuple(vec![k, v]))
                .collect();
            let elem = TypeAnnotation::Tuple(Rc::new(vec![key_ty, val_ty]));
            R::ok(R::array(items, elem))
        }
        None => R::err(R::from_string(format!(
            "map_to_array: accepts only maps, found {}",
            R::type_name(&map)
        ))),
    }
}

#[native_fn(module = "collections", sig(map[K, V], K -> result[V]))]
pub fn map_get<R: Runtime>(map: R::Value, key: R::Value) -> R::Value {
    if !R::is_valid_key(&key) {
        return R::err(R::from_string(format!(
            "map_get: cannot use {} as a map key",
            R::type_name(&key)
        )));
    }
    match R::map_get(&map, &key) {
        Some(Some(value)) => R::ok(value),
        Some(None) => R::err(R::from_string(format!(
            "map_get: key {} not found in map",
            R::display(&key)
        ))),
        None => R::err(R::from_string(format!(
            "map_get: accepts only maps, found {}",
            R::type_name(&map)
        ))),
    }
}

#[native_fn(module = "collections", sig(map[K, V] -> result[array[K]]))]
pub fn map_keys<R: Runtime>(map: R::Value) -> R::Value {
    match R::as_map(&map) {
        Some((entries, key_ty, _)) => {
            let items: Vec<R::Value> = entries.into_iter().map(|(k, _)| k).collect();
            R::ok(R::array(items, key_ty))
        }
        None => R::err(R::from_string(format!(
            "map_keys: accepts only maps, found {}",
            R::type_name(&map)
        ))),
    }
}

#[native_fn(module = "collections", sig(map[K, V] -> result[array[V]]))]
pub fn map_values<R: Runtime>(map: R::Value) -> R::Value {
    match R::as_map(&map) {
        Some((entries, _, val_ty)) => {
            let items: Vec<R::Value> = entries.into_iter().map(|(_, v)| v).collect();
            R::ok(R::array(items, val_ty))
        }
        None => R::err(R::from_string(format!(
            "map_values: accepts only maps, found {}",
            R::type_name(&map)
        ))),
    }
}

#[native_fn(module = "collections", sig(map[K, V] -> result[map[K, V]]))]
pub fn map_clear<R: Runtime>(map: R::Value) -> R::Value {
    if R::map_clear(&map) {
        R::ok(map)
    } else {
        R::err(R::from_string(format!(
            "map_clear: accepts only maps, found {}",
            R::type_name(&map)
        )))
    }
}

#[native_fn(module = "collections", sig(map[K, V], map[K, V] -> result[map[K, V]]))]
pub fn map_merge<R: Runtime>(map1: R::Value, map2: R::Value) -> R::Value {
    if !R::map_for_each(&map2, |k, v| {
        R::map_insert(&map1, &k, &v);
    }) {
        return R::err(R::from_string(format!(
            "map_merge: accepts only maps, found {}",
            R::type_name(&map1)
        )));
    }
    R::ok(map1)
}

// ---- set binary operations ------------------------------------------------

#[native_fn(module = "collections", sig(set[T], set[T] -> result[set[T]]))]
pub fn set_union<R: Runtime>(a: R::Value, b: R::Value) -> R::Value {
    let Some((items_a, elem)) = R::as_set(&a) else {
        return R::err(R::from_string(format!(
            "set_union: accepts only sets, found {}",
            R::type_name(&a)
        )));
    };
    let Some((items_b, _)) = R::as_set(&b) else {
        return R::err(R::from_string(format!(
            "set_union: accepts only sets, found {}",
            R::type_name(&b)
        )));
    };
    let mut result = items_a;
    for item in items_b {
        if !result.iter().any(|s| R::keys_equal(s, &item)) {
            result.push(item);
        }
    }
    R::ok(R::set(result, elem))
}

#[native_fn(module = "collections", sig(set[T], set[T] -> result[set[T]]))]
pub fn set_intersection<R: Runtime>(a: R::Value, b: R::Value) -> R::Value {
    let Some((items_a, elem)) = R::as_set(&a) else {
        return R::err(R::from_string(format!(
            "set_intersection: accepts only sets, found {}",
            R::type_name(&a)
        )));
    };
    let Some((items_b, _)) = R::as_set(&b) else {
        return R::err(R::from_string(format!(
            "set_intersection: accepts only sets, found {}",
            R::type_name(&b)
        )));
    };
    let result: Vec<R::Value> = items_a
        .into_iter()
        .filter(|item| items_b.iter().any(|s| R::keys_equal(s, item)))
        .collect();
    R::ok(R::set(result, elem))
}

#[native_fn(module = "collections", sig(set[T], set[T] -> result[set[T]]))]
pub fn set_difference<R: Runtime>(a: R::Value, b: R::Value) -> R::Value {
    let Some((items_a, elem)) = R::as_set(&a) else {
        return R::err(R::from_string(format!(
            "set_difference: accepts only sets, found {}",
            R::type_name(&a)
        )));
    };
    let Some((items_b, _)) = R::as_set(&b) else {
        return R::err(R::from_string(format!(
            "set_difference: accepts only sets, found {}",
            R::type_name(&b)
        )));
    };
    let result: Vec<R::Value> = items_a
        .into_iter()
        .filter(|item| !items_b.iter().any(|s| R::keys_equal(s, item)))
        .collect();
    R::ok(R::set(result, elem))
}

#[native_fn(module = "collections", sig(set[T], set[T] -> result[set[T]]))]
pub fn set_symmetric_difference<R: Runtime>(a: R::Value, b: R::Value) -> R::Value {
    let Some((items_a, elem)) = R::as_set(&a) else {
        return R::err(R::from_string(format!(
            "set_symmetric_difference: accepts only sets, found {}",
            R::type_name(&a)
        )));
    };
    let Some((items_b, _)) = R::as_set(&b) else {
        return R::err(R::from_string(format!(
            "set_symmetric_difference: accepts only sets, found {}",
            R::type_name(&b)
        )));
    };
    let mut result: Vec<R::Value> = Vec::new();
    for item in &items_a {
        if !items_b.iter().any(|s| R::keys_equal(s, item)) {
            result.push(item.clone());
        }
    }
    for item in &items_b {
        if !items_a.iter().any(|s| R::keys_equal(s, item)) {
            result.push(item.clone());
        }
    }
    R::ok(R::set(result, elem))
}

#[native_fn(module = "collections", sig(set[T], set[T] -> result[bool]))]
pub fn set_is_subset<R: Runtime>(a: R::Value, b: R::Value) -> R::Value {
    let Some((items_a, _)) = R::as_set(&a) else {
        return R::err(R::from_string(format!(
            "set_is_subset: accepts only sets, found {}",
            R::type_name(&a)
        )));
    };
    let Some((items_b, _)) = R::as_set(&b) else {
        return R::err(R::from_string(format!(
            "set_is_subset: accepts only sets, found {}",
            R::type_name(&b)
        )));
    };
    let is_subset = items_a
        .iter()
        .all(|item| items_b.iter().any(|s| R::keys_equal(s, item)));
    R::ok(R::from_bool(is_subset))
}

#[native_fn(module = "collections", sig(set[T], set[T] -> result[bool]))]
pub fn set_is_superset<R: Runtime>(a: R::Value, b: R::Value) -> R::Value {
    set_is_subset::<R>(b, a)
}

// ---- map accessors with defaults ------------------------------------------

#[native_fn(module = "collections", sig(map[K, V], K, V -> result[V]))]
pub fn map_get_or<R: Runtime>(map: R::Value, key: R::Value, default: R::Value) -> R::Value {
    if !R::is_valid_key(&key) {
        return R::err(R::from_string(format!(
            "map_get_or: cannot use {} as a map key",
            R::type_name(&key)
        )));
    }
    match R::map_get(&map, &key) {
        Some(Some(value)) => R::ok(value),
        Some(None) => R::ok(default),
        None => R::err(R::from_string(format!(
            "map_get_or: accepts only maps, found {}",
            R::type_name(&map)
        ))),
    }
}

#[native_fn(module = "collections", sig(map[K, V], K, V -> result[V]))]
pub fn map_get_or_insert<R: Runtime>(map: R::Value, key: R::Value, default: R::Value) -> R::Value {
    if !R::is_valid_key(&key) {
        return R::err(R::from_string(format!(
            "map_get_or_insert: cannot use {} as a map key",
            R::type_name(&key)
        )));
    }
    match R::map_get(&map, &key) {
        Some(Some(value)) => R::ok(value),
        Some(None) => {
            R::map_insert(&map, &key, &default);
            R::ok(default)
        }
        None => R::err(R::from_string(format!(
            "map_get_or_insert: accepts only maps, found {}",
            R::type_name(&map)
        ))),
    }
}

// ---- heap operations (return new arrays, preserving element type) ----------

fn sift_up<R: Runtime>(items: &mut [R::Value], idx: usize) {
    let mut i = idx;
    while i > 0 {
        let parent = (i - 1) / 2;
        let should_swap = match (R::as_i64(&items[i]), R::as_i64(&items[parent])) {
            (Some(a), Some(b)) => a < b,
            _ => match (R::as_f64(&items[i]), R::as_f64(&items[parent])) {
                (Some(a), Some(b)) => a < b,
                _ => false,
            },
        };
        if should_swap {
            items.swap(i, parent);
            i = parent;
        } else {
            break;
        }
    }
}

fn sift_down<R: Runtime>(items: &mut [R::Value], idx: usize) {
    let len = items.len();
    let mut i = idx;
    loop {
        let left = 2 * i + 1;
        let right = 2 * i + 2;
        let mut smallest = i;

        if left < len {
            let should_swap = match (R::as_i64(&items[left]), R::as_i64(&items[smallest])) {
                (Some(a), Some(b)) => a < b,
                _ => match (R::as_f64(&items[left]), R::as_f64(&items[smallest])) {
                    (Some(a), Some(b)) => a < b,
                    _ => false,
                },
            };
            if should_swap {
                smallest = left;
            }
        }
        if right < len {
            let should_swap = match (R::as_i64(&items[right]), R::as_i64(&items[smallest])) {
                (Some(a), Some(b)) => a < b,
                _ => match (R::as_f64(&items[right]), R::as_f64(&items[smallest])) {
                    (Some(a), Some(b)) => a < b,
                    _ => false,
                },
            };
            if should_swap {
                smallest = right;
            }
        }
        if smallest != i {
            items.swap(i, smallest);
            i = smallest;
        } else {
            break;
        }
    }
}

#[native_fn(module = "collections", sig(array[T], T -> result[array[T]]))]
pub fn heap_push<R: Runtime>(array: R::Value, value: R::Value) -> R::Value {
    let Some((slice, elem)) = R::as_array(&array) else {
        return R::err(R::from_string(format!(
            "heap_push: accepts only arrays, found {}",
            R::type_name(&array)
        )));
    };
    let mut items = slice.to_vec();
    items.push(value);
    let len = items.len();
    sift_up::<R>(&mut items, len - 1);
    R::ok(R::array(items, elem))
}

#[native_fn(module = "collections", sig(array[T] -> result[array[T]]))]
pub fn heap_pop<R: Runtime>(array: R::Value) -> R::Value {
    let Some((slice, elem)) = R::as_array(&array) else {
        return R::err(R::from_string(format!(
            "heap_pop: accepts only arrays, found {}",
            R::type_name(&array)
        )));
    };
    if slice.is_empty() {
        return R::err(R::from_string("heap_pop: called on empty array".to_string()));
    }
    let mut items = slice.to_vec();
    let last = items.pop().unwrap();
    if !items.is_empty() {
        items[0] = last;
        sift_down::<R>(&mut items, 0);
    }
    R::ok(R::array(items, elem))
}

#[native_fn(module = "collections", sig(array[T] -> result[T]))]
pub fn heap_peek<R: Runtime>(array: R::Value) -> R::Value {
    let Some((slice, _)) = R::as_array(&array) else {
        return R::err(R::from_string(format!(
            "heap_peek: accepts only arrays, found {}",
            R::type_name(&array)
        )));
    };
    match slice.first() {
        Some(v) => R::ok(v.clone()),
        None => R::err(R::from_string(
            "heap_peek: called on empty array".to_string(),
        )),
    }
}

// ---- deque operations (return new arrays) ---------------------------------

#[native_fn(module = "collections", sig(array[T], T -> result[array[T]]))]
pub fn deque_push_front<R: Runtime>(array: R::Value, value: R::Value) -> R::Value {
    let Some((slice, elem)) = R::as_array(&array) else {
        return R::err(R::from_string(format!(
            "deque_push_front: accepts only arrays, found {}",
            R::type_name(&array)
        )));
    };
    let mut items = slice.to_vec();
    items.insert(0, value);
    R::ok(R::array(items, elem))
}

#[native_fn(module = "collections", sig(array[T] -> result[array[T]]))]
pub fn deque_pop_front<R: Runtime>(array: R::Value) -> R::Value {
    let Some((slice, elem)) = R::as_array(&array) else {
        return R::err(R::from_string(format!(
            "deque_pop_front: accepts only arrays, found {}",
            R::type_name(&array)
        )));
    };
    if slice.is_empty() {
        return R::err(R::from_string(
            "deque_pop_front: called on empty array".to_string(),
        ));
    }
    let mut items = slice.to_vec();
    items.remove(0);
    R::ok(R::array(items, elem))
}

// ---- bisect operations (on sorted arrays) ---------------------------------

#[native_fn(module = "collections", sig(array[int], int -> result[int]))]
pub fn bisect_left<R: Runtime>(array: R::Value, value: i64) -> R::Value {
    let Some((slice, _)) = R::as_array(&array) else {
        return R::err(R::from_string(format!(
            "bisect_left: accepts only arrays, found {}",
            R::type_name(&array)
        )));
    };
    let mut lo = 0usize;
    let mut hi = slice.len();
    while lo < hi {
        let mid = lo + (hi - lo) / 2;
        match R::as_i64(&slice[mid]) {
            Some(v) if v < value => lo = mid + 1,
            _ => hi = mid,
        }
    }
    R::ok(R::from_i64(lo as i64))
}

#[native_fn(module = "collections", sig(array[int], int -> result[int]))]
pub fn bisect_right<R: Runtime>(array: R::Value, value: i64) -> R::Value {
    let Some((slice, _)) = R::as_array(&array) else {
        return R::err(R::from_string(format!(
            "bisect_right: accepts only arrays, found {}",
            R::type_name(&array)
        )));
    };
    let mut lo = 0usize;
    let mut hi = slice.len();
    while lo < hi {
        let mid = lo + (hi - lo) / 2;
        match R::as_i64(&slice[mid]) {
            Some(v) if v <= value => lo = mid + 1,
            _ => hi = mid,
        }
    }
    R::ok(R::from_i64(lo as i64))
}

#[native_fn(module = "collections", sig(array[int], int -> result[array[int]]))]
pub fn sorted_insert<R: Runtime>(array: R::Value, value: i64) -> R::Value {
    let Some((slice, elem)) = R::as_array(&array) else {
        return R::err(R::from_string(format!(
            "sorted_insert: accepts only arrays, found {}",
            R::type_name(&array)
        )));
    };
    let mut items = slice.to_vec();
    let pos = match items.iter().position(|v| {
        R::as_i64(v)
            .map(|x| x > value)
            .unwrap_or(false)
    }) {
        Some(p) => p,
        None => items.len(),
    };
    items.insert(pos, R::from_i64(value));
    R::ok(R::array(items, elem))
}

rl_std_core::native_module!("collections";
    funcs: [
        set_add, set_remove, set_contains, set_len, set_is_empty, set_to_array,
        set_union, set_intersection, set_difference, set_symmetric_difference,
        set_is_subset, set_is_superset,
        map_contains, map_remove, map_len, map_is_empty, map_to_array, map_get,
        map_keys, map_values, map_clear, map_merge,
        map_get_or, map_get_or_insert,
        heap_push, heap_pop, heap_peek,
        deque_push_front, deque_pop_front,
        bisect_left, bisect_right, sorted_insert,
    ],
);
