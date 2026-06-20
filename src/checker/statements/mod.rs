mod expression;
mod statement;

use crate::{
    ast::statements::TypeAnnotation,
    checker::structs::{CheckType, TypeChecker},
    utils::span::Span,
};

impl TypeChecker {
    // functions for lambdas and functions to track return type
    pub fn current_return_type(&self) -> Option<&TypeAnnotation> {
        self.return_type_stack.last()
    }
    pub fn push_return_type(&mut self, ty: TypeAnnotation) {
        self.return_type_stack.push(ty);
    }
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
    pub fn check_block(&mut self, statements: &[crate::ast::statements::Statement]) {
        self.push_scope();
        for stmt in statements {
            self.check_statement(stmt);
        }
        self.pop_scope();
    }

    // checks weather the item has known type or not
    pub fn check_is_null(&mut self, item_type: &CheckType, span: Span) -> bool {
        if item_type.is_null() {
            self.error("value is null", span);
            false
        } else {
            true
        }
    }

    // transforms CheckType to TypeAnnotation
    // skips unknown types and assigns Null to them
    pub(crate) fn to_type_annotation(item_type: &CheckType) -> TypeAnnotation {
        match item_type {
            CheckType::Known(t) => t.clone(),
            CheckType::Function { .. } => TypeAnnotation::Fn,
            CheckType::Unknown => TypeAnnotation::Null,
        }
    }
}
