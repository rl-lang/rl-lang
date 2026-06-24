//! Expression type checking - walks every [`ExpressionKind`] variant and
//! returns the static [`CheckType`] of the expression.

use crate::{
    ast::{
        nodes::{Expression, ExpressionKind},
        statements::TypeAnnotation,
    },
    checker::{TypeChecker, structs::CheckType},
    utils::span::Span,
};

impl TypeChecker {
    pub fn check_expression(&mut self, expression: &Expression) -> CheckType {
        match &expression.kind {
            // returns as type
            ExpressionKind::Null => CheckType::Known(TypeAnnotation::Null),
            ExpressionKind::Integer(_) => CheckType::Known(TypeAnnotation::Int),
            ExpressionKind::Byte(_) => CheckType::Known(TypeAnnotation::Byte),
            ExpressionKind::String(_) => CheckType::Known(TypeAnnotation::String),
            ExpressionKind::Bool(_) => CheckType::Known(TypeAnnotation::Bool),
            ExpressionKind::Float(_) => CheckType::Known(TypeAnnotation::Float),
            ExpressionKind::Character(_) => CheckType::Known(TypeAnnotation::Char),
            // returns the inner type
            ExpressionKind::Grouping(inner) => self.check_expression(inner),
            // does this identifier exist?
            ExpressionKind::Identifier(name) => self.lookup(name, expression.span),
            // checks array items
            ExpressionKind::ArrayLiteral(items) => {
                // checks every item type in items then push it to the new
                // item_types vec
                let mut item_types = Vec::with_capacity(items.len());
                for item in items {
                    item_types.push((self.check_expression(item), item.span));
                }
                // sets the items types to same first item type otherwise null
                let items_type = item_types
                    .first()
                    .map(|(t, _)| Self::to_type_annotation(t))
                    .unwrap_or(TypeAnnotation::Null);

                // same items type or not?
                if let Some((first_type, _)) = item_types.first().cloned() {
                    for (item_type, span) in item_types.iter().skip(1) {
                        if !item_type.is_null()
                            && !first_type.is_null()
                            && !item_type.matches(&first_type)
                        {
                            self.error(
                                format!(
                                    "array element type mismatch: expected {}, got {}",
                                    first_type.info(),
                                    item_type.info()
                                ),
                                *span,
                            );
                        }
                    }
                }
                // returns the array type
                CheckType::Known(TypeAnnotation::Array(Box::new(items_type)))
            }

            ExpressionKind::Index { target, index } => {
                // is the target (array) null?
                let target_type = self.check_expression(target);
                self.check_is_null(&target_type, target.span);
                // is the index null??
                let index_type = self.check_expression(index);
                self.check_is_null(&index_type, index.span);

                // is it integer?
                if !matches!(
                    index_type,
                    CheckType::Known(
                        TypeAnnotation::Int
                            | TypeAnnotation::CInt
                            | TypeAnnotation::Byte
                            | TypeAnnotation::CByte
                    ) | CheckType::Unknown
                ) {
                    self.error(
                        format!("invalid index operation: index is {}", index_type.info()),
                        expression.span,
                    );
                }

                // is the target actually an array?
                // if it is array return its items type
                match &target_type {
                    CheckType::Known(TypeAnnotation::Array(inner))
                    | CheckType::Known(TypeAnnotation::CArray(inner)) => {
                        CheckType::Known((**inner).clone())
                    }
                    CheckType::Unknown | CheckType::Known(TypeAnnotation::Null) => {
                        CheckType::Unknown
                    }
                    other => {
                        self.error(
                            format!("invalid index operation: this is {}", other.info()),
                            expression.span,
                        );
                        CheckType::Unknown
                    }
                }
            }

            // offloads to index_assign
            ExpressionKind::IndexAssign {
                target,
                index,
                value,
            } => self.check_index_assign(target, index, value, expression.span),

            // offloads to binary
            ExpressionKind::Binary {
                left,
                operator,
                right,
            } => {
                // is the left operand null?
                let left_type = self.check_expression(left);
                self.check_is_null(&left_type, left.span);
                // is the right operand null?
                let right_type = self.check_expression(right);
                self.check_is_null(&right_type, right.span);
                // is the binary correct?
                self.check_binary_operator(left_type, right_type, operator, expression.span)
            }

            // offloads to unary
            ExpressionKind::Unary { operator, operand } => {
                // is the operand null?
                let operand_type = self.check_expression(operand);
                self.check_is_null(&operand_type, operand.span);
                // is the unary correct?
                self.check_unary_operator(operand_type, operand.span, operator, expression.span)
            }

            // assigns the value to the variable then returns it
            ExpressionKind::Assign { name, value } => {
                let value_type = self.check_expression(value);
                self.assign(name, value_type.clone(), expression.span);
                value_type
            }

            // checks the call path of the function
            ExpressionKind::Call { path, args } => {
                let arg_types: Vec<(CheckType, Span)> = args
                    .iter()
                    .map(|a| (self.check_expression(a), a.span))
                    .collect();
                self.check_call_path(path, &arg_types, expression.span)
            }

            // checks the call of the function
            ExpressionKind::CallExpr { callee, args } => {
                let callee_type = self.check_expression(callee);
                let arg_types: Vec<(CheckType, Span)> = args
                    .iter()
                    .map(|a| (self.check_expression(a), a.span))
                    .collect();
                self.check_call_value(callee_type, &arg_types, expression.span)
            }

            // checks the method call
            ExpressionKind::MethodCall {
                caller,
                method,
                args,
            } => {
                let caller_type = self.check_expression(caller);
                let mut arg_types: Vec<(CheckType, Span)> = vec![(caller_type, caller.span)];
                for arg in args {
                    arg_types.push((self.check_expression(arg), arg.span));
                }
                self.check_call_path(method, &arg_types, expression.span)
            }

            // checks the lambda and transforms it to function type
            ExpressionKind::Lambda {
                params,
                return_type,
                body,
            } => {
                // resolves the return type
                let resolved_return = return_type.clone().unwrap_or(TypeAnnotation::Null);
                // add scope level
                self.push_scope();
                // declare the params
                for param in params {
                    self.declare(
                        param.param_name.clone(),
                        CheckType::Known(param.param_type.clone()),
                        false,
                        expression.span,
                    );
                }
                // add the resolved return type as the expected return
                self.push_return_type(resolved_return.clone());
                // is the body correct?
                for statement in body {
                    self.check_statement(statement);
                }
                // removes return type
                self.pop_return_type();
                // remove scope level
                self.pop_scope();

                CheckType::Function {
                    params: params.iter().map(|p| p.param_type.clone()).collect(),
                    return_type: resolved_return,
                }
            }
            _ => CheckType::Unknown,
        }
    }
}
