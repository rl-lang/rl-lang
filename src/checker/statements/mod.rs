//! Statement-level helpers shared between expression and statement checking.

mod expression;
mod statement;

use crate::{
    ast::statements::TypeAnnotation,
    checker::structs::{CheckType, TypeChecker},
    utils::span::Span,
};

impl TypeChecker {
    /// Returns the expected return type of the current function or lambda, if any.
    pub fn current_return_type(&self) -> Option<&TypeAnnotation> {
        self.return_type_stack.last()
    }
    /// Pushes `ty` as the expected return type when entering a function or lambda body.
    pub fn push_return_type(&mut self, ty: TypeAnnotation) {
        self.return_type_stack.push(ty);
    }
    /// Pops the expected return type when exiting a function or lambda body.
    pub fn pop_return_type(&mut self) {
        self.return_type_stack.pop();
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
    pub fn check_block(&mut self, statements: &[crate::ast::statements::Statement]) {
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
}
