use std::collections::HashMap;
use std::sync::Arc;

use crate::interpreter::evaluator::Evaluator;
use crate::interpreter::values::Value;
use crate::utils::errors::Error;

pub type NativeFn = Arc<dyn Fn(&mut Evaluator, Vec<Value>) -> Value + Send + Sync>;

pub struct Module {
    pub name: String,
    pub functions: HashMap<String, NativeFn>,
    pub submodules: HashMap<String, Module>,
}

impl Module {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            functions: HashMap::new(),
            submodules: HashMap::new(),
        }
    }

    pub fn with_function<F, A>(mut self, name: impl Into<String>, f: F) -> Self
    where
        F: IntoNativeFn<A>,
    {
        self.functions.insert(name.into(), f.into_native());
        self
    }

    pub fn with_raw_function<F>(mut self, name: impl Into<String>, f: F) -> Self
    where
        F: Fn(&mut Evaluator, Vec<Value>) -> Value + Send + Sync + 'static,
    {
        self.functions.insert(name.into(), Arc::new(f));
        self
    }

    pub fn with_module(mut self, m: Module) -> Self {
        self.submodules.insert(m.name.clone(), m);
        self
    }

    pub fn resolve(&self, path: &[String]) -> Option<&NativeFn> {
        if path.is_empty() {
            return None;
        }
        let mut module = self;
        for seg in &path[..path.len() - 1] {
            module = module.submodules.get(seg)?;
        }
        module.functions.get(&path[path.len() - 1])
    }
}

pub trait FromValue: Sized {
    fn from_value(v: Value) -> Self;
}

fn type_error(expected: &str, got: &Value) -> ! {
    Error::init(
        format!("expected {}, got {:?}", expected, got),
        None,
        None,
    )
    .print_error();
    unreachable!()
}

impl FromValue for Value {
    fn from_value(v: Value) -> Self {
        v
    }
}

impl FromValue for i64 {
    fn from_value(v: Value) -> Self {
        match v {
            Value::Integer(i) => i,
            other => type_error("integer", &other),
        }
    }
}

impl FromValue for f64 {
    fn from_value(v: Value) -> Self {
        match v {
            Value::Float(f) => f,
            Value::Integer(i) => i as f64,
            other => type_error("float", &other),
        }
    }
}

impl FromValue for String {
    fn from_value(v: Value) -> Self {
        match v {
            Value::String(s) => s,
            other => type_error("string", &other),
        }
    }
}

impl FromValue for bool {
    fn from_value(v: Value) -> Self {
        match v {
            Value::Bool(b) => b,
            other => type_error("bool", &other),
        }
    }
}

impl FromValue for char {
    fn from_value(v: Value) -> Self {
        match v {
            Value::Char(c) => c,
            other => type_error("char", &other),
        }
    }
}

impl<T: FromValue> FromValue for Vec<T> {
    fn from_value(v: Value) -> Self {
        match v {
            Value::Values(items) => items.into_iter().map(T::from_value).collect(),
            other => type_error("array", &other),
        }
    }
}

pub trait IntoValue {
    fn into_value(self) -> Value;
}

impl IntoValue for Value {
    fn into_value(self) -> Value {
        self
    }
}

impl IntoValue for () {
    fn into_value(self) -> Value {
        Value::Null
    }
}

impl IntoValue for i64 {
    fn into_value(self) -> Value {
        Value::Integer(self)
    }
}

impl IntoValue for f64 {
    fn into_value(self) -> Value {
        Value::Float(self)
    }
}

impl IntoValue for String {
    fn into_value(self) -> Value {
        Value::String(self)
    }
}

impl IntoValue for bool {
    fn into_value(self) -> Value {
        Value::Bool(self)
    }
}

impl IntoValue for char {
    fn into_value(self) -> Value {
        Value::Char(self)
    }
}

impl<T: IntoValue> IntoValue for Vec<T> {
    fn into_value(self) -> Value {
        Value::Values(self.into_iter().map(T::into_value).collect())
    }
}

pub trait IntoNativeFn<Args> {
    fn into_native(self) -> NativeFn;
}

fn arity_error(expected: usize, got: usize) -> ! {
    Error::init(
        format!("expected {} argument(s), got {}", expected, got),
        None,
        None,
    )
    .print_error();
    unreachable!()
}

impl<F, R> IntoNativeFn<()> for F
where
    F: Fn(&mut Evaluator) -> R + Send + Sync + 'static,
    R: IntoValue,
{
    fn into_native(self) -> NativeFn {
        Arc::new(move |rt: &mut Evaluator, args: Vec<Value>| -> Value {
            if !args.is_empty() {
                arity_error(0, args.len());
            }
            self(rt).into_value()
        })
    }
}

macro_rules! impl_into_native_fn {
    ($count:literal, $(($ty:ident, $var:ident)),+) => {
        impl<F, R, $($ty),+> IntoNativeFn<($($ty,)+)> for F
        where
            F: Fn(&mut Evaluator, $($ty),+) -> R + Send + Sync + 'static,
            R: IntoValue,
            $($ty: FromValue),+
        {
            fn into_native(self) -> NativeFn {
                Arc::new(move |rt: &mut Evaluator, args: Vec<Value>| -> Value {
                    if args.len() != $count {
                        arity_error($count, args.len());
                    }
                    let mut iter = args.into_iter();
                    $(
                        let $var = <$ty>::from_value(iter.next().unwrap());
                    )+
                    self(rt, $($var),+).into_value()
                })
            }
        }
    };
}

impl_into_native_fn!(1, (A1, a1));
impl_into_native_fn!(2, (A1, a1), (A2, a2));
impl_into_native_fn!(3, (A1, a1), (A2, a2), (A3, a3));
impl_into_native_fn!(4, (A1, a1), (A2, a2), (A3, a3), (A4, a4));
impl_into_native_fn!(5, (A1, a1), (A2, a2), (A3, a3), (A4, a4), (A5, a5));
impl_into_native_fn!(6, (A1, a1), (A2, a2), (A3, a3), (A4, a4), (A5, a5), (A6, a6));
impl_into_native_fn!(7, (A1, a1), (A2, a2), (A3, a3), (A4, a4), (A5, a5), (A6, a6), (A7, a7));
impl_into_native_fn!(8, (A1, a1), (A2, a2), (A3, a3), (A4, a4), (A5, a5), (A6, a6), (A7, a7), (A8, a8));
impl_into_native_fn!(9, (A1, a1), (A2, a2), (A3, a3), (A4, a4), (A5, a5), (A6, a6), (A7, a7), (A8, a8), (A9, a9));
impl_into_native_fn!(10, (A1, a1), (A2, a2), (A3, a3), (A4, a4), (A5, a5), (A6, a6), (A7, a7), (A8, a8), (A9, a9), (A10, a10));
