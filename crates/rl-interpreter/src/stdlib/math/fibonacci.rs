use crate::evaluator::Evaluator;

pub fn std_fibonacci(_: &mut Evaluator, x: i64) -> i64 {
    let (mut a, mut b) = (0, 1);
    for _ in 0..x {
        (a, b) = (b, a + b);
    }
    a
}
