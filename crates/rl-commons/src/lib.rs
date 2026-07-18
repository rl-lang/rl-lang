pub mod keywords;

use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone, Default)]
pub struct ModuleNames {
    pub name: String,
    pub functions: HashSet<String>,
    pub submodules: HashMap<String, ModuleNames>,
}

impl ModuleNames {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            functions: HashSet::new(),
            submodules: HashMap::new(),
        }
    }

    pub fn with_functions(mut self, names: &[&str]) -> Self {
        self.functions.extend(names.iter().map(|s| s.to_string()));
        self
    }

    pub fn with_module(mut self, m: ModuleNames) -> Self {
        self.submodules.insert(m.name.clone(), m);
        self
    }

    pub fn resolve(&self, path: &[String]) -> bool {
        if path.is_empty() {
            return false;
        }
        let path = if path.first().map(String::as_str) == Some(self.name.as_str()) {
            &path[1..]
        } else {
            path
        };
        if path.is_empty() {
            return false;
        }
        let mut module = self;
        for seg in &path[..path.len() - 1] {
            match module.submodules.get(seg) {
                Some(m) => module = m,
                None => return false,
            }
        }
        module.functions.contains(&path[path.len() - 1])
    }

    pub fn collect_fn_names(&self, out: &mut HashSet<String>) {
        out.extend(self.functions.iter().cloned());
        for sub in self.submodules.values() {
            sub.collect_fn_names(out);
        }
    }
}

pub fn stdlib_names() -> ModuleNames {
    ModuleNames::new("std")
        .with_module(
            ModuleNames::new("math")
                .with_functions(keywords::math::KEYWORDS)
                .with_module(
                    ModuleNames::new("constants")
                        .with_functions(keywords::math::constants::KEYWORDS),
                ),
        )
        .with_module(ModuleNames::new("io").with_functions(keywords::io::KEYWORDS))
        .with_module(ModuleNames::new("bitwise").with_functions(keywords::bitwise::KEYWORDS))
        .with_module(ModuleNames::new("string").with_functions(keywords::string::KEYWORDS))
        .with_module(ModuleNames::new("types").with_functions(keywords::types::KEYWORDS))
        .with_module(ModuleNames::new("array").with_functions(keywords::array::KEYWORDS))
        .with_module(ModuleNames::new("path").with_functions(keywords::path::KEYWORDS))
        .with_module(ModuleNames::new("fs").with_functions(keywords::fs::KEYWORDS))
        .with_module(ModuleNames::new("random").with_functions(keywords::random::KEYWORDS))
        .with_module(ModuleNames::new("time").with_functions(keywords::time::KEYWORDS))
        .with_module(ModuleNames::new("process").with_functions(keywords::process::KEYWORDS))
        .with_module(ModuleNames::new("result").with_functions(keywords::result::KEYWORDS))
        .with_module(ModuleNames::new("terminal").with_functions(keywords::terminal::KEYWORDS))
        .with_module(ModuleNames::new("rl").with_functions(keywords::rl::KEYWORDS))
        .with_module(ModuleNames::new("debug").with_functions(keywords::debug::KEYWORDS))
        .with_module(ModuleNames::new("net").with_functions(keywords::net::KEYWORDS))
        .with_module(ModuleNames::new("http").with_functions(keywords::http::KEYWORDS))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn resolves_nested_path() {
        let tree = stdlib_names();
        let path = vec!["math".to_string(), "sin".to_string()];
        assert!(tree.resolve(&path));
    }

    #[test]
    fn resolves_double_nested_path() {
        let tree = stdlib_names();
        let path = vec![
            "math".to_string(),
            "constants".to_string(),
            "PI".to_string(),
        ];
        assert!(tree.resolve(&path));
    }

    #[test]
    fn rejects_unknown_path() {
        let tree = stdlib_names();
        let path = vec!["math".to_string(), "not_a_real_fn".to_string()];
        assert!(!tree.resolve(&path));
    }
}
