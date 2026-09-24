use std::fmt;

pub const REPO: &str = "rl-lang/rl-lang";

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Variant {
    pub name: &'static str,
    pub actual: &'static str,
    pub group: &'static str,
}

pub fn all_variants() -> Vec<Variant> {
    let mut v = Vec::new();
    // Group 1: Standard (vm)
    for (name, actual) in [
        ("rl", "rl"),
        ("rl_no_docs", "rl_nd"),
        ("rl_no_repl", "rl_nr"),
        ("rl_no_docs_repl", "rl_ndr"),
    ] {
        v.push(Variant { name, actual, group: "Standard (vm)" });
    }
    // Group 2: VM-only
    for (name, actual) in [
        ("rl_vm", "rlc"),
        ("rl_vm_no_docs", "rlc_nd"),
        ("rl_vm_no_repl", "rlc_nr"),
        ("rl_vm_no_docs_repl", "rlc_ndr"),
    ] {
        v.push(Variant { name, actual, group: "VM-only" });
    }
    // Group 3: Debug builds
    for (name, actual) in [
        ("rl_debug", "rld"),
        ("rl_debug_no_docs", "rld_nd"),
        ("rl_debug_no_repl", "rld_nr"),
        ("rl_debug_no_docs_repl", "rld_ndr"),
        ("rl_vm_debug", "rlcd"),
        ("rl_vm_debug_no_docs", "rlcd_nd"),
        ("rl_vm_debug_no_repl", "rlcd_nr"),
        ("rl_vm_debug_no_docs_repl", "rlcd_ndr"),
    ] {
        v.push(Variant { name, actual, group: "Debug builds" });
    }
    // Group 4: Language server
    v.push(Variant { name: "rl_lsp", actual: "rlsp", group: "Language server" });
    v
}

impl fmt::Display for Variant {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} ({})", self.name, self.actual)
    }
}
