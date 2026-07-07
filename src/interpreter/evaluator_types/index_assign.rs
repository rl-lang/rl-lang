//! Index-assign evaluation (`arr[i] = value`, including nested `arr[i][j] = value`).
//!
//! Uses two recursive helpers:
//! - `get_root_addr` - walks the chain of `Index` nodes to find the root `ResolvedIdentifier`
//! - `get_indices_as_vec` - evaluates all intermediate indices into a `Vec<usize>`
//!
//! The final index is appended, then the function traverses into the nested `Value::Values`
//! structure mutably to perform the assignment, enforcing type compatibility and bounds.

use crate::{
    ast::{ExprId, nodes::ExpressionKind, statements::TypeAnnotation},
    interpreter::{
        evaluator::{EnvironmentItem, Evaluator},
        evaluator_types::addressing::{get_indices_as_vec, get_root_addr},
        values::{MapKey, Value},
    },
    utils::{errors::Error, span::Span},
};

impl Evaluator {
    pub fn index_assign(
        &mut self,
        target: ExprId,
        index: ExprId,
        value: ExprId,
        span: Span,
    ) -> Result<Value, Error> {
        // Map assignment: `target[key] = value` where `target` evaluates to
        // a map.
        if let ExpressionKind::ResolvedIdentifier { depth, slot, .. } =
            &self.resolver.ast_arena.exprs.get(target).kind
        {
            let (depth, slot) = (*depth, *slot);
            let is_map = matches!(
                self.slot_ref(depth, slot),
                Some(EnvironmentItem::PItem(p)) if matches!(p.value, Value::Map { .. })
            );
            let is_set = matches!(self.slot_ref(depth, slot),
            Some(EnvironmentItem::PItem(p)) if matches!(p.value, Value::Set { .. }));
            if is_set {
                return Err(self.err("sets does not support direct indexical assignments", span));
            }

            if is_map {
                let Some(EnvironmentItem::PItem(p)) = self.slot_ref(depth, slot) else {
                    unreachable!("checked is_map above");
                };
                let Value::Map {
                    key_type,
                    value_type,
                    entries,
                } = p.value.clone()
                else {
                    unreachable!("checked is_map above");
                };
                if p.is_const {
                    return Err(self.err("cannot assign to constant", span));
                }

                let idx = self.evaluate(index)?;
                let val = self.evaluate(value)?;

                let map_key = MapKey::from_value(&idx).ok_or_else(|| {
                    self.err(
                        format!("type {} cannot be used as a map key", idx.type_name()),
                        span,
                    )
                })?;

                let idx_type = Evaluator::infer_type(&idx, false);
                if idx_type != key_type && idx_type != TypeAnnotation::Null {
                    return Err(self.err(
                        format!(
                            "type mismatch: map key is {:?}, got {:?}",
                            key_type, idx_type
                        ),
                        span,
                    ));
                }
                let val_type = Evaluator::infer_type(&val, false);
                if val_type != value_type && val_type != TypeAnnotation::Null {
                    return Err(self.err(
                        format!(
                            "type mismatch: map value is {:?}, cannot assign {:?}",
                            value_type, val_type
                        ),
                        span,
                    ));
                }

                entries.borrow_mut().insert(map_key, val.clone());
                return Ok(val);
            }
            if is_set {}
        }

        let idx = self.evaluate(index)?;
        let val = self.evaluate(value)?;
        let (depth, slot) = get_root_addr(target, &self.resolver.ast_arena);
        let mut indices = get_indices_as_vec(target, self, span)?;
        if let Value::Integer(i) = idx {
            if i < 0 {
                return Err(self.err(format!("index cannot be negative: {}", i), span));
            }
            indices.push(i as usize);
        } else if let Value::Byte(b) = idx {
            indices.push(b as usize);
        } else {
            return Err(self.err(
                format!("invalid index operation: index is {}", idx.type_name()),
                span,
            ));
        }

        let index_error = self.err("index assignment requires at least one index", span);
        let out_of_bounds_err = |i: usize| {
            Error::at(
                crate::utils::errors::Reason::Interpreter,
                format!("index {} out of bounds", i),
                span,
            )
        };

        // `depth >= environment.len()` means the target lives in the global
        // scope, addressed via `self.globals` rather than the local frame
        // stack. See `scopes.rs` for the same convention.
        let entry: &mut EnvironmentItem = if depth >= self.environment.len() {
            if self.globals.get(slot).is_none() {
                return Err(self.err(format!("undefined slot {} at depth {}", slot, depth), span));
            }
            &mut self.globals[slot]
        } else {
            let env_idx = self.environment.len() - 1 - depth;
            if slot >= self.environment[env_idx].len() {
                return Err(self.err(format!("undefined slot {} at depth {}", slot, depth), span));
            }
            &mut self.environment[env_idx][slot]
        };

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
                    let val_type = Evaluator::infer_type(&val, false);
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
