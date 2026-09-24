//! Static proving for contracts (phase 2): evaluates parameter
//! refinements and `requires` clauses at call sites with constant
//! arguments. Proven violations are compile errors; anything else falls
//! through to the runtime guards unchanged. `ensures` stays runtime-only
//! (it needs body analysis, not call-site values).
//!
//! Only literal arguments fold (integers, floats, strings, bools).
//! Anything else - variables, calls, uncertain shapes - is silently
//! skipped, so proving never rejects a program the runtime would accept.

use std::collections::HashMap;

use rl_ast::{
    ExprId,
    nodes::ExpressionKind,
    statements::{ContractClause, Param, ParamRefinement, RefineOp, RefineOperand},
};
use rl_lexer::tokentypes::TokenType;

use crate::TypeChecker;

/// Contract info retained per user function for call-site proving.
#[derive(Debug, Clone, Default)]
pub struct FnContracts {
    pub params: Vec<Param>,
    pub requires: Vec<ContractClause>,
}

/// A folded constant: the only shapes the prover reasons about.
#[derive(Debug, Clone, PartialEq)]
enum ConstVal {
    Int(i64),
    Float(f64),
    Str(String),
    Bool(bool),
}

impl TypeChecker {
    /// Records a function's contracts for later call-site proving.
    /// Call with the declared params and `requires` clauses; skips
    /// contract-free functions so the table stays small.
    pub fn record_fn_contracts(&mut self, name: &str, params: &[Param], requires: &[ContractClause]) {
        if requires.is_empty() && params.iter().all(|p| p.refinement.is_none()) {
            return;
        }
        self.fn_contracts.insert(
            name.to_string(),
            FnContracts {
                params: params.to_vec(),
                requires: requires.to_vec(),
            },
        );
    }

    /// Proves a call's contracts against constant arguments. Emits a
    /// compile error for each proven violation; proven-satisfied and
    /// unknown predicates stay silent (runtime guards enforce them).
    pub fn prove_call(&mut self, name: &str, args: &[ExprId], span: rl_utils::span::Span) {
        let Some(contracts) = self.fn_contracts.get(name).cloned() else {
            return;
        };
        // Caller-scope bindings are deliberately out of scope: only
        // literals fold, so `env` maps callee params to constants.
        let mut env: HashMap<String, ConstVal> = HashMap::new();
        for (param, arg) in contracts.params.iter().zip(args.iter()) {
            if let Some(value) = self.const_eval(*arg, &env) {
                env.insert(param.param_name.clone(), value);
            }
        }
        for (param, arg) in contracts.params.iter().zip(args.iter()) {
            let Some(refinement) = &param.refinement else {
                continue;
            };
            // The argument must itself be constant; otherwise there is
            // nothing to prove (the runtime guard decides).
            let Some(actual) = self.const_eval(*arg, &env) else {
                continue;
            };
            let Some(expected) = Self::refine_bound(refinement, &env) else {
                continue;
            };
            if let Some(false) = Self::compare_values(refinement.op, &actual, &expected) {
                self.error(
                    format!(
                        "contract violation (proven at compile time): refinement failed: {} {} {}",
                        param.param_name,
                        refine_op_str(refinement.op),
                        operand_str(&refinement.operand),
                    ),
                    span,
                );
            }
        }
        for clause in &contracts.requires {
            if let Some(ConstVal::Bool(false)) = self.const_eval(clause.condition, &env) {
                self.error(
                    format!(
                        "contract violation (proven at compile time): {}",
                        Self::clause_message(self, clause),
                    ),
                    span,
                );
            }
        }
    }

    /// Resolves a refinement operand to a constant: literals directly,
    /// parameter names through already-folded arguments.
    fn refine_bound(
        refinement: &ParamRefinement,
        env: &HashMap<String, ConstVal>,
    ) -> Option<ConstVal> {
        match &refinement.operand {
            RefineOperand::Integer(v) => Some(ConstVal::Int(*v)),
            RefineOperand::Str(s) => Some(ConstVal::Str(s.clone())),
            RefineOperand::Bool(b) => Some(ConstVal::Bool(*b)),
            RefineOperand::Param(p) => env.get(p).cloned(),
        }
    }

    /// Reads a clause's custom message when it is a string literal.
    fn clause_message(&self, clause: &ContractClause) -> String {
        if let Some(id) = clause.message
            && let ExpressionKind::String(s) = &self.ast_arena.exprs.get(id).kind
        {
            return s.clone();
        }
        "requires violated".to_string()
    }

    /// Folds an expression to a constant under `env` (callee params bound
    /// to constant arguments). Anything non-constant yields `None`.
    fn const_eval(&self, id: ExprId, env: &HashMap<String, ConstVal>) -> Option<ConstVal> {
        match &self.ast_arena.exprs.get(id).kind {
            ExpressionKind::Integer(v) => Some(ConstVal::Int(*v)),
            ExpressionKind::SInt(v) => Some(ConstVal::Int(*v as i64)),
            ExpressionKind::UInt(v) => i64::try_from(*v).ok().map(ConstVal::Int),
            ExpressionKind::Float(v) => Some(ConstVal::Float(*v)),
            ExpressionKind::SFloat(v) => Some(ConstVal::Float(*v as f64)),
            ExpressionKind::String(s) => Some(ConstVal::Str(s.clone())),
            ExpressionKind::Bool(b) => Some(ConstVal::Bool(*b)),
            ExpressionKind::Identifier(name) => env.get(name).cloned(),
            ExpressionKind::Grouping(inner) => self.const_eval(*inner, env),
            ExpressionKind::Unary { operator, operand } => {
                let value = self.const_eval(*operand, env)?;
                match operator {
                    TokenType::Bang => match value {
                        ConstVal::Bool(b) => Some(ConstVal::Bool(!b)),
                        _ => None,
                    },
                    TokenType::Minus => match value {
                        ConstVal::Int(i) => i.checked_neg().map(ConstVal::Int),
                        ConstVal::Float(f) => Some(ConstVal::Float(-f)),
                        _ => None,
                    },
                    _ => None,
                }
            }
            ExpressionKind::Binary { left, operator, right } => {
                let a = self.const_eval(*left, env)?;
                let b = self.const_eval(*right, env)?;
                match operator {
                    TokenType::Compare => Self::compare_values(RefineOp::Eq, &a, &b).map(ConstVal::Bool),
                    TokenType::BangEqual => Self::compare_values(RefineOp::Ne, &a, &b).map(ConstVal::Bool),
                    TokenType::Less => Self::compare_values(RefineOp::Lt, &a, &b).map(ConstVal::Bool),
                    TokenType::LessEqual => Self::compare_values(RefineOp::Le, &a, &b).map(ConstVal::Bool),
                    TokenType::Greater => Self::compare_values(RefineOp::Gt, &a, &b).map(ConstVal::Bool),
                    TokenType::GreaterEqual => Self::compare_values(RefineOp::Ge, &a, &b).map(ConstVal::Bool),
                    _ => Self::arith_values(operator, &a, &b),
                }
            }
            _ => None,
        }
    }

    /// Constant arithmetic (`+ - * /`): same-type ints, floats with int
    /// promotion, nothing else. Division by zero yields `None`.
    fn arith_values(op: &TokenType, a: &ConstVal, b: &ConstVal) -> Option<ConstVal> {
        use TokenType::{Minus, Plus, Slash, Star};
        let is_arith = matches!(op, Plus | Minus | Star | Slash);
        if !is_arith {
            return None;
        }
        match (a, b) {
            (ConstVal::Int(x), ConstVal::Int(y)) => match op {
                Plus => x.checked_add(*y).map(ConstVal::Int),
                Minus => x.checked_sub(*y).map(ConstVal::Int),
                Star => x.checked_mul(*y).map(ConstVal::Int),
                Slash => {
                    if *y == 0 {
                        None
                    } else {
                        x.checked_div(*y).map(ConstVal::Int)
                    }
                }
                _ => None,
            },
            (ConstVal::Float(_), _) | (_, ConstVal::Float(_)) => {
                let (x, y) = (as_f64(a), as_f64(b));
                if x.is_nan() || y.is_nan() {
                    return None;
                }
                match op {
                    Plus => Some(ConstVal::Float(x + y)),
                    Minus => Some(ConstVal::Float(x - y)),
                    Star => Some(ConstVal::Float(x * y)),
                    Slash => {
                        if y == 0.0 {
                            None
                        } else {
                            Some(ConstVal::Float(x / y))
                        }
                    }
                    _ => None,
                }
            }
            _ => None,
        }
    }

    /// Constant comparison: strict same-type equality plus int/float
    /// promotion for ordering. Anything else yields `None`.
    fn compare_values(op: RefineOp, a: &ConstVal, b: &ConstVal) -> Option<bool> {
        match (a, b) {
            (ConstVal::Int(x), ConstVal::Int(y)) => Some(cmp_i64(op, *x, *y)),
            (ConstVal::Float(_), _) | (_, ConstVal::Float(_)) => {
                let (x, y) = (as_f64(a), as_f64(b));
                if x.is_nan() || y.is_nan() {
                    return None;
                }
                Some(match op {
                    RefineOp::Eq => x == y,
                    RefineOp::Ne => x != y,
                    RefineOp::Lt => x < y,
                    RefineOp::Le => x <= y,
                    RefineOp::Gt => x > y,
                    RefineOp::Ge => x >= y,
                })
            }
            (ConstVal::Str(x), ConstVal::Str(y)) => Some(match op {
                RefineOp::Eq => x == y,
                RefineOp::Ne => x != y,
                RefineOp::Lt => x < y,
                RefineOp::Le => x <= y,
                RefineOp::Gt => x > y,
                RefineOp::Ge => x >= y,
            }),
            (ConstVal::Bool(x), ConstVal::Bool(y)) => Some(match op {
                RefineOp::Eq => x == y,
                RefineOp::Ne => x != y,
                _ => return None,
            }),
            _ => None,
        }
    }
}

fn as_f64(v: &ConstVal) -> f64 {
    match v {
        ConstVal::Int(i) => *i as f64,
        ConstVal::Float(f) => *f,
        ConstVal::Str(_) => f64::NAN,
        ConstVal::Bool(_) => f64::NAN,
    }
}

fn cmp_i64(op: RefineOp, x: i64, y: i64) -> bool {
    match op {
        RefineOp::Eq => x == y,
        RefineOp::Ne => x != y,
        RefineOp::Lt => x < y,
        RefineOp::Le => x <= y,
        RefineOp::Gt => x > y,
        RefineOp::Ge => x >= y,
    }
}

fn refine_op_str(op: RefineOp) -> &'static str {
    match op {
        RefineOp::Gt => ">",
        RefineOp::Ge => ">=",
        RefineOp::Lt => "<",
        RefineOp::Le => "<=",
        RefineOp::Eq => "==",
        RefineOp::Ne => "!=",
    }
}

fn operand_str(operand: &RefineOperand) -> String {
    match operand {
        RefineOperand::Integer(v) => v.to_string(),
        RefineOperand::Str(s) => format!("\"{s}\""),
        RefineOperand::Bool(b) => b.to_string(),
        RefineOperand::Param(p) => p.clone(),
    }
}
