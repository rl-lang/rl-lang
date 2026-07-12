//! Expression type checking - walks every [`ExpressionKind`] variant and
//! returns the static [`CheckType`] of the expression.

use std::rc::Rc;

use crate::{TypeChecker, structs::CheckType};
use rl_ast::{ExprId, nodes::ExpressionKind, statements::TypeAnnotation};
use rl_utils::span::Span;

impl TypeChecker {
    pub fn check_expression(&mut self, expression: ExprId) -> CheckType {
        let expr_span = self.ast_arena.exprs.get(expression).span;
        let expr_kind = self.ast_arena.exprs.get(expression).kind.clone();
        match expr_kind {
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
            ExpressionKind::Identifier(name) => self.lookup(&name, expr_span),
            ExpressionKind::MapLiteral(entries) => {
                let mut key_types = Vec::with_capacity(entries.len());
                let mut value_types = Vec::with_capacity(entries.len());
                for (key, value) in entries {
                    let key_span = self.ast_arena.exprs.get(key).span;
                    let value_span = self.ast_arena.exprs.get(value).span;
                    key_types.push((self.check_expression(key), key_span));
                    value_types.push((self.check_expression(value), value_span));
                }

                let key_type = key_types
                    .first()
                    .map(|(t, _)| Self::to_type_annotation(t))
                    .unwrap_or(TypeAnnotation::Null);
                let value_type = value_types
                    .first()
                    .map(|(t, _)| Self::to_type_annotation(t))
                    .unwrap_or(TypeAnnotation::Null);

                if let Some((first_key, _)) = key_types.first().cloned() {
                    for (kt, span) in key_types.iter().skip(1) {
                        if !kt.is_null() && !first_key.is_null() && !kt.matches(&first_key) {
                            self.error(
                                format!(
                                    "map key type mismatch: expected {}, got {}",
                                    first_key.info(),
                                    kt.info()
                                ),
                                *span,
                            );
                        }
                    }
                }
                if let Some((first_value, _)) = value_types.first().cloned() {
                    for (vt, span) in value_types.iter().skip(1) {
                        if !vt.is_null() && !first_value.is_null() && !vt.matches(&first_value) {
                            self.error(
                                format!(
                                    "map value type mismatch: expected {}, got {}",
                                    first_value.info(),
                                    vt.info()
                                ),
                                *span,
                            );
                        }
                    }
                }

                for (kt, span) in &key_types {
                    if !kt.is_null() && !Self::is_hashable_key_type(&Self::to_type_annotation(kt)) {
                        self.error(
                            format!("type {} cannot be used as a map key", kt.info()),
                            *span,
                        );
                    }
                }

                CheckType::Known(TypeAnnotation::Map(
                    Box::new(key_type),
                    Box::new(value_type),
                ))
            }
            // checks array items
            ExpressionKind::ArrayLiteral(items) => {
                // checks every item type in items then push it to the new
                // item_types vec
                let mut item_types = Vec::with_capacity(items.len());
                for item in items {
                    let item_span = self.ast_arena.exprs.get(item).span;
                    item_types.push((self.check_expression(item), item_span));
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
                let target_span = self.ast_arena.exprs.get(target).span;
                let index_span = self.ast_arena.exprs.get(index).span;
                let target_type = self.check_expression(target);
                self.check_is_null(&target_type, target_span);
                // is the index null??
                let index_type = self.check_expression(index);
                self.check_is_null(&index_type, index_span);

                // is it integer? (only enforced for array/tuple targets -
                // maps validate the index against their declared key type
                // further down instead)
                let target_is_map = matches!(
                    target_type,
                    CheckType::Known(TypeAnnotation::Map(_, _) | TypeAnnotation::CMap(_, _))
                );
                if !target_is_map
                    && !matches!(
                        index_type,
                        CheckType::Known(
                            TypeAnnotation::Int
                                | TypeAnnotation::CInt
                                | TypeAnnotation::Byte
                                | TypeAnnotation::CByte
                        ) | CheckType::Unknown
                    )
                {
                    self.error(
                        format!("invalid index operation: index is {}", index_type.info()),
                        expr_span,
                    );
                }

                // is the target actually an array?
                // if it is array return its items type
                match &target_type {
                    CheckType::Known(TypeAnnotation::Array(inner))
                    | CheckType::Known(TypeAnnotation::CArray(inner)) => {
                        CheckType::Known((**inner).clone())
                    }
                    CheckType::Known(TypeAnnotation::Set(inner))
                    | CheckType::Known(TypeAnnotation::CSet(inner)) => {
                        CheckType::Known((**inner).clone())
                    }
                    CheckType::Known(TypeAnnotation::Map(key_ty, value_ty))
                    | CheckType::Known(TypeAnnotation::CMap(key_ty, value_ty)) => {
                        let expected_key = CheckType::Known((**key_ty).clone());
                        if !index_type.matches(&expected_key) {
                            self.error(
                                format!(
                                    "map key type mismatch: expected {}, got {}",
                                    expected_key.info(),
                                    index_type.info()
                                ),
                                index_span,
                            );
                        }
                        CheckType::Known((**value_ty).clone())
                    }
                    CheckType::Unknown | CheckType::Known(TypeAnnotation::Null) => {
                        CheckType::Unknown
                    }
                    CheckType::Known(TypeAnnotation::Tuple(_) | TypeAnnotation::CTuple(_)) => {
                        CheckType::Unknown
                    }
                    other => {
                        self.error(
                            format!("invalid index operation: this is {}", other.info()),
                            expr_span,
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
            } => self.check_index_assign(target, index, value, expr_span),

            // offloads to binary
            ExpressionKind::Binary {
                left,
                operator,
                right,
            } => {
                // is the left operand null?
                let left_type = self.check_expression(left);
                let left_id = self.ast_arena.exprs.get(left);
                self.check_is_null(&left_type, left_id.span);
                // is the right operand null?
                let right_type = self.check_expression(right);
                let right_id = self.ast_arena.exprs.get(right);
                self.check_is_null(&right_type, right_id.span);
                // is the binary correct?
                self.check_binary_operator(left_type, right_type, &operator, expr_span)
            }

            // offloads to unary
            ExpressionKind::Unary { operator, operand } => {
                // is the operand null?
                let operand_span = self.ast_arena.exprs.get(operand).span;
                let operand_type = self.check_expression(operand);
                self.check_is_null(&operand_type, operand_span);
                // is the unary correct?
                self.check_unary_operator(operand_type, operand_span, &operator, expr_span)
            }

            // assigns the value to the variable then returns it
            ExpressionKind::Assign { name, value } => {
                let value_type = self.check_expression(value);
                self.assign(&name, value_type.clone(), expr_span);
                value_type
            }

            // checks the call path of the function
            ExpressionKind::Call { path, args } => {
                let arg_types: Vec<(CheckType, Span)> = args
                    .iter()
                    .map(|a| {
                        let a = *a;
                        let a_span = self.ast_arena.exprs.get(a).span;
                        (self.check_expression(a), a_span)
                    })
                    .collect();
                self.check_call_path(&path, &arg_types, expr_span)
            }

            // checks the call of the function
            ExpressionKind::CallExpr { callee, args } => {
                let callee_type = self.check_expression(callee);
                let arg_types: Vec<(CheckType, Span)> = args
                    .iter()
                    .map(|a| {
                        let a = *a;
                        let a_span = self.ast_arena.exprs.get(a).span;
                        (self.check_expression(a), a_span)
                    })
                    .collect();
                self.check_call_value(callee_type, &arg_types, expr_span)
            }

            // checks the method call
            ExpressionKind::MethodCall {
                caller,
                method,
                args,
            } => {
                let caller_type = self.check_expression(caller);
                let caller_id = self.ast_arena.exprs.get(caller);
                let mut arg_types: Vec<(CheckType, Span)> = vec![(caller_type, caller_id.span)];
                for arg in args {
                    let arg_span = self.ast_arena.exprs.get(arg).span;
                    arg_types.push((self.check_expression(arg), arg_span));
                }
                self.check_call_path(&method, &arg_types, expr_span)
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
                for param in &params {
                    self.declare(
                        param.param_name.clone(),
                        CheckType::Known(param.param_type.clone()),
                        false,
                        expr_span,
                    );
                }
                // add the resolved return type as the expected return
                self.push_return_type(resolved_return.clone());
                // is the body correct?
                for statement in &body {
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

            ExpressionKind::Cast { value, target_type } => {
                let value_type = self.check_expression(value);
                let value_id = self.ast_arena.exprs.get(value);
                self.check_is_null(&value_type, value_id.span);

                let castable = matches!(
                    &value_type,
                    CheckType::Known(
                        TypeAnnotation::CInt
                            | TypeAnnotation::CByte
                            | TypeAnnotation::CFloat
                            | TypeAnnotation::Float
                            | TypeAnnotation::Int
                            | TypeAnnotation::Byte
                    ) | CheckType::Unknown
                );

                let valid_target = matches!(
                    target_type,
                    TypeAnnotation::Int | TypeAnnotation::Float | TypeAnnotation::Byte
                );

                if !castable || !valid_target {
                    self.error(
                        format!(
                            "invalid cast: cannot cast {} to {:?}",
                            value_type.info(),
                            target_type
                        ),
                        expr_span,
                    );
                }
                CheckType::Unknown
            }

            ExpressionKind::TupleLiteral(items) => {
                let types: Vec<TypeAnnotation> = items
                    .iter()
                    .map(|item| {
                        let t = self.check_expression(*item);
                        Self::to_type_annotation(&t)
                    })
                    .collect();
                CheckType::Known(TypeAnnotation::Tuple(Rc::new(types)))
            }
            ExpressionKind::ErrorLiteral(inner) => {
                let inner_type = self.check_expression(inner);
                if matches!(
                    inner_type,
                    CheckType::Known(TypeAnnotation::Error | TypeAnnotation::CError)
                ) {
                    self.error("error cannot wrap another error", expr_span);
                }
                CheckType::Known(TypeAnnotation::Error)
            }
            ExpressionKind::OkLiteral(inner) => {
                let inner_ann = Self::to_type_annotation(&self.check_expression(inner));
                CheckType::Known(TypeAnnotation::Result(Box::new(inner_ann)))
            }
            ExpressionKind::ErrLiteral(inner) => {
                let inner_ann = Self::to_type_annotation(&self.check_expression(inner));
                CheckType::Known(TypeAnnotation::Result(Box::new(inner_ann)))
            }

            ExpressionKind::Propagate(inner) => {
                self.check_expression(inner);
                CheckType::Unknown
            }

            ExpressionKind::StructLiteral { name, fields } => {
                if let Some(declared_fields) = self.records.get(&name).cloned() {
                    for (field_name, value) in &fields {
                        let value_span = self.ast_arena.exprs.get(*value).span;
                        let value_type = self.check_expression(*value);
                        match declared_fields.iter().find(|(n, _)| n == field_name) {
                            Some((_, field_type)) => {
                                let expected = CheckType::Known(field_type.clone());
                                if !value_type.matches(&expected) {
                                    self.error(
                                        format!(
                                            "field `{}` of record `{}` expects {}, got {}",
                                            field_name,
                                            name,
                                            expected.info(),
                                            value_type.info()
                                        ),
                                        value_span,
                                    );
                                }
                            }
                            None => {
                                self.error(
                                    format!("record `{}` has no field `{}`", name, field_name),
                                    value_span,
                                );
                            }
                        }
                    }
                    if fields.len() != declared_fields.len() {
                        self.error(
                            format!(
                                "record `{}` expects {} field(s), got {}",
                                name,
                                declared_fields.len(),
                                fields.len()
                            ),
                            expr_span,
                        );
                    }
                } else {
                    self.error(format!("unknown record type `{}`", name), expr_span);
                    for (_, value) in &fields {
                        self.check_expression(*value);
                    }
                }
                CheckType::Known(TypeAnnotation::Record(name))
            }

            ExpressionKind::FieldAccess { target, field } => {
                let target_type = self.check_expression(target);
                match &target_type {
                    CheckType::Known(
                        TypeAnnotation::Record(name) | TypeAnnotation::CRecord(name),
                    ) => {
                        match self.records.get(name).and_then(|fs| {
                            fs.iter().find(|(n, _)| *n == field).map(|(_, t)| t.clone())
                        }) {
                            Some(field_type) => CheckType::Known(field_type),
                            None => {
                                self.error(
                                    format!("record `{}` has no field `{}`", name, field),
                                    expr_span,
                                );
                                CheckType::Unknown
                            }
                        }
                    }
                    CheckType::Unknown => CheckType::Unknown,
                    other => {
                        self.error(
                            format!("cannot access field `{}` on {}", field, other.info()),
                            expr_span,
                        );
                        CheckType::Unknown
                    }
                }
            }

            ExpressionKind::FieldAssign {
                target,
                field,
                value,
            } => {
                let target_type = self.check_expression(target);
                let value_type = self.check_expression(value);
                match &target_type {
                    CheckType::Known(
                        TypeAnnotation::Record(name) | TypeAnnotation::CRecord(name),
                    ) => {
                        match self.records.get(name).and_then(|fs| {
                            fs.iter().find(|(n, _)| *n == field).map(|(_, t)| t.clone())
                        }) {
                            Some(field_type) => {
                                let expected = CheckType::Known(field_type);
                                if !value_type.matches(&expected) {
                                    self.error(
                                        format!(
                                            "field `{}` of record `{}` expects {}, got {}",
                                            field,
                                            name,
                                            expected.info(),
                                            value_type.info()
                                        ),
                                        expr_span,
                                    );
                                }
                            }
                            None => {
                                self.error(
                                    format!("record `{}` has no field `{}`", name, field),
                                    expr_span,
                                );
                            }
                        }
                    }
                    CheckType::Unknown => {}
                    other => {
                        self.error(
                            format!("cannot assign field `{}` on {}", field, other.info()),
                            expr_span,
                        );
                    }
                }
                value_type
            }

            ExpressionKind::EnumVariant { enum_name, variant } => {
                match self.tags.get(&enum_name) {
                    Some(variants) => {
                        if !variants.contains(&variant) {
                            self.error(
                                format!("tag `{}` has no variant `{}`", enum_name, variant),
                                expr_span,
                            );
                        }
                    }
                    None => {
                        self.error(format!("unknown tag type `{}`", enum_name), expr_span);
                    }
                }
                CheckType::Known(TypeAnnotation::Enum(enum_name))
            }

            ExpressionKind::SetLiteral(items) => {
                let mut item_types = Vec::with_capacity(items.len());
                for item in items {
                    let item_span = self.ast_arena.exprs.get(item).span;
                    item_types.push((self.check_expression(item), item_span));
                }

                let items_type = item_types
                    .first()
                    .map(|(t, _)| Self::to_type_annotation(t))
                    .unwrap_or(TypeAnnotation::Null);

                if let Some((first_type, _)) = item_types.first().cloned() {
                    for (item_type, span) in item_types.iter().skip(1) {
                        if !item_type.is_null()
                            && !first_type.is_null()
                            && !item_type.matches(&first_type)
                        {
                            self.error(
                                format!(
                                    "set element type mismatch: expected {}, got {}",
                                    first_type.info(),
                                    item_type.info()
                                ),
                                *span,
                            );
                        }
                    }
                }

                CheckType::Known(TypeAnnotation::Set(Box::new(items_type)))
            }

            _ => CheckType::Unknown,
        }
    }
}
