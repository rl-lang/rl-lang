use crate::{
    ast::statements::TypeAnnotation,
    interpreter::{
        evaluator::{EnvironmentItem, Evaluator, PItem},
        values::Value,
    },
    utils::{errors::Error, span::Span},
};

impl Evaluator {
    pub fn push_scope(&mut self) {
        self.environment.push(vec![]);
    }

    pub fn pop_scope(&mut self) {
        self.environment.pop();
    }

    fn ensure_slot(frame: &mut Vec<EnvironmentItem>, slot: usize, item: EnvironmentItem) {
        if slot >= frame.len() {
            frame.resize_with(slot + 1, || {
                EnvironmentItem::PItem(PItem {
                    value: Value::Null,
                    type_annotation: TypeAnnotation::Null,
                    is_const: false,
                })
            });
        }
        frame[slot] = item;
    }

    pub fn get_value(&self, depth: usize, slot: usize, span: Span) -> Result<Value, Error> {
        let idx = self.environment.len().saturating_sub(1 + depth);
        match self.environment.get(idx).and_then(|f| f.get(slot)) {
            Some(EnvironmentItem::PItem(p)) => Ok(p.value.clone()),
            None => Err(self.err(format!("undefined variable at ({}, {})", depth, slot), span)),
        }
    }

    pub fn insert_value(
        &mut self,
        slot: usize,
        value: Value,
        type_annotation: TypeAnnotation,
        span: Span,
    ) -> Result<(), Error> {
        let e = self.err("no active scope", span);
        let frame = self.environment.last_mut().ok_or(e)?;
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

    pub fn insert_const(
        &mut self,
        slot: usize,
        value: Value,
        type_annotation: TypeAnnotation,
        span: Span,
    ) -> Result<(), Error> {
        let e = self.err("no active scope", span);
        let frame = self.environment.last_mut().ok_or(e)?;
        if let Some(EnvironmentItem::PItem(p)) = frame.get(slot)
            && p.is_const
        {
            return Err(self.err(format!("slot {} is already a constant", slot), span));
        }
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

    pub fn assign_value(
        &mut self,
        depth: usize,
        slot: usize,
        value: Value,
        value_type: TypeAnnotation,
        span: Span,
    ) -> Result<(), Error> {
        let e = self.err(format!("no scope at depth {}", depth), span);
        let e2 = self.err(format!("undefined slot {} at depth {}", slot, depth), span);
        let idx = self.environment.len().saturating_sub(1 + depth);
        let frame = self.environment.get_mut(idx).ok_or(e)?;
        let entry = frame.get_mut(slot).ok_or(e2)?;
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

    // tests only
    pub fn get_value_raw(&self, name: &str) -> Option<Value> {
        let (depth, slot) = self.resolver.resolve_name(name)?;
        let idx = self.environment.len().saturating_sub(1 + depth);
        match self.environment.get(idx)?.get(slot)? {
            crate::interpreter::evaluator::EnvironmentItem::PItem(p) => Some(p.value.clone()),
        }
    }

    pub fn get_declared_type(&self, depth: usize, slot: usize) -> Option<TypeAnnotation> {
        let idx = self.environment.len().saturating_sub(1 + depth);
        match self.environment.get(idx)?.get(slot)? {
            EnvironmentItem::PItem(p) => Some(p.type_annotation.clone()),
        }
    }
}
