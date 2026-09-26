use crate::entry::FnEntry;

pub static POLL: FnEntry = FnEntry {
    signature: "__poll(job)",
    description: "intrinsic: non-blocking read of one worker message. `ok(text)` for progress or the final value, `err(\"pending\")` while running, `err(text)` when the worker failed. Consuming the outcome reaps the job; polling afterwards is an unknown-job err",
    example: r#"get __spawn, __poll from core
get is_err, result_unwrap from std::res

dec int job = __spawn("40 + 2")
while (true) {
    dec r = __poll(job)
    if (!is_err(r)) {
        break
    }
}"#,
    expected_output: None,
    returns: "result[string]",
    errors: Some("err(\"pending\") while running, err(text) on worker failure, unknown-job err after reaping"),
    see_also: &["__spawn", "__emit"],
    since: Some("v2.3.0"),
    deprecated: None,
    updated: Some("v2.3.0"),
};
