//! Unary operator type checking.
//!
//! | Operator | Operand         | Result |
//! |----------|-----------------|--------|
//! | `!`      | bool            | bool   |
//! | `-`      | int             | int    |
//! | `-`      | float           | float  |
//!
//! `uint` deliberately has no `-` rule - negating an unsigned value doesn't
//! produce another valid `uint`, so it's rejected here as a type error
//! rather than silently wrapping or falling back to `int`.
//!
//! A unit-carrying operand keeps its unit through `-` (negation doesn't
//! change dimensionality); `!` always produces a dimensionless `bool`.

use crate::operators::op_str;
use crate::structs::{CheckType, CheckedExpr, TypeChecker};
use crate::units::Unit;
use rl_ast::statements::TypeAnnotation;
use rl_lexer::tokentypes::TokenType;
use rl_utils::span::Span;

impl TypeChecker {
    pub fn check_unary_operator(
        &mut self,
        operand: CheckedExpr,
        _operand_span: Span,
        op: &TokenType,
        span: Span,
    ) -> CheckedExpr {
        if operand.ty.is_unknown() {
            return CheckedExpr::new(CheckType::Unknown, None);
        }
        // union operand: every member must satisfy the operator
        // (probed without emitting), results merge back into one type
        if matches!(
            &operand.ty,
            CheckType::Known(TypeAnnotation::Any(_) | TypeAnnotation::CAny(_))
        ) {
            return self.check_unary_any(operand, op, span);
        }
        match op {
            // is it correct bang unary?
            TokenType::Bang => match &operand.ty {
                CheckType::Known(TypeAnnotation::Bool | TypeAnnotation::CBool) => {
                    CheckedExpr::new(CheckType::Known(TypeAnnotation::Bool), None)
                }
                _ => {
                    self.error(
                        format!("type mismatch on !: got {}", operand.ty.info()),
                        span,
                    );
                    CheckedExpr::new(CheckType::Unknown, None)
                }
            },
            // is it correect minus unary?
            TokenType::Minus => match &operand.ty {
                CheckType::Known(TypeAnnotation::Int | TypeAnnotation::CInt) => {
                    // negation keeps the operand's unit
                    CheckedExpr::new(CheckType::Known(TypeAnnotation::Int), operand.unit)
                }
                CheckType::Known(TypeAnnotation::Float | TypeAnnotation::CFloat) => {
                    CheckedExpr::new(CheckType::Known(TypeAnnotation::Float), operand.unit)
                }
                CheckType::Known(TypeAnnotation::UInt | TypeAnnotation::CUInt) => {
                    self.error(
                        "cannot negate a uint value - uint has no negative range".to_string(),
                        span,
                    );
                    CheckedExpr::new(CheckType::Unknown, None)
                }
                _ => {
                    self.error(
                        format!("type mismatch on unary -: got {}", operand.ty.info()),
                        span,
                    );
                    CheckedExpr::new(CheckType::Unknown, None)
                }
            },
            // undefined unary
            _ => {
                self.error(format!("unknown unary operator {:?}", op), span);
                CheckedExpr::new(CheckType::Unknown, None)
            }
        }
    }

    /// Member-wise unary check for union operands: every member must
    /// satisfy the operator (each probed with diagnostics truncated, so
    /// only one loud error names the failing members). Result types
    /// merge: one distinct type stays concrete, several widen to `any`.
    fn check_unary_any(
        &mut self,
        operand: CheckedExpr,
        op: &TokenType,
        span: Span,
    ) -> CheckedExpr {
        let members: Vec<TypeAnnotation> = match &operand.ty {
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
            _ => return self.check_unary_operator(operand, span, op, span),
        };
        let mut results: Vec<(TypeAnnotation, Option<Unit>)> = Vec::new();
        let mut failures: Vec<TypeAnnotation> = Vec::new();
        for m in &members {
            let err_len = self.errors.len();
            let warn_len = self.warnings.len();
            let r = self.check_unary_operator(
                CheckedExpr::new(CheckType::Known(m.clone()), operand.unit.clone()),
                span,
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
                _ => failures.push(m.clone()),
            }
        }
        // `!` renders via the shared operator name table
        let op_name = op_str(op);
        if !failures.is_empty() {
            let bad: Vec<String> = failures.iter().map(|t| format!("{:?}", t)).collect();
            self.error(
                format!(
                    "operator {} not supported for every member of {}: {}",
                    op_name,
                    operand.ty.info(),
                    bad.join(", ")
                ),
                span,
            );
            return CheckedExpr::new(CheckType::Unknown, None);
        }
        match results.len() {
            0 => {
                self.error(
                    format!(
                        "operator {} not supported for {}",
                        op_name,
                        operand.ty.info()
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
}
