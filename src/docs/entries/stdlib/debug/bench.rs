use crate::docs::entry::FnEntry;

pub static BENCH: FnEntry = FnEntry {
    signature: "bench(function, iterations)",
    description: "calls a zero-argument function or lambda `iterations` times and returns the total elapsed time in milliseconds",
    example: r#"
get std::debug::bench
get std::math::factorial
get std::io::println

dec float ms = bench(fn() { factorial(10) }, 1000)?
println(ms)
"#,
    expected_output: None,
    returns: "result[float]",
    errors: Some(
        "returns error when `function` is not callable, or `iterations` is not a positive int",
    ),
    see_also: &[],
    since: Some("v0.1.5"),
};
