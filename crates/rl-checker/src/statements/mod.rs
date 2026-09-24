//! Statement-level helpers shared between expression and statement checking.

mod expression;
mod statement;

use crate::{TypeChecker, structs::CheckType};
use rl_ast::statements::{Statement, StatementKind, TypeAnnotation};
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
