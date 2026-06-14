use crate::docs::entry::{FnEntry, StdEntry};

pub static MATH: StdEntry = StdEntry {
    name: "math",
    description: "functions for math",
    functions: FUNCTIONS,
};

static FUNCTIONS: &'static [&'static FnEntry] = &[&FnEntry {
    signature: "abs(number)",
    description: "returns the absolute value of number",
    example: "get std::math::abs\nget std::display::println\n\ndec int x = -1\nx.abs()\t//1",
}];
