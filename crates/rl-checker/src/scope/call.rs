//! Function call type checking - path resolution and argument validation.

use crate::structs::{CheckType, TypeChecker};
use crate::types::{has_generic, substitute, unify_arg};
use rl_ast::statements::{Lint, TypeAnnotation};
use rl_commons::{StdFn, keywords};
use rl_utils::{span::Span, suggest::closest_match};
use std::collections::HashMap;

impl TypeChecker {
    /// Resolves a call path and checks argument types.
    ///
    /// Resolution order:
    /// 1. Full stdlib path (`std::io::print`) via [`root_module`]
    /// 2. Single-name stdlib shorthand (`print`) via [`stdlib_fn_names`]
    /// 3. User-defined name via [`lookup`]
    /// 4. Error - unknown function, with a "did you mean?" suggestion from stdlib keywords
    ///
    /// Stdlib calls are checked against their known signature(s), if any
    /// (see [`TypeChecker::check_stdlib_call`]) - functions not yet typed
    /// still return [`CheckType::Unknown`] unchecked.
    pub fn check_call_path(
        &mut self,
        path: &[String],
        arg_types: &[(CheckType, Span)],
        span: Span,
    ) -> CheckType {
        // `Record::method` associated function, e.g. `Point::new(1, 2)`.
        if path.len() == 2
            && let Some(sig) = self
                .methods
                .get(&(path[0].clone(), path[1].clone()))
                .cloned()
        {
            return self.check_call_value(sig, arg_types, span);
        }

        // stdlib path (std::io::print)
        if let Some(f) = self.root_module.resolve(path).cloned() {
            self.push_stdlib_hover(path, span);
            // Check for deprecated stdlib functions
            if let Some(msg) = self.deprecated_stdlib.get(path) {
                let fn_name = path.join("::");
                let text = format!("'{}' is deprecated: {}", fn_name, msg);
                if !self
                    .allow_stack
                    .last()
                    .is_some_and(|s| s.contains(&Lint::Deprecated))
                {
                    self.warn_lint(Lint::Deprecated, text, span);
                }
            }
            return self.check_stdlib_call(&f, arg_types, span);
        }

        if path.len() == 1 {
            let name = &path[0];

            if let Some(f) = self.imported_std_fns.get(name.as_str()).cloned() {
                self.push_stdlib_hover(path, span);
                // Check for deprecated stdlib functions (same as qualified
                // paths above; the canonical path was recorded at import).
                if let Some(canonical) = self.imported_std_paths.get(name.as_str())
                    && let Some(msg) = self.deprecated_stdlib.get(canonical)
                {
                    let text = format!("'{}' is deprecated: {}", canonical.join("::"), msg);
                    if !self
                        .allow_stack
                        .last()
                        .is_some_and(|s| s.contains(&Lint::Deprecated))
                    {
                        self.warn_lint(Lint::Deprecated, text, span);
                    }
                }
                return self.check_stdlib_call(&f, arg_types, span);
            }
            if self.stdlib_fn_names.contains_key(name.as_str())
                && self
                    .scopes
                    .iter()
                    .rev()
                    .find_map(|scope| scope.get(name))
                    .map(|item| item.type_annotation.clone())
                    .is_none()
            {
                self.error(
                    format!("'{name}' is a stdlib function - import it before use"),
                    span,
                );
                return CheckType::Unknown;
            }
            let item_type = self.lookup(name, span);
            return self.check_call_value(item_type.ty, arg_types, span);
        }

        let suggestion = if let Some(last) = path.last() {
            let candidates = keywords::array::KEYWORDS
                .iter()
                .chain(keywords::audio::KEYWORDS)
                .chain(keywords::bitwise::KEYWORDS)
                .chain(keywords::c::KEYWORDS)
                .chain(keywords::cli::KEYWORDS)
                .chain(keywords::crypto::KEYWORDS)
                .chain(keywords::collections::KEYWORDS)
                .chain(keywords::debug::KEYWORDS)
                .chain(keywords::fs::KEYWORDS)
                .chain(keywords::gui::KEYWORDS)
                .chain(keywords::http::KEYWORDS)
                .chain(keywords::io::KEYWORDS)
                .chain(keywords::math::KEYWORDS)
                .chain(keywords::math::constants::KEYWORDS)
                .chain(keywords::net::KEYWORDS)
                .chain(keywords::path::KEYWORDS)
                .chain(keywords::process::KEYWORDS)
                .chain(keywords::random::KEYWORDS)
                .chain(keywords::result::KEYWORDS)
                .chain(keywords::rl::KEYWORDS)
                .chain(keywords::string::KEYWORDS)
                .chain(keywords::terminal::KEYWORDS)
                .chain(keywords::time::KEYWORDS)
                .chain(keywords::types::KEYWORDS)
                .copied();
            closest_match(last, candidates)
        } else {
            None
        };

        self.error_with_help(
            format!("undefined function {}", path.join("::")),
            span,
            suggestion,
        );

        CheckType::Unknown
    }

    /// Checks a call to a `std` function against its known signature(s), if any (rl-lang#250).
    ///
    /// - An empty `f.signatures` means the function isn't typed yet: the call
    ///   passes through unchecked as [`CheckType::Unknown`] (pre-#250 behavior).
    /// - Otherwise each `(params, return_type)` overload is tried in turn via
    ///   [`unify_arg`], the first whose param types all unify with the call's
    ///   argument types wins, and its `return_type` (after substituting any
    ///   generic bindings) is returned as `CheckType::Known`. This also
    ///   correctly matches a typed lambda (`CheckType::Function`) against a
    ///   concrete (non-generic) `Callback` parameters.
    /// - If no overload matches, an error is reported (arity mismatch if the
    ///   argument *count* doesn't match any overload, otherwise a type
    ///   mismatch) and the first overload's return type is used to keep
    ///   type inference going for the rest of the check pass.
    pub fn check_stdlib_call(
        &mut self,
        f: &StdFn,
        arg_types: &[(CheckType, Span)],
        span: Span,
    ) -> CheckType {
        if f.signatures.is_empty() {
            return CheckType::Unknown;
        }

        for (params, return_type) in &f.signatures {
            let expected = tuple_members(params);
            if expected.len() != arg_types.len() {
                continue;
            }

            let mut bindings = HashMap::new();
            let mut all_match = true;
            for (expected_type, (actual_type, _)) in expected.iter().zip(arg_types.iter()) {
                if !unify_arg(expected_type, actual_type, &mut bindings) {
                    all_match = false;
                    break;
                }
            }
            if all_match {
                let resolved = substitute(return_type, &bindings);
                return if has_generic(&resolved) {
                    CheckType::Unknown
                } else {
                    CheckType::Known(resolved)
                };
            }
        }

        // No overload matched - report the most useful error we can.
        let first_return = &f.signatures[0].1;
        let arities: Vec<usize> = f
            .signatures
            .iter()
            .map(|(p, _)| tuple_members(p).len())
            .collect();

        if !arities.contains(&arg_types.len()) {
            let expected = if arities.iter().all(|a| *a == arities[0]) {
                arities[0].to_string()
            } else {
                let mut sorted = arities.clone();
                sorted.sort_unstable();
                sorted.dedup();
                sorted
                    .iter()
                    .map(usize::to_string)
                    .collect::<Vec<_>>()
                    .join(" or ")
            };
            self.error(
                format!(
                    "{} expects {} argument(s), got {}",
                    f.name,
                    expected,
                    arg_types.len()
                ),
                span,
            );
        } else {
            let got: Vec<String> = arg_types.iter().map(|(t, _)| t.info()).collect();
            self.error(
                format!(
                    "{}: no overload matches argument type(s) ({})",
                    f.name,
                    got.join(", ")
                ),
                span,
            );
        }

        CheckType::Known(first_return.clone())
    }

    /// Checks that a resolved callee value is callable and receives the correct arguments.
    ///
    /// - `Unknown` and `Known(Fn)` pass through without argument checking
    /// - `Function { params, return_type }` validates arity and argument types,
    ///   then returns `Known(return_type)`
    /// - Anything else emits a "not callable" error
    pub fn check_call_value(
        &mut self,
        callee_type: CheckType,
        arg_types: &[(CheckType, Span)],
        span: Span,
    ) -> CheckType {
        match callee_type {
            CheckType::Unknown => CheckType::Unknown,
            CheckType::Known(TypeAnnotation::Fn) => CheckType::Unknown,
            CheckType::Function {
                params,
                return_type,
            } => {
                if params.len() != arg_types.len() {
                    self.error(
                        format!(
                            "function expects {} argument(s), got {}",
                            params.len(),
                            arg_types.len()
                        ),
                        span,
                    );
                    return CheckType::Known(return_type);
                }
                for (expected_type, (actual_type, arg_span)) in params.iter().zip(arg_types.iter())
                {
                    let expected = CheckType::Known(expected_type.clone());

                    if !actual_type.matches(&expected) {
                        self.error(
                            format!(
                                "type mismatch: expected {}, got {}",
                                expected.info(),
                                actual_type.info()
                            ),
                            *arg_span,
                        );
                    }
                }
                CheckType::Known(return_type)
            }

            other => {
                self.error(format!("{} is not callable", other.info()), span);
                CheckType::Unknown
            }
        }
    }
}

/// Unwraps the `params` half of a `StdFn` overload (built as
/// `TypeAnnotation::Tuple(..)` by `rl_commons::stdlib_signatures`) back into
/// a plain list of expected argument types.
fn tuple_members(params: &TypeAnnotation) -> Vec<TypeAnnotation> {
    match params {
        TypeAnnotation::Tuple(v) | TypeAnnotation::CTuple(v) => v.as_ref().clone(),
        other => vec![other.clone()],
    }
}
