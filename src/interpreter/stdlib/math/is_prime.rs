use crate::interpreter::evaluator::Evaluator;

pub fn std_is_prime(_: &mut Evaluator, x: i64) -> bool {
    let x = x as u64;
    if x < 2 {
        return false;
    }
    if x < 4 {
        return true;
    }
    if x % 2 == 0 || x % 3 == 0 {
        return false;
    }
    let mut i = 5;
    while i * i <= x {
        if x % i == 0 || x % (i + 2) == 0 {
            return false;
        }
        i += 6;
    }
    true
}
