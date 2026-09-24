//! `std::random` - random number generation using a custom Xoshiro256** PRNG.
//!
//! The PRNG state is stored on the runtime context (accessed via
//! [`Runtime::rng`]) and seeded from the system clock at startup. All random
//! functions share this single instance. Ported once from the former
//! per-runtime `stdlib/random/*.rs` copies.

use rl_std_core::Runtime;
use rl_std_macros::native_fn;

// ---- plain scalar generators ----------------------------------------------

#[native_fn(module = "random")]
pub fn rand_int<R: Runtime>(cx: &mut R::Cx) -> i64 {
    R::rng(cx).generate_random_int_range(i64::MIN, i64::MAX)
}

#[native_fn(module = "random")]
pub fn rand_float<R: Runtime>(cx: &mut R::Cx) -> f64 {
    R::rng(cx).generate_random_float()
}

#[native_fn(module = "random")]
pub fn rand_bool<R: Runtime>(cx: &mut R::Cx) -> bool {
    let rand_float = R::rng(cx).generate_random_float();
    R::rng(cx).generate_random_bool(rand_float)
}

#[native_fn(module = "random")]
pub fn rand_bool_weighted<R: Runtime>(cx: &mut R::Cx, weight: f64) -> bool {
    R::rng(cx).generate_random_bool(weight)
}

#[native_fn(module = "random")]
pub fn rand_char<R: Runtime>(cx: &mut R::Cx) -> char {
    R::rng(cx).generate_random_int_range(32, 126) as u8 as char
}

#[native_fn(module = "random")]
pub fn rand_byte<R: Runtime>(cx: &mut R::Cx) -> u8 {
    R::rng(cx).generate_random_int_range(0, 255) as u8
}

// ---- fallible scalar generators (language `result[T]`) --------------------

#[native_fn(module = "random")]
pub fn rand_int_range<R: Runtime>(cx: &mut R::Cx, min: i64, max: i64) -> Result<i64, String> {
    if min >= max {
        return Err("min value shouldn't be bigger than or equal to maximum value".to_string());
    }

    Ok(R::rng(cx).generate_random_int_range(min, max))
}

#[native_fn(module = "random")]
pub fn rand_float_range<R: Runtime>(cx: &mut R::Cx, min: f64, max: f64) -> Result<f64, String> {
    if min >= max {
        return Err("min value shouldn't be bigger than or equal to maximum value".to_string());
    }

    Ok(R::rng(cx).generate_random_float_range(min, max))
}

#[native_fn(module = "random")]
pub fn rand_dice<R: Runtime>(cx: &mut R::Cx, sides: i64) -> Result<i64, String> {
    if sides <= 0 {
        return Err("sides should be 1 or higher".to_string());
    }
    Ok(R::rng(cx).generate_random_int_range(1, sides))
}

#[native_fn(module = "random")]
pub fn rand_range<R: Runtime>(cx: &mut R::Cx, stop: i64) -> Result<i64, String> {
    if 0 == stop {
        return Err("rand_range() stop shouldn't be zero".to_string());
    }
    if 0 > stop {
        return Err("rand_range() stop shouldn't be less than zero".to_string());
    }

    Ok(R::rng(cx).generate_random_int_range(0, stop))
}

#[native_fn(module = "random")]
pub fn rand_range_step<R: Runtime>(
    cx: &mut R::Cx,
    start: i64,
    end: i64,
    step: i64,
) -> Result<i64, String> {
    if 0 == step {
        return Err("rand_range_step() stop shouldn't be zero".to_string());
    }
    if start >= end {
        return Err(format!(
            "rand_range_step() end shouldn't be less than or equal to {}",
            start
        ));
    }

    let count = ((end - start) / step) + 1;
    let i = R::rng(cx).generate_random_int_range(0, count - 1);
    Ok(start + i * step)
}

#[native_fn(module = "random")]
pub fn rand_string<R: Runtime>(cx: &mut R::Cx, count: i64) -> Result<String, String> {
    if count <= 0 {
        return Err("count cannot be less than or equal to zero".to_string());
    }

    let result: String = (0..count)
        .map(|_| R::rng(cx).generate_random_int_range(32, 126) as u8 as char)
        .collect();

    Ok(result)
}

// ---- fallible array generators (return raw `R::Value` -> `result[array]`) --

#[native_fn(module = "random", sig(int, int -> result[array[int]]))]
pub fn rand_dices<R: Runtime>(cx: &mut R::Cx, count: i64, sides: i64) -> R::Value {
    if count <= 0 {
        return R::err(R::from_string("count should be 1 or higher".to_string()));
    }
    if sides <= 0 {
        return R::err(R::from_string("sides should be 1 or higher".to_string()));
    }

    let result: Vec<R::Value> = (0..count)
        .map(|_| R::from_i64(R::rng(cx).generate_random_int_range(1, sides)))
        .collect();

    R::ok(R::array(result, rl_ast::statements::TypeAnnotation::Infer))
}

#[native_fn(module = "random", sig(int -> result[array[byte]]))]
pub fn rand_bytes<R: Runtime>(cx: &mut R::Cx, count: i64) -> R::Value {
    if count <= 0 {
        return R::err(R::from_string("count cannot be less than zero".to_string()));
    }

    let result: Vec<R::Value> = (0..count)
        .map(|_| R::from_u8(R::rng(cx).generate_random_int_range(0, 255) as u8))
        .collect();

    R::ok(R::array(result, rl_ast::statements::TypeAnnotation::Infer))
}

// ---- fallible array pickers (generic element type) ------------------------

#[native_fn(module = "random", sig(array[T] -> result[T]))]
pub fn rand_choice<R: Runtime>(_cx: &mut R::Cx, array: R::Value) -> R::Value {
    let Some((items, _)) = R::as_array(&array) else {
        return R::err(R::from_string(format!(
            "rand_choice() expected array found {}",
            R::display(&array)
        )));
    };
    if items.is_empty() {
        return R::err(R::from_string("array is empty".to_string()));
    }
    let index = R::rng(_cx).generate_random_int_range(0, items.len() as i64 - 1) as usize;
    R::ok(items[index].clone())
}

#[native_fn(module = "random", sig(array[T], int -> result[array[T]]))]
pub fn rand_choices<R: Runtime>(_cx: &mut R::Cx, array: R::Value, count: i64) -> R::Value {
    if count <= 0 {
        return R::err(R::from_string("count should be 1 or higher".to_string()));
    }

    let Some((items, _)) = R::as_array(&array) else {
        return R::err(R::from_string(format!(
            "rand_choices() expected array found {}",
            R::display(&array)
        )));
    };
    if items.is_empty() {
        return R::err(R::from_string("array is empty".to_string()));
    }
    let len = items.len();

    let mut result: Vec<R::Value> = Vec::with_capacity(count as usize);
    for _ in 0..count {
        let index = R::rng(_cx).generate_random_int_range(0, len as i64 - 1) as usize;
        result.push(items[index].clone());
    }

    R::ok(R::array(result, rl_ast::statements::TypeAnnotation::Infer))
}

#[native_fn(module = "random", sig(array[T], int -> result[array[T]]))]
pub fn rand_sample<R: Runtime>(_cx: &mut R::Cx, array: R::Value, count: i64) -> R::Value {
    if count <= 0 {
        return R::err(R::from_string("count should be 1 or higher".to_string()));
    }

    let Some((items, _)) = R::as_array(&array) else {
        return R::err(R::from_string(format!(
            "rand_sample() expected array found {}",
            R::display(&array)
        )));
    };
    let len = items.len();
    if count as usize > len {
        return R::err(R::from_string("count larger than array".to_string()));
    }
    let mut indices: Vec<usize> = (0..len).collect();
    for i in (1..len).rev() {
        let j = R::rng(_cx).generate_random_int_range(0, i as i64) as usize;
        indices.swap(i, j);
    }

    let result: Vec<R::Value> = indices[..count as usize]
        .iter()
        .map(|&i| items[i].clone())
        .collect();

    R::ok(R::array(result, rl_ast::statements::TypeAnnotation::Infer))
}

#[native_fn(module = "random", sig(array[T] -> result[array[T]]))]
pub fn rand_shuffle<R: Runtime>(_cx: &mut R::Cx, array: R::Value) -> R::Value {
    let Some((items, _)) = R::as_array(&array) else {
        return R::err(R::from_string(format!(
            "rand_shuffle() expected array found {}",
            R::display(&array)
        )));
    };
    if items.is_empty() {
        return R::err(R::from_string("array is empty".to_string()));
    }

    let mut items: Vec<R::Value> = items.to_vec();
    for i in (1..items.len()).rev() {
        let j = R::rng(_cx).generate_random_int_range(0, i as i64) as usize;
        items.swap(i, j);
    }

    R::ok(R::array(items, rl_ast::statements::TypeAnnotation::Infer))
}

// ---- seeding --------------------------------------------------------------

#[native_fn(module = "random")]
pub fn rand_seed<R: Runtime>(cx: &mut R::Cx, seed: i64) {
    R::rng(cx).reseed(seed as u64);
}

rl_std_core::native_module!("random";
    funcs: [
        rand_int, rand_int_range,
        rand_float, rand_float_range,
        rand_bool, rand_bool_weighted,
        rand_dice, rand_dices,
        rand_range, rand_range_step,
        rand_choice, rand_choices, rand_sample, rand_shuffle,
        rand_byte, rand_bytes,
        rand_char, rand_string,
        rand_seed,
    ],
);
