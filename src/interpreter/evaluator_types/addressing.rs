use crate::{
    ast::nodes::{Expression, ExpressionKind},
    interpreter::{
        evaluator::{EnvironmentItem, Evaluator},
        values::Value,
    },
    utils::{errors::Error, span::Span},
};

pub fn get_root_addr(expression: &Expression) -> (usize, usize) {
    match &expression.kind {
        ExpressionKind::ResolvedIdentifier { depth, slot, .. } => (*depth, *slot),
        ExpressionKind::Index { target, .. } => get_root_addr(target),
        _ => unreachable!("index_assign: unexpected root expression"),
    }
}

pub fn get_indices_as_vec(
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
                    return Err(evaluator.err(format!("index cannot be negative: {}", i), span));
                }
                indices.push(i as usize);
            }

            Ok(indices)
        }
        _ => unreachable!(),
    }
}

