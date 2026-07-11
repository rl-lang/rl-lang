use crate::{
    ast::statements::TypeAnnotation,
    interpreter::evaluator::Evaluator,
    interpreter::stdlib::common::{verr, vi, vok, vs},
    interpreter::values::Value,
};

pub fn dice(eval: &mut Evaluator, sides: i64) -> Value {
    if sides <= 0 {
        return verr!(vs!("sides should be 1 or higher".to_string()));
    }
    vok!(vi!(eval.rng.generate_random_int_range(1, sides)))
}

pub fn dices(eval: &mut Evaluator, count: i64, sides: i64) -> Value {
    if count <= 0 {
        return verr!(vs!("count should be 1 or higher".to_string()));
    }
    if sides <= 0 {
        return verr!(vs!("sides should be 1 or higher".to_string()));
    }

    let result: Vec<Value> = (0..count)
        .map(|_| vi!(eval.rng.generate_random_int_range(1, sides)))
        .collect();

    vok!(Value::Values {
        items_type: TypeAnnotation::Int,
        items: result,
    })
}

pub fn range(eval: &mut Evaluator, stop: i64) -> Value {
    if 0 == stop {
        return verr!(vs!("rand_range() stop shouldn't be zero".to_string()));
    }
    if 0 > stop {
        return verr!(vs!(
            "rand_range() stop shouldn't be less than zero".to_string()
        ));
    }

    vok!(vi!(eval.rng.generate_random_int_range(0, stop)))
}

pub fn range_step(eval: &mut Evaluator, start: i64, end: i64, step: i64) -> Value {
    if 0 == step {
        return verr!(vs!("rand_range_step() stop shouldn't be zero".to_string()));
    }
    if start >= end {
        return verr!(vs!(format!(
            "rand_range_step() end shouldn't be less than or equal to {}",
            start
        )));
    }

    let count = ((end - start) / step) + 1;
    let i = eval.rng.generate_random_int_range(0, count - 1);
    vok!(vi!(start + i * step))
}

pub fn choice(eval: &mut Evaluator, array: Value) -> Value {
    match array {
        Value::Values { items, .. } => {
            if items.is_empty() {
                return verr!(vs!("array is empty".to_string()));
            }
            vok!(
                items[eval
                    .rng
                    .generate_random_int_range(0, items.len() as i64 - 1)
                    as usize]
                    .clone()
            )
        }

        other => verr!(vs!(format!("rand_choice() expected array found {}", other))),
    }
}

pub fn choices(eval: &mut Evaluator, array: Value, count: i64) -> Value {
    if count <= 0 {
        return verr!(vs!("count should be 1 or higher".to_string()));
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
            vok!(Value::Values {
                items_type,
                items: result,
            })
        }

        other => verr!(vs!(format!(
            "rand_choices() expected array found {}",
            other
        ))),
    }
}

pub fn sample(eval: &mut Evaluator, array: Value, count: i64) -> Value {
    if count <= 0 {
        return verr!(vs!("count should be 1 or higher".to_string()));
    }

    match array.clone() {
        Value::Values { items_type, items } => {
            if count as usize > items.len() {
                return verr!(vs!("count larger than array".to_string()));
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

            vok!(Value::Values {
                items_type,
                items: result,
            })
        }

        other => verr!(vs!(format!("rand_sample() expected array found {}", other))),
    }
}

pub fn char(eval: &mut Evaluator) -> char {
    eval.rng.generate_random_int_range(32, 126) as u8 as char
}

pub fn byte(eval: &mut Evaluator) -> i64 {
    eval.rng.generate_random_int_range(0, 255)
}

pub fn string(eval: &mut Evaluator, count: i64) -> Value {
    if count as usize <= 0 {
        return verr!(vs!("count cannot be less than zero".to_string()));
    }

    let result: String = (0..count).map(|_| char(eval)).collect();

    vok!(vs!(result))
}

pub fn bytes(eval: &mut Evaluator, count: i64) -> Value {
    if count as usize <= 0 {
        return verr!(vs!("count cannot be less than zero".to_string()));
    }

    let result: Vec<Value> = (0..count).map(|_| vi!(byte(eval))).collect();

    vok!(Value::Values {
        items_type: TypeAnnotation::Int,
        items: result,
    })
}

pub fn shuffle(eval: &mut Evaluator, array: Value) -> Value {
    match array {
        Value::Values { items, items_type } => {
            if items.is_empty() {
                return verr!(vs!("array is empty".to_string()));
            }

            let mut items = items;
            for i in (1..items.len()).rev() {
                let j = eval.rng.generate_random_int_range(0, i as i64) as usize;
                items.swap(i, j);
            }

            vok!(Value::Values { items_type, items })
        }

        other => verr!(vs!(format!(
            "rand_shuffle() expected array found {}",
            other
        ))),
    }
}
