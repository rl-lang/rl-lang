use std::fmt;

pub const REPO: &str = "rl-lang/rl-lang";

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Variant {
    pub name: &'static str,
    pub actual: &'static str,
    pub group: &'static str,
}

pub fn all_variants() -> Vec<Variant> {
    // Must match the release assets published by build-variants.sh / _build.yml:
    //   {binary}-{platform}-{arch}.tar.gz (linux/macos/android) or .zip (windows)
    // where binary in {rl, rlc, rlt, rlrepl, rlsp, rldocs, rlm}.
    // See https://github.com/rl-lang/rl-lang/releases
    let mut v = Vec::new();
    for (name, actual, group) in [
        ("rl", "rl", "Core (run, check, new, dev, format, pm)"),
        ("rlc", "rlc", "Compiler (VM backend)"),
        ("rlt", "rlt", "Transpiler (to C99)"),
        ("rlrepl", "rlrepl", "Interactive TUI REPL"),
        ("rlsp", "rlsp", "LSP server"),
        ("rldocs", "rldocs", "Documentation viewer"),
        ("rlm", "rlm", "Toolchain manager"),
    ] {
        v.push(Variant { name, actual, group });
    }
    v
}

impl fmt::Display for Variant {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} ({})", self.name, self.actual)
    }
}
