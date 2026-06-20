use crate::{
    ast::statements::TypeAnnotation,
    checker::structs::{CheckType, TypeChecker},
    interpreter::stdlib,
    utils::{span::Span, suggest::closest_match},
};

impl TypeChecker {
    // is it callable path?
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

            if path.len() == 1 {
                let name = &path[0];

                if self.stdlib_fn_names.contains(name.as_str()) {
                    self.push_stdlib_hover(path, span);
                    return CheckType::Unknown;
                }

                let item_type = self.lookup(name, span);
                return self.check_call_value(item_type, arg_types, span);
            }

            // user defined function in scope
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

    // is it callable?
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
