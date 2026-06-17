use crate::interpreter::evaluator::Evaluator;
use crate::interpreter::values::Value;
use crate::utils::errors::{Error, ErrorReason, Reason};

pub fn std_format(_: &mut Evaluator, args: Vec<Value>) -> Result<Value, Error> {
    // checks for incorrect usage
    if args.is_empty() {
        return Err(Error::init(
            "expected arguments".to_string(),
            None,
            Some(ErrorReason::init(Reason::Runtime, None)),
        ));
    }

    // making mutable empty string for the transformation
    let mut result = String::new();
    // transforming the values to string and collecting to Vec<String>
    let args: Vec<String> = args.iter().map(|s| s.to_string()).collect();
    // assigning the first value as the to be transformed value
    let text = &args[0];
    // assiging the rest of Vec<String> as replacement items
    let mut rest_args = args[1..].iter();
    // making the to be transformed value as characters to detect '{' and '}'
    // this is better for escape codes
    let mut chars = text.chars().peekable();
    // check for unused args
    let mut used = 0;
    // check for missing args
    let mut missing = 0;

    // loop through the characters of to be transforming value
    while let Some(c) = chars.next() {
        // detect for "{}" while "{\}" or similiar doesnt work
        // better than split on "{}"
        if c == '{' && chars.peek() == Some(&'}') {
            // consume the next '}'
            chars.next();
            // replace current "{}" with value from replacements
            match rest_args.next() {
                Some(v) => {
                    // add the replacement value
                    result.push_str(v);
                    used += 1;
                }
                None => {
                    // push "{}" to continue checking for how many is missing
                    result.push_str("{}");
                    missing += 1;
                }
            }
        } else {
            // push the character if not '{' that has '}' after
            result.push(c);
        }
    }

    // check for missing value that wasnt replaced
    if missing > 0 {
        return Err(Error::init(
            format!(
                "format() has {} placeholder(s) with no matching argument",
                missing
            ),
            None,
            Some(ErrorReason::init(Reason::Runtime, None)),
        ));
    }

    // check for additional values that is not used
    if used < args.len() - 1 {
        return Err(Error::init(
            format!(
                "format() received {} argument(s) but only {} placeholder(s) were used",
                args.len() - 1,
                used
            ),
            None,
            Some(ErrorReason::init(Reason::Runtime, None)),
        ));
    }

    // returns the final transformed value
    Ok(Value::String(result))
}
