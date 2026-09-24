use crate::entry::FnEntry;

pub static HEAP_POP: FnEntry = FnEntry {
    signature: "heap_pop(heap)",
    description: "removes and returns the smallest value from the min-heap",
    example: "get heap_push, heap_pop from std::collections\n\ndec heap[int] h = []\nheap_push(h, 3)\nheap_push(h, 1)\nheap_push(h, 2)\nheap_pop(h)?",
    expected_output: Some("1"),
    returns: "result[T]",
    errors: Some(
        "Will return error on the following:\n\n- `heap` is not a heap\n- `heap` is empty",
    ),
    see_also: &["heap_push", "heap_peek"],
    since: Some("v2.1.0"),
    deprecated: None,
    updated: Some("v2.1.0"),
};
