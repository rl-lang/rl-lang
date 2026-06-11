use crate::{interpreter::evaluator::Evaluator, utils::errors::Error};

pub fn std_parse_float(_: &mut Evaluator, string: String) -> Result<f64, Error> {
    string
        .trim()
        .parse::<f64>()
        .map_err(|_| Error::init(format!("cannot parse \"{}\" as float", string), None, None))
}
