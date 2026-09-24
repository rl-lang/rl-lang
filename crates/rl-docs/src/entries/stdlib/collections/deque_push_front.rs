use crate::entry::FnEntry;

pub static DEQUE_PUSH_FRONT: FnEntry = FnEntry {
    signature: "deque_push_front(deque, value)",
    description: "adds a value to the front of the deque",
    example: "get deque_push_front, deque_pop_front from std::collections\n\ndec deque[int] d = []\ndeque_push_front(d, 2)\ndeque_push_front(d, 1)\ndeque_pop_front(d)?",
    expected_output: Some("1"),
    returns: "null",
    errors: Some(
        "Will return error on the following:\n\n- `deque` is not a deque\n- `value`'s type does not match the deque element type",
    ),
    see_also: &["deque_pop_front"],
    since: Some("v2.1.0"),
    deprecated: None,
    updated: Some("v2.1.0"),
};
