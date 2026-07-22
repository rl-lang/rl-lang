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
            match module.submodules.get(seg) {
                Some(m) => module = m,
                None => return None,
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
        .with_module(ModuleNames::new("string").with_functions(keywords::string::KEYWORDS))
        .with_module(ModuleNames::new("array").with_functions(keywords::array::KEYWORDS))
        .with_module(ModuleNames::new("path").with_functions(keywords::path::KEYWORDS))
        .with_module(ModuleNames::new("fs").with_functions(keywords::fs::KEYWORDS))
        .with_module(ModuleNames::new("random").with_functions(keywords::random::KEYWORDS))
        .with_module(ModuleNames::new("time").with_functions(keywords::time::KEYWORDS))
        .with_module(ModuleNames::new("process").with_functions(keywords::process::KEYWORDS))
        .with_module(ModuleNames::new("res").with_functions(keywords::result::KEYWORDS))
        .with_module(ModuleNames::new("rl").with_functions(keywords::rl::KEYWORDS))
        .with_module(ModuleNames::new("debug").with_functions(keywords::debug::KEYWORDS))
        .with_module(ModuleNames::new("net").with_functions(keywords::net::KEYWORDS))
        .with_module(ModuleNames::new("http").with_functions(keywords::http::KEYWORDS))
        .with_module(ModuleNames::new("collections").with_functions(keywords::set::KEYWORDS))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::stdlib_signatures::params;

    #[test]
    fn resolves_nested_path() {
        let tree = stdlib_names();
        let path = vec!["math".to_string(), "sin".to_string()];
        assert!(tree.resolve(&path).is_some());
    }

    #[test]
    fn resolves_double_nested_path() {
        let tree = stdlib_names();
        let path = vec!["math".to_string(), "consts".to_string(), "PI".to_string()];
        assert!(tree.resolve(&path).is_some());
    }

    #[test]
    fn rejects_unknown_path() {
        let tree = stdlib_names();
        let path = vec!["math".to_string(), "not_a_real_fn".to_string()];
        assert!(tree.resolve(&path).is_none());
    }

    #[test]
    fn pow_has_four_overloads() {
        let tree = stdlib_names();
        let path = vec!["math".to_string(), "pow".to_string()];
        let f = tree.resolve(&path).expect("pow should resolve");
        assert_eq!(f.signatures.len(), 4);
    }

    #[test]
    fn bitwise_bit_and_resolves_with_signatures() {
        let tree = stdlib_names();
        let path = vec!["bitwise".to_string(), "bit_and".to_string()];
        let f = tree.resolve(&path).expect("bit_and should resolve");
        assert_eq!(f.signatures.len(), 4);
    }

    #[test]
    fn io_read_int_resolves_with_signatures() {
        let tree = stdlib_names();
        let path = vec!["io".to_string(), "read_int".to_string()];
        let f = tree.resolve(&path).expect("read_int should resolve");
        assert!(!f.signatures.is_empty());
    }

    #[test]
    fn terminal_set_fg_has_eight_overloads() {
        let tree = stdlib_names();
        let path = vec!["term".to_string(), "term_set_fg".to_string()];
        let f = tree.resolve(&path).expect("term_set_fg should resolve");
        assert_eq!(f.signatures.len(), 8);
    }

    #[test]
    fn terminal_read_key_returns_result_array_string() {
        let tree = stdlib_names();
        let path = vec!["term".to_string(), "term_read_key".to_string()];
        let f = tree.resolve(&path).expect("term_read_key should resolve");
        assert_eq!(f.signatures.len(), 1);
        assert_eq!(
            f.signatures[0].1,
            TypeAnnotation::Result(Box::new(TypeAnnotation::Array(Box::new(
                TypeAnnotation::String
            ))))
        );
    }

    #[test]
    fn terminal_get_size_returns_result_array_int() {
        let tree = stdlib_names();
        let path = vec!["term".to_string(), "term_get_size".to_string()];
        let f = tree.resolve(&path).expect("term_get_size should resolve");
        assert_eq!(f.signatures.len(), 1);
        assert_eq!(
            f.signatures[0].1,
            TypeAnnotation::Result(Box::new(TypeAnnotation::Array(Box::new(
                TypeAnnotation::Int
            ))))
        );
    }

    #[test]
    fn math_sin_returns_plain_float_not_result() {
        let tree = stdlib_names();
        let path = vec!["math".to_string(), "sin".to_string()];
        let f = tree.resolve(&path).expect("sin should resolve");
        assert_eq!(f.signatures.len(), 1);
        assert_eq!(
            f.signatures[0],
            (params(vec![TypeAnnotation::Float]), TypeAnnotation::Float)
        );
    }

    #[test]
    fn math_max_has_no_mixed_overload() {
        let tree = stdlib_names();
        let path = vec!["math".to_string(), "max".to_string()];
        let f = tree.resolve(&path).expect("max should resolve");
        assert_eq!(f.signatures.len(), 2);
    }

    #[test]
    fn math_log_always_returns_float() {
        let tree = stdlib_names();
        let path = vec!["math".to_string(), "log".to_string()];
        let f = tree.resolve(&path).expect("log should resolve");
        assert_eq!(f.signatures.len(), 4);
        assert!(
            f.signatures
                .iter()
                .all(|(_, ret)| *ret == TypeAnnotation::Result(Box::new(TypeAnnotation::Float)))
        );
    }

    #[test]
    fn math_constants_pi_is_zero_arg_float() {
        let tree = stdlib_names();
        let path = vec!["math".to_string(), "consts".to_string(), "PI".to_string()];
        let f = tree.resolve(&path).expect("PI should resolve");
        assert_eq!(f.signatures, vec![(params(vec![]), TypeAnnotation::Float)]);
    }

    #[test]
    fn math_constants_is_nan_takes_float_returns_bool() {
        let tree = stdlib_names();
        let path = vec![
            "math".to_string(),
            "consts".to_string(),
            "is_nan".to_string(),
        ];
        let f = tree.resolve(&path).expect("is_nan should resolve");
        assert_eq!(
            f.signatures,
            vec![(params(vec![TypeAnnotation::Float]), TypeAnnotation::Bool)]
        );
    }

    #[test]
    fn types_to_int_has_six_overloads_no_null() {
        let tree = stdlib_names();
        let path = vec!["types".to_string(), "to_int".to_string()];
        let f = tree.resolve(&path).expect("to_int should resolve");
        assert_eq!(f.signatures.len(), 6);
        assert!(
            f.signatures
                .iter()
                .all(|(p, _)| *p != params(vec![TypeAnnotation::Null]))
        );
    }

    #[test]
    fn types_is_int_stays_untyped() {
        let tree = stdlib_names();
        let path = vec!["types".to_string(), "is_int".to_string()];
        let f = tree.resolve(&path).expect("is_int should resolve");
        assert!(f.signatures.is_empty());
    }

    #[test]
    fn types_error_unwrap_stays_untyped() {
        let tree = stdlib_names();
        let path = vec!["types".to_string(), "error_unwrap".to_string()];
        let f = tree.resolve(&path).expect("error_unwrap should resolve");
        assert!(f.signatures.is_empty());
    }

    #[test]
    fn types_to_char_rejects_bool() {
        let tree = stdlib_names();
        let path = vec!["types".to_string(), "to_char".to_string()];
        let f = tree.resolve(&path).expect("to_char should resolve");
        assert!(
            f.signatures
                .iter()
                .all(|(p, _)| *p != params(vec![TypeAnnotation::Bool]))
        );
    }
}
