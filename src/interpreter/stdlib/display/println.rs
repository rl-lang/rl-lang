use crate::interpreter::evaluator::Evaluator;
use crate::interpreter::values::Value;

pub fn std_println(_: &mut Evaluator, args: Vec<Value>) -> Value {
    for (i, arg) in args.iter().enumerate() {
        if i > 0 {
            print!(" ");
        }
        print!("{}", arg);
    }
    println!();
    Value::Null
}
