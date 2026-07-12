//! Environment stack operations - scope management and slot-indexed value access.
//!
//! The environment is a `Vec<Vec<EnvironmentItem>>` - a stack of frames.
//! Each frame is a flat vec of slots indexed by the resolver's slot numbers.
//! Depth 0 = current frame, depth 1 = one scope up, etc.
//!
//! Globals live in a separate `self.globals` field, addressed when `depth`
//! points past the end of the local `environment` stack. This keeps globals
//! from ever needing to be cloned in/out on function calls.

use crate::{
    evaluator::{EnvironmentItem, Evaluator, PItem},
    values::Value,
};
use rl_ast::statements::TypeAnnotation;
use rl_utils::{errors::Error, span::Span};

impl Evaluator {
    /// Pushes a new empty scope frame onto the scope pool stack.
    pub fn push_scope(&mut self) {
        let frame = self.scope_pool.pop().unwrap_or_default();
        self.environment.push(frame);
    }

    /// Pops the innermost scope frame.
    pub fn pop_scope(&mut self) {
        if let Some(mut frame) = self.environment.pop() {
            frame.clear();
            self.scope_pool.push(frame);
        }
    }

    fn ensure_slot(frame: &mut Vec<EnvironmentItem>, slot: usize, item: EnvironmentItem) {
        match slot.cmp(&frame.len()) {
            std::cmp::Ordering::Less => frame[slot] = item,
            std::cmp::Ordering::Equal => frame.push(item),
            std::cmp::Ordering::Greater => {
                frame.resize_with(slot, || {
                    EnvironmentItem::PItem(PItem {
                        value: Value::Null,
                        type_annotation: TypeAnnotation::Null,
                        is_const: false,
                    })
                });
                frame.push(item);
            }
        }
    }

    /// Reads the value at `(depth, slot)` in the environment.
    ///
    /// `depth` counts from the innermost frame outward (0 = current).
    /// `depth >= environment.len()` means the target is the global scope.
    pub fn get_value(&self, depth: usize, slot: usize, span: Span) -> Result<Value, Error> {
        if depth >= self.environment.len() {
            return match self.globals.get(slot) {
                Some(EnvironmentItem::PItem(p)) => Ok(p.value.clone()),
                None => Err(self.err(format!("undefined variable at ({}, {})", depth, slot), span)),
            };
        }
        let idx = self.environment.len() - 1 - depth;
        match self.environment.get(idx).and_then(|f| f.get(slot)) {
            Some(EnvironmentItem::PItem(p)) => Ok(p.value.clone()),
            None => Err(self.err(format!("undefined variable at ({}, {})", depth, slot), span)),
        }
    }

    /// Inserts a mutable value at `slot` in the current (innermost) frame.
    ///
    /// Grows the frame with `Null` placeholders if `slot` is past the end.
    pub fn insert_value(
        &mut self,
        slot: usize,
        value: Value,
        type_annotation: TypeAnnotation,
        _: Span,
    ) -> Result<(), Error> {
        if self.environment.is_empty() {
            Self::ensure_slot(
                &mut self.globals,
                slot,
                EnvironmentItem::PItem(PItem {
                    value,
                    type_annotation,
                    is_const: false,
                }),
            );
            return Ok(());
        }

        let frame = self
            .environment
            .last_mut()
            .expect("checked non-empty above");

        Self::ensure_slot(
            frame,
            slot,
            EnvironmentItem::PItem(PItem {
                value,
                type_annotation,
                is_const: false,
            }),
        );
        Ok(())
    }

    /// Inserts a mutable value at `slot` in the global scope.
    ///
    /// Used for top-level `dec` declarations, which live in `self.globals`
    /// rather than the local `environment` stack.
    pub fn insert_global(&mut self, slot: usize, value: Value, type_annotation: TypeAnnotation) {
        Self::ensure_slot(
            &mut self.globals,
            slot,
            EnvironmentItem::PItem(PItem {
                value,
                type_annotation,
                is_const: false,
            }),
        );
    }

    /// Inserts an immutable (const) value at `slot` in the current frame.
    ///
    /// Returns an error if the slot already holds a const.
    pub fn insert_const(
        &mut self,
        slot: usize,
        value: Value,
        type_annotation: TypeAnnotation,
        span: Span,
    ) -> Result<(), Error> {
        if self.environment.is_empty() {
            Self::ensure_slot(
                &mut self.globals,
                slot,
                EnvironmentItem::PItem(PItem {
                    value,
                    type_annotation,
                    is_const: true,
                }),
            );
            return Ok(());
        }

        if let Some(EnvironmentItem::PItem(p)) = self.environment.last().and_then(|f| f.get(slot))
            && p.is_const
        {
            return Err(self.err(format!("slot {} is already a constant", slot), span));
        }

        let frame = self
            .environment
            .last_mut()
            .expect("checked non-empty above");

        Self::ensure_slot(
            frame,
            slot,
            EnvironmentItem::PItem(PItem {
                value,
                type_annotation,
                is_const: true,
            }),
        );
        Ok(())
    }

    /// Inserts an immutable (const) value at `slot` in the global scope.
    ///
    /// Used for top-level `const` declarations. Returns an error if the slot
    /// already holds a const.
    pub fn insert_const_global(
        &mut self,
        slot: usize,
        value: Value,
        type_annotation: TypeAnnotation,
        span: Span,
    ) -> Result<(), Error> {
        if let Some(EnvironmentItem::PItem(p)) = self.globals.get(slot)
            && p.is_const
        {
            return Err(self.err(format!("slot {} is already a constant", slot), span));
        }
        Self::ensure_slot(
            &mut self.globals,
            slot,
            EnvironmentItem::PItem(PItem {
                value,
                type_annotation,
                is_const: true,
            }),
        );
        Ok(())
    }

    /// Overwrites the value at `(depth, slot)`, enforcing type compatibility and immutability.
    ///
    /// `depth >= environment.len()` means the target is the global scope.
    pub fn assign_value(
        &mut self,
        depth: usize,
        slot: usize,
        value: Value,
        value_type: TypeAnnotation,
        span: Span,
    ) -> Result<(), Error> {
        if depth >= self.environment.len() {
            if self.globals.get(slot).is_none() {
                return Err(self.err(format!("undefined slot {} at depth {}", slot, depth), span));
            }
            let entry = &mut self.globals[slot];
            return match entry {
                EnvironmentItem::PItem(p) => {
                    if p.is_const {
                        return Err(self.err("cannot assign to constant", span));
                    }
                    let declared = p.type_annotation.clone();
                    let types_match = matches!(value, Value::Null)
                        || match (&declared, &value_type) {
                            (TypeAnnotation::Array(_), TypeAnnotation::Array(inner))
                                if **inner == TypeAnnotation::Null =>
                            {
                                true
                            }
                            (TypeAnnotation::Array(a), TypeAnnotation::Array(_))
                                if **a == TypeAnnotation::Null =>
                            {
                                true
                            }
                            _ => Evaluator::types_compatible(&value_type, &declared),
                        };
                    if !types_match {
                        return Err(self.err(
                            format!(
                                "type mismatch: cannot assign {:?} to {:?}",
                                value_type, declared
                            ),
                            span,
                        ));
                    }
                    p.value = value;
                    Ok(())
                }
            };
        }

        let idx = self.environment.len() - 1 - depth;
        if slot >= self.environment[idx].len() {
            return Err(self.err(format!("undefined slot {} at depth {}", slot, depth), span));
        }
        let entry = &mut self.environment[idx][slot];
        match entry {
            EnvironmentItem::PItem(p) => {
                if p.is_const {
                    return Err(self.err("cannot assign to constant", span));
                }
                let declared = p.type_annotation.clone();
                let types_match = matches!(value, Value::Null)
                    || match (&declared, &value_type) {
                        (TypeAnnotation::Array(_), TypeAnnotation::Array(inner))
                            if **inner == TypeAnnotation::Null =>
                        {
                            true
                        }
                        (TypeAnnotation::Array(a), TypeAnnotation::Array(_))
                            if **a == TypeAnnotation::Null =>
                        {
                            true
                        }
                        _ => Evaluator::types_compatible(&value_type, &declared),
                    };
                if !types_match {
                    return Err(self.err(
                        format!(
                            "type mismatch: cannot assign {:?} to {:?}",
                            value_type, declared
                        ),
                        span,
                    ));
                }
                p.value = value;
                Ok(())
            }
        }
    }

    /// Looks up a variable by name via the resolver - used only in tests.
    pub fn get_value_raw(&self, name: &str) -> Option<Value> {
        let (depth, slot) = self.resolver.resolve_name(name)?;
        if depth >= self.environment.len() {
            return match self.globals.get(slot)? {
                EnvironmentItem::PItem(p) => Some(p.value.clone()),
            };
        }
        let idx = self.environment.len() - 1 - depth;
        match self.environment.get(idx)?.get(slot)? {
            EnvironmentItem::PItem(p) => Some(p.value.clone()),
        }
    }

    /// Returns the declared [`TypeAnnotation`] of the slot at `(depth, slot)`, if it exists.
    pub fn get_declared_type(&self, depth: usize, slot: usize) -> Option<TypeAnnotation> {
        if depth >= self.environment.len() {
            return match self.globals.get(slot)? {
                EnvironmentItem::PItem(p) => Some(p.type_annotation.clone()),
            };
        }
        let idx = self.environment.len() - 1 - depth;
        match self.environment.get(idx)?.get(slot)? {
            EnvironmentItem::PItem(p) => Some(p.type_annotation.clone()),
        }
    }
}
