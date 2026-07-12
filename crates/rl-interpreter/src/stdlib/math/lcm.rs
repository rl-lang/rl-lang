use crate::interpreter::evaluator::Evaluator;

pub fn std_lcm(_: &mut Evaluator, x: i64, y: i64) -> i64 {
    let mut a = x as u64;
    let mut b = y as u64;
    let a_ = x as u64;
    let b_ = y as u64;
    while b != 0 {
        (a, b) = (b, a % b);
    }
    (a_ / a * b_) as i64
}
