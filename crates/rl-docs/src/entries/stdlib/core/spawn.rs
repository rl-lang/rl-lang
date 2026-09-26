use crate::entry::FnEntry;

pub static SPAWN: FnEntry = FnEntry {
    signature: "__spawn(source)",
    description: "intrinsic: queues RL `source` on the shared worker pool (CPU count threads, queue of 256; saturation is an err, not a block), returning a job id int. Each job runs on a fresh Vm; compiled chunks memoize per pool thread by source hash. Only strings cross the thread boundary (Vm is !Send): progress via `__emit`, worker prints captured and replayed before the outcome, final value or error via `__poll`. Worker sources are self-contained (own imports); `std::gui` calls fail there, and workers cannot spawn (refused loudly)",
    example: r#"get __spawn from core

dec int job = __spawn("40 + 2")"#,
    expected_output: None,
    returns: "int",
    errors: None,
    see_also: &["__poll", "__emit"],
    since: Some("v2.3.0"),
    deprecated: None,
    updated: Some("v2.3.0"),
};
