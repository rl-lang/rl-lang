//! Index-assign type checking (`arr[i] = value`).
//!
//! Validates:
//! - The index is an `int` or `byte`
//! - The value type matches the array's element type
//! - `byte` widens to `int` when the array element type is `int`
//! - Assigning `null` is always allowed (absence of value)

use crate::structs::{CheckType, TypeChecker};
use rl_ast::{ExprId, statements::TypeAnnotation};
use rl_utils::span::Span;

impl TypeChecker {
    pub fn check_index_assign(
        &mut self,
        target: ExprId,
        index: ExprId,
        value: ExprId,
        span: Span,
    ) -> CheckType {
        let target_type = self.check_expression(target);
        let index_type = self.check_expression(index);
        let value_type = self.check_expression(value);
        let index_id = self.ast_arena.exprs.get(index);

        // does the value type match the array/map values type?
        match &target_type {
            CheckType::Known(TypeAnnotation::Array(inner))
            | CheckType::Known(TypeAnnotation::CArray(inner)) => {
                // is the index int? (array-only rule; maps validate their
                // own key type below instead)
                if !matches!(
                    index_type,
                    CheckType::Known(TypeAnnotation::Int | TypeAnnotation::CInt)
                        | CheckType::Unknown
                ) {
                    self.error(
                        format!("index must be int, got {}", index_type.info()),
                        index_id.span,
                    );
                }

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
            CheckType::Known(TypeAnnotation::Set(_))
            | CheckType::Known(TypeAnnotation::CSet(_)) => {
                self.error("sets does not support index assigning", span);
                CheckType::Unknown
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
                        index_id.span,
                    );
                }

                let expected_value = CheckType::Known((**value_ty).clone());
                if !value_type.matches(&expected_value) && !value_type.is_null() {
                    self.error(
                        format!(
                            "type mismatch: map value is {}, cannot assign {}",
                            expected_value.info(),
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
