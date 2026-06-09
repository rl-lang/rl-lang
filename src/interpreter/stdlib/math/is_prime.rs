use crate::interpreter::evaluator::Evaluator;

pub fn std_is_prime(_: &mut Evaluator, x: i64) -> bool {
    let x = x as u64;
    if x < 2 {
        return false;
    }
    if x < 4 {
        return true;
    }
    if x.is_multiple_of(2) || x.is_multiple_of(3) {
        return false;
    }
    let mut i = 5;
    while i * i <= x {
        if x.is_multiple_of(i) || x.is_multiple_of(i + 2) {
            return false;
        }
        i += 6;
    }
    true
}
