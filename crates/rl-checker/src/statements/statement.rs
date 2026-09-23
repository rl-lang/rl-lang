//! Statement type checking - walks every [`StatementKind`] variant,
//! declares names into scope, and validates control flow constraints.

use std::{collections::HashSet, path::PathBuf};

use crate::{TypeChecker, structs::CheckType, units::Unit};
use rl_ast::statements::{ItemAttribute, Lint, MatchPattern, Statement, StatementKind, TypeAnnotation};
use rl_lexer::tokenizer::Tokenizer;
use rl_parser::parser_logic::Parser;
use rl_utils::{source::SourceFile, span::Span};

impl TypeChecker {
    /// Collects `Lint` values and deprecated message from a statement's `item_attributes`.
    fn collect_item_attrs(stmt: &Statement) -> (HashSet<Lint>, Option<String>) {
        let mut lints = HashSet::new();
        let mut deprecated = None;
        let attrs = match &stmt.kind {
            StatementKind::VariableDeclaration { item_attributes, .. } => item_attributes,
            StatementKind::ConstantDeclaration { item_attributes, .. } => item_attributes,
            StatementKind::FunctionDeclaration { item_attributes, .. } => item_attributes,
            _ => return (lints, deprecated),
        };
        for attr in attrs {
            match attr {
                ItemAttribute::Allow(lints_vec) => lints.extend(lints_vec),
                ItemAttribute::Deprecated(msg) => deprecated = msg.clone(),
                // Custom markers carry no lint meaning; queried directly.
                ItemAttribute::Custom { .. } => {}
            }
        }
        (lints, deprecated)
    }

    // checks the current statement and push errors via error() if any found
    pub fn check_statement(&mut self, statement: &Statement) {
        let (allowed, deprecated) = Self::collect_item_attrs(statement);
        let pushed = !allowed.is_empty();
        if pushed {
            self.allow_stack.push(allowed.clone());
        }

        self.check_statement_inner(statement);

        // For variable/constant/function declarations, attach suppressed lints
        // and deprecation message to the scope item.
        let name = match &statement.kind {
            StatementKind::VariableDeclaration { name, .. } => Some(name.as_str()),
            StatementKind::ConstantDeclaration { name, .. } => Some(name.as_str()),
            StatementKind::FunctionDeclaration { name, .. } => Some(name.as_str()),
            _ => None,
        };
        if let Some(n) = name {
            if !allowed.is_empty() {
                self.set_suppressed_lints_for_last_declared(n, allowed);
            }
            if deprecated.is_some() {
                self.set_deprecated_for_last_declared(n, deprecated);
            }
        }

        if pushed {
            self.allow_stack.pop();
        }
    }

    fn check_statement_inner(&mut self, statement: &Statement) {
        match &statement.kind {
            // checks if the type null or same type then declare it as variable
            // otherwise pushs error
            StatementKind::VariableDeclaration {
                name,
                type_annotation,
                unit_annotation,
                value,
                item_attributes: _,
            } => {
                let declared_unit = unit_annotation.as_ref().map(Unit::from_annotation);
                let value_typed = self.check_expression_typed(*value);

                // `dec name = value` - the type wasn't stated, so whatever
                // the initialiser resolved to *is* the declared type. There
                // is nothing to mismatch against.
                if *type_annotation == TypeAnnotation::Infer {
                    self.declare(name.clone(), value_typed.ty, false, statement.span);
                    return;
                }

                if type_annotation.contains_handle_infer() {
                    match &value_typed.ty {
                        CheckType::Unknown => {
                            self.declare(name.clone(), CheckType::Unknown, false, statement.span);
                        }
                        CheckType::Known(actual) => {
                            match type_annotation.resolve_handle_infer(actual) {
                                Some(resolved) => {
                                    self.declare(
                                        name.clone(),
                                        CheckType::Known(resolved),
                                        false,
                                        statement.span,
                                    );
                                }
                                None => {
                                    self.error(
                                        format!(
                                            "`dec handle` requires a std module call returning a handle, got {}",
                                            value_typed.ty.info()),
                                        statement.span);
                                    self.declare(
                                        name.clone(),
                                        CheckType::Unknown,
                                        false,
                                        statement.span,
                                    );
                                }
                            }
                        }
                        _ => {
                            self.error(
                                format!(
                                    "`dec handle` requires a std module call returning a handle, got {}",
                                    value_typed.ty.info()),
                                statement.span);
                            self.declare(name.clone(), CheckType::Unknown, false, statement.span);
                        }
                    }
                    return;
                }

                let declared = CheckType::Known(type_annotation.clone());

                if !value_typed.ty.matches(&declared) {
                    self.error(
                        format!(
                            "type mismatch: expected {}, got {}",
                            declared.info(),
                            value_typed.ty.info()
                        ),
                        statement.span,
                    );
                }

                if let Some(msg) =
                    declaration_unit_mismatch(&declared_unit, &value_typed.unit, &self.conversions)
                {
                    self.error(
                        format!("unit mismatch on declaration: {}", msg),
                        statement.span,
                    );
                }

                self.declare_with_unit(
                    name.clone(),
                    declared,
                    declared_unit,
                    false,
                    statement.span,
                );
            }

            // checks if the type is null or same type and declares it as
            // constant otherwise pushs error
            StatementKind::ConstantDeclaration {
                name,
                type_annotation,
                unit_annotation,
                value,
                item_attributes: _,
            } => {
                let declared_unit = unit_annotation.as_ref().map(Unit::from_annotation);
                let value_typed = self.check_expression_typed(*value).into_const();
                let declared = CheckType::Known(type_annotation.clone());

                if !value_typed.ty.matches(&declared) {
                    self.error(
                        format!(
                            "type mismatch: expected {}, got {}",
                            declared.info(),
                            value_typed.ty.info()
                        ),
                        statement.span,
                    );
                }

                if let Some(msg) =
                    declaration_unit_mismatch(&declared_unit, &value_typed.unit, &self.conversions)
                {
                    self.error(
                        format!("unit mismatch on declaration: {}", msg),
                        statement.span,
                    );
                }

                self.declare_with_unit(name.clone(), declared, declared_unit, true, statement.span);
            }

            // checks the array if valid or not and declares it with correct
            // type (weather constant or variable) otherwise pushs error
            StatementKind::Array {
                name,
                type_annotation,
                value,
            } => {
                for item in value {
                    let item_span = self.ast_arena.exprs.get(*item).span;
                    let item_type = self.check_expression(*item);
                    let expected = CheckType::Known(type_annotation.clone());

                    if !item_type.matches(&expected) {
                        self.error(
                            format!(
                                "type mismatch: array expects {}, got {}",
                                expected.info(),
                                item_type.info()
                            ),
                            item_span,
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
                    let item_span = self.ast_arena.exprs.get(*item).span;
                    let item_type = self.check_expression(*item);
                    let expected = CheckType::Known(type_annotation.clone());

                    if !item_type.matches(&expected) {
                        self.error(
                            format!(
                                "type mismatch: array expects {}, got {}",
                                expected.info(),
                                item_type.info()
                            ),
                            item_span,
                        );
                    }
                }

                let array_type =
                    CheckType::Known(TypeAnnotation::CArray(Box::new(type_annotation.clone())));

                self.declare(name.clone(), array_type, true, statement.span);
            }

            StatementKind::Set {
                name,
                type_annotation,
                items,
            } => {
                for item in items {
                    let item_span = self.ast_arena.exprs.get(*item).span;
                    let item_type = self.check_expression(*item);
                    let expected = CheckType::Known(type_annotation.clone());

                    if !item_type.matches(&expected) {
                        self.error(
                            format!(
                                "type mismatch: set expects {}, got {}",
                                expected.info(),
                                item_type.info()
                            ),
                            item_span,
                        );
                    }
                }

                let array_type =
                    CheckType::Known(TypeAnnotation::Set(Box::new(type_annotation.clone())));

                self.declare(name.clone(), array_type, false, statement.span);
            }
            StatementKind::ConstantSet {
                name,
                type_annotation,
                items,
            } => {
                for item in items {
                    let item_span = self.ast_arena.exprs.get(*item).span;
                    let item_type = self.check_expression(*item);
                    let expected = CheckType::Known(type_annotation.clone());

                    if !item_type.matches(&expected) {
                        self.error(
                            format!(
                                "type mismatch: set expects {}, got {}",
                                expected.info(),
                                item_type.info()
                            ),
                            item_span,
                        );
                    }
                }

                let array_type =
                    CheckType::Known(TypeAnnotation::CSet(Box::new(type_annotation.clone())));

                self.declare(name.clone(), array_type, true, statement.span);
            }

            // checks map entries against the declared key/value types and
            // declares it with the correct mutability.
            StatementKind::Map {
                name,
                type_annotation,
                entries,
            } => {
                let (key_ty, value_ty) = match type_annotation {
                    TypeAnnotation::Map(k, v) => (k.as_ref().clone(), v.as_ref().clone()),
                    other => (TypeAnnotation::Null, other.clone()),
                };
                let expected_key = CheckType::Known(key_ty.clone());
                let expected_value = CheckType::Known(value_ty.clone());

                for (key, value) in entries {
                    let key_span = self.ast_arena.exprs.get(*key).span;
                    let value_span = self.ast_arena.exprs.get(*value).span;
                    let key_type = self.check_expression(*key);
                    let value_type = self.check_expression(*value);

                    if !key_type.matches(&expected_key) {
                        self.error(
                            format!(
                                "type mismatch: map key expects {}, got {}",
                                expected_key.info(),
                                key_type.info()
                            ),
                            key_span,
                        );
                    }
                    if !value_type.matches(&expected_value) {
                        self.error(
                            format!(
                                "type mismatch: map value expects {}, got {}",
                                expected_value.info(),
                                value_type.info()
                            ),
                            value_span,
                        );
                    }
                }

                if !Self::is_hashable_key_type(&key_ty) {
                    self.error(
                        format!("type {:?} cannot be used as a map key", key_ty),
                        statement.span,
                    );
                }

                let map_type =
                    CheckType::Known(TypeAnnotation::Map(Box::new(key_ty), Box::new(value_ty)));

                self.declare(name.clone(), map_type, false, statement.span);
            }
            StatementKind::ConstantMap {
                name,
                type_annotation,
                entries,
            } => {
                let (key_ty, value_ty) = match type_annotation {
                    TypeAnnotation::CMap(k, v) => (k.as_ref().clone(), v.as_ref().clone()),
                    other => (TypeAnnotation::Null, other.clone()),
                };
                let expected_key = CheckType::Known(key_ty.clone());
                let expected_value = CheckType::Known(value_ty.clone());

                for (key, value) in entries {
                    let key_span = self.ast_arena.exprs.get(*key).span;
                    let value_span = self.ast_arena.exprs.get(*value).span;
                    let key_type = self.check_expression(*key).into_const();
                    let value_type = self.check_expression(*value).into_const();

                    if !key_type.matches(&expected_key) {
                        self.error(
                            format!(
                                "type mismatch: map key expects {}, got {}",
                                expected_key.info(),
                                key_type.info()
                            ),
                            key_span,
                        );
                    }
                    if !value_type.matches(&expected_value) {
                        self.error(
                            format!(
                                "type mismatch: map value expects {}, got {}",
                                expected_value.info(),
                                value_type.info()
                            ),
                            value_span,
                        );
                    }
                }

                if !Self::is_hashable_key_type(&key_ty) {
                    self.error(
                        format!("type {:?} cannot be used as a map key", key_ty),
                        statement.span,
                    );
                }

                let map_type =
                    CheckType::Known(TypeAnnotation::CMap(Box::new(key_ty), Box::new(value_ty)));

                self.declare(name.clone(), map_type, true, statement.span);
            }

            // offloads to expression checker
            StatementKind::Expression(expr) => {
                let ty = self.check_expression(*expr);
                // candidate trailing-expression return; read only when the
                // enclosing body actually ends with this statement
                self.last_expr_type = Some(ty);
            }

            // loops checker
            StatementKind::While { condition, body } => {
                // is condition type is bool?
                let condition_type = self.check_expression(*condition);
                if !matches!(
                    condition_type,
                    CheckType::Known(TypeAnnotation::Bool | TypeAnnotation::CBool)
                        | CheckType::Unknown
                ) {
                    let condition_span = self.ast_arena.exprs.get(*condition).span;
                    self.error(
                        format!(
                            "while condition must be bool, got {}",
                            condition_type.info()
                        ),
                        condition_span,
                    );
                }
                // add loop depth
                self.enter_loop();
                // checks the blocks
                self.check_block(body);
                // remove loop depth
                self.exit_loop();
            }

            StatementKind::Loop(body) => {
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
                let condition_type = self.check_expression(*condition);
                if !matches!(
                    condition_type,
                    CheckType::Known(TypeAnnotation::Bool | TypeAnnotation::CBool)
                        | CheckType::Unknown
                ) {
                    let condition_span = self.ast_arena.exprs.get(*condition).span;
                    self.error(
                        format!("for condition must be bool, got {}", condition_type.info()),
                        condition_span,
                    );
                }
                // is the increment correct?
                self.check_expression(*increment);
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
                let iter_type = self.check_expression(*iterable);
                // is the ieterable items correct?
                let item_type = match &iter_type {
                    CheckType::Known(TypeAnnotation::Array(inner))
                    | CheckType::Known(TypeAnnotation::CArray(inner)) => {
                        CheckType::Known((**inner).clone())
                    }
                    CheckType::Unknown => CheckType::Unknown,
                    other => {
                        let iterable_span = self.ast_arena.exprs.get(*iterable).span;
                        self.error(
                            format!("for-each: expected an array, got {}", other.info()),
                            iterable_span,
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
            StatementKind::ConditionalBranch {
                condition, body, ..
            } => {
                // is there condition? or is it else?
                if let Some(cond) = condition {
                    // is the condition bool?
                    let condition_type = self.check_expression(*cond);
                    if !matches!(
                        condition_type,
                        CheckType::Known(TypeAnnotation::Bool | TypeAnnotation::CBool)
                            | CheckType::Unknown
                    ) {
                        let cond_span = self.ast_arena.exprs.get(*cond).span;
                        self.error(
                            format!("condition must be bool, got {}", condition_type.info()),
                            cond_span,
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
                self.push_scope();
                for param in params {
                    self.declare(
                        param.param_name.clone(),
                        CheckType::Known(param.param_type.clone()),
                        false,
                        statement.span,
                    );
                }
                self.push_return_type(return_type.clone());
                let saved_propagate = std::mem::replace(&mut self.saw_propagate, false);
                for stmt in body {
                    self.check_statement(stmt);
                }
                let returned = self.pop_return_type();
                let body_propagated = std::mem::replace(&mut self.saw_propagate, saved_propagate);
                self.pop_scope();
                // No `->` annotation (`Null` default): infer the return
                // type from the body so results/handles flow through calls.
                // Conservative: all `return`s (or the trailing expression)
                // must agree on one concrete type, else keep `Null`.
                if *return_type == TypeAnnotation::Null {
                    let ends_with_expr = matches!(
                        body.last().map(|s| &s.kind),
                        Some(StatementKind::Expression(_))
                    );
                    let trailing = if ends_with_expr {
                        self.last_expr_type.clone()
                    } else {
                        None
                    };
                    if let Some(inferred) =
                        Self::infer_fn_return(returned, trailing, ends_with_expr, body_propagated)
                    {
                        self.declare(
                            name.clone(),
                            CheckType::Function {
                                params: params
                                    .iter()
                                    .map(|p| p.param_type.clone())
                                    .collect(),
                                return_type: inferred,
                            },
                            false,
                            statement.span,
                        );
                    }
                }
            }

            StatementKind::ImplBlock { record, methods } => {
                if !self.records.contains_key(record) {
                    self.error(format!("unknown record type `{}`", record), statement.span);
                }
                for m in methods {
                    let StatementKind::FunctionDeclaration {
                        name,
                        params,
                        return_type,
                        body,
                        ..
                    } = &m.kind
                    else {
                        continue;
                    };
                    self.push_scope();
                    for param in params {
                        self.declare(
                            param.param_name.clone(),
                            CheckType::Known(param.param_type.clone()),
                            false,
                            m.span,
                        );
                    }
                    self.push_return_type(return_type.clone());
                    let saved_propagate = std::mem::replace(&mut self.saw_propagate, false);
                    for stmt in body {
                        self.check_statement(stmt);
                    }
                    let returned = self.pop_return_type();
                    let body_propagated =
                        std::mem::replace(&mut self.saw_propagate, saved_propagate);
                    self.pop_scope();
                    if *return_type == TypeAnnotation::Null {
                        let ends_with_expr = matches!(
                            body.last().map(|s| &s.kind),
                            Some(StatementKind::Expression(_))
                        );
                        let trailing = if ends_with_expr {
                            self.last_expr_type.clone()
                        } else {
                            None
                        };
                        if let Some(inferred) =
                            Self::infer_fn_return(returned, trailing, ends_with_expr, body_propagated)
                        {
                            self.methods.insert(
                                (record.clone(), name.clone()),
                                CheckType::Function {
                                    params: params
                                        .iter()
                                        .map(|p| p.param_type.clone())
                                        .collect(),
                                    return_type: inferred,
                                },
                            );
                        }
                    }
                }
            }
            StatementKind::Return(expr) => {
                // is the expression a valid type? otherwise null
                let actual_type = match expr {
                    Some(e) => self.check_expression(*e),
                    None => CheckType::Known(TypeAnnotation::Null),
                };
                // record for undeclared-return inference (ignored when annotated)
                if let Some(top) = self.inferred_return_stack.last_mut() {
                    top.push(actual_type.clone());
                }
                // is the actual type same as the expected return one?
                if let Some(expected) = self.current_return_type().cloned() {
                    let widens = matches!(
                        (expected.clone(), actual_type.clone()),
                        (
                            TypeAnnotation::Int | TypeAnnotation::CInt,
                            CheckType::Known(TypeAnnotation::Byte | TypeAnnotation::CByte)
                        ) | (
                            TypeAnnotation::UInt | TypeAnnotation::CUInt,
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
            StatementKind::Break if self.loop_depth() == 0 => {
                self.error("break outside of loop", statement.span);
            }
            StatementKind::Continue if self.loop_depth() == 0 => {
                self.error("continue outside of loop", statement.span);
            }

            StatementKind::ImportFile { path } => {
                self.import_module(path, None, statement.span);
            }
            StatementKind::ImportFileNamed { path, names } => {
                self.import_module(path, Some(names), statement.span);
            }
            // Type aliases are fully resolved at parse time; uses were
            // substituted inline and warnings come from the alias_uses
            // pass. Nothing to check here.
            StatementKind::TypeAlias { .. } => {}
            StatementKind::Import { names, wildcard, path } => {
                let module_path = path.join("::");
                let mut module = &self.root_module;
                for seg in path {
                    if seg == &module.name {
                        continue;
                    }
                    let Some(next) = module.submodules.get(seg) else {
                        self.error(
                            format!("unknown module '{seg}' in 'std::{module_path}'"),
                            statement.span,
                        );
                        return;
                    };
                    module = next;
                }

                if *wildcard {
                    for (name, f) in &module.functions {
                        self.imported_std_fns.insert(name.clone(), f.clone());
                        let mut canonical = path.clone();
                        canonical.push(name.clone());
                        self.imported_std_paths.insert(name.clone(), canonical);
                    }
                } else {
                    let mut imported = Vec::new();
                    let mut missing = Vec::new();
                    for (name, alias) in names {
                        match module.functions.get(name) {
                            Some(f) => imported.push((
                                alias.as_deref().unwrap_or(name),
                                name.clone(),
                                f.clone(),
                            )),
                            None => missing.push(name),
                        }
                    }

                    for (visible, original, f) in imported {
                        self.imported_std_fns.insert(visible.to_string(), f);
                        let mut canonical = path.clone();
                        canonical.push(original.clone());
                        self.imported_std_paths
                            .insert(visible.to_string(), canonical);
                    }
                    for name in missing {
                        self.error(
                            format!("'{name}' is not defined in 'std::{module_path}'"),
                            statement.span,
                        );
                    }
                }
            }
            StatementKind::DestructureDeclaration { bindings, value } => {
                let value_type = self.check_expression(*value);
                let tuple_types = match &value_type {
                    CheckType::Known(
                        TypeAnnotation::Tuple(types) | TypeAnnotation::CTuple(types),
                    ) => Some(types.clone()),
                    CheckType::Unknown => None,
                    other => {
                        self.error(
                            format!(
                                "expected tuple on right side of destructure, got {}",
                                other.info()
                            ),
                            statement.span,
                        );
                        None
                    }
                };
                if let Some(types) = tuple_types {
                    if types.len() != bindings.len() {
                        self.error(
                            format!(
                                "destructure mismatch: {} bindings but tuple has {} elements",
                                bindings.len(),
                                types.len()
                            ),
                            statement.span,
                        );
                    } else {
                        for ((type_annotation, name), actual) in bindings.iter().zip(types.iter()) {
                            let declared = CheckType::Known(type_annotation.clone());
                            let actual = CheckType::Known(actual.clone());
                            if !actual.matches(&declared) {
                                self.error(
                                    format!(
                                        "destructure type mismatch: expected {}, got {}",
                                        declared.info(),
                                        actual.info()
                                    ),
                                    statement.span,
                                );
                            }
                            self.declare(name.clone(), declared, false, statement.span);
                        }
                    }
                } else {
                    for (type_annotation, name) in bindings {
                        self.declare(
                            name.clone(),
                            CheckType::Known(type_annotation.clone()),
                            false,
                            statement.span,
                        );
                    }
                }
            }

            StatementKind::Match { value, arms } => {
                let val_type = self.check_expression(*value);
                for (pattern, body) in arms {
                    if let MatchPattern::Literal(expr) = pattern {
                        let pat_type = self.check_expression(*expr);
                        if !pat_type.is_unknown() && !val_type.is_unknown() && pat_type != val_type
                        {
                            let expr_span = self.ast_arena.exprs.get(*expr).span;
                            self.error("match pattern type does not match value type", expr_span);
                        }
                    }
                    for stmt in body {
                        self.check_statement(stmt);
                    }
                }
            }

            _ => {}
        }
    }

    fn import_module(&mut self, path: &[String], names: Option<&[String]>, span: Span) {
        let import_name = format!("{}.rl", path.join("/"));
        let mut import_path = match &self.base_dir {
            Some(dir) => dir.join(&import_name),
            None => PathBuf::from(&import_name),
        };

        // try direct file first, then deps/
        if !import_path.exists() {
            if let Some(first) = path.first() {
                let dep_path = self.base_dir.as_ref()
                    .map(|d| d.join("deps").join(first).join("lib.rl"))
                    .unwrap_or_default();
                if dep_path.exists() {
                    import_path = dep_path;
                } else {
                    self.error(
                        format!("could not import '{}': file not found", path.join("::")),
                        span,
                    );
                    return;
                }
            } else {
                self.error(
                    format!("could not import '{}': file not found", path.join("::")),
                    span,
                );
                return;
            }
        }

        let canonical = import_path
            .canonicalize()
            .unwrap_or_else(|_| import_path.clone());

        if self.importing.contains(&canonical) {
            self.error(format!("import cycle detected: {}", path.join("::")), span);
            return;
        }

        let prior: Option<HashSet<String>> = self.imported.get(&canonical).cloned().flatten();
        let missing: Option<Vec<String>> = match self.imported.get(&canonical) {
            Some(None) => return,
            Some(Some(already)) => match names {
                None => None,
                Some(requested) => {
                    let missing: Vec<String> = requested
                        .iter()
                        .filter(|n| !already.contains(*n))
                        .cloned()
                        .collect();
                    if missing.is_empty() {
                        return;
                    }
                    Some(missing)
                }
            },
            None => names.map(<[String]>::to_vec),
        };

        let new_cache_entry: Option<HashSet<String>> = match &missing {
            None => None,
            Some(added) => {
                let mut set = prior.unwrap_or_default();
                set.extend(added.iter().cloned());
                Some(set)
            }
        };
        let wanted = |name: &String| -> bool {
            match &missing {
                None => true, // whole module
                Some(only) => only.contains(name),
            }
        };

        let Ok(source_text) = std::fs::read_to_string(&import_path) else {
            self.error(
                format!(
                    "cannot find module `{}` ({})",
                    path.join("::"),
                    import_path.display()
                ),
                span,
            );
            return;
        };

        let source_file = SourceFile::new(import_path.display().to_string(), source_text);
        let Ok(tokens) = Tokenizer::lex(source_file.clone()) else {
            self.error(
                format!(
                    "module `{}` has syntax error and could not be lexed",
                    path.join("::")
                ),
                span,
            );
            return;
        };

        let Ok((imported_ast, stmts)) = Parser::parse(tokens, source_file.clone()) else {
            self.error(
                format!(
                    "module `{}` has syntax error and could not be parsed",
                    path.join("::")
                ),
                span,
            );
            return;
        };

        self.importing.push(canonical.clone());

        let prev_ast = std::mem::replace(&mut self.ast_arena, imported_ast);
        // Errors raised while checking the imported file must carry its
        // name and text (spans are relative to it), not the importer's.
        // Restored next to `ast_arena` below; nesting-safe (strict scope).
        let prev_source = std::mem::replace(&mut self.source_file, Some(source_file));

        for stmt in &stmts {
            match &stmt.kind {
                StatementKind::FunctionDeclaration {
                    name,
                    params,
                    return_type,
                    ..
                } if wanted(name) => {
                    self.declare(
                        name.clone(),
                        CheckType::Function {
                            params: params.iter().map(|p| p.param_type.clone()).collect(),
                            return_type: return_type.clone(),
                        },
                        false,
                        stmt.span,
                    );
                }
                StatementKind::VariableDeclaration {
                    name,
                    type_annotation,
                    unit_annotation,
                    value,
                    item_attributes: _,
                } if wanted(name) => {
                    let declared_unit = unit_annotation.as_ref().map(Unit::from_annotation);
                    let declared = if *type_annotation == TypeAnnotation::Infer {
                        self.check_expression(*value)
                    } else {
                        CheckType::Known(type_annotation.clone())
                    };
                    self.declare_with_unit(name.clone(), declared, declared_unit, false, stmt.span);
                }
                StatementKind::ConstantDeclaration {
                    name,
                    type_annotation,
                    unit_annotation,
                    ..
                } if wanted(name) => {
                    let declared_unit = unit_annotation.as_ref().map(Unit::from_annotation);
                    self.declare_with_unit(
                        name.clone(),
                        CheckType::Known(type_annotation.clone()),
                        declared_unit,
                        true,
                        stmt.span,
                    );
                }
                StatementKind::Array {
                    name,
                    type_annotation,
                    ..
                } if wanted(name) => {
                    self.declare(
                        name.clone(),
                        CheckType::Known(TypeAnnotation::Array(Box::new(type_annotation.clone()))),
                        false,
                        stmt.span,
                    );
                }
                StatementKind::ConstantArray {
                    name,
                    type_annotation,
                    ..
                } if wanted(name) => {
                    self.declare(
                        name.clone(),
                        CheckType::Known(TypeAnnotation::CArray(Box::new(type_annotation.clone()))),
                        true,
                        stmt.span,
                    );
                }

                StatementKind::Set {
                    name,
                    type_annotation,
                    ..
                } if wanted(name) => {
                    self.declare(
                        name.clone(),
                        CheckType::Known(TypeAnnotation::Set(Box::new(type_annotation.clone()))),
                        false,
                        stmt.span,
                    );
                }
                StatementKind::ConstantSet {
                    name,
                    type_annotation,
                    ..
                } if wanted(name) => {
                    self.declare(
                        name.clone(),
                        CheckType::Known(TypeAnnotation::CSet(Box::new(type_annotation.clone()))),
                        true,
                        stmt.span,
                    );
                }

                StatementKind::Map {
                    name,
                    type_annotation,
                    ..
                } if wanted(name) => {
                    self.declare(
                        name.clone(),
                        CheckType::Known(type_annotation.clone()),
                        false,
                        stmt.span,
                    );
                }
                StatementKind::ConstantMap {
                    name,
                    type_annotation,
                    ..
                } if wanted(name) => {
                    self.declare(
                        name.clone(),
                        CheckType::Known(type_annotation.clone()),
                        true,
                        stmt.span,
                    );
                }

                StatementKind::RecordDeclaration { name, fields } if wanted(name) => {
                    self.records.insert(name.clone(), fields.clone());
                }
                StatementKind::TagDeclaration { name, variants } if wanted(name) => {
                    self.tags.insert(name.clone(), variants.clone());
                }

                StatementKind::ImplBlock { record, methods } if wanted(record) => {
                    for m in methods {
                        if let StatementKind::FunctionDeclaration {
                            name,
                            params,
                            return_type,
                            ..
                        } = &m.kind
                        {
                            let fn_type = CheckType::Function {
                                params: params.iter().map(|p| p.param_type.clone()).collect(),
                                return_type: return_type.clone(),
                            };
                            self.methods.insert((record.clone(), name.clone()), fn_type);
                        }
                    }
                }

                StatementKind::ImportFile { path: nested } => {
                    self.import_module(nested, None, stmt.span);
                }
                StatementKind::ImportFileNamed {
                    path: nested,
                    names: nested_names,
                } => {
                    self.import_module(nested, Some(nested_names), stmt.span);
                }

                StatementKind::RecordDeclaration { name, fields } => {
                    self.records.insert(name.clone(), fields.clone());
                }
                StatementKind::TagDeclaration { name, variants } => {
                    self.tags.insert(name.clone(), variants.clone());
                }

                _ => {}
            }
        }

        self.ast_arena = prev_ast;
        self.source_file = prev_source;
        self.importing.pop();
        self.imported.insert(canonical, new_cache_entry);
    }
}

/// Returns an error message when a declaration's declared unit and its
/// initialiser's unit are incompatible.
///
/// Rules:
/// - A declaration with a unit accepts the same unit or a dimensionless value
///   (plain literals adopt the unit, F#-style).
/// - A declaration without a unit expects dimensionless values; a
///   non-dimensionless initialiser is an error.
fn declaration_unit_mismatch(
    declared: &Option<Unit>,
    value: &Option<Unit>,
    conversions: &crate::units::ConversionTable,
) -> Option<String> {
    match (value.as_ref(), declared.as_ref()) {
        // declared unit present, value carries a non-dimensionless unit
        (Some(v), Some(d)) if !v.is_compatible_with(d) && !v.is_convertible_to(d, conversions) => {
            Some(format!("expected {}, got {}", d, v))
        }
        // no declared unit, value carries a non-dimensionless unit
        (Some(v), None) if !v.is_dimensionless() => {
            Some(format!("expected dimensionless, got {}", v))
        }
        // dimensionless value, matching units, or no units at all
        _ => None,
    }
}
