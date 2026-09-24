//! Binary operator type and unit checking.
//!
//! # Rules
//!
//! | Operator          | Left / Right                    | Result  |
//! |-------------------|---------------------------------|---------|
//! | `+` `-` `*` `/`  | int + int                       | int     |
//! | `+` `-` `*` `/`  | uint + uint                     | uint    |
//! | `+` `-` `*` `/`  | float + float                   | float   |
//! | `+` `-` `*` `/`  | byte + byte                     | byte    |
//! | `+` `-` `*` `/`  | byte + int (or int + byte)      | int     |
//! | `<` `>` `<=` `>=`| int/byte pairs                  | bool    |
//! | `<` `>` `<=` `>=`| uint/uint pairs                 | bool    |
//! | `<` `>` `<=` `>=`| float + float                   | bool    |
//! | `==` `!=`         | matching primitive types        | bool    |
//!
//! `uint` does not mix with `int` or `byte` in arithmetic/comparison - both
//! sides must be `uint` (mirroring how `byte` is already strict about not
//! mixing with `int`, despite the table above's aspirational `byte + int`
//! row which isn't actually implemented below either).
//!
//! # Units
//!
//! When the operands carry units of measure:
//!
//! - `+` / `-` require compatible units (a dimensionless operand adopts the
//!   other side's unit); the result keeps that unit.
//! - `*` / `/` combine units by adding / subtracting exponents, e.g.
//!   `(m / s) * s = m`.
//! - comparisons and equality require compatible units and produce `bool`.
//!
//! Any side being `Unknown` short-circuits to `Unknown` to suppress cascading errors.

use crate::{
    operators::op_str,
    structs::{CheckType, CheckedExpr, TypeChecker},
    units::Unit,
};
use rl_ast::statements::TypeAnnotation;
use rl_lexer::tokentypes::TokenType;
use rl_utils::span::Span;

impl TypeChecker {
    #[allow(clippy::too_many_arguments)]
    #[allow(clippy::needless_borrow)]
    pub fn check_binary_operator(
        &mut self,
        left: &CheckedExpr,
        right: &CheckedExpr,
        op: &TokenType,
        span: Span,
    ) -> CheckedExpr {
        // if any of sides is unknown then it is unknown
        if left.ty.is_unknown() || right.ty.is_unknown() {
            return CheckedExpr::new(CheckType::Unknown, None);
        }

        // union operand: every member pair must satisfy the operator
        // (probed without emitting), results merge back into one type
        if matches!(
            &left.ty,
            CheckType::Known(TypeAnnotation::Any(_) | TypeAnnotation::CAny(_))
        ) || matches!(
            &right.ty,
            CheckType::Known(TypeAnnotation::Any(_) | TypeAnnotation::CAny(_))
        ) {
            return self.check_binary_any(left, right, op, span);
        }

        match op {
            // arithmetic check if both same type or not
            TokenType::Plus | TokenType::Minus | TokenType::Star | TokenType::Slash => {
                let result_type = match (&left.ty, &right.ty) {
                    (
                        CheckType::Known(TypeAnnotation::Int | TypeAnnotation::CInt),
                        CheckType::Known(TypeAnnotation::Int | TypeAnnotation::CInt),
                    ) => CheckType::Known(TypeAnnotation::Int),
                    (
                        CheckType::Known(TypeAnnotation::UInt | TypeAnnotation::CUInt),
                        CheckType::Known(TypeAnnotation::UInt | TypeAnnotation::CUInt),
                    ) => CheckType::Known(TypeAnnotation::UInt),
                    (
                        CheckType::Known(TypeAnnotation::Float | TypeAnnotation::CFloat),
                        CheckType::Known(TypeAnnotation::Float | TypeAnnotation::CFloat),
                    ) => CheckType::Known(TypeAnnotation::Float),
                    (
                        CheckType::Known(TypeAnnotation::Byte | TypeAnnotation::CByte),
                        CheckType::Known(TypeAnnotation::Byte | TypeAnnotation::CByte),
                    ) => CheckType::Known(TypeAnnotation::Byte),

                    _ => {
                        self.error(
                            format!(
                                "type mismatch on {}: got {} and {}",
                                op_str(op),
                                left.ty.info(),
                                right.ty.info()
                            ),
                            span,
                        );
                        CheckType::Unknown
                    }
                };

                if let CheckType::Unknown = result_type {
                    return CheckedExpr::new(result_type, None);
                }

                // `+` and `-` need compatible units; `*` and `/` combine them.
                let result_unit = match op {
                    TokenType::Plus | TokenType::Minus => {
                        if let Some(msg) =
                            add_unit_mismatch(&left.unit, &right.unit, &self.conversions)
                        {
                            self.error(format!("unit mismatch on {}: {}", op_str(op), msg), span);
                            None
                        } else {
                            // a dimensionless operand adopts the other side's unit
                            left.unit
                                .clone()
                                .filter(|u| !u.is_dimensionless())
                                .or_else(|| right.unit.clone().filter(|u| !u.is_dimensionless()))
                        }
                    }
                    TokenType::Star => combine_units(&left.unit, &right.unit, Unit::multiply),
                    TokenType::Slash => combine_units(&left.unit, &right.unit, Unit::divide),
                    _ => unreachable!("arithmetic operators are handled above"),
                };

                CheckedExpr::new(result_type, result_unit)
            }

            // comparisons should be same type
            TokenType::Less
            | TokenType::Greater
            | TokenType::LessEqual
            | TokenType::GreaterEqual => match (&left.ty, &right.ty) {
                (
                    CheckType::Known(TypeAnnotation::Int | TypeAnnotation::CInt),
                    CheckType::Known(TypeAnnotation::Int | TypeAnnotation::CInt),
                )
                | (
                    CheckType::Known(TypeAnnotation::Float | TypeAnnotation::CFloat),
                    CheckType::Known(TypeAnnotation::Float | TypeAnnotation::CFloat),
                ) => {
                    self.check_comparable_units(&left.unit, &right.unit, op, span);
                    CheckedExpr::new(CheckType::Known(TypeAnnotation::Bool), None)
                }
                (
                    CheckType::Known(TypeAnnotation::UInt | TypeAnnotation::CUInt),
                    CheckType::Known(TypeAnnotation::UInt | TypeAnnotation::CUInt),
                )
                | (
                    CheckType::Known(TypeAnnotation::Byte | TypeAnnotation::CByte),
                    CheckType::Known(TypeAnnotation::Byte | TypeAnnotation::CByte),
                ) => CheckedExpr::new(CheckType::Known(TypeAnnotation::Bool), None),

                _ => {
                    self.error(
                        format!(
                            "type mismatch on {}: got {} and {}",
                            op_str(op),
                            left.ty.info(),
                            right.ty.info()
                        ),
                        span,
                    );
                    CheckedExpr::new(CheckType::Unknown, None)
                }
            },

            // equality should be between same types
            TokenType::Compare | TokenType::BangEqual => {
                let ok = matches!(
                    (&left.ty, &right.ty),
                    (
                        CheckType::Known(TypeAnnotation::Int | TypeAnnotation::CInt),
                        CheckType::Known(TypeAnnotation::Int | TypeAnnotation::CInt),
                    ) | (
                        CheckType::Known(TypeAnnotation::UInt | TypeAnnotation::CUInt),
                        CheckType::Known(TypeAnnotation::UInt | TypeAnnotation::CUInt),
                    ) | (
                        CheckType::Known(TypeAnnotation::Byte | TypeAnnotation::CByte),
                        CheckType::Known(TypeAnnotation::Byte | TypeAnnotation::CByte),
                    ) | (
                        CheckType::Known(TypeAnnotation::Float | TypeAnnotation::CFloat),
                        CheckType::Known(TypeAnnotation::Float | TypeAnnotation::CFloat),
                    ) | (
                        CheckType::Known(TypeAnnotation::String | TypeAnnotation::CString),
                        CheckType::Known(TypeAnnotation::String | TypeAnnotation::CString),
                    ) | (
                        CheckType::Known(TypeAnnotation::Char | TypeAnnotation::CChar),
                        CheckType::Known(TypeAnnotation::Char | TypeAnnotation::CChar),
                    ) | (
                        CheckType::Known(TypeAnnotation::Bool | TypeAnnotation::CBool),
                        CheckType::Known(TypeAnnotation::Bool | TypeAnnotation::CBool),
                    ) | (
                        CheckType::Known(TypeAnnotation::Enum(_) | TypeAnnotation::CEnum(_)),
                        CheckType::Known(TypeAnnotation::Enum(_) | TypeAnnotation::CEnum(_)),
                    ) | (
                        CheckType::Known(TypeAnnotation::Record(_) | TypeAnnotation::CRecord(_)),
                        CheckType::Known(TypeAnnotation::Record(_) | TypeAnnotation::CRecord(_)),
                    )
                );
                if !ok {
                    self.error(
                        format!(
                            "type mismatch on {}: got {} and {}",
                            op_str(op),
                            left.ty.info(),
                            right.ty.info()
                        ),
                        span,
                    );
                } else {
                    self.check_comparable_units(&left.unit, &right.unit, op, span);
                }
                CheckedExpr::new(CheckType::Known(TypeAnnotation::Bool), None)
            }

            TokenType::And | TokenType::Or => {
                if !matches!(
                    left.ty,
                    CheckType::Known(TypeAnnotation::Bool | TypeAnnotation::CBool)
                ) {
                    self.error(
                        format!("expected bool on the left side of {}", op_str(op)),
                        span,
                    );
                }
                if !matches!(
                    right.ty,
                    CheckType::Known(TypeAnnotation::Bool | TypeAnnotation::CBool)
                ) {
                    self.error(
                        format!("expected bool on the right side of {}", op_str(op)),
                        span,
                    );
                }

                CheckedExpr::new(CheckType::Known(TypeAnnotation::Bool), None)
            }

            // unknown operator
            _ => {
                self.error(format!("unknown binary operator {:?}", op), span);
                CheckedExpr::new(CheckType::Unknown, None)
            }
        }
    }

    /// Member-wise binary check for union operands: every member pair
    /// must satisfy the operator (each pair is probed with diagnostics
    /// truncated, so only one loud error names the failing pairs).
    /// Result types merge: one distinct type stays concrete, several
    /// widen back into `any[...]`.
    fn check_binary_any(
        &mut self,
        left: &CheckedExpr,
        right: &CheckedExpr,
        op: &TokenType,
        span: Span,
    ) -> CheckedExpr {
        fn members(ty: &CheckType) -> Vec<TypeAnnotation> {
            match ty {
                // defensive flatten (the parser already normalizes)
                CheckType::Known(TypeAnnotation::Any(m) | TypeAnnotation::CAny(m)) => {
                    let mut out = Vec::new();
                    for t in m.iter() {
                        match t {
                            TypeAnnotation::Any(n) | TypeAnnotation::CAny(n) => {
                                out.extend(n.iter().cloned())
                            }
                            other => out.push(other.clone()),
                        }
                    }
                    out
                }
                CheckType::Known(t) => vec![t.clone()],
                _ => vec![],
            }
        }
        let lms = members(&left.ty);
        let rms = members(&right.ty);
        let mut results: Vec<(TypeAnnotation, Option<Unit>)> = Vec::new();
        let mut failures: Vec<(TypeAnnotation, TypeAnnotation)> = Vec::new();
        for lm in &lms {
            for rm in &rms {
                let err_len = self.errors.len();
                let warn_len = self.warnings.len();
                let r = self.check_binary_operator(
                    &CheckedExpr::new(CheckType::Known(lm.clone()), left.unit.clone()),
                    &CheckedExpr::new(CheckType::Known(rm.clone()), right.unit.clone()),
                    op,
                    span,
                );
                let failed = self.errors.len() > err_len;
                self.errors.truncate(err_len);
                self.warnings.truncate(warn_len);
                match r.ty {
                    CheckType::Known(t) if !failed => {
                        if !results.iter().any(|(e, _)| *e == t) {
                            results.push((t, r.unit));
                        }
                    }
                    _ => failures.push((lm.clone(), rm.clone())),
                }
            }
        }
        if !failures.is_empty() {
            let pairs: Vec<String> = failures
                .iter()
                .map(|(l, r)| {
                    format!("{:?} {} {:?}", l, op_str(op), r)
                })
                .collect();
            self.error(
                format!(
                    "operator {} not supported for every member of {} and {}: {}",
                    op_str(op),
                    left.ty.info(),
                    right.ty.info(),
                    pairs.join(", ")
                ),
                span,
            );
            return CheckedExpr::new(CheckType::Unknown, None);
        }
        match results.len() {
            0 => {
                self.error(
                    format!(
                        "operator {} not supported for {} and {}",
                        op_str(op),
                        left.ty.info(),
                        right.ty.info()
                    ),
                    span,
                );
                CheckedExpr::new(CheckType::Unknown, None)
            }
            1 => {
                let (t, u) = results.into_iter().next().unwrap();
                CheckedExpr::new(CheckType::Known(t), u)
            }
            _ => {
                let ts: Vec<TypeAnnotation> =
                    results.into_iter().map(|(t, _)| t).collect();
                CheckedExpr::new(
                    CheckType::Known(TypeAnnotation::Any(std::rc::Rc::new(ts))),
                    None,
                )
            }
        }
    }

    /// Emits a unit mismatch error for comparisons/equality when the two
    /// operands carry incompatible (non-dimensionless, non-convertible) units.
    fn check_comparable_units(
        &mut self,
        left: &Option<Unit>,
        right: &Option<Unit>,
        op: &TokenType,
        span: Span,
    ) {
        if let Some(msg) = add_unit_mismatch(left, right, &self.conversions) {
            self.error(format!("unit mismatch on {}: {}", op_str(op), msg), span);
        }
    }
}

/// Returns an error message when `left` and `right` carry incompatible units
/// for an adding/comparing operation. A dimensionless side never conflicts,
/// and symbols registered via `#![convert(...)]` are interchangeable.
fn add_unit_mismatch(
    left: &Option<Unit>,
    right: &Option<Unit>,
    conversions: &crate::units::ConversionTable,
) -> Option<String> {
    match (left.as_ref(), right.as_ref()) {
        (Some(l), Some(r)) if !l.is_compatible_with(r) && !l.is_convertible_to(r, conversions) => {
            Some(format!("got {} and {}", l, r))
        }
        _ => None,
    }
}

/// Combines the units of two operands under `op` (multiply or divide).
///
/// A missing unit is treated as dimensionless (`1`), and a dimensionless
/// result is normalized back to `None`.
fn combine_units(
    left: &Option<Unit>,
    right: &Option<Unit>,
    op: fn(&Unit, &Unit) -> Unit,
) -> Option<Unit> {
    let l = left.clone().unwrap_or_default();
    let r = right.clone().unwrap_or_default();
    let result = op(&l, &r);

    if result.is_dimensionless() {
        None
    } else {
        Some(result)
    }
}
