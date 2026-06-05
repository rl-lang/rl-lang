pub mod cos;
pub mod modulo;
pub mod power;
pub mod sin;
pub mod tan;

use crate::interpreter::native::Module;

pub fn module() -> Module {
    Module::new("math")
        .with_function("sin", sin::std_sin)
        .with_function("cos", cos::std_cos)
        .with_function("tan", tan::std_tan)
        .with_function("pow", power::std_pow)
        .with_function("mod", modulo::std_mod)
}
