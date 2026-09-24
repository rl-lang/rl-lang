//! Expression type checking - walks every [`ExpressionKind`] variant and
//! returns the static [`CheckType`] (and unit of measure, when known) of the
//! expression.

use std::rc::Rc;

use crate::{
    TypeChecker,
    structs::{CheckType, CheckedExpr},
};
use rl_ast::{ExprId, nodes::ExpressionKind, statements::TypeAnnotation};
use rl_utils::span::Span;

impl TypeChecker {
    /// Checks `expression` and returns only its static [`CheckType`].
    pub fn check_expression(&mut self, expression: ExprId) -> CheckType {
        self.check_expression_typed(expression).ty
    }

    /// Checks `expression` and returns its static type together with the
    /// compile-time unit of measure it resolves to (if any).
    pub fn check_expression_typed(&mut self, expression: ExprId) -> CheckedExpr {
        let expr_span = self.ast_arena.exprs.get(expression).span;
        let expr_kind = self.ast_arena.exprs.get(expression).kind.clone();
        match expr_kind {
            // returns as type
            ExpressionKind::Null => CheckedExpr::new(CheckType::Known(TypeAnnotation::Null), None),
            ExpressionKind::Integer(_) => {
                CheckedExpr::new(CheckType::Known(TypeAnnotation::Int), None)
            }
            ExpressionKind::UInt(_) => {
                CheckedExpr::new(CheckType::Known(TypeAnnotation::UInt), None)
            }
            ExpressionKind::Byte(_) => {
                CheckedExpr::new(CheckType::Known(TypeAnnotation::Byte), None)
            }
            ExpressionKind::String(_) => {
                CheckedExpr::new(CheckType::Known(TypeAnnotation::String), None)
            }
            ExpressionKind::Bool(_) => {
                CheckedExpr::new(CheckType::Known(TypeAnnotation::Bool), None)
            }
            ExpressionKind::Float(_) => {
                CheckedExpr::new(CheckType::Known(TypeAnnotation::Float), None)
            }
            ExpressionKind::Character(_) => {
                CheckedExpr::new(CheckType::Known(TypeAnnotation::Char), None)
            }
            // returns the inner type and unit
            ExpressionKind::Grouping(inner) => self.check_expression_typed(inner),
            // does this identifier exist?
            ExpressionKind::Identifier(name) => self.lookup(&name, expr_span),
            ExpressionKind::MapLiteral(entries) => {
                let mut key_types = Vec::with_capacity(entries.len());
                let mut value_types = Vec::with_capacity(entries.len());
                for (key, value) in entries {
                    let key_span = self.ast_arena.exprs.get(key).span;
                    let value_span = self.ast_arena.exprs.get(value).span;
                    key_types.push((self.check_expression_typed(key), key_span));
                    value_types.push((self.check_expression_typed(value), value_span));
                }

                let key_type = key_types
                    .first()
                    .map(|(t, _)| Self::to_type_annotation(&t.ty))
                    .unwrap_or(TypeAnnotation::Null);
                let value_type = value_types
                    .first()
                    .map(|(t, _)| Self::to_type_annotation(&t.ty))
                    .unwrap_or(TypeAnnotation::Null);

                if let Some((first_key, _)) = key_types.first().cloned() {
                    for (kt, span) in key_types.iter().skip(1) {
                        if !kt.ty.is_null()
                            && !first_key.ty.is_null()
                            && !kt.ty.matches(&first_key.ty)
                        {
                            self.error(
                                format!(
                                    "map key type mismatch: expected {}, got {}",
                                    first_key.ty.info(),
                                    kt.ty.info()
                                ),
                                *span,
                            );
                        }
                    }
                }
                if let Some((first_value, _)) = value_types.first().cloned() {
                    for (vt, span) in value_types.iter().skip(1) {
                        if !vt.ty.is_null()
                            && !first_value.ty.is_null()
                            && !vt.ty.matches(&first_value.ty)
                        {
                            self.error(
                                format!(
                                    "map value type mismatch: expected {}, got {}",
                                    first_value.ty.info(),
                                    vt.ty.info()
                                ),
                                *span,
                            );
                        }
                    }
                }

                for (kt, span) in &key_types {
                    if !kt.ty.is_null()
                        && !Self::is_hashable_key_type(&Self::to_type_annotation(&kt.ty))
                    {
                        self.error(
                            format!("type {} cannot be used as a map key", kt.ty.info()),
                            *span,
                        );
                    }
                }

                CheckedExpr::new(
                    CheckType::Known(TypeAnnotation::Map(
                        Box::new(key_type),
                        Box::new(value_type),
                    )),
                    None,
                )
            }
            // checks array items
            ExpressionKind::ArrayLiteral(items) => {
                // checks every item type in items then push it to the new
                // item_types vec
                let mut item_types = Vec::with_capacity(items.len());
                for item in items {
                    let item_span = self.ast_arena.exprs.get(item).span;
                    item_types.push((self.check_expression_typed(item), item_span));
                }
                // sets the items types to same first item type otherwise null
                let items_type = item_types
                    .first()
                    .map(|(t, _)| Self::to_type_annotation(&t.ty))
                    .unwrap_or(TypeAnnotation::Null);

                // same items type or not?
                if let Some((first_type, _)) = item_types.first().cloned() {
                    for (item_type, span) in item_types.iter().skip(1) {
                        if !item_type.ty.is_null()
                            && !first_type.ty.is_null()
                            && !item_type.ty.matches(&first_type.ty)
                        {
                            self.error(
                                format!(
                                    "array element type mismatch: expected {}, got {}",
                                    first_type.ty.info(),
                                    item_type.ty.info()
                                ),
                                *span,
                            );
                        }
                    }
                }
                // returns the array type
                CheckedExpr::new(
                    CheckType::Known(TypeAnnotation::Array(Box::new(items_type))),
                    None,
                )
            }

            ExpressionKind::Index { target, index } => {
                // is the target (array) null?
                let target_span = self.ast_arena.exprs.get(target).span;
                let index_span = self.ast_arena.exprs.get(index).span;
                let target_typed = self.check_expression_typed(target);
                let target_type = target_typed.ty;
                self.check_is_null(&target_type, target_span);
                // is the index null??
                let index_typed = self.check_expression_typed(index);
                let index_type = index_typed.ty;
                self.check_is_null(&index_type, index_span);

                // is it integer? (only enforced for array/tuple targets -
                // maps validate the index against their declared key type
                // further down instead; an all-map union counts as a map)
                let target_is_map = match &target_type {
                    CheckType::Known(TypeAnnotation::Map(_, _) | TypeAnnotation::CMap(_, _)) => true,
                    CheckType::Known(TypeAnnotation::Any(members) | TypeAnnotation::CAny(members)) => {
                        members.iter().all(|m| {
                            matches!(m, TypeAnnotation::Map(_, _) | TypeAnnotation::CMap(_, _))
                        })
                    }
                    _ => false,
                };
                if !target_is_map
                    && !matches!(
                        index_type,
                        CheckType::Known(
                            TypeAnnotation::Int
                                | TypeAnnotation::CInt
                                | TypeAnnotation::UInt
                                | TypeAnnotation::CUInt
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

                // element type for the target; unions probe every member
                // (diagnostics truncated) and merge back into one type
                let result = match &target_type {
                    CheckType::Known(TypeAnnotation::Any(members) | TypeAnnotation::CAny(members)) => {
                        self.check_index_any(members.to_vec(), &index_type, expr_span, index_span)
                    }
                    _ => self.check_index_target(&target_type, &index_type, expr_span, index_span),
                };
                CheckedExpr::new(result, None)
            }

            // offloads to index_assign
            ExpressionKind::IndexAssign {
                target,
                index,
                value,
            } => CheckedExpr::new(
                self.check_index_assign(target, index, value, expr_span),
                None,
            ),

            // offloads to binary
            ExpressionKind::Binary {
                left,
                operator,
                right,
            } => {
                // is the left operand null?
                let left_typed = self.check_expression_typed(left);
                let left_id = self.ast_arena.exprs.get(left);
                self.check_is_null(&left_typed.ty, left_id.span);
                // is the right operand null?
                let right_typed = self.check_expression_typed(right);
                let right_id = self.ast_arena.exprs.get(right);
                self.check_is_null(&right_typed.ty, right_id.span);
                // is the binary correct?
                self.check_binary_operator(&left_typed, &right_typed, &operator, expr_span)
            }

            // offloads to unary
            ExpressionKind::Unary { operator, operand } => {
                // is the operand null?
                let operand_span = self.ast_arena.exprs.get(operand).span;
                let operand_typed = self.check_expression_typed(operand);
                self.check_is_null(&operand_typed.ty, operand_span);
                // is the unary correct?
                self.check_unary_operator(operand_typed, operand_span, &operator, expr_span)
            }

            // assigns the value to the variable then returns it
            ExpressionKind::Assign { name, value } => {
                let value_typed = self.check_expression_typed(value);
                self.assign(&name, value_typed.clone(), expr_span);
                value_typed
            }

            // checks the call path of the function
            ExpressionKind::Call { path, args } => {
                let arg_types: Vec<(CheckType, Span)> = args
                    .iter()
                    .map(|a| {
                        let a = *a;
                        let a_span = self.ast_arena.exprs.get(a).span;
                        let t = self.check_expression_typed(a).ty;
                        (t, a_span)
                    })
                    .collect();
                CheckedExpr::new(self.check_call_path(&path, &arg_types, expr_span), None)
            }

            // checks the call of the function
            ExpressionKind::CallExpr { callee, args } => {
                let callee_type = self.check_expression_typed(callee).ty;
                let arg_types: Vec<(CheckType, Span)> = args
                    .iter()
                    .map(|a| {
                        let a = *a;
                        let a_span = self.ast_arena.exprs.get(a).span;
                        let t = self.check_expression_typed(a).ty;
                        (t, a_span)
                    })
                    .collect();
                CheckedExpr::new(
                    self.check_call_value(callee_type, &arg_types, expr_span),
                    None,
                )
            }

            // checks the method call
            ExpressionKind::MethodCall {
                caller,
                method,
                args,
            } => {
                let caller_typed = self.check_expression_typed(caller);
                let caller_type = caller_typed.ty;
                let caller_id = self.ast_arena.exprs.get(caller);
                let mut arg_types: Vec<(CheckType, Span)> =
                    vec![(caller_type.clone(), caller_id.span)];
                for arg in args {
                    let arg_span = self.ast_arena.exprs.get(arg).span;
                    arg_types.push((self.check_expression_typed(arg).ty, arg_span));
                }

                if method.len() == 1
                    && let CheckType::Known(
                        TypeAnnotation::Record(rname) | TypeAnnotation::CRecord(rname),
                    ) = &caller_type
                    && let Some(sig) = self
                        .methods
                        .get(&(rname.clone(), method[0].clone()))
                        .cloned()
                {
                    return CheckedExpr::new(
                        self.check_call_value(sig, &arg_types, expr_span),
                        None,
                    );
                }

                // union receiver: every member must resolve the method
                // (each probed with diagnostics truncated, so only one
                // loud error names the failing members); return types
                // merge back into one type.
                if let CheckType::Known(
                    TypeAnnotation::Any(members) | TypeAnnotation::CAny(members),
                ) = &caller_type
                {
                    return CheckedExpr::new(
                        self.check_method_any(members.clone(), &arg_types, &method, expr_span),
                        None,
                    );
                }

                CheckedExpr::new(self.check_call_path(&method, &arg_types, expr_span), None)
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
                // isolate `?` tracking: a lambda's propagate is its own
                let saved_propagate = std::mem::replace(&mut self.saw_propagate, false);
                // is the body correct?
                for statement in &body {
                    self.check_statement(statement);
                }
                // removes return type
                let _ = self.pop_return_type();
                self.saw_propagate = saved_propagate;
                // remove scope level
                self.pop_scope();

                CheckedExpr::new(
                    CheckType::Function {
                        params: params.iter().map(|p| p.param_type.clone()).collect(),
                        return_type: resolved_return,
                    },
                    None,
                )
            }

            ExpressionKind::Is { value, target_type } => {
                let value_typed = self.check_expression_typed(value);
                let value_id = self.ast_arena.exprs.get(value);
                self.check_is_null(&value_typed.ty, value_id.span);
                // unions as targets narrow to nothing: test members
                match target_type {
                    TypeAnnotation::Any(_) | TypeAnnotation::CAny(_) => {
                        self.error(
                            "`is` needs one concrete type - test union members one by one",
                            expr_span,
                        );
                    }
                    _ => {}
                }
                CheckedExpr::new(CheckType::Known(TypeAnnotation::Bool), None)
            }

            ExpressionKind::Cast { value, target_type } => {
                let value_typed = self.check_expression_typed(value);
                let value_id = self.ast_arena.exprs.get(value);
                self.check_is_null(&value_typed.ty, value_id.span);

                let castable = matches!(
                    &value_typed.ty,
                    CheckType::Known(
                        TypeAnnotation::CInt
                            | TypeAnnotation::CUInt
                            | TypeAnnotation::CByte
                            | TypeAnnotation::CFloat
                            | TypeAnnotation::Float
                            | TypeAnnotation::Int
                            | TypeAnnotation::UInt
                            | TypeAnnotation::Byte
                    ) | CheckType::Unknown
                ) || matches!(
                    &value_typed.ty,
                    // narrowing cast out of a union: valid iff the target
                    // matches some member (verified again at runtime)
                    CheckType::Known(
                        TypeAnnotation::Any(members) | TypeAnnotation::CAny(members)
                    ) if members.iter().any(|m| {
                        CheckType::Known(m.clone())
                            .matches(&CheckType::Known(target_type.clone()))
                    })
                );

                let valid_target = matches!(
                    target_type,
                    TypeAnnotation::Int
                        | TypeAnnotation::UInt
                        | TypeAnnotation::Float
                        | TypeAnnotation::Byte
                );

                if !castable || !valid_target {
                    self.error(
                        format!(
                            "invalid cast: cannot cast {} to {:?}",
                            value_typed.ty.info(),
                            target_type
                        ),
                        expr_span,
                    );
                }
                // casts intentionally drop the unit of the value
                CheckedExpr::new(CheckType::Known(target_type.clone()), None)
            }

            ExpressionKind::TupleLiteral(items) => {
                let types: Vec<TypeAnnotation> = items
                    .iter()
                    .map(|item| {
                        let t = self.check_expression_typed(*item).ty;
                        Self::to_type_annotation(&t)
                    })
                    .collect();
                CheckedExpr::new(
                    CheckType::Known(TypeAnnotation::Tuple(Rc::new(types))),
                    None,
                )
            }
            ExpressionKind::ErrorLiteral(inner) => {
                let inner_typed = self.check_expression_typed(inner);
                if matches!(
                    inner_typed.ty,
                    CheckType::Known(TypeAnnotation::Error | TypeAnnotation::CError)
                ) {
                    self.error("error cannot wrap another error", expr_span);
                }
                CheckedExpr::new(CheckType::Known(TypeAnnotation::Error), None)
            }
            ExpressionKind::OkLiteral(inner) => {
                let inner_ann = Self::to_type_annotation(&self.check_expression_typed(inner).ty);
                CheckedExpr::new(
                    CheckType::Known(TypeAnnotation::Result(Box::new(inner_ann))),
                    None,
                )
            }
            ExpressionKind::ErrLiteral(inner) => {
                let inner_ann = Self::to_type_annotation(&self.check_expression_typed(inner).ty);
                CheckedExpr::new(
                    CheckType::Known(TypeAnnotation::Result(Box::new(inner_ann))),
                    None,
                )
            }

            ExpressionKind::Propagate(inner) => {
                // A body using `?` can return `err`: inference wraps.
                // Saved/restored per function/lambda by the caller arms.
                self.saw_propagate = true;
                let inner_typed = self.check_expression_typed(inner);
                let result = match inner_typed.ty {
                    CheckType::Known(
                        TypeAnnotation::Result(inner_ty) | TypeAnnotation::CResult(inner_ty),
                    ) => {
                        if let Some(return_ty) = self.current_return_type()
                            && !matches!(
                                return_ty,
                                // `Null` = undeclared: inference decides
                                // from the unwrapped type
                                TypeAnnotation::Null
                                | TypeAnnotation::Result(_)
                                | TypeAnnotation::CResult(_)
                            )
                        {
                            self.error(
                                "`?` cannot be used in a function that does not return a result",
                                expr_span,
                            );
                        }
                        CheckType::Known(*inner_ty)
                    }
                    // `?` over a union: every member must be a result;
                    // unwraps to the union of payloads.
                    CheckType::Known(
                        TypeAnnotation::Any(ref members) | TypeAnnotation::CAny(ref members),
                    ) => {
                        if let Some(return_ty) = self.current_return_type()
                            && !matches!(
                                return_ty,
                                TypeAnnotation::Null
                                    | TypeAnnotation::Result(_)
                                    | TypeAnnotation::CResult(_)
                            )
                        {
                            self.error(
                                "`?` cannot be used in a function that does not return a result",
                                expr_span,
                            );
                        }
                        let mut inners = Vec::with_capacity(members.len());
                        let mut ok = true;
                        for m in members.iter() {
                            match m {
                                TypeAnnotation::Result(inner)
                                | TypeAnnotation::CResult(inner) => inners.push((**inner).clone()),
                                _ => {
                                    ok = false;
                                    break;
                                }
                            }
                        }
                        if ok {
                            CheckType::Known(TypeAnnotation::Any(std::rc::Rc::new(inners)))
                        } else {
                            self.error(
                                format!(
                                    "`?` operator requires a result, got {}",
                                    inner_typed.ty.info()
                                ),
                                expr_span,
                            );
                            CheckType::Unknown
                        }
                    }
                    CheckType::Unknown => CheckType::Unknown,
                    other => {
                        self.error(
                            format!("`?` operator requires a result, got {}", other.info()),
                            expr_span,
                        );
                        CheckType::Unknown
                    }
                };
                CheckedExpr::new(result, None)
            }

            ExpressionKind::StructLiteral { name, fields } => {
                if let Some(declared_fields) = self.records.get(&name).cloned() {
                    if let Some(decl_span) = self.record_spans.get(&name) {
                        self.definitions.push((expr_span, *decl_span));
                    }
                    for (field_name, value) in &fields {
                        let value_span = self.ast_arena.exprs.get(*value).span;
                        let value_typed = self.check_expression_typed(*value);
                        match declared_fields.iter().find(|(n, _)| n == field_name) {
                            Some((_, field_type)) => {
                                let expected = CheckType::Known(field_type.clone());
                                if !value_typed.ty.matches(&expected) {
                                    self.error(
                                        format!(
                                            "field `{}` of record `{}` expects {}, got {}",
                                            field_name,
                                            name,
                                            expected.info(),
                                            value_typed.ty.info()
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
                        self.check_expression_typed(*value);
                    }
                }
                CheckedExpr::new(CheckType::Known(TypeAnnotation::Record(name)), None)
            }

            ExpressionKind::FieldAccess { target, field } => {
                let target_typed = self.check_expression_typed(target);
                let result = match &target_typed.ty {
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
                };
                CheckedExpr::new(result, None)
            }

            ExpressionKind::FieldAssign {
                target,
                field,
                value,
            } => {
                let target_typed = self.check_expression_typed(target);
                let value_typed = self.check_expression_typed(value);
                match &target_typed.ty {
                    CheckType::Known(
                        TypeAnnotation::Record(name) | TypeAnnotation::CRecord(name),
                    ) => {
                        match self.records.get(name).and_then(|fs| {
                            fs.iter().find(|(n, _)| *n == field).map(|(_, t)| t.clone())
                        }) {
                            Some(field_type) => {
                                let expected = CheckType::Known(field_type);
                                if !value_typed.ty.matches(&expected) {
                                    self.error(
                                        format!(
                                            "field `{}` of record `{}` expects {}, got {}",
                                            field,
                                            name,
                                            expected.info(),
                                            value_typed.ty.info()
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
                value_typed
            }

            ExpressionKind::EnumVariant { enum_name, variant } => {
                if let Some(decl_span) = self.tag_spans.get(&enum_name) {
                    self.definitions.push((expr_span, *decl_span));
                }
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
                CheckedExpr::new(CheckType::Known(TypeAnnotation::Enum(enum_name)), None)
            }

            ExpressionKind::SetLiteral(items) => {
                let mut item_types = Vec::with_capacity(items.len());
                for item in items {
                    let item_span = self.ast_arena.exprs.get(item).span;
                    item_types.push((self.check_expression_typed(item), item_span));
                }

                let items_type = item_types
                    .first()
                    .map(|(t, _)| Self::to_type_annotation(&t.ty))
                    .unwrap_or(TypeAnnotation::Null);

                if let Some((first_type, _)) = item_types.first().cloned() {
                    for (item_type, span) in item_types.iter().skip(1) {
                        if !item_type.ty.is_null()
                            && !first_type.ty.is_null()
                            && !item_type.ty.matches(&first_type.ty)
                        {
                            self.error(
                                format!(
                                    "set element type mismatch: expected {}, got {}",
                                    first_type.ty.info(),
                                    item_type.ty.info()
                                ),
                                *span,
                            );
                        }
                    }
                }

                CheckedExpr::new(
                    CheckType::Known(TypeAnnotation::Set(Box::new(items_type))),
                    None,
                )
            }

            _ => CheckedExpr::new(CheckType::Unknown, None),
        }
    }
}
