//! Function call type checking - path resolution and argument validation.

use crate::{
    ast::statements::TypeAnnotation,
    checker::structs::{CheckType, TypeChecker},
    interpreter::stdlib,
    utils::{span::Span, suggest::closest_match},
};

impl TypeChecker {
    /// Resolves a call path and checks argument types.
    ///
    /// Resolution order:
    /// 1. Full stdlib path (`std::io::print`) via [`root_module`]
    /// 2. Single-name stdlib shorthand (`print`) via [`stdlib_fn_names`]
    /// 3. User-defined name via [`lookup`]
    /// 4. Error - unknown function, with a "did you mean?" suggestion from stdlib keywords
    ///
    /// Stdlib calls always return [`CheckType::Unknown`] since their return
    /// types are not tracked statically.
    pub fn check_call_path(
        &mut self,
        path: &[String],
        arg_types: &[(CheckType, Span)],
        span: Span,
    ) -> CheckType {
        // stdlib path (std::io::print)
        if self.root_module.resolve(path).is_some() {
            self.push_stdlib_hover(path, span);
            return CheckType::Unknown;
        }

        if path.len() == 1 {
            let name = &path[0];

            if self.stdlib_fn_names.contains(name.as_str()) {
                self.push_stdlib_hover(path, span);
                return CheckType::Unknown;
            }
            let item_type = self.lookup(name, span);
            return self.check_call_value(item_type, arg_types, span);
        }

        let suggestion = if let Some(last) = path.last() {
            let candidates = stdlib::math::KEYWORDS
                .iter()
                .chain(stdlib::math::constants::KEYWORDS)
                .chain(stdlib::io::KEYWORDS)
                .chain(stdlib::string::KEYWORDS)
                .chain(stdlib::types::KEYWORDS)
                .chain(stdlib::array::KEYWORDS)
                .chain(stdlib::path::KEYWORDS)
                .chain(stdlib::fs::KEYWORDS)
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
