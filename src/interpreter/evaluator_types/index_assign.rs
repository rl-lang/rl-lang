//! Index-assign evaluation (`arr[i] = value`, including nested `arr[i][j] = value`).
//!
//! Uses two recursive helpers:
//! - `get_root_addr` - walks the chain of `Index` nodes to find the root `ResolvedIdentifier`
//! - `get_indices_as_vec` - evaluates all intermediate indices into a `Vec<usize>`
//!
//! The final index is appended, then the function traverses into the nested `Value::Values`
//! structure mutably to perform the assignment, enforcing type compatibility and bounds.

use crate::{
    ast::{
        nodes::{Expression, ExpressionKind},
        statements::TypeAnnotation,
    },
    interpreter::{
        evaluator::{EnvironmentItem, Evaluator},
        values::Value,
    },
    utils::{errors::Error, span::Span},
};

impl Evaluator {
    pub fn index_assign(
        &mut self,
        target: &Expression,
        index: &Expression,
        value: &Expression,
        span: Span,
    ) -> Result<Value, Error> {
        let idx = self.evaluate(index)?;
        let val = self.evaluate(value)?;

        fn get_root_addr(expression: &Expression) -> (usize, usize) {
            match &expression.kind {
                ExpressionKind::ResolvedIdentifier { depth, slot, .. } => (*depth, *slot),
                ExpressionKind::Index { target, .. } => get_root_addr(target),
                _ => unreachable!("index_assign: unexpected root expression"),
            }
        }

        fn get_indices_as_vec(
            expression: &Expression,
            evaluator: &mut Evaluator,
            span: Span,
        ) -> Result<Vec<usize>, Error> {
            match &expression.kind {
                ExpressionKind::ResolvedIdentifier { .. } => Ok(vec![]),
                ExpressionKind::Index { target, index } => {
                    let mut indices = get_indices_as_vec(target, evaluator, span)?;
                    if let Value::Integer(i) = evaluator.evaluate(index)? {
                        if i < 0 {
                            return Err(
                                evaluator.err(format!("index cannot be negative: {}", i), span)
                            );
                        }
                        indices.push(i as usize);
                    }

                    Ok(indices)
                }
                _ => unreachable!(),
            }
        }

        let (depth, slot) = get_root_addr(target);
        let mut indices = get_indices_as_vec(target, self, span)?;
        if let Value::Integer(i) = idx {
            if i < 0 {
                return Err(self.err(format!("index cannot be negative: {}", i), span));
            }
            indices.push(i as usize);
        }

        let index_error = self.err("index assignment requires at least one index", span);
        let out_of_bounds_err = |i: usize| {
            Error::at(
                crate::utils::errors::Reason::Interpreter,
                format!("index {} out of bounds", i),
                span,
            )
        };

        let env_idx = self.environment.len().saturating_sub(1 + depth);
        let e = self.err(format!("no scope at depth {}", depth), span);
        let e2 = self.err(format!("undefined slot {} at depth {}", slot, depth), span);
        let frame = self.environment.get_mut(env_idx).ok_or(e)?;
        let entry = frame.get_mut(slot).ok_or(e2)?;

        match entry {
            EnvironmentItem::PItem(p) => {
                if p.is_const {
                    return Err(self.err("cannot assign to constant", span));
                }
                let mut current = &mut p.value;
                if !indices.is_empty() {
                    for i in &indices[..indices.len() - 1] {
                        if let Value::Values { items, .. } = current {
                            current = items.get_mut(*i).ok_or_else(|| out_of_bounds_err(*i))?;
                        }
                    }
                }
                if let Value::Values { items_type, items } = current {
                    let val_type = Self::infer_type(&val, false);
                    if val_type != *items_type && val_type != TypeAnnotation::Null {
                        return Err(Error::at(
                            crate::utils::errors::Reason::Interpreter,
                            format!(
                                "type mismatch: array is {:?}, cannot assign {:?}",
                                items_type, val_type
                            ),
                            span,
                        ));
                    }
                    let last = indices.last().ok_or(index_error)?;
                    if *last >= items.len() {
                        return Err(out_of_bounds_err(*last));
                    }
                    items[*last] = val.clone();
                }
                Ok(val)
            }
        }
    }
}
