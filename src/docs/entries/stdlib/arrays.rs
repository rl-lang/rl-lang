use crate::docs::entry::{FnEntry, StdEntry};

pub static ARRAY: StdEntry = StdEntry {
    name: "array",
    description: "functions for array manipulation",
    functions: FUNCTIONS,
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
];

static ARR_CONCAT: FnEntry = FnEntry {
    signature: "arr_concat(arr1, arr2)",
    description: "concatenates two arrays of the same type into one",
    example: "get std::array::arr_concat\n\narr_concat([1, 2], [3, 4]) // [1, 2, 3, 4]",
};

static ARR_CONTAINS: FnEntry = FnEntry {
    signature: "arr_contains(arr, value)",
    description: "true if the array contains the given value",
    example: "get std::array::arr_contains\n\narr_contains([1, 2, 3], 2) // true",
};

static ARR_COUNT: FnEntry = FnEntry {
    signature: "arr_count(arr)",
    description: "returns the number of elements in the array",
    example: "get std::array::arr_count\n\narr_count([1, 2, 3]) // 3",
};

static ARR_FILL: FnEntry = FnEntry {
    signature: "arr_fill(value, count)",
    description: "creates an array filled with value repeated count times",
    example: "get std::array::arr_fill\n\narr_fill(0, 3) // [0, 0, 0]",
};

static ARR_FIRST: FnEntry = FnEntry {
    signature: "arr_first(arr)",
    description: "returns the first element of the array",
    example: "get std::array::arr_first\n\narr_first([1, 2, 3]) // 1",
};

static ARR_FLATTEN: FnEntry = FnEntry {
    signature: "arr_flatten(arr)",
    description: "flattens a nested array into a single array",
    example: "get std::array::arr_flatten\n\narr_flatten([[1, 2], [3, 4]]) // [1, 2, 3, 4]",
};

static ARR_INDEX_OF: FnEntry = FnEntry {
    signature: "arr_index_of(arr, index)",
    description: "returns the element at the given index",
    example: "get std::array::arr_index_of\n\narr_index_of([10, 20, 30], 1) // 20",
};

static ARR_INSERT: FnEntry = FnEntry {
    signature: "arr_insert(arr, value, index)",
    description: "inserts value at the given index, shifting elements right",
    example: "get std::array::arr_insert\n\narr_insert([1, 3], 2, 1) // [1, 2, 3]",
};

static ARR_IS_EMPTY: FnEntry = FnEntry {
    signature: "arr_is_empty(arr)",
    description: "true if the array has no elements",
    example: "get std::array::arr_is_empty\n\narr_is_empty([]) // true",
};

static ARR_LAST: FnEntry = FnEntry {
    signature: "arr_last(arr)",
    description: "returns the last element of the array",
    example: "get std::array::arr_last\n\narr_last([1, 2, 3]) // 3",
};

static ARR_MAX: FnEntry = FnEntry {
    signature: "arr_max(arr)",
    description: "returns the largest element in an int or float array",
    example: "get std::array::arr_max\n\narr_max([3, 1, 4, 1, 5]) // 5",
};

static ARR_MIN: FnEntry = FnEntry {
    signature: "arr_min(arr)",
    description: "returns the smallest element in an int or float array",
    example: "get std::array::arr_min\n\narr_min([3, 1, 4, 1, 5]) // 1",
};

static ARR_POP: FnEntry = FnEntry {
    signature: "arr_pop(arr)",
    description: "removes the last element and returns the updated array",
    example: "get std::array::arr_pop\n\narr_pop([1, 2, 3]) // [1, 2]",
};

static ARR_PRODUCT: FnEntry = FnEntry {
    signature: "arr_product(arr)",
    description: "returns the product of all elements in an int or float array",
    example: "get std::array::arr_product\n\narr_product([1, 2, 3, 4]) // 24",
};

static ARR_PUSH: FnEntry = FnEntry {
    signature: "arr_push(arr, value)",
    description: "appends value to the end of the array and returns the updated array",
    example: "get std::array::arr_push\n\narr_push([1, 2], 3) // [1, 2, 3]",
};

static ARR_RANGE: FnEntry = FnEntry {
    signature: "arr_range(start, end, step)",
    description: "creates an int array from start to end (exclusive) with the given step",
    example: "get std::array::arr_range\n\narr_range(0, 6, 2) // [0, 2, 4]",
};

static ARR_REMOVE: FnEntry = FnEntry {
    signature: "arr_remove(arr, index)",
    description: "removes the element at the given index and returns the updated array",
    example: "get std::array::arr_remove\n\narr_remove([1, 2, 3], 1) // [1, 3]",
};

static ARR_REVERSE: FnEntry = FnEntry {
    signature: "arr_reverse(arr)",
    description: "reverses the order of elements in the array",
    example: "get std::array::arr_reverse\n\narr_reverse([1, 2, 3]) // [3, 2, 1]",
};

static ARR_SLICE: FnEntry = FnEntry {
    signature: "arr_slice(arr, start, end)",
    description: "returns a sub-array from start to end (exclusive)",
    example: "get std::array::arr_slice\n\narr_slice([1, 2, 3, 4], 1, 3) // [2, 3]",
};

static ARR_SORT: FnEntry = FnEntry {
    signature: "arr_sort(arr)",
    description: "returns the array sorted in ascending order, only int or float arrays",
    example: "get std::array::arr_sort\n\narr_sort([3, 1, 2]) // [1, 2, 3]",
};

static ARR_SUM: FnEntry = FnEntry {
    signature: "arr_sum(arr)",
    description: "returns the sum of all elements in an int or float array",
    example: "get std::array::arr_sum\n\narr_sum([1, 2, 3, 4]) // 10",
};

static ARR_UNIQUE: FnEntry = FnEntry {
    signature: "arr_unique(arr)",
    description: "returns the array with duplicate values removed, preserving order",
    example: "get std::array::arr_unique\n\narr_unique([1, 2, 2, 3, 1]) // [1, 2, 3]",
};

static LEN: FnEntry = FnEntry {
    signature: "len(x)",
    description: "length of string or array",
    example: "get std::array::len\n\nlen(\"hello\") // 5",
};

static ARR_ALL: FnEntry = FnEntry {
    signature: "arr_all(arr, fn)",
    description: "true if every element satisfies the predicate",
    example: "get std::array::arr_all\n\narr_all([2, 4, 6], fn(int x) -> bool { return mod(x, 2) == 0 }) // true",
};

static ARR_ANY: FnEntry = FnEntry {
    signature: "arr_any(arr, fn)",
    description: "true if at least one element satisfies the predicate",
    example: "get std::array::arr_any\n\narr_any([1, 2, 3], fn(int x) -> bool { return x > 2 }) // true",
};

static ARR_FILTER: FnEntry = FnEntry {
    signature: "arr_filter(arr, fn)",
    description: "returns a new array containing only elements where the predicate returns true",
    example: "get std::array::arr_filter\n\narr_filter([1, 2, 3, 4], fn(int x) -> bool { return x > 2 }) // [3, 4]",
};

static ARR_FIND: FnEntry = FnEntry {
    signature: "arr_find(arr, fn)",
    description: "returns the first element where the predicate returns true, or null if none match",
    example: "get std::array::arr_find\n\narr_find([1, 2, 3, 4], fn(int x) -> bool { return x > 2 }) // 3",
};

static ARR_FIND_INDEX: FnEntry = FnEntry {
    signature: "arr_find_index(arr, fn)",
    description: "returns the index of the first element where the predicate returns true, or -1 if none match",
    example: "get std::array::arr_find_index\n\narr_find_index([10, 20, 30], fn(int x) -> bool { return x == 20 }) // 1",
};

static ARR_FLAT_MAP: FnEntry = FnEntry {
    signature: "arr_flat_map(arr, fn)",
    description: "maps each element to an array via the callback then flattens the results one level",
    example: "get std::array::arr_flat_map\n\narr_flat_map([1, 2, 3], fn(int x) -> arr[int] { return [x, x * 10] }) // [1, 10, 2, 20, 3, 30]",
};

static ARR_FOR_EACH: FnEntry = FnEntry {
    signature: "arr_for_each(arr, fn)",
    description: "calls the callback on every element for side effects, returns null",
    example: "get std::array::arr_for_each\n\narr_for_each([1, 2, 3], fn(int x) { println(x) })",
};

static ARR_MAP: FnEntry = FnEntry {
    signature: "arr_map(arr, fn)",
    description: "returns a new array with each element transformed by the callback",
    example: "get std::array::arr_map\n\narr_map([1, 2, 3], fn(int x) -> int { return x * 2 }) // [2, 4, 6]",
};

static ARR_REDUCE: FnEntry = FnEntry {
    signature: "arr_reduce(arr, fn, initial)",
    description: "folds the array into a single value using the callback and a starting accumulator",
    example: "get std::array::arr_reduce\n\narr_reduce([1, 2, 3, 4], fn(int acc, int x) -> int { return acc + x }, 0) // 10",
};

static ARR_SORT_BY: FnEntry = FnEntry {
    signature: "arr_sort_by(arr, fn)",
    description: "sorts the array using a comparator callback that returns -1, 0, or 1",
    example: "get std::array::arr_sort_by\n\narr_sort_by([3, 1, 2], fn(int a, int b) -> int { return a - b }) // [1, 2, 3]",
};
