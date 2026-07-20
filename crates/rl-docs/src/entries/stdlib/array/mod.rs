use crate::entry::{FnEntry, StdEntry};

pub static ARRAY: StdEntry = StdEntry {
    name: "array",
    description: "functions for array manipulation",
    functions: FUNCTIONS,
    since: Some("v0.1.5"),
    unstable: false,
};

static FUNCTIONS: &[&FnEntry] = &[
    &ARR_CONCAT,
    &ARR_CONTAINS,
    &ARR_COUNT,
    &ARR_FILL,
    &ARR_FIRST,
    &ARR_FLATTEN,
    &ARR_INDEX_OF,
    &ARR_INSERT,
    &ARR_IS_EMPTY,
    &ARR_LAST,
    &ARR_MAX,
    &ARR_MIN,
    &ARR_POP,
    &ARR_PRODUCT,
    &ARR_PUSH,
    &ARR_RANGE,
    &ARR_REMOVE,
    &ARR_REVERSE,
    &ARR_SLICE,
    &ARR_SORT,
    &ARR_SUM,
    &ARR_UNIQUE,
    &LEN,
    &ARR_ALL,
    &ARR_ANY,
    &ARR_FILTER,
    &ARR_FIND,
    &ARR_FIND_INDEX,
    &ARR_FLAT_MAP,
    &ARR_FOR_EACH,
    &ARR_MAP,
    &ARR_REDUCE,
    &ARR_SORT_BY,
    &ARR_ZIP,
];

static ARR_CONCAT: FnEntry = FnEntry {
    signature: "arr_concat(arr1, arr2)",
    description: "concatenates two arrays of the same type into one",
    example: "get std::array::arr_concat\n\narr_concat([1, 2], [3, 4])?",
    expected_output: Some("[1, 2, 3, 4]"),
    returns: "result[arr[T]]",
    errors: Some(
        "Will return error on the following:\n\n- `arr1` or `arr2` is not an array\n- `arr1` and `arr2` have different element types",
    ),
    see_also: &["arr_push", "arr_flatten"],
    since: Some("v0.1.5"),
};

static ARR_CONTAINS: FnEntry = FnEntry {
    signature: "arr_contains(arr, value)",
    description: "true if the array contains the given value",
    example: "get std::array::arr_contains\n\narr_contains([1, 2, 3], 2)?",
    expected_output: Some("true"),
    returns: "result[bool]",
    errors: Some("Will return error if `arr` is not an array"),
    see_also: &["arr_index_of", "arr_find"],
    since: Some("v0.1.5"),
};

static ARR_COUNT: FnEntry = FnEntry {
    signature: "arr_count(arr)",
    description: "returns the number of elements in the array",
    example: "get std::array::arr_count\n\narr_count([1, 2, 3])?",
    expected_output: Some("3"),
    returns: "result[int]",
    errors: Some("Will return error if `arr` is not an array"),
    see_also: &["len", "arr_is_empty"],
    since: Some("v0.1.5"),
};

static ARR_FILL: FnEntry = FnEntry {
    signature: "arr_fill(value, count)",
    description: "creates an array filled with value repeated count times",
    example: "get std::array::arr_fill\n\narr_fill(0, 3)",
    expected_output: Some("[0, 0, 0]"),
    returns: "arr[T]",
    errors: Some(
        "Not validated: a negative `count` casts to a very large length rather\nthan erroring, and will attempt a huge allocation instead of failing\ncleanly.",
    ),
    see_also: &["arr_range"],
    since: Some("v0.1.5"),
};

static ARR_FIRST: FnEntry = FnEntry {
    signature: "arr_first(arr)",
    description: "returns the first element of the array",
    example: "get std::array::arr_first\n\narr_first([1, 2, 3])?",
    expected_output: Some("1"),
    returns: "result[T]",
    errors: Some(
        "Will return error on the following:\n\n- `arr` is not an array\n- `arr` is empty",
    ),
    see_also: &["arr_last"],
    since: Some("v0.1.5"),
};

static ARR_FLATTEN: FnEntry = FnEntry {
    signature: "arr_flatten(arr)",
    description: "flattens a nested array into a single array",
    example: "get std::array::arr_flatten\n\narr_flatten([[1, 2], [3, 4]])?",
    expected_output: Some("[1, 2, 3, 4]"),
    returns: "result[arr[T]]",
    errors: Some("Will return error if `arr` is not an array"),
    see_also: &["arr_concat", "arr_flat_map"],
    since: Some("v0.1.5"),
};

static ARR_INDEX_OF: FnEntry = FnEntry {
    signature: "arr_index_of(arr, value)",
    description: "returns the index of the first occurrence of value in the array, or -1 if not found",
    example: "get std::array::arr_index_of\n\narr_index_of([10, 20, 30], 20)?",
    expected_output: Some("1"),
    returns: "result[int]",
    errors: Some("Will return error if `arr` is not an array"),
    see_also: &["arr_contains", "arr_find_index"],
    since: Some("v0.1.5"),
};

static ARR_INSERT: FnEntry = FnEntry {
    signature: "arr_insert(arr, value, index)",
    description: "inserts value at the given index, shifting elements right; index may equal arr_count(arr) to append",
    example: "get std::array::arr_insert\n\narr_insert([1, 3], 2, 1)?",
    expected_output: Some("[1, 2, 3]"),
    returns: "result[arr[T]]",
    errors: Some(
        "Will return error on the following:\n\n- `arr` is not an array\n- `index` is negative or greater than `arr_count(arr)`\n- `value`'s type does not match `arr`'s element type",
    ),
    see_also: &["arr_push", "arr_remove"],
    since: Some("v0.1.5"),
};

static ARR_IS_EMPTY: FnEntry = FnEntry {
    signature: "arr_is_empty(arr)",
    description: "true if the array has no elements",
    example: "get std::array::arr_is_empty\n\narr_is_empty([])?",
    expected_output: Some("true"),
    returns: "result[bool]",
    errors: Some("Will return error if `arr` is not an array"),
    see_also: &["arr_count"],
    since: Some("v0.1.5"),
};

static ARR_LAST: FnEntry = FnEntry {
    signature: "arr_last(arr)",
    description: "returns the last element of the array",
    example: "get std::array::arr_last\n\narr_last([1, 2, 3])?",
    expected_output: Some("3"),
    returns: "result[T]",
    errors: Some(
        "Will return error on the following:\n\n- `arr` is not an array\n- `arr` is empty",
    ),
    see_also: &["arr_first"],
    since: Some("v0.1.5"),
};

static ARR_MAX: FnEntry = FnEntry {
    signature: "arr_max(arr)",
    description: "returns the largest element in an int or float array",
    example: "get std::array::arr_max\n\narr_max([3, 1, 4, 1, 5])?",
    expected_output: Some("5"),
    returns: "result[int] or result[float]",
    errors: Some(
        "Will return error on the following:\n\n- `arr` is not an array\n- `arr` is not an int or float array\n- `arr` is empty",
    ),
    see_also: &["arr_min", "arr_sort"],
    since: Some("v0.1.5"),
};

static ARR_MIN: FnEntry = FnEntry {
    signature: "arr_min(arr)",
    description: "returns the smallest element in an int or float array",
    example: "get std::array::arr_min\n\narr_min([3, 1, 4, 1, 5])?",
    expected_output: Some("1"),
    returns: "result[int] or result[float]",
    errors: Some(
        "Will return error on the following:\n\n- `arr` is not an array\n- `arr` is not an int or float array\n- `arr` is empty",
    ),
    see_also: &["arr_max", "arr_sort"],
    since: Some("v0.1.5"),
};

static ARR_POP: FnEntry = FnEntry {
    signature: "arr_pop(arr)",
    description: "removes the last element and returns the updated array",
    example: "get std::array::arr_pop\n\narr_pop([1, 2, 3])?",
    expected_output: Some("[1, 2]"),
    returns: "result[arr[T]]",
    errors: Some(
        "Will return error on the following:\n\n- `arr` is not an array\n- `arr` is empty",
    ),
    see_also: &["arr_push", "arr_remove"],
    since: Some("v0.1.5"),
};

static ARR_PRODUCT: FnEntry = FnEntry {
    signature: "arr_product(arr)",
    description: "returns the product of all elements in an int or float array",
    example: "get std::array::arr_product\n\narr_product([1, 2, 3, 4])?",
    expected_output: Some("24"),
    returns: "result[int] or result[float]",
    errors: Some(
        "Will return error on the following:\n\n- `arr` is not an array\n- `arr` is not an int or float array\n\nUnlike `arr_max`/`arr_min`, an empty array is not rejected - it returns\n`1` (int) or `1.0` (float), the empty product.",
    ),
    see_also: &["arr_sum"],
    since: Some("v0.1.5"),
};

static ARR_PUSH: FnEntry = FnEntry {
    signature: "arr_push(arr, value)",
    description: "appends value to the end of the array and returns the updated array",
    example: "get std::array::arr_push\n\narr_push([1, 2], 3)?",
    expected_output: Some("[1, 2, 3]"),
    returns: "result[arr[T]]",
    errors: Some(
        "Will return error on the following:\n\n- `arr` is not an array\n- `value`'s type does not match `arr`'s element type",
    ),
    see_also: &["arr_pop", "arr_insert"],
    since: Some("v0.1.5"),
};

static ARR_RANGE: FnEntry = FnEntry {
    signature: "arr_range(start, end, step)",
    description: "creates an int array from start to end (exclusive) with the given step",
    example: "get std::array::arr_range\n\narr_range(0, 6, 2)?",
    expected_output: Some("[0, 2, 4]"),
    returns: "result[arr[int]]",
    errors: Some("Will return error if `step` is 0 or negative"),
    see_also: &["arr_fill"],
    since: Some("v0.1.5"),
};

static ARR_REMOVE: FnEntry = FnEntry {
    signature: "arr_remove(arr, index)",
    description: "removes the element at the given index and returns the updated array",
    example: "get std::array::arr_remove\n\narr_remove([1, 2, 3], 1)?",
    expected_output: Some("[1, 3]"),
    returns: "result[arr[T]]",
    errors: Some(
        "Will return error on the following:\n\n- `arr` is not an array\n- `index` is out of bounds, or negative",
    ),
    see_also: &["arr_insert", "arr_pop"],
    since: Some("v0.1.5"),
};

static ARR_REVERSE: FnEntry = FnEntry {
    signature: "arr_reverse(arr)",
    description: "reverses the order of elements in the array",
    example: "get std::array::arr_reverse\n\narr_reverse([1, 2, 3])?",
    expected_output: Some("[3, 2, 1]"),
    returns: "result[arr[T]]",
    errors: Some("Will return error if `arr` is not an array"),
    see_also: &["arr_sort"],
    since: Some("v0.1.5"),
};

static ARR_SLICE: FnEntry = FnEntry {
    signature: "arr_slice(arr, start, end)",
    description: "returns a sub-array from start to end (exclusive)",
    example: "get std::array::arr_slice\n\narr_slice([1, 2, 3, 4], 1, 3)?",
    expected_output: Some("[2, 3]"),
    returns: "result[arr[T]]",
    errors: Some(
        "Will return error on the following:\n\n- `arr` is not an array\n- `start` or `end` is out of bounds for `arr` (this also catches\n  negative `start`/`end`, since they cast to a very large index)\n- `start` is greater than `end`",
    ),
    see_also: &["arr_slice"],
    since: Some("v0.1.5"),
};

static ARR_SORT: FnEntry = FnEntry {
    signature: "arr_sort(arr)",
    description: "returns the array sorted in ascending order, only int or float arrays",
    example: "get std::array::arr_sort\n\narr_sort([3, 1, 2])?",
    expected_output: Some("[1, 2, 3]"),
    returns: "result[arr[int]] or result[arr[float]]",
    errors: Some(
        "Will return error on the following:\n\n- `arr` is not an array\n- `arr` is not an int or float array",
    ),
    see_also: &["arr_sort_by", "arr_max", "arr_min"],
    since: Some("v0.1.5"),
};

static ARR_SUM: FnEntry = FnEntry {
    signature: "arr_sum(arr)",
    description: "returns the sum of all elements in an int or float array",
    example: "get std::array::arr_sum\n\narr_sum([1, 2, 3, 4])?",
    expected_output: Some("10"),
    returns: "result[int] or result[float]",
    errors: Some(
        "Will return error on the following:\n\n- `arr` is not an array\n- `arr` is not an int or float array\n\nAn empty array is not rejected - it returns `0` (int) or `0.0` (float).",
    ),
    see_also: &["arr_product"],
    since: Some("v0.1.5"),
};

static ARR_UNIQUE: FnEntry = FnEntry {
    signature: "arr_unique(arr)",
    description: "returns the array with duplicate values removed, preserving order",
    example: "get std::array::arr_unique\n\narr_unique([1, 2, 2, 3, 1])?",
    expected_output: Some("[1, 2, 3]"),
    returns: "result[arr[T]]",
    errors: Some("Will return error if `arr` is not an array"),
    see_also: &[],
    since: Some("v0.1.5"),
};

static LEN: FnEntry = FnEntry {
    signature: "len(x)",
    description: "length of a string, array, or tuple",
    example: "get std::array::len\n\nlen(\"hello\")",
    expected_output: Some("5"),
    returns: "int",
    errors: Some(
        "Will panic at runtime (not a catchable `result[..]` err) if `x` is not a\nstring, array, or tuple.",
    ),
    see_also: &["arr_count", "arr_is_empty"],
    since: Some("v0.1.5"),
};

static ARR_ALL: FnEntry = FnEntry {
    signature: "arr_all(arr, fn)",
    description: "true if every element satisfies the predicate",
    example: "get std::array::arr_all\n\narr_all([2, 4, 6], fn(int x) -> bool { return mod(x, 2) == 0 })?",
    expected_output: Some("true"),
    returns: "result[bool]",
    errors: Some(
        "Will return error on the following:\n\n- `arr` is not an array\n- `fn` is not a function/lambda, or does not declare a `bool` return type\n\nAn error raised inside `fn` while it runs propagates as an\ninterpreter-level runtime error, not a catchable `result[..]` err.",
    ),
    see_also: &["arr_any", "arr_filter"],
    since: Some("v0.1.5"),
};

static ARR_ANY: FnEntry = FnEntry {
    signature: "arr_any(arr, fn)",
    description: "true if at least one element satisfies the predicate",
    example: "get std::array::arr_any\n\narr_any([1, 2, 3], fn(int x) -> bool { return x > 2 })?",
    expected_output: Some("true"),
    returns: "result[bool]",
    errors: Some(
        "Will return error on the following:\n\n- `arr` is not an array\n- `fn` is not a function/lambda, or does not declare a `bool` return type\n\nAn error raised inside `fn` while it runs propagates as an\ninterpreter-level runtime error, not a catchable `result[..]` err.",
    ),
    see_also: &["arr_all", "arr_find"],
    since: Some("v0.1.5"),
};

static ARR_FILTER: FnEntry = FnEntry {
    signature: "arr_filter(arr, fn)",
    description: "returns a new array containing only elements where the predicate returns true",
    example: "get std::array::arr_filter\n\narr_filter([1, 2, 3, 4], fn(int x) -> bool { return x > 2 })?",
    expected_output: Some("[3, 4]"),
    returns: "result[arr[T]]",
    errors: Some(
        "Will return error on the following:\n\n- `arr` is not an array\n- `fn` is not a function/lambda, or does not declare a `bool` return type\n\nAn error raised inside `fn` while it runs propagates as an\ninterpreter-level runtime error, not a catchable `result[..]` err.",
    ),
    see_also: &["arr_map", "arr_find"],
    since: Some("v0.1.5"),
};

static ARR_FIND: FnEntry = FnEntry {
    signature: "arr_find(arr, fn)",
    description: "returns the first element where the predicate returns true, or null if none match",
    example: "get std::array::arr_find\n\narr_find([1, 2, 3, 4], fn(int x) -> bool { return x > 2 })?",
    expected_output: Some("3"),
    returns: "result[T]",
    errors: Some(
        "Will return error on the following:\n\n- `arr` is not an array\n- `fn` is not a function/lambda, or does not declare a `bool` return type\n\nAn error raised inside `fn` while it runs propagates as an\ninterpreter-level runtime error, not a catchable `result[..]` err.",
    ),
    see_also: &["arr_find_index", "arr_filter"],
    since: Some("v0.1.5"),
};

static ARR_FIND_INDEX: FnEntry = FnEntry {
    signature: "arr_find_index(arr, fn)",
    description: "returns the index of the first element where the predicate returns true, or -1 if none match",
    example: "get std::array::arr_find_index\n\narr_find_index([10, 20, 30], fn(int x) -> bool { return x == 20 })?",
    expected_output: Some("1"),
    returns: "result[int]",
    errors: Some(
        "Will return error on the following:\n\n- `arr` is not an array\n- `fn` is not a function/lambda, or does not declare a `bool` return type\n\nAn error raised inside `fn` while it runs propagates as an\ninterpreter-level runtime error, not a catchable `result[..]` err.",
    ),
    see_also: &["arr_find", "arr_index_of"],
    since: Some("v0.1.5"),
};

static ARR_FLAT_MAP: FnEntry = FnEntry {
    signature: "arr_flat_map(arr, fn)",
    description: "maps each element to an array via the callback then flattens the results one level",
    example: "get std::array::arr_flat_map\n\narr_flat_map([1, 2, 3], fn(int x) -> arr[int] { return [x, x * 10] })?",
    expected_output: Some("[1, 10, 2, 20, 3, 30]"),
    returns: "result[arr[T]]",
    errors: Some(
        "Will return error on the following:\n\n- `arr` is not an array\n- `fn` is not a function/lambda, or does not declare an array return type\n\nAn error raised inside `fn` while it runs propagates as an\ninterpreter-level runtime error, not a catchable `result[..]` err. Also,\nif `fn` returns a non-array value for some element (which the return-type\ncheck should normally prevent), that element is silently dropped rather\nthan erroring.",
    ),
    see_also: &["arr_map", "arr_flatten"],
    since: Some("v0.1.5"),
};

static ARR_FOR_EACH: FnEntry = FnEntry {
    signature: "arr_for_each(arr, fn)",
    description: "calls the callback on every element for side effects, returns null",
    example: "get std::array::arr_for_each\n\narr_for_each([1, 2, 3], fn(int x) { println(x) })?",
    expected_output: None,
    returns: "result[null]",
    errors: Some(
        "Will return error on the following:\n\n- `arr` is not an array\n- `fn` is not a function/lambda, or declares a non-null return type\n\nAn error raised inside `fn` while it runs propagates as an\ninterpreter-level runtime error, not a catchable `result[..]` err.",
    ),
    see_also: &["arr_map"],
    since: Some("v0.1.5"),
};

static ARR_MAP: FnEntry = FnEntry {
    signature: "arr_map(arr, fn)",
    description: "returns a new array with each element transformed by the callback",
    example: "get std::array::arr_map\n\narr_map([1, 2, 3], fn(int x) -> int { return x * 2 })?",
    expected_output: Some("[2, 4, 6]"),
    returns: "result[arr[T]]",
    errors: Some(
        "Will return error on the following:\n\n- `arr` is not an array\n- `fn` is not a function/lambda\n\nAn error raised inside `fn` while it runs propagates as an\ninterpreter-level runtime error, not a catchable `result[..]` err.\n\nNote: the resulting array's element type is inferred from the first\nmapped element - `arr_map` does not check `fn`'s declared return type\nthe way `arr_filter`/`arr_flat_map`/`arr_for_each` do.",
    ),
    see_also: &["arr_filter", "arr_flat_map", "arr_reduce"],
    since: Some("v0.1.5"),
};

static ARR_REDUCE: FnEntry = FnEntry {
    signature: "arr_reduce(arr, fn, initial)",
    description: "folds the array into a single value using the callback and a starting accumulator",
    example: "get std::array::arr_reduce\n\narr_reduce([1, 2, 3, 4], fn(int acc, int x) -> int { return acc + x }, 0)?",
    expected_output: Some("10"),
    returns: "result[T]",
    errors: Some(
        "Will return error on the following:\n\n- `arr` is not an array\n- `fn` is not a function/lambda\n\nAn error raised inside `fn` while it runs propagates as an\ninterpreter-level runtime error, not a catchable `result[..]` err.",
    ),
    see_also: &["arr_map", "arr_sum"],
    since: Some("v0.1.5"),
};

static ARR_SORT_BY: FnEntry = FnEntry {
    signature: "arr_sort_by(arr, fn)",
    description: "sorts the array using a comparator callback that returns -1, 0, or 1",
    example: "get std::array::arr_sort_by\n\narr_sort_by([3, 1, 2], fn(int a, int b) -> int { return a - b })?",
    expected_output: Some("[1, 2, 3]"),
    returns: "result[arr[T]]",
    errors: Some(
        "Will return error on the following:\n\n- `arr` is not an array\n- `fn` is not a function/lambda\n- `fn` returns a non-int value\n\nAn error raised inside `fn` while it runs propagates as an\ninterpreter-level runtime error, not a catchable `result[..]` err.\n\nNote: this is an insertion sort, so it's O(n^2) - fine for small arrays,\nbut consider `arr_sort` (which is not comparator-based) for large\nnumeric arrays.",
    ),
    see_also: &["arr_sort"],
    since: Some("v0.1.5"),
};

static ARR_ZIP: FnEntry = FnEntry {
    signature: "arr_zip(arr1, arr2)",
    description: "zips two arrays into an array of tuples; stops at the shorter array",
    example: "get std::array::arr_zip\n\narr_zip([1, 2, 3], [\"a\", \"b\", \"c\"])",
    expected_output: Some("[(1, \"a\"), (2, \"b\"), (3, \"c\")]"),
    returns: "arr[T]",
    errors: Some(
        "Will panic at runtime (not a catchable `result[..]` err) on the\nfollowing:\n\n- called with a number of arguments other than 2\n- `arr1` or `arr2` is not an array",
    ),
    see_also: &["arr_map", "arr_concat"],
    since: Some("v0.1.5"),
};
