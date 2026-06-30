//! Index-assign type checking (`arr[i] = value`).
//!
//! Validates:
//! - The index is an `int` or `byte`
//! - The value type matches the array's element type
//! - `byte` widens to `int` when the array element type is `int`
//! - Assigning `null` is always allowed (absence of value)

use crate::{
    ast::{nodes::Expression, statements::TypeAnnotation},
    checker::structs::{CheckType, TypeChecker},
    utils::span::Span,
};

impl TypeChecker {
    pub fn check_index_assign(
        &mut self,
        target: &Expression,
        index: &Expression,
        value: &Expression,
        span: Span,
    ) -> CheckType {
        let target_type = self.check_expression(target);
        let index_type = self.check_expression(index);
        let value_type = self.check_expression(value);

        // is the index int?
        if !matches!(
            index_type,
            CheckType::Known(TypeAnnotation::Int | TypeAnnotation::CInt) | CheckType::Unknown
        ) {
            self.error(
                format!("index must be int, got {}", index_type.info()),
                index.span,
            );
        }

        // does the value type match the array values type?
        match &target_type {
            CheckType::Known(TypeAnnotation::Array(inner))
            | CheckType::Known(TypeAnnotation::CArray(inner)) => {
                let inner_ty = CheckType::Known((**inner).clone());

                if !value_type.matches(&inner_ty) && !value_type.is_null() {
                    self.error(
                        format!(
                            "type mismatch: array is {}, cannot assign {}",
                            inner_ty.info(),
                            value_type.info()
                        ),
                        span,
                    );
                }
                value_type
            }
            CheckType::Unknown => CheckType::Unknown,
            other => {
                self.error(format!("cannot index-assign into {}", other.info()), span);
                CheckType::Unknown
            }
        }
    }
}
