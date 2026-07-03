//! Helpers for resolving the "root address" (depth, slot) of an addressable
//! expression chain (`ResolvedIdentifier`, possibly wrapped in one or more
//! `Index` nodes), used by both index-read fast paths and index-assign.

use crate::{
    ast::{Ast, ExprId, nodes::ExpressionKind},
    interpreter::{
        evaluator::{EnvironmentItem, Evaluator},
        values::Value,
    },
    utils::{errors::Error, span::Span},
};

pub fn get_root_addr(id: ExprId, ast: &Ast) -> (usize, usize) {
    match &ast.exprs.get(id).kind {
        ExpressionKind::ResolvedIdentifier { depth, slot, .. } => (*depth, *slot),
        ExpressionKind::Index { target, .. } => get_root_addr(*target, ast),
        _ => unreachable!("index_assign: unexpected root expression"),
    }
}

/// Non-panicking variant of `get_root_addr`, for call sites (like Index
/// reads) where the target may not be addressable - e.g. foo()[0].
/// Returns `None` instead of panicking so the caller can fall back to
/// normal evaluation.
pub fn try_get_root_addr(id: ExprId, ast: &Ast) -> Option<(usize, usize)> {
    match &ast.exprs.get(id).kind {
        ExpressionKind::ResolvedIdentifier { depth, slot, .. } => Some((*depth, *slot)),
        ExpressionKind::Index { target, .. } => try_get_root_addr(*target, ast),
        _ => None,
    }
}

pub fn get_indices_as_vec(
    id: ExprId,
    evaluator: &mut Evaluator,
    span: Span,
) -> Result<Vec<usize>, Error> {
    let kind = evaluator.resolver.ast_arena.exprs.get(id).kind.clone();
    match kind {
        ExpressionKind::ResolvedIdentifier { .. } => Ok(vec![]),
        ExpressionKind::Index { target, index } => {
            let mut indices = get_indices_as_vec(target, evaluator, span)?;
            if let Value::Integer(i) = evaluator.evaluate(index)? {
                if i < 0 {
                    return Err(evaluator.err(format!("index cannot be negative: {}", i), span));
                }
                indices.push(i as usize);
            }

            Ok(indices)
        }
        _ => unreachable!(),
    }
}

impl Evaluator {
    pub fn slot_ref(&self, depth: usize, slot: usize) -> Option<&EnvironmentItem> {
        if depth >= self.environment.len() {
            self.globals.get(slot)
        } else {
            let idx = self.environment.len() - 1 - depth;
            self.environment.get(idx)?.get(slot)
        }
    }
    pub fn slot_mut(&mut self, depth: usize, slot: usize) -> Option<&mut EnvironmentItem> {
        if depth >= self.environment.len() {
            self.globals.get_mut(slot)
        } else {
            let idx = self.environment.len() - 1 - depth;
            self.environment.get_mut(idx)?.get_mut(slot)
        }
    }

    /// Reads a `value` by walking indices from the root `(depth, slot)`,
    /// borrowing at every step and cloning only the single final element
    /// avoids cloning the whole container just to read one item out of it.
    pub fn index_read(
        &self,
        depth: usize,
        slot: usize,
        indices: &[usize],
        span: Span,
    ) -> Result<Value, Error> {
        let entry = self.slot_ref(depth, slot).ok_or_else(|| {
            self.err(format!("undefined variable at ({}, {})", depth, slot), span)
        })?;
        let EnvironmentItem::PItem(p) = entry;

        let mut current = &p.value;
        for &i in indices {
            current = match current {
                Value::Values { items, .. } => items.get(i),
                Value::Tuple(items) => items.get(i),
                _ => None,
            }
            .ok_or_else(|| self.err(format!("index {} out of bounds", i), span))?;
        }
        Ok(current.clone())
    }
}
