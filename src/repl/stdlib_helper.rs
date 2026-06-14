pub struct StdEntry {
    pub name: &'static str,
    pub functions: &'static [(&'static str, &'static str)],
}

pub fn stdlib_entries() -> Vec<StdEntry> {
    vec![
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
