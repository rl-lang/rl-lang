#![allow(unused_macros, unused_imports)]

macro_rules! vok {
    ($e:expr) => {
        crate::values::VmValue::Ok(Box::new($e))
    };
}
pub(crate) use vok;

macro_rules! verr {
    ($e:expr) => {
        crate::values::VmValue::Err(Box::new($e))
    };
}
pub(crate) use verr;

macro_rules! vb {
    ($e:expr) => {
        crate::values::VmValue::Bool($e)
    };
}
pub(crate) use vb;

macro_rules! vi {
    ($e:expr) => {
        crate::values::VmValue::Int($e)
    };
}
pub(crate) use vi;

macro_rules! vf {
    ($e:expr) => {
        crate::values::VmValue::Float($e)
    };
}
pub(crate) use vf;

macro_rules! vs {
    ($e:expr) => {
        crate::values::VmValue::Str(std::rc::Rc::from($e.as_str()))
    };
}
pub(crate) use vs;

macro_rules! vc {
    ($e:expr) => {
        crate::values::VmValue::Char($e)
    };
}
pub(crate) use vc;

macro_rules! vby {
    ($e:expr) => {
        crate::values::VmValue::Byte($e)
    };
}
pub(crate) use vby;

macro_rules! vnl {
    () => {
        crate::values::VmValue::Null
    };
}
pub(crate) use vnl;

macro_rules! try_fn {
    ($s:expr, $e:expr) => {
        if let Err(e) = $e {
            return verr!(vs!(format!("{}: {}", $s, e)));
        }
    };
}
pub(crate) use try_fn;
