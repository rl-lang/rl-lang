use crate::evaluator::Evaluator;
use crate::stdlib::net::NetHandle;

/// Registers a new handle and returns its id.
pub fn insert_handle(eval: &mut Evaluator, handle: NetHandle) -> i64 {
    let id = eval.net_next_handle;
    eval.net_next_handle += 1;
    eval.net_handles.insert(id, handle);
    id
}
