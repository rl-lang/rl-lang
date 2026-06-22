use crate::{
    ast::statements::{StatementKind, TypeAnnotation},
    checker::structs::{CheckType, TypeChecker},
};

impl TypeChecker {
    // checks the current statement and push errors via error() if any found
    pub fn check_statement(&mut self, statement: &crate::ast::statements::Statement) {
        match &statement.kind {
            // checks if the type null or same type then declare it as variable
            // otherwise pushs error
            StatementKind::VariableDeclaration {
                name,
                type_annotation,
                value,
            } => {
                let value_type = self.check_expression(value);
                let declared = CheckType::Known(type_annotation.clone());

                let widens = matches!(
                    (type_annotation, &value_type),
                    (
                        TypeAnnotation::Int | TypeAnnotation::CInt,
                        CheckType::Known(TypeAnnotation::Byte | TypeAnnotation::CByte)
                    )
                );

                if !widens && !value_type.matches(&declared) {
                    self.error(
                        format!(
                            "type mismatch: expected {}, got {}",
                            declared.info(),
                            value_type.info()
                        ),
                        statement.span,
                    );
                }

                self.declare(name.clone(), declared, false, statement.span);
            }

            // checks if the type is null or same type and declares it as
            // constant otherwise pushs error
            StatementKind::ConstantDeclaration {
                name,
                type_annotation,
                value,
            } => {
                let value_type = self.check_expression(value).into_const();
                let declared = CheckType::Known(type_annotation.clone());

                let widens = matches!(
                    (type_annotation, &value_type),
                    (
                        TypeAnnotation::Int | TypeAnnotation::CInt,
                        CheckType::Known(TypeAnnotation::Byte | TypeAnnotation::CByte)
                    )
                );

                if !widens && !value_type.matches(&declared) {
                    self.error(
                        format!(
                            "type mismatch: expected {}, got {}",
                            declared.info(),
                            value_type.info()
                        ),
                        statement.span,
                    );
                }

                self.declare(name.clone(), declared, true, statement.span);
            }

            // checks the array if valid or not and declares it with correct
            // type (weather constant or variable) otherwise pushs error
            StatementKind::Array {
                name,
                type_annotation,
                value,
            } => {
                for item in value {
                    let item_type = self.check_expression(item);
                    let expected = CheckType::Known(type_annotation.clone());

                    let widens = matches!(
                        (type_annotation, &item_type),
                        (
                            TypeAnnotation::Int | TypeAnnotation::CInt,
                            CheckType::Known(TypeAnnotation::Byte | TypeAnnotation::CByte)
                        ) | (
                            TypeAnnotation::Array(_) | TypeAnnotation::CArray(_),
                            CheckType::Known(TypeAnnotation::Array(_) | TypeAnnotation::CArray(_))
                        )
                    );

                    if !widens && !item_type.matches(&expected) {
                        self.error(
                            format!(
                                "type mismatch: array expects {}, got {}",
                                expected.info(),
                                item_type.info()
                            ),
                            item.span,
                        );
                    }
                }

                let array_type =
                    CheckType::Known(TypeAnnotation::Array(Box::new(type_annotation.clone())));

                self.declare(name.clone(), array_type, false, statement.span);
            }
            StatementKind::ConstantArray {
                name,
                type_annotation,
                value,
            } => {
                for item in value {
                    let item_type = self.check_expression(item);
                    let expected = CheckType::Known(type_annotation.clone());

                    let widens = matches!(
                        (type_annotation, &item_type),
                        (
                            TypeAnnotation::Int | TypeAnnotation::CInt,
                            CheckType::Known(TypeAnnotation::Byte | TypeAnnotation::CByte)
                        ) | (
                            TypeAnnotation::Array(_) | TypeAnnotation::CArray(_),
                            CheckType::Known(TypeAnnotation::Array(_) | TypeAnnotation::CArray(_))
                        )
                    );

                    if !widens && !item_type.matches(&expected) {
                        self.error(
                            format!(
                                "type mismatch: array expects {}, got {}",
                                expected.info(),
                                item_type.info()
                            ),
                            item.span,
                        );
                    }
                }

                let array_type =
                    CheckType::Known(TypeAnnotation::CArray(Box::new(type_annotation.clone())));

                self.declare(name.clone(), array_type, true, statement.span);
            }

            // offloads to expression checker
            StatementKind::Expression(expr) => {
                self.check_expression(expr);
            }

            // loops checker
            StatementKind::While { condition, body } => {
                // is condition type is bool?
                let condition_type = self.check_expression(condition);
                if !matches!(
                    condition_type,
                    CheckType::Known(TypeAnnotation::Bool | TypeAnnotation::CBool)
                        | CheckType::Unknown
                ) {
                    self.error(
                        format!(
                            "while condition must be bool, got {}",
                            condition_type.info()
                        ),
                        condition.span,
                    );
                }
                // add loop depth
                self.enter_loop();
                // checks the blocks
                self.check_block(body);
                // remove loop depth
                self.exit_loop();
            }

            StatementKind::For {
                initializer,
                condition,
                increment,
                body,
            } => {
                // add scope level
                self.push_scope();
                // is the initializer correct?
                self.check_statement(initializer);
                // is the condition bool?
                let condition_type = self.check_expression(condition);
                if !matches!(
                    condition_type,
                    CheckType::Known(TypeAnnotation::Bool | TypeAnnotation::CBool)
                        | CheckType::Unknown
                ) {
                    self.error(
                        format!("for condition must be bool, got {}", condition_type.info()),
                        condition.span,
                    );
                }
                // is the increment correct?
                self.check_expression(increment);
                // add loop depth
                self.enter_loop();
                // is body correct?
                for stmt in body {
                    self.check_statement(stmt);
                }
                // remove loop depth
                self.exit_loop();
                // remove scope level
                self.pop_scope();
            }

            StatementKind::ForRange {
                variable,
                range,
                body,
            } => {
                // range for StatementKind::Range
                let _ = range;
                // add loop depth
                self.enter_loop();
                // add scope level
                self.push_scope();
                // declare the range variable
                self.declare(
                    variable.clone(),
                    CheckType::Known(TypeAnnotation::Int),
                    false,
                    statement.span,
                );
                // is the body correct?
                for stmt in body {
                    self.check_statement(stmt);
                }
                // remove scope level
                self.pop_scope();
                // remove loop depth
                self.exit_loop();
            }

            StatementKind::ForEach {
                variable,
                iterable,
                body,
            } => {
                // is the iterable correct?
                let iter_type = self.check_expression(iterable);
                // is the ieterable items correct?
                let item_type = match &iter_type {
                    CheckType::Known(TypeAnnotation::Array(inner))
                    | CheckType::Known(TypeAnnotation::CArray(inner)) => {
                        CheckType::Known((**inner).clone())
                    }
                    CheckType::Unknown => CheckType::Unknown,
                    other => {
                        self.error(
                            format!("for-each: expected an array, got {}", other.info()),
                            iterable.span,
                        );
                        CheckType::Unknown
                    }
                };
                // add loop depth
                self.enter_loop();
                // add scope depth
                self.push_scope();
                // declares the iterable variable
                self.declare(variable.clone(), item_type, false, statement.span);
                // is body correct?
                for stmt in body {
                    self.check_statement(stmt);
                }
                // remove scope depth
                self.pop_scope();
                // remove loop depth
                self.exit_loop();
            }

            StatementKind::Range(_) => {}

            // if - else if - else
            StatementKind::ConditionalBranch { condition, body } => {
                // is there condition? or is it else?
                if let Some(cond) = condition {
                    // is the condition bool?
                    let condition_type = self.check_expression(cond);
                    if !matches!(
                        condition_type,
                        CheckType::Known(TypeAnnotation::Bool | TypeAnnotation::CBool)
                            | CheckType::Unknown
                    ) {
                        self.error(
                            format!("condition must be bool, got {}", condition_type.info()),
                            cond.span,
                        );
                    }
                }
                // is the body correect?
                self.check_block(body);
            }

            StatementKind::Conditional {
                if_branch,
                else_branch,
            } => {
                // is the branch correct?
                self.check_statement(if_branch);
                // if there is another branch is it correct?
                if let Some(branch) = else_branch {
                    self.check_statement(branch);
                }
            }

            // functions and lambdas
            StatementKind::FunctionDeclaration {
                name,
                params,
                return_type,
                body,
                ..
            } => {
                let fn_type = CheckType::Function {
                    params: params.iter().map(|p| p.param_type.clone()).collect(),
                    return_type: return_type.clone(),
                };
                // declaring for recusring to work
                self.declare(name.clone(), fn_type, false, statement.span);
                // add scope level
                self.push_scope();
                // declare the params
                for param in params {
                    self.declare(
                        param.param_name.clone(),
                        CheckType::Known(param.param_type.clone()),
                        false,
                        statement.span,
                    );
                }
                // add return type
                self.push_return_type(return_type.clone());
                // is the body correct?
                for stmt in body {
                    self.check_statement(stmt);
                }
                // remove the return type
                self.pop_return_type();
                // remove scope level
                self.pop_scope();
            }

            StatementKind::Return(expr) => {
                // is the expression a valid type? otherwise null
                let actual_type = match expr {
                    Some(e) => self.check_expression(e),
                    None => CheckType::Known(TypeAnnotation::Null),
                };
                // is the actual type same as the expected return one?
                if let Some(expected) = self.current_return_type().cloned() {
                    let widens = matches!(
                        (expected.clone(), actual_type.clone()),
                        (
                            TypeAnnotation::Int | TypeAnnotation::CInt,
                            CheckType::Known(TypeAnnotation::Byte | TypeAnnotation::CByte)
                        )
                    );
                    if expected != TypeAnnotation::Null {
                        let expected_type = CheckType::Known(expected.clone());
                        if !widens && !actual_type.matches(&expected_type) {
                            self.error(
                                format!(
                                    "return type mismatch: expected {}, got {}",
                                    expected_type.info(),
                                    actual_type.info()
                                ),
                                statement.span,
                            );
                        }
                    }
                } else {
                    // return outside a function
                    self.error("return outside of function", statement.span);
                }
            }

            // checks weather break or continue used outside of loops
            StatementKind::Break => {
                if self.loop_depth() == 0 {
                    self.error("break outside of loop", statement.span);
                }
            }
            StatementKind::Continue => {
                if self.loop_depth() == 0 {
                    self.error("continue outside of loop", statement.span);
                }
            }

            // runtime job maybe revisting later
            StatementKind::Import { .. }
            | StatementKind::ImportFile { .. }
            | StatementKind::ImportFileNamed { .. } => {}
        }
    }
}
