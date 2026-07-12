use crate::interpreter::{
    evaluator::Evaluator, stdlib::http::common::ureq_result_to_value, values::Value,
};

pub fn func(_: &mut Evaluator, url: String) -> Value {
    let result = ureq::get(&url).call();
    ureq_result_to_value(&url, result)
}
