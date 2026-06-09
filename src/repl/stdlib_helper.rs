pub struct StdEntry {
    pub name: &'static str,
    pub functions: &'static [(&'static str, &'static str)],
}

pub fn stdlib_entries() -> Vec<StdEntry> {
    vec![
        StdEntry {
            name: "math",
            functions: &[
                ("pow(x, n)", "x raised to the power n"),
                ("abs(x)", "absolute value"),
                ("sqrt(x)", "square root"),
                ("floor(x)", "round down"),
                ("ceil(x)", "round up"),
                ("round(x)", "round to nearest"),
                ("min(a, b)", "minimum of two values"),
                ("max(a, b)", "maximum of two values"),
                ("log(x, base)", "logarithm"),
                ("clamp(x, lo, hi)", "clamp x between lo and hi"),
            ],
        },
        StdEntry {
            name: "display",
            functions: &[
                ("print(x)", "print without newline"),
                ("println(x)", "print with newline"),
                ("len(x)", "length of string or array"),
            ],
        },
        StdEntry {
            name: "io",
            functions: &[
                ("input()", "read a line from stdin"),
                ("input(prompt)", "prints prompt and read a line from stdin"),
            ],
        },
    ]
}
