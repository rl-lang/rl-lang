pub mod keywords;
pub mod stdlib_signatures;

use rl_ast::statements::TypeAnnotation;
use std::collections::HashMap;

/// A `std` function's known signature(s), used by the checker to validate
/// call arguments and infer the result type statically (see rl-lang#250).
///
/// Each entry in `signatures` is one accepted overload: `(params, return_type)`.
/// `params` is a `TypeAnnotation::Tuple(..)` listing the expected argument
/// types in order (an empty tuple means the function takes no arguments).
/// Several functions accept more than one combination of argument types
/// (e.g. `pow(int, int)`, `pow(int, float)`, ...) - that's why this is a
/// `Vec` rather than a single pair: the checker tries each overload in turn
/// and uses the first one whose params match the call's argument types.
///
/// A function with an **empty** `signatures` vec is considered "not yet
/// typed": the checker treats calls to it as fully permissive
/// (`CheckType::Unknown`), which is the behavior every stdlib function had
/// before this field existed. This lets modules be typed incrementally.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct StdFn {
    pub name: String,
    pub signatures: Vec<(TypeAnnotation, TypeAnnotation)>,
}

impl StdFn {
    /// A function with no recorded signature - calls to it are unchecked.
    pub fn untyped(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            signatures: Vec::new(),
        }
    }

    /// A function with one or more known `(params, return_type)` overloads.
    pub fn typed(
        name: impl Into<String>,
        signatures: Vec<(TypeAnnotation, TypeAnnotation)>,
    ) -> Self {
        Self {
            name: name.into(),
            signatures,
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct ModuleNames {
    pub name: String,
    pub functions: HashMap<String, StdFn>,
    pub submodules: HashMap<String, ModuleNames>,
}

impl ModuleNames {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            functions: HashMap::new(),
            submodules: HashMap::new(),
        }
    }

    /// Bulk-adds function names with no signature (unchecked calls).
    /// Use [`ModuleNames::with_typed_function`] to give a specific function
    /// known argument/return types.
    pub fn with_functions(mut self, names: &[&str]) -> Self {
        self.functions
            .extend(names.iter().map(|s| (s.to_string(), StdFn::untyped(*s))));
        self
    }

    /// Adds (or overwrites) a single function with a known signature.
    pub fn with_typed_function(mut self, f: StdFn) -> Self {
        self.functions.insert(f.name.clone(), f);
        self
    }

    pub fn with_module(mut self, m: ModuleNames) -> Self {
        self.submodules.insert(m.name.clone(), m);
        self
    }

    /// Resolves a full stdlib path (e.g. `std::io::print`) to its [`StdFn`].
    pub fn resolve(&self, path: &[String]) -> Option<&StdFn> {
        if path.is_empty() {
            return None;
        }
        let path = if path.first().map(String::as_str) == Some(self.name.as_str()) {
            &path[1..]
        } else {
            path
        };
        if path.is_empty() {
            return None;
        }
        let mut module = self;
        for seg in &path[..path.len() - 1] {
            {
                let m = module.submodules.get(seg)?;
                module = m
            }
        }
        module.functions.get(&path[path.len() - 1])
    }

    pub fn collect_fn_names(&self, out: &mut HashMap<String, StdFn>) {
        out.extend(self.functions.iter().map(|(k, v)| (k.clone(), v.clone())));
        for sub in self.submodules.values() {
            sub.collect_fn_names(out);
        }
    }
}

pub fn stdlib_names() -> ModuleNames {
    ModuleNames::new("std")
        .with_module(stdlib_signatures::math::module())
        .with_module(stdlib_signatures::io::module())
        .with_module(stdlib_signatures::bitwise::module())
        .with_module(stdlib_signatures::terminal::module())
        .with_module(stdlib_signatures::types::module())
        .with_module(stdlib_signatures::debug::module())
        .with_module(stdlib_signatures::array::module())
        .with_module(stdlib_signatures::str::module())
        .with_module(stdlib_signatures::time::module())
        .with_module(stdlib_signatures::rl::module())
        .with_module(stdlib_signatures::path::module())
        .with_module(stdlib_signatures::fs::module())
        .with_module(stdlib_signatures::random::module())
        .with_module(ModuleNames::new("process").with_functions(keywords::process::KEYWORDS))
        .with_module(ModuleNames::new("res").with_functions(keywords::result::KEYWORDS))
        .with_module(ModuleNames::new("net").with_functions(keywords::net::KEYWORDS))
        .with_module(ModuleNames::new("http").with_functions(keywords::http::KEYWORDS))
        .with_module(ModuleNames::new("collections").with_functions(keywords::set::KEYWORDS))
}
