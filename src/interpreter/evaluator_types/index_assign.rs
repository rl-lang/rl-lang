use crate::{
    ast::nodes::{Expression, ExpressionKind},
    interpreter::{evaluator::Evaluator, values::Value},
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
        ) -> Result<Vec<usize>, Error> {
            match &expression.kind {
                ExpressionKind::Identifier(_) => Ok(vec![]),
                ExpressionKind::Index { target, index } => {
                    let mut indices = get_indices_as_vec(target, evaluator)?;
                    if let Value::Integer(i) = evaluator.evaluate(index)? {
                        indices.push(i as usize);
                    }
                    Ok(indices)
                }
                _ => unreachable!(),
            }
        }

        let root = get_root_name(target).to_string();
        let mut indices = get_indices_as_vec(target, self)?;
        if let Value::Integer(i) = idx {
            indices.push(i as usize);
        }

        if let Some((_, true)) = self.environment.get(&root) {
            return Err(self.err(format!("cannot assign to constant '{}'", root), span));
        }

        let (root_array, _) = self.environment.get_mut(&root).unwrap();
        let mut current_array = root_array;

                for i in &indices[..indices.len() - 1] {
                    if let Value::Values(items) = current_array {
                        current_array = &mut items[*i];
                    }
                }
                if let Value::Values(items) = current_array {
                    items[*indices.last().unwrap()] = val.clone();
                }

                return val;
            }
        }
        if let Value::Values(items) = current_array {
            items[*indices.last().unwrap()] = val.clone();
        }

        Ok(val)
    }
}
