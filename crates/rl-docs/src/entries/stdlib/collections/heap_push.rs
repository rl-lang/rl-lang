use crate::entry::FnEntry;

pub static HEAP_PUSH: FnEntry = FnEntry {
    signature: "heap_push(heap, value)",
    description: "pushes a value onto the min-heap, maintaining the heap property",
    example: "get heap_push, heap_pop from std::collections\n\ndec heap[int] h = []\nheap_push(h, 3)\nheap_push(h, 1)\nheap_push(h, 2)\nheap_pop(h)?",
    expected_output: Some("1"),
    returns: "null",
    errors: Some(
        "Will return error on the following:\n\n- `heap` is not a heap\n- `value`'s type does not match the heap element type",
    ),
    see_also: &["heap_pop", "heap_peek"],
    since: Some("v2.1.0"),
    deprecated: None,
    updated: Some("v2.1.0"),
};
