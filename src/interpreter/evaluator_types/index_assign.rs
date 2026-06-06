use crate::{
    ast::nodes::Expression,
    interpreter::{evaluator::Evaluator, values::Value},
    utils::errors::Error,
};

impl Evaluator {
    pub fn index_assign(
        &mut self,
        target: &Expression,
        index: &Expression,
        value: &Expression,
    ) -> Value {
        let idx = self.evaluate(index);
        let val = self.evaluate(value);

        // get the root array name
        fn get_root_name(expression: &Expression) -> &str {
            match expression {
                Expression::Identifier(array_name) => array_name,
                Expression::Index { target, .. } => get_root_name(target),
                _ => unreachable!(),
            }
        }

        fn get_indices_as_vec(expression: &Expression, evaluator: &mut Evaluator) -> Vec<usize> {
            match expression {
                Expression::Identifier(_) => vec![],
                Expression::Index { target, index } => {
                    let mut indices = get_indices_as_vec(target, evaluator);
                    if let Value::Integer(i) = evaluator.evaluate(index) {
                        indices.push(i as usize);
                    }
                    indices
                }
                _ => unreachable!(),
            }
        }

        let root = get_root_name(target).to_string();
        let mut indices = get_indices_as_vec(target, self);
        if let Value::Integer(i) = idx {
            indices.push(i as usize);
        }

        for scope in self.environment.iter_mut().rev() {
            if let Some((root_array, is_const)) = scope.get_mut(&root) {
                if *is_const {
                    Error::init(
                        format!("cannot assign to constant '{}'", root_array),
                        None,
                        None,
                    )
                    .print_error();
                    unreachable!();
                }

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
        Error::init(format!("undefined variable {}", root), None, None).print_error();
        unreachable!()
    }
}
