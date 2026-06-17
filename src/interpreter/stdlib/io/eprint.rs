use crate::interpreter::evaluator::Evaluator;
use crate::interpreter::values::Value;
use crate::utils::errors::Error;

pub fn std_eprint(_: &mut Evaluator, string: String) -> Result<Value, Error> {
    Err(Error::init(string, None, None))
}
