use crate::{interpreter::evaluator::Evaluator, utils::errors::Error};

pub fn std_parse_int(_: &mut Evaluator, string: String) -> Result<i64, Error> {
    string
        .trim()
        .parse::<i64>()
        .map_err(|_| Error::init(format!("cannot parse \"{}\" as int", string), None, None))
}
