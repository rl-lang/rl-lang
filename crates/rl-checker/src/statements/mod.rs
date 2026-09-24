//! Statement-level helpers shared between expression and statement checking.

mod expression;
mod statement;

use crate::{TypeChecker, structs::CheckType};
use rl_ast::nodes::ExpressionKind;
use rl_ast::statements::{Statement, StatementKind, TypeAnnotation};
use rl_lexer::tokentypes::TokenType;
use rl_utils::span::Span;

impl TypeChecker {
    /// Returns the expected return type of the current function or lambda, if any.
    pub fn current_return_type(&self) -> Option<&TypeAnnotation> {
        self.return_type_stack.last()
    }
    /// Pushes `ty` as the expected return type when entering a function or lambda body.
    pub fn push_return_type(&mut self, ty: TypeAnnotation) {
        self.return_type_stack.push(ty);
        self.inferred_return_stack.push(Vec::new());
    }
    /// Pops the expected return type when exiting a function or lambda body,
    /// returning the `return`-statement types collected for inference.
    pub fn pop_return_type(&mut self) -> Vec<CheckType> {
        self.return_type_stack.pop();
        self.inferred_return_stack.pop().unwrap_or_default()
    }

    /// Infers an undeclared function return type: explicit `return`s win,
    /// else the trailing-expression type when the body ends with one.
    /// Returns `None` unless every candidate agrees on one concrete type.
    /// When the body uses `?` (`propagated`), the unwrapped type is
    /// wrapped in `Result`, mirroring a `-> result[T]` annotation. A
    /// union of results maps to the union of payloads instead.
    pub fn infer_fn_return(
        returned: Vec<CheckType>,
        trailing: Option<CheckType>,
        body_ends_with_expr: bool,
        propagated: bool,
    ) -> Option<TypeAnnotation> {
        let candidates: Vec<CheckType> = if returned.is_empty() {
            match (body_ends_with_expr, trailing) {
                (true, Some(ty)) => vec![ty],
                _ => return None,
            }
        } else {
            returned
        };
        let mut tys = candidates.into_iter();
        let first = match tys.next() {
            Some(CheckType::Known(ty)) => ty,
            _ => return None,
        };
        for ty in tys {
            match ty {
                CheckType::Known(other) if other == first => {}
                _ => return None,
            }
        }
        if propagated
            && !matches!(
                first,
                TypeAnnotation::Result(_) | TypeAnnotation::CResult(_)
            )
        {
            // union of results: unwrap each member instead of wrapping
            if let TypeAnnotation::Any(members) | TypeAnnotation::CAny(members) = &first {
                let mut inners = Vec::with_capacity(members.len());
                let mut all_results = true;
                for m in members.iter() {
                    match m {
                        TypeAnnotation::Result(inner) | TypeAnnotation::CResult(inner) => {
                            inners.push((**inner).clone())
                        }
                        _ => {
                            all_results = false;
                            break;
                        }
                    }
                }
                if all_results {
                    return Some(TypeAnnotation::Any(std::rc::Rc::new(inners)));
                }
            }
            return Some(TypeAnnotation::Result(Box::new(first)));
        }
        Some(first)
    }

    /// Element type for indexing `target_type` with `index_type`
    /// (extracted from the `Index` arm so union members probe it one by
    /// one). Emits diagnostics on failure.
    pub fn check_index_target(
        &mut self,
        target_type: &CheckType,
        index_type: &CheckType,
        expr_span: Span,
        index_span: Span,
    ) -> CheckType {
        match target_type {
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
            CheckType::Unknown | CheckType::Known(TypeAnnotation::Null) => CheckType::Unknown,
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

    /// Member-wise index for union targets: every member must index with
    /// the same index (each probed with diagnostics truncated, so only
    /// one loud error names the failing members). Element types merge:
    /// one distinct type stays concrete, several widen back into `any`.
    pub fn check_index_any(
        &mut self,
        members: Vec<TypeAnnotation>,
        index_type: &CheckType,
        expr_span: Span,
        index_span: Span,
    ) -> CheckType {
        let mut flat: Vec<TypeAnnotation> = Vec::new();
        for t in members {
            match t {
                TypeAnnotation::Any(n) | TypeAnnotation::CAny(n) => {
                    flat.extend(n.iter().cloned())
                }
                other => flat.push(other),
            }
        }
        let mut results: Vec<TypeAnnotation> = Vec::new();
        let mut failures: Vec<TypeAnnotation> = Vec::new();
        for m in &flat {
            let err_len = self.errors.len();
            let warn_len = self.warnings.len();
            let r = self.check_index_target(
                &CheckType::Known(m.clone()),
                index_type,
                expr_span,
                index_span,
            );
            let failed = self.errors.len() > err_len;
            self.errors.truncate(err_len);
            self.warnings.truncate(warn_len);
            match r {
                CheckType::Known(t) if !failed => {
                    if !results.contains(&t) {
                        results.push(t);
                    }
                }
                _ => failures.push(m.clone()),
            }
        }
        if !failures.is_empty() {
            let bad: Vec<String> = failures.iter().map(|t| format!("{:?}", t)).collect();
            self.error(
                format!(
                    "invalid index operation for every member of the union: {}",
                    bad.join(", ")
                ),
                expr_span,
            );
            return CheckType::Unknown;
        }
        match results.len() {
            0 => {
                self.error("invalid index operation for union".to_string(), expr_span);
                CheckType::Unknown
            }
            1 => CheckType::Known(results.into_iter().next().unwrap()),
            _ => CheckType::Known(TypeAnnotation::Any(std::rc::Rc::new(results))),
        }
    }

    // functions for loops to track `break` and similiar
    /// Member-wise method resolution for union receivers: every member
    /// must resolve the method with the same arguments (each probed
    /// with diagnostics truncated, so only one loud error names the
    /// failing members). Return types merge: one distinct type stays
    /// concrete, several widen back into `any`.
    pub fn check_method_any(
        &mut self,
        members: std::rc::Rc<Vec<TypeAnnotation>>,
        arg_types: &[(CheckType, Span)],
        method: &[String],
        span: Span,
    ) -> CheckType {
        let mut flat: Vec<TypeAnnotation> = Vec::new();
        for t in members.iter() {
            match t {
                TypeAnnotation::Any(n) | TypeAnnotation::CAny(n) => {
                    flat.extend(n.iter().cloned())
                }
                other => flat.push(other.clone()),
            }
        }
        let mut results: Vec<TypeAnnotation> = Vec::new();
        let mut failures: Vec<TypeAnnotation> = Vec::new();
        let mut saw_unknown = false;
        for m in &flat {
            let mut args = arg_types.to_vec();
            if let Some(first) = args.first_mut() {
                first.0 = CheckType::Known(m.clone());
            }
            let err_len = self.errors.len();
            let warn_len = self.warnings.len();
            // impl-block methods resolve via the methods map, like the
            // non-union fast path; everything else goes generic
            let r = if method.len() == 1 {
                match m {
                    TypeAnnotation::Record(rname) | TypeAnnotation::CRecord(rname) => {
                        match self
                            .methods
                            .get(&(rname.clone(), method[0].clone()))
                            .cloned()
                        {
                            Some(sig) => self.check_call_value(sig, &args, span),
                            None => self.check_call_path(method, &args, span),
                        }
                    }
                    _ => self.check_call_path(method, &args, span),
                }
            } else {
                self.check_call_path(method, &args, span)
            };
            let failed = self.errors.len() > err_len;
            self.errors.truncate(err_len);
            self.warnings.truncate(warn_len);
            match r {
                // untyped callees resolve silently to Unknown (same
                // leniency as non-union calls); only recorded errors
                // fail. Unknown dominates the merge: one unknowable
                // member means the result is unknowable.
                CheckType::Unknown if !failed => saw_unknown = true,
                CheckType::Known(t) if !failed => {
                    if !results.contains(&t) {
                        results.push(t);
                    }
                }
                _ => failures.push(m.clone()),
            }
        }
        if !failures.is_empty() {
            let bad: Vec<String> = failures.iter().map(|t| format!("{:?}", t)).collect();
            self.error(
                format!(
                    "method {} not supported for every member of the union: {}",
                    method.join("::"),
                    bad.join(", ")
                ),
                span,
            );
            return CheckType::Unknown;
        }
        if saw_unknown {
            return CheckType::Unknown;
        }
        match results.len() {
            0 => {
                self.error(
                    format!("method {} not supported for union", method.join("::")),
                    span,
                );
                CheckType::Unknown
            }
            1 => CheckType::Known(results.into_iter().next().unwrap()),
            _ => CheckType::Known(TypeAnnotation::Any(std::rc::Rc::new(results))),
        }
    }

    /// Detects `name is Type` (or its `!`/grouped negation) in a branch
    /// condition for type refinement. Returns the binding name and the
    /// type it takes in the taken branch: the tested type for `if`, the
    /// union minus the tested type for `else` (unchanged when that would
    /// empty it). Only plain variable references refine.
    pub fn branch_refinement(
        &mut self,
        cond: rl_ast::ExprId,
        polarity: bool,
    ) -> Option<(String, TypeAnnotation)> {
        let kind = self.ast_arena.exprs.get(cond).kind.clone();
        match kind {
            ExpressionKind::Is { value, target_type } => {
                let v = self.ast_arena.exprs.get(value);
                let (name, span) = match &v.kind {
                    ExpressionKind::Identifier(n) => (n.to_string(), v.span),
                    ExpressionKind::ResolvedIdentifier { name, .. } => (name.to_string(), v.span),
                    _ => return None,
                };
                let current = self.lookup(&name, span);
                if polarity {
                    Some((name, target_type))
                } else {
                    match &current.ty {
                        CheckType::Known(
                            TypeAnnotation::Any(members) | TypeAnnotation::CAny(members),
                        ) => {
                            let mut rest: Vec<TypeAnnotation> = members
                            .iter()
                            .filter(|m| *m != &target_type)
                            .cloned()
                            .collect();
                        // a lone remainder collapses to the member itself
                        // (single-member unions only arise from inference)
                        match rest.len() {
                            0 => None,
                            1 => Some((name, rest.pop().unwrap())),
                            _ => Some((
                                name,
                                TypeAnnotation::Any(std::rc::Rc::new(rest)),
                            )),
                        }
                        }
                        _ => None,
                    }
                }
            }
            ExpressionKind::Unary { operator, operand } if operator == TokenType::Bang => {
                self.branch_refinement(operand, !polarity)
            }
            ExpressionKind::Grouping(inner) => self.branch_refinement(inner, polarity),
            _ => None,
        }
    }

    /// Whether any statement in `statements` (at any depth, including
    /// nested function bodies whose closures capture the binding)
    /// assigns to `name`. Conservative: unknown shapes count as
    /// assignments, so refinement fails closed rather than unsound.
    /// Element mutation (`x[0] = v`) never rebinds and is ignored.
    pub fn body_assigns_name(
        statements: &[Statement],
        arena: &rl_ast::arena::Arena<rl_ast::nodes::Expression>,
        name: &str,
    ) -> bool {
        Self::body_assigns_name_inner(statements, arena, name, false)
    }

    /// Inner assign scan with `count_fns`: when true, any function or
    /// lambda definition also counts. Closures observe reassignment
    /// (verified live), so in a body that reassigns the refined binding
    /// a closure created anywhere may observe the new value.
    /// Checks `body` with `name` refined to `refined`, dropping the
    /// refinement (popping its scope) at the first statement assigning
    /// `name` or defining a function that could observe a later
    /// reassignment. Linear and order-sensitive: statements before the
    /// cutoff see the narrow type, statements after see the declared one.
    pub fn check_block_refined(
        &mut self,
        body: &[Statement],
        name: String,
        refined: TypeAnnotation,
        span: Span,
    ) {
        self.push_scope();
        self.declare(name.clone(), CheckType::Known(refined), false, span);
        // the refinement's "use" was the condition test itself: never
        // warn it as unused when the body only writes the binding
        if let Some(scope) = self.scopes.last_mut()
            && let Some(item) = scope.get_mut(&name)
        {
            item.suppressed_lints.insert(rl_ast::statements::Lint::Unused);
        }
        let mut active = true;
        for stmt in body {
            if active
                && Self::stmt_assigns_name(stmt, &self.ast_arena.exprs, &name, true)
            {
                active = false;
                self.pop_scope();
            }
            self.check_statement(stmt);
        }
        if active {
            self.pop_scope();
        }
    }

    pub fn body_assigns_name_inner(
        statements: &[Statement],
        arena: &rl_ast::arena::Arena<rl_ast::nodes::Expression>,
        name: &str,
        count_fns: bool,
    ) -> bool {
        statements
            .iter()
            .any(|s| Self::stmt_assigns_name(s, arena, name, count_fns))
    }

    fn stmt_assigns_name(
        stmt: &Statement,
        arena: &rl_ast::arena::Arena<rl_ast::nodes::Expression>,
        name: &str,
        count_fns: bool,
    ) -> bool {
        match &stmt.kind {
            StatementKind::VariableDeclaration { value, .. }
            | StatementKind::ResolvedVariableDeclaration { value, .. }
            | StatementKind::ConstantDeclaration { value, .. }
            | StatementKind::ResolvedConstantDeclaration { value, .. }
            | StatementKind::ResolvedArray { value, .. }
            | StatementKind::ResolvedConstantArray { value, .. }
            | StatementKind::ResolvedMap { value, .. }
            | StatementKind::ResolvedConstantMap { value, .. }
            | StatementKind::ResolvedSet { value, .. }
            | StatementKind::ResolvedConstantSet { value, .. }
            | StatementKind::ResolvedDestructureDeclaration { value, .. } => {
                Self::expr_assigns_name(arena, *value, name, count_fns)
            }
            StatementKind::DestructureDeclaration { bindings, value } => {
                bindings.iter().any(|(_, n)| n == name)
                    || Self::expr_assigns_name(arena, *value, name, count_fns)
            }
            StatementKind::Array { value, .. } | StatementKind::ConstantArray { value, .. } => {
                value.iter().any(|id| Self::expr_assigns_name(arena, *id, name, count_fns))
            }
            StatementKind::Map { entries, .. } | StatementKind::ConstantMap { entries, .. } => {
                entries
                    .iter()
                    .any(|(k, v)| Self::expr_assigns_name(arena, *k, name, count_fns) || Self::expr_assigns_name(arena, *v, name, count_fns))
            }
            StatementKind::Set { items, .. } | StatementKind::ConstantSet { items, .. } => {
                items.iter().any(|id| Self::expr_assigns_name(arena, *id, name, count_fns))
            }
            StatementKind::Expression(id) => Self::expr_assigns_name(arena, *id, name, count_fns),
            StatementKind::Return(Some(id)) => Self::expr_assigns_name(arena, *id, name, count_fns),
            StatementKind::While { condition, body } => {
                Self::expr_assigns_name(arena, *condition, name, count_fns)
                    || Self::body_assigns_name_inner(body, arena, name, count_fns)
            }
            StatementKind::Loop(body) => Self::body_assigns_name_inner(body, arena, name, count_fns),
            StatementKind::For {
                initializer,
                condition,
                increment,
                body,
            }
            | StatementKind::ResolvedFor {
                initializer,
                condition,
                increment,
                body,
            } => {
                Self::stmt_assigns_name(initializer, arena, name, count_fns)
                    || Self::expr_assigns_name(arena, *condition, name, count_fns)
                    || Self::expr_assigns_name(arena, *increment, name, count_fns)
                    || Self::body_assigns_name_inner(body, arena, name, count_fns)
            }
            StatementKind::ForRange {
                variable, range, body, ..
            }
            | StatementKind::ResolvedForRange {
                variable, range, body, ..
            } => {
                variable == name
                    || Self::stmt_assigns_name(range, arena, name, count_fns)
                    || Self::body_assigns_name_inner(body, arena, name, count_fns)
            }
            StatementKind::ForEach {
                variable, iterable, body, ..
            }
            | StatementKind::ResolvedForEach {
                variable, iterable, body, ..
            } => {
                variable == name
                    || Self::expr_assigns_name(arena, *iterable, name, count_fns)
                    || Self::body_assigns_name_inner(body, arena, name, count_fns)
            }
            StatementKind::Conditional { if_branch, else_branch } => {
                Self::stmt_assigns_name(if_branch, arena, name, count_fns)
                    || else_branch
                        .as_ref()
                        .is_some_and(|b| Self::stmt_assigns_name(b, arena, name, count_fns))
            }
            StatementKind::ConditionalBranch { condition, body, .. } => {
                condition.as_ref().is_some_and(|c| Self::expr_assigns_name(arena, *c, name, count_fns))
                    || Self::body_assigns_name_inner(body, arena, name, count_fns)
            }
            StatementKind::FunctionDeclaration { params, body, .. }
            | StatementKind::ResolvedFunctionDeclaration { params, body, .. } => {
                count_fns
                    || params.iter().any(|p| p.param_name == name)
                    || Self::body_assigns_name_inner(body, arena, name, count_fns)
            }
            StatementKind::ImplBlock { methods, .. } | StatementKind::ResolvedImplBlock { methods, .. } => {
                count_fns
                    || methods.iter().any(|m| Self::stmt_assigns_name(m, arena, name, count_fns))
            }
            StatementKind::Match { value, arms } => {
                Self::expr_assigns_name(arena, *value, name, count_fns)
                    || arms.iter().any(|(pat, b)| {
                        let pat_assigns = match pat {
                            rl_ast::statements::MatchPattern::Literal(id) => {
                                Self::expr_assigns_name(arena, *id, name, count_fns)
                            }
                            rl_ast::statements::MatchPattern::Wildcard => false,
                        };
                        pat_assigns || Self::body_assigns_name_inner(b, arena, name, count_fns)
                    })
            }
            StatementKind::TypeAlias { .. }
            | StatementKind::RecordDeclaration { .. }
            | StatementKind::TagDeclaration { .. }
            | StatementKind::Import { .. }
            | StatementKind::ImportFile { .. }
            | StatementKind::ResolvedImportFile { .. }
            | StatementKind::ImportFileNamed { .. }
            | StatementKind::Return(None)
            | StatementKind::Break
            | StatementKind::Continue
            | StatementKind::Range(_) => false,
            _ => true,
        }
    }

    fn expr_assigns_name(
        arena: &rl_ast::arena::Arena<rl_ast::nodes::Expression>,
        id: rl_ast::ExprId,
        name: &str,
        count_fns: bool,
    ) -> bool {
        let expr = arena.get(id);
        match &expr.kind {
            ExpressionKind::Assign { name: n, value }
            | ExpressionKind::ResolvedAssign { name: n, value, .. } => {
                n == name || Self::expr_assigns_name(arena, *value, name, count_fns)
            }
            ExpressionKind::Lambda { params, body, .. }
            | ExpressionKind::ResolvedLambda { params, body, .. } => {
                count_fns
                    || params.iter().any(|p| p.param_name == name)
                    || Self::body_assigns_name_inner(body, arena, name, count_fns)
            }
            ExpressionKind::Binary { left, right, .. } => {
                Self::expr_assigns_name(arena, *left, name, count_fns)
                    || Self::expr_assigns_name(arena, *right, name, count_fns)
            }
            ExpressionKind::Unary { operand, .. }
            | ExpressionKind::Grouping(operand)
            | ExpressionKind::Propagate(operand)
            | ExpressionKind::OkLiteral(operand)
            | ExpressionKind::ErrLiteral(operand)
            | ExpressionKind::ErrorLiteral(operand) => Self::expr_assigns_name(arena, *operand, name, count_fns),
            ExpressionKind::Call { args, .. } => args
                .iter()
                .any(|a| Self::expr_assigns_name(arena, *a, name, count_fns)),
            ExpressionKind::MethodCall { caller, args, .. } => {
                Self::expr_assigns_name(arena, *caller, name, count_fns)
                    || args.iter().any(|a| Self::expr_assigns_name(arena, *a, name, count_fns))
            }
            ExpressionKind::CallExpr { callee, args } => {
                Self::expr_assigns_name(arena, *callee, name, count_fns)
                    || args.iter().any(|a| Self::expr_assigns_name(arena, *a, name, count_fns))
            }
            ExpressionKind::FieldAccess { target, .. } => {
                Self::expr_assigns_name(arena, *target, name, count_fns)
            }
            ExpressionKind::Index { target, index } => {
                Self::expr_assigns_name(arena, *target, name, count_fns)
                    || Self::expr_assigns_name(arena, *index, name, count_fns)
            }
            ExpressionKind::IndexAssign {
                target,
                index,
                value,
            } => {
                Self::expr_assigns_name(arena, *target, name, count_fns)
                    || Self::expr_assigns_name(arena, *index, name, count_fns)
                    || Self::expr_assigns_name(arena, *value, name, count_fns)
            }
            ExpressionKind::FieldAssign { target, value, .. } => {
                Self::expr_assigns_name(arena, *target, name, count_fns)
                    || Self::expr_assigns_name(arena, *value, name, count_fns)
            }
            ExpressionKind::ArrayLiteral(elems)
            | ExpressionKind::SetLiteral(elems)
            | ExpressionKind::TupleLiteral(elems) => {
                elems.iter().any(|e| Self::expr_assigns_name(arena, *e, name, count_fns))
            }
            ExpressionKind::MapLiteral(pairs) => pairs.iter().any(|(k, v)| {
                Self::expr_assigns_name(arena, *k, name, count_fns) || Self::expr_assigns_name(arena, *v, name, count_fns)
            }),
            ExpressionKind::StructLiteral { fields, .. } => fields
                .iter()
                .any(|(_, v)| Self::expr_assigns_name(arena, *v, name, count_fns)),
            ExpressionKind::Cast { value, .. } | ExpressionKind::Is { value, .. } => {
                Self::expr_assigns_name(arena, *value, name, count_fns)
            }
            // leaves never assign
            ExpressionKind::Null
            | ExpressionKind::Integer(_)
            | ExpressionKind::SInt(_)
            | ExpressionKind::UInt(_)
            | ExpressionKind::SUInt(_)
            | ExpressionKind::Float(_)
            | ExpressionKind::SFloat(_)
            | ExpressionKind::Bool(_)
            | ExpressionKind::String(_)
            | ExpressionKind::Character(_)
            | ExpressionKind::Byte(_)
            | ExpressionKind::SByte(_)
            | ExpressionKind::BByte(_)
            | ExpressionKind::BSByte(_)
            | ExpressionKind::Identifier(_)
            | ExpressionKind::ResolvedIdentifier { .. }
            | ExpressionKind::EnumVariant { .. } => false,
            _ => true,
        }
    }

    pub fn loop_depth(&self) -> u32 {
        self.loop_depth
    }
    pub fn enter_loop(&mut self) {
        self.loop_depth += 1;
    }
    pub fn exit_loop(&mut self) {
        self.loop_depth = self.loop_depth.saturating_sub(1);
    }
    /// Checks all statements in `statements` inside a fresh scope.
    pub fn check_block(&mut self, statements: &[Statement]) {
        self.push_scope();
        for stmt in statements {
            self.check_statement(stmt);
        }
        self.pop_scope();
    }

    /// Emits a `"value is null"` error if `item_type` is [`CheckType::Known(Null)`].
    ///
    /// Returns `false` if null, `true` otherwise.
    pub fn check_is_null(&mut self, item_type: &CheckType, span: Span) -> bool {
        if item_type.is_null() {
            self.error("value is null", span);
            false
        } else {
            true
        }
    }

    /// Converts a [`CheckType`] to a [`TypeAnnotation`], mapping `Unknown` to `Null`.
    pub(crate) fn to_type_annotation(item_type: &CheckType) -> TypeAnnotation {
        match item_type {
            CheckType::Known(t) => t.clone(),
            CheckType::Function { .. } => TypeAnnotation::Fn,
            CheckType::Unknown => TypeAnnotation::Null,
        }
    }

    pub(crate) fn is_hashable_key_type(ty: &TypeAnnotation) -> bool {
        matches!(
            ty,
            TypeAnnotation::Int
                | TypeAnnotation::CInt
                | TypeAnnotation::UInt
                | TypeAnnotation::CUInt
                | TypeAnnotation::String
                | TypeAnnotation::CString
                | TypeAnnotation::Bool
                | TypeAnnotation::CBool
                | TypeAnnotation::Byte
                | TypeAnnotation::CByte
                | TypeAnnotation::Char
                | TypeAnnotation::CChar
                | TypeAnnotation::Null
        )
    }
}
