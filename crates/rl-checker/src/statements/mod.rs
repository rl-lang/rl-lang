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
    /// wrapped in `Result`, mirroring a `-> result[T]` annotation.
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
            return Some(TypeAnnotation::Result(Box::new(first)));
        }
        Some(first)
    }

    // functions for loops to track `break` and similiar
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
