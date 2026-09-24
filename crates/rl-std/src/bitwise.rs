//! `std::bitwise` - bitwise operations on `byte` and `int` values.
//!
//! Mixed `byte`/`int` operands widen to `int`. `bit_xor` requires matching
//! types. Ported once from the former per-runtime `stdlib/bitwise/*.rs` copies.
//!
//! Each function inspects the raw runtime value (`byte` vs `int`) and returns a
//! language `result[...]`, so the bodies operate on `R::Value` directly via
//! `R::as_u8`/`R::as_i64` and build the wrapped `ok`/`err` themselves.

use rl_std_core::Runtime;
use rl_std_macros::native_fn;

// ---- two-operand widening ops ---------------------------------------------

#[native_fn(
    module = "bitwise",
    sig(byte, byte -> result[byte]),
    sig(int, int -> result[int]),
    sig(byte, int -> result[int]),
    sig(int, byte -> result[int]),
    sig(sbyte, sbyte -> result[sbyte]),
    sig(bsbyte, bsbyte -> result[bsbyte]),
    sig(bbyte, bbyte -> result[bbyte]),
    sig(sint, sint -> result[sint]),
    sig(suint, suint -> result[suint]),
    sig(uint, uint -> result[uint])
)]
pub fn bit_and<R: Runtime>(a: R::Value, b: R::Value) -> R::Value {
    match (R::as_u8(&a), R::as_i64(&a), R::as_u8(&b), R::as_i64(&b)) {
        (Some(x), _, Some(y), _) => R::ok(R::from_u8(x & y)),
        (Some(x), _, _, Some(y)) => R::ok(R::from_i64(x as i64 & y)),
        (_, Some(x), Some(y), _) => R::ok(R::from_i64(x & y as i64)),
        (_, Some(x), _, Some(y)) => R::ok(R::from_i64(x & y)),
        _ => R::err(R::from_string(
            "bit_and expects byte or integer arguments".to_string(),
        )),
    }
}

#[native_fn(
    module = "bitwise",
    sig(byte, byte -> result[byte]),
    sig(int, int -> result[int]),
    sig(byte, int -> result[int]),
    sig(int, byte -> result[int]),
    sig(sbyte, sbyte -> result[sbyte]),
    sig(bsbyte, bsbyte -> result[bsbyte]),
    sig(bbyte, bbyte -> result[bbyte]),
    sig(sint, sint -> result[sint]),
    sig(suint, suint -> result[suint]),
    sig(uint, uint -> result[uint])
)]
pub fn bit_or<R: Runtime>(a: R::Value, b: R::Value) -> R::Value {
    match (R::as_u8(&a), R::as_i64(&a), R::as_u8(&b), R::as_i64(&b)) {
        (Some(x), _, Some(y), _) => R::ok(R::from_u8(x | y)),
        (Some(x), _, _, Some(y)) => R::ok(R::from_i64(x as i64 | y)),
        (_, Some(x), Some(y), _) => R::ok(R::from_i64(x | y as i64)),
        (_, Some(x), _, Some(y)) => R::ok(R::from_i64(x | y)),
        _ => R::err(R::from_string(
            "bit_or expects byte or integer arguments".to_string(),
        )),
    }
}

// Unlike `bit_and`/`bit_or`, `bit_xor` requires matching operand types -
// no mixed `byte`/`int` overload.
#[native_fn(
    module = "bitwise",
    sig(byte, byte -> result[byte]),
    sig(int, int -> result[int]),
    sig(sbyte, sbyte -> result[sbyte]),
    sig(bsbyte, bsbyte -> result[bsbyte]),
    sig(bbyte, bbyte -> result[bbyte]),
    sig(sint, sint -> result[sint]),
    sig(suint, suint -> result[suint]),
    sig(uint, uint -> result[uint])
)]
pub fn bit_xor<R: Runtime>(a: R::Value, b: R::Value) -> R::Value {
    if let (Some(x), Some(y)) = (R::as_u8(&a), R::as_u8(&b)) {
        R::ok(R::from_u8(x ^ y))
    } else if let (Some(x), Some(y)) = (R::as_i64(&a), R::as_i64(&b)) {
        R::ok(R::from_i64(x ^ y))
    } else {
        R::err(R::from_string(
            "bit_xor expects (byte, byte) or (int, int) arguments".to_string(),
        ))
    }
}

// ---- single-operand ops ----------------------------------------------------

#[native_fn(
    module = "bitwise",
    sig(byte -> result[byte]),
    sig(int -> result[int]),
    sig(sbyte -> result[sbyte]),
    sig(bsbyte -> result[bsbyte]),
    sig(bbyte -> result[bbyte]),
    sig(sint -> result[sint]),
    sig(suint -> result[suint]),
    sig(uint -> result[uint])
)]
pub fn bit_not<R: Runtime>(v: R::Value) -> R::Value {
    if let Some(x) = R::as_u8(&v) {
        R::ok(R::from_u8(!x))
    } else if let Some(x) = R::as_i64(&v) {
        R::ok(R::from_i64(!x))
    } else {
        R::err(R::from_string(
            "bit_not expects a byte or an int".to_string(),
        ))
    }
}

// ---- shift ops -------------------------------------------------------------

// The shift amount may independently be `byte` or `int`; the result type
// tracks the shifted value, not the shift amount.
#[native_fn(
    module = "bitwise",
    sig(byte, byte -> result[byte]),
    sig(byte, int -> result[byte]),
    sig(int, byte -> result[int]),
    sig(int, int -> result[int]),
    sig(sbyte, sbyte -> result[sbyte]),
    sig(bsbyte, bsbyte -> result[bsbyte]),
    sig(bbyte, bbyte -> result[bbyte]),
    sig(sint, sint -> result[sint]),
    sig(suint, suint -> result[suint]),
    sig(uint, uint -> result[uint])
)]
pub fn bit_shift_left<R: Runtime>(a: R::Value, shift: R::Value) -> R::Value {
    let s = match (R::as_u8(&shift), R::as_i64(&shift)) {
        (Some(s), _) => s as u32,
        (_, Some(s)) => s as u32,
        _ => {
            return R::err(R::from_string(
                "bit_shift_left expects ((byte|int), (int|byte))".to_string(),
            ));
        }
    };
    if let Some(x) = R::as_u8(&a) {
        R::ok(R::from_u8(x << s))
    } else if let Some(x) = R::as_i64(&a) {
        R::ok(R::from_i64(x << s))
    } else {
        R::err(R::from_string(
            "bit_shift_left expects ((byte|int), (int|byte))".to_string(),
        ))
    }
}

// The shift amount may independently be `byte` or `int`; the result type
// tracks the shifted value, not the shift amount.
#[native_fn(
    module = "bitwise",
    sig(byte, byte -> result[byte]),
    sig(byte, int -> result[byte]),
    sig(int, byte -> result[int]),
    sig(int, int -> result[int]),
    sig(sbyte, sbyte -> result[sbyte]),
    sig(bsbyte, bsbyte -> result[bsbyte]),
    sig(bbyte, bbyte -> result[bbyte]),
    sig(sint, sint -> result[sint]),
    sig(suint, suint -> result[suint]),
    sig(uint, uint -> result[uint])
)]
pub fn bit_shift_right<R: Runtime>(a: R::Value, shift: R::Value) -> R::Value {
    let s = match (R::as_u8(&shift), R::as_i64(&shift)) {
        (Some(s), _) => s as u32,
        (_, Some(s)) => s as u32,
        _ => {
            return R::err(R::from_string(
                "bit_shift_right expects ((byte|int), (int|byte))".to_string(),
            ));
        }
    };
    if let Some(x) = R::as_u8(&a) {
        R::ok(R::from_u8(x >> s))
    } else if let Some(x) = R::as_i64(&a) {
        R::ok(R::from_i64(x >> s))
    } else {
        R::err(R::from_string(
            "bit_shift_right expects ((byte|int), (int|byte))".to_string(),
        ))
    }
}

// ---- bit-counting ops ------------------------------------------------------

#[native_fn(
    module = "bitwise",
    sig(byte -> result[byte]),
    sig(int -> result[int]),
    sig(sbyte -> result[sbyte]),
    sig(bsbyte -> result[bsbyte]),
    sig(bbyte -> result[bbyte]),
    sig(sint -> result[sint]),
    sig(suint -> result[suint]),
    sig(uint -> result[uint])
)]
pub fn count_bits<R: Runtime>(v: R::Value) -> R::Value {
    if let Some(x) = R::as_u8(&v) {
        R::ok(R::from_u8(x.count_ones() as u8))
    } else if let Some(x) = R::as_i64(&v) {
        R::ok(R::from_i64(x.count_ones() as i64))
    } else {
        R::err(R::from_string(
            "count_bits expects a byte or an int".to_string(),
        ))
    }
}

#[native_fn(
    module = "bitwise",
    sig(byte -> result[byte]),
    sig(int -> result[int]),
    sig(sbyte -> result[sbyte]),
    sig(bsbyte -> result[bsbyte]),
    sig(bbyte -> result[bbyte]),
    sig(sint -> result[sint]),
    sig(suint -> result[suint]),
    sig(uint -> result[uint])
)]
pub fn leading_zeros<R: Runtime>(v: R::Value) -> R::Value {
    if let Some(x) = R::as_u8(&v) {
        R::ok(R::from_u8(u8::leading_zeros(x) as u8))
    } else if let Some(x) = R::as_i64(&v) {
        R::ok(R::from_i64(i64::leading_zeros(x) as i64))
    } else {
        R::err(R::from_string(
            "leading_zeros expects a byte or an int".to_string(),
        ))
    }
}

#[native_fn(
    module = "bitwise",
    sig(byte -> result[byte]),
    sig(int -> result[int]),
    sig(sbyte -> result[sbyte]),
    sig(bsbyte -> result[bsbyte]),
    sig(bbyte -> result[bbyte]),
    sig(sint -> result[sint]),
    sig(suint -> result[suint]),
    sig(uint -> result[uint])
)]
pub fn trailing_zeros<R: Runtime>(v: R::Value) -> R::Value {
    if let Some(x) = R::as_u8(&v) {
        R::ok(R::from_u8(u8::trailing_zeros(x) as u8))
    } else if let Some(x) = R::as_i64(&v) {
        R::ok(R::from_i64(i64::trailing_zeros(x) as i64))
    } else {
        R::err(R::from_string(
            "trailing_zeros expects a byte or an int".to_string(),
        ))
    }
}

// ---- rotate ops -----------------------------------------------------------

#[native_fn(
    module = "bitwise",
    sig(byte, byte -> result[byte]),
    sig(byte, int -> result[byte]),
    sig(int, byte -> result[int]),
    sig(int, int -> result[int]),
    sig(sbyte, sbyte -> result[sbyte]),
    sig(bsbyte, bsbyte -> result[bsbyte]),
    sig(bbyte, bbyte -> result[bbyte]),
    sig(sint, sint -> result[sint]),
    sig(suint, suint -> result[suint]),
    sig(uint, uint -> result[uint])
)]
pub fn rotate_left<R: Runtime>(a: R::Value, shift: R::Value) -> R::Value {
    let s = match (R::as_u8(&shift), R::as_i64(&shift)) {
        (Some(s), _) => s as u32,
        (_, Some(s)) => s as u32,
        _ => {
            return R::err(R::from_string(
                "rotate_left expects ((byte|int), (int|byte))".to_string(),
            ));
        }
    };
    if let Some(x) = R::as_u8(&a) {
        let n = s % 8;
        R::ok(R::from_u8(x.rotate_left(n)))
    } else if let Some(x) = R::as_i64(&a) {
        let n = s % 64;
        R::ok(R::from_i64(x.rotate_left(n)))
    } else {
        R::err(R::from_string(
            "rotate_left expects ((byte|int), (int|byte))".to_string(),
        ))
    }
}

#[native_fn(
    module = "bitwise",
    sig(byte, byte -> result[byte]),
    sig(byte, int -> result[byte]),
    sig(int, byte -> result[int]),
    sig(int, int -> result[int]),
    sig(sbyte, sbyte -> result[sbyte]),
    sig(bsbyte, bsbyte -> result[bsbyte]),
    sig(bbyte, bbyte -> result[bbyte]),
    sig(sint, sint -> result[sint]),
    sig(suint, suint -> result[suint]),
    sig(uint, uint -> result[uint])
)]
pub fn rotate_right<R: Runtime>(a: R::Value, shift: R::Value) -> R::Value {
    let s = match (R::as_u8(&shift), R::as_i64(&shift)) {
        (Some(s), _) => s as u32,
        (_, Some(s)) => s as u32,
        _ => {
            return R::err(R::from_string(
                "rotate_right expects ((byte|int), (int|byte))".to_string(),
            ));
        }
    };
    if let Some(x) = R::as_u8(&a) {
        let n = s % 8;
        R::ok(R::from_u8(x.rotate_right(n)))
    } else if let Some(x) = R::as_i64(&a) {
        let n = s % 64;
        R::ok(R::from_i64(x.rotate_right(n)))
    } else {
        R::err(R::from_string(
            "rotate_right expects ((byte|int), (int|byte))".to_string(),
        ))
    }
}

// ---- bit query / set / clear / toggle -------------------------------------

#[native_fn(
    module = "bitwise",
    sig(byte, byte -> result[byte]),
    sig(byte, int -> result[byte]),
    sig(int, byte -> result[int]),
    sig(int, int -> result[int]),
    sig(sbyte, sbyte -> result[sbyte]),
    sig(bsbyte, bsbyte -> result[bsbyte]),
    sig(bbyte, bbyte -> result[bbyte]),
    sig(sint, sint -> result[sint]),
    sig(suint, suint -> result[suint]),
    sig(uint, uint -> result[uint])
)]
pub fn bit_set<R: Runtime>(a: R::Value, n: R::Value) -> R::Value {
    let pos = match (R::as_u8(&n), R::as_i64(&n)) {
        (Some(s), _) => s as u32,
        (_, Some(s)) => s as u32,
        _ => {
            return R::err(R::from_string(
                "bit_set expects ((byte|int), (int|byte))".to_string(),
            ));
        }
    };
    if let Some(x) = R::as_u8(&a) {
        R::ok(R::from_u8(x | (1 << pos)))
    } else if let Some(x) = R::as_i64(&a) {
        R::ok(R::from_i64(x | (1i64 << pos)))
    } else {
        R::err(R::from_string(
            "bit_set expects ((byte|int), (int|byte))".to_string(),
        ))
    }
}

#[native_fn(
    module = "bitwise",
    sig(byte, byte -> result[byte]),
    sig(byte, int -> result[byte]),
    sig(int, byte -> result[int]),
    sig(int, int -> result[int]),
    sig(sbyte, sbyte -> result[sbyte]),
    sig(bsbyte, bsbyte -> result[bsbyte]),
    sig(bbyte, bbyte -> result[bbyte]),
    sig(sint, sint -> result[sint]),
    sig(suint, suint -> result[suint]),
    sig(uint, uint -> result[uint])
)]
pub fn bit_clear<R: Runtime>(a: R::Value, n: R::Value) -> R::Value {
    let pos = match (R::as_u8(&n), R::as_i64(&n)) {
        (Some(s), _) => s as u32,
        (_, Some(s)) => s as u32,
        _ => {
            return R::err(R::from_string(
                "bit_clear expects ((byte|int), (int|byte))".to_string(),
            ));
        }
    };
    if let Some(x) = R::as_u8(&a) {
        R::ok(R::from_u8(x & !(1 << pos)))
    } else if let Some(x) = R::as_i64(&a) {
        R::ok(R::from_i64(x & !(1i64 << pos)))
    } else {
        R::err(R::from_string(
            "bit_clear expects ((byte|int), (int|byte))".to_string(),
        ))
    }
}

#[native_fn(
    module = "bitwise",
    sig(byte, byte -> result[byte]),
    sig(byte, int -> result[byte]),
    sig(int, byte -> result[int]),
    sig(int, int -> result[int]),
    sig(sbyte, sbyte -> result[sbyte]),
    sig(bsbyte, bsbyte -> result[bsbyte]),
    sig(bbyte, bbyte -> result[bbyte]),
    sig(sint, sint -> result[sint]),
    sig(suint, suint -> result[suint]),
    sig(uint, uint -> result[uint])
)]
pub fn bit_toggle<R: Runtime>(a: R::Value, n: R::Value) -> R::Value {
    let pos = match (R::as_u8(&n), R::as_i64(&n)) {
        (Some(s), _) => s as u32,
        (_, Some(s)) => s as u32,
        _ => {
            return R::err(R::from_string(
                "bit_toggle expects ((byte|int), (int|byte))".to_string(),
            ));
        }
    };
    if let Some(x) = R::as_u8(&a) {
        R::ok(R::from_u8(x ^ (1 << pos)))
    } else if let Some(x) = R::as_i64(&a) {
        R::ok(R::from_i64(x ^ (1i64 << pos)))
    } else {
        R::err(R::from_string(
            "bit_toggle expects ((byte|int), (int|byte))".to_string(),
        ))
    }
}

#[native_fn(
    module = "bitwise",
    sig(byte, byte -> result[bool]),
    sig(byte, int -> result[bool]),
    sig(int, byte -> result[bool]),
    sig(int, int -> result[bool]),
    sig(sbyte, sbyte -> result[bool]),
    sig(bsbyte, bsbyte -> result[bool]),
    sig(bbyte, bbyte -> result[bool]),
    sig(sint, sint -> result[bool]),
    sig(suint, suint -> result[bool]),
    sig(uint, uint -> result[bool])
)]
pub fn bit_is_set<R: Runtime>(a: R::Value, n: R::Value) -> R::Value {
    let pos = match (R::as_u8(&n), R::as_i64(&n)) {
        (Some(s), _) => s as u32,
        (_, Some(s)) => s as u32,
        _ => {
            return R::err(R::from_string(
                "bit_is_set expects ((byte|int), (int|byte))".to_string(),
            ));
        }
    };
    if let Some(x) = R::as_u8(&a) {
        R::ok(R::from_bool((x & (1 << pos)) != 0))
    } else if let Some(x) = R::as_i64(&a) {
        R::ok(R::from_bool((x & (1i64 << pos)) != 0))
    } else {
        R::err(R::from_string(
            "bit_is_set expects ((byte|int), (int|byte))".to_string(),
        ))
    }
}

rl_std_core::native_module!("bitwise";
    funcs: [
        bit_and, bit_or, bit_xor, bit_not,
        bit_shift_left, bit_shift_right,
        count_bits, leading_zeros, trailing_zeros,
        rotate_left, rotate_right,
        bit_set, bit_clear, bit_toggle, bit_is_set,
    ],
);
