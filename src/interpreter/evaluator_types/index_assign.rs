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

        fn get_root_name(expression: &Expression) -> &str {
            match &expression.kind {
                ExpressionKind::Identifier(array_name) => array_name,
                ExpressionKind::Index { target, .. } => get_root_name(target),
                _ => unreachable!(),
            }
        }

        fn get_indices_as_vec(
            expression: &Expression,
            evaluator: &mut Evaluator,
            span: Span,
        ) -> Result<Vec<usize>, Error> {
            match &expression.kind {
                ExpressionKind::Identifier(_) => Ok(vec![]),
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

        let root = get_root_name(target).to_string();
        let mut indices = get_indices_as_vec(target, self, span)?;
        if let Value::Integer(i) = idx {
            if i < 0 {
                return Err(self.err(format!("index cannot be negative: {}", i), span));
            }
            indices.push(i as usize);
        }

        for scope in self.environment.iter().rev() {
            if let Some(env_item) = scope.get(&root) {
                match env_item {
                    EnvironmentItem::PItem(p) => {
                        if p.is_const {
                            return Err(
                                self.err(format!("cannot assign to constant '{}'", root), span)
                            );
                        }
                    }
                }
            }
        }

        let index_error = self.err("index assignment requires at least one index", span);
        let out_of_bounds_err = |i: usize| {
            Error::at(
                crate::utils::errors::Reason::Interpreter,
                format!("index {} out of bounds", i),
                span,
            )
        };
        for scope in self.environment.iter_mut().rev() {
            if let Some(env_item) = scope.get_mut(&root) {
                match env_item {
                    EnvironmentItem::PItem(p) => {
                        let mut current = &mut p.value;
                        if !indices.is_empty() {
                            for i in &indices[..indices.len() - 1] {
                                if let Value::Values { items, .. } = current {
                                    current =
                                        items.get_mut(*i).ok_or_else(|| out_of_bounds_err(*i))?;
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
                        return Ok(val);
                    }
                }
            }
        }
        Err(self.err(format!("undefined variable '{}'", root), span))
    }
}
