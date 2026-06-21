use crate::{
    ast::statements::TypeAnnotation,
    interpreter::{evaluator::Evaluator, values::Value},
    utils::{errors::Error, span::Span},
};

pub fn dice(eval: &mut Evaluator, sides: i64) -> i64 {
    eval.rng.generate_random_int_range(1, sides)
}

pub fn range(eval: &mut Evaluator, stop: i64, span: Span) -> Result<Value, Error> {
    if 0 == stop {
        return Err(eval.err("rand_range() stop shouldn't be zero", span));
    }
    if 0 > stop {
        return Err(eval.err("rand_range() stop shouldn't be less than zero", span));
    }

    Ok(Value::Integer(eval.rng.generate_random_int_range(0, stop)))
}

pub fn range_step(
    eval: &mut Evaluator,
    start: i64,
    end: i64,
    step: i64,
    span: Span,
) -> Result<Value, Error> {
    if 0 == step {
        return Err(eval.err("rand_range_step() stop shouldn't be zero", span));
    }
    if start >= end {
        return Err(eval.err(
            format!(
                "rand_range_step() end shouldn't be less than or equal to {}",
                start
            ),
            span,
        ));
    }

    let count = ((end - start) / step) + 1;
    let i = eval.rng.generate_random_int_range(0, count - 1);
    Ok(Value::Integer(start + i * step))
}

pub fn choice(eval: &mut Evaluator, array: Value, span: Span) -> Result<Value, Error> {
    match array {
        Value::Values { items, .. } => {
            if items.is_empty() {
                return Err(eval.err("array is empty", span));
            }
            Ok(items[eval
                .rng
                .generate_random_int_range(0, items.len() as i64 - 1)
                as usize]
                .clone())
        }

        other => Err(eval.err(
            format!("rand_choice() expected array found {}", other),
            span,
        )),
    }
}

pub fn choices(eval: &mut Evaluator, array: Value, count: i64, span: Span) -> Result<Value, Error> {
    if count <= 0 {
        return Err(eval.err("count should be 1 or higher", span));
    }

    match array.clone() {
        Value::Values { items_type, items } => {
            let result = (0..count)
                .map(|_| {
                    items[eval
                        .rng
                        .generate_random_int_range(0, items.len() as i64 - 1)
                        as usize]
                        .clone()
                })
                .collect();
            Ok(Value::Values {
                items_type,
                items: result,
            })
        }

        other => Err(eval.err(
            format!("rand_choices() expected array found {}", other),
            span,
        )),
    }
}

pub fn sample(eval: &mut Evaluator, array: Value, count: i64, span: Span) -> Result<Value, Error> {
    if count <= 0 {
        return Err(eval.err("count should be 1 or higher", span));
    }

    match array.clone() {
        Value::Values { items_type, items } => {
            if count as usize > items.len() {
                return Err(eval.err("count larger than array", span));
            }
            let mut indices: Vec<usize> = (0..items.len()).collect();
            for i in (1..items.len()).rev() {
                let j = eval.rng.generate_random_int_range(0, i as i64) as usize;
                indices.swap(i, j);
            }

            let result = indices[..count as usize]
                .iter()
                .map(|&i| items[i].clone())
                .collect();

            Ok(Value::Values {
                items_type,
                items: result,
            })
        }

        other => Err(eval.err(
            format!("rand_sample() expected array found {}", other),
            span,
        )),
    }
}

pub fn char(eval: &mut Evaluator) -> char {
    eval.rng.generate_random_int_range(32, 126) as u8 as char
}

pub fn byte(eval: &mut Evaluator) -> i64 {
    eval.rng.generate_random_int_range(0, 255)
}

pub fn string(eval: &mut Evaluator, count: i64, span: Span) -> Result<Value, Error> {
    if count as usize <= 0 {
        return Err(eval.err("count cannot be less than zero", span));
    }

    let result: String = (0..count).map(|_| char(eval)).collect();

    Ok(Value::String(result))
}

pub fn bytes(eval: &mut Evaluator, count: i64, span: Span) -> Result<Value, Error> {
    if count as usize <= 0 {
        return Err(eval.err("count cannot be less than zero", span));
    }

    let result: Vec<Value> = (0..count).map(|_| Value::Integer(byte(eval))).collect();

    Ok(Value::Values {
        items_type: TypeAnnotation::Int,
        items: result,
    })
}

pub fn shuffle(eval: &mut Evaluator, array: Value, span: Span) -> Result<Value, Error> {
    match array {
        Value::Values { items, items_type } => {
            if items.is_empty() {
                return Err(eval.err("array is empty", span));
            }

            let mut items = items;
            for i in (1..items.len()).rev() {
                let j = eval.rng.generate_random_int_range(0, i as i64) as usize;
                items.swap(i, j);
            }

            Ok(Value::Values { items_type, items })
        }

        other => Err(eval.err(
            format!("rand_shuffle() expected array found {}", other),
            span,
        )),
    }
}
