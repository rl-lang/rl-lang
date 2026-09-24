use crate::entry::{FnEntry, StdEntry};

mod arr_all;
mod arr_any;
mod arr_chunk;
mod arr_concat;
mod arr_contains;
mod arr_count;
mod arr_cycle_take;
mod arr_fill;
mod arr_filter;
mod arr_find;
mod arr_find_index;
mod arr_first;
mod arr_flatten;
mod arr_flat_map;
mod arr_for_each;
mod arr_index_of;
mod arr_insert;
mod arr_is_empty;
mod arr_last;
mod arr_map;
mod arr_max;
mod arr_max_by;
mod arr_min;
mod arr_min_by;
mod arr_partition;
mod arr_pop;
mod arr_product;
mod arr_push;
mod arr_range;
mod arr_reduce;
mod arr_remove;
mod arr_reverse;
mod arr_slice;
mod arr_sort;
mod arr_sort_by;
mod arr_sum;
mod arr_swap;
mod arr_unique;
mod arr_windows;
mod arr_zip;
mod arr_zip_longest;
mod len;

pub static ARRAY: StdEntry = StdEntry {
    name: "array",
    description: "functions for array manipulation",
    functions: FUNCTIONS,
    since: Some("v0.1.5"),
    unstable: false,
};

static FUNCTIONS: &[&FnEntry] = &[
    &arr_all::ARR_ALL,
    &arr_any::ARR_ANY,
    &arr_chunk::ARR_CHUNK,
    &arr_concat::ARR_CONCAT,
    &arr_contains::ARR_CONTAINS,
    &arr_count::ARR_COUNT,
    &arr_cycle_take::ARR_CYCLE_TAKE,
    &arr_fill::ARR_FILL,
    &arr_filter::ARR_FILTER,
    &arr_find::ARR_FIND,
    &arr_find_index::ARR_FIND_INDEX,
    &arr_first::ARR_FIRST,
    &arr_flatten::ARR_FLATTEN,
    &arr_flat_map::ARR_FLAT_MAP,
    &arr_for_each::ARR_FOR_EACH,
    &arr_index_of::ARR_INDEX_OF,
    &arr_insert::ARR_INSERT,
    &arr_is_empty::ARR_IS_EMPTY,
    &arr_last::ARR_LAST,
    &len::LEN,
    &arr_map::ARR_MAP,
    &arr_max::ARR_MAX,
    &arr_max_by::ARR_MAX_BY,
    &arr_min::ARR_MIN,
    &arr_min_by::ARR_MIN_BY,
    &arr_partition::ARR_PARTITION,
    &arr_pop::ARR_POP,
    &arr_product::ARR_PRODUCT,
    &arr_push::ARR_PUSH,
    &arr_range::ARR_RANGE,
    &arr_reduce::ARR_REDUCE,
    &arr_remove::ARR_REMOVE,
    &arr_reverse::ARR_REVERSE,
    &arr_slice::ARR_SLICE,
    &arr_sort::ARR_SORT,
    &arr_sort_by::ARR_SORT_BY,
    &arr_sum::ARR_SUM,
    &arr_swap::ARR_SWAP,
    &arr_unique::ARR_UNIQUE,
    &arr_windows::ARR_WINDOWS,
    &arr_zip::ARR_ZIP,
    &arr_zip_longest::ARR_ZIP_LONGEST,
];
