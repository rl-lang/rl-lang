use std::collections::HashMap;
use std::sync::Arc;

use crate::ast::statements::TypeAnnotation;
use crate::interpreter::evaluator::Evaluator;
use crate::interpreter::values::Value;
use crate::utils::errors::{Error, ErrorReason, Reason};

pub type NativeFn = Arc<dyn Fn(&mut Evaluator, Vec<Value>) -> Result<Value, Error> + Send + Sync>;

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
        F: Fn(&mut Evaluator, Vec<Value>) -> Result<Value, Error> + Send + Sync + 'static,
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

pub trait ValueType {
    fn type_annotation() -> TypeAnnotation;
}

impl ValueType for i64 {
    fn type_annotation() -> TypeAnnotation {
        TypeAnnotation::Int
    }
}

impl ValueType for f64 {
    fn type_annotation() -> TypeAnnotation {
        TypeAnnotation::Float
    }
}

impl ValueType for String {
    fn type_annotation() -> TypeAnnotation {
        TypeAnnotation::String
    }
}

impl ValueType for bool {
    fn type_annotation() -> TypeAnnotation {
        TypeAnnotation::Bool
    }
}

impl ValueType for char {
    fn type_annotation() -> TypeAnnotation {
        TypeAnnotation::Char
    }
}

impl<T: ValueType> ValueType for Vec<T> {
    fn type_annotation() -> TypeAnnotation {
        TypeAnnotation::Array(Box::new(T::type_annotation()))
    }
}

pub trait FromValue: Sized {
    fn from_value(v: Value) -> Result<Self, Error>;
}

impl FromValue for Value {
    fn from_value(v: Value) -> Result<Self, Error> {
        Ok(v)
    }
}

impl FromValue for i64 {
    fn from_value(v: Value) -> Result<Self, Error> {
        match v {
            Value::Integer(i) => Ok(i),
            other => Err(Error::init(
                format!("expected integer, got {}", other.type_name()),
                None,
                Some(ErrorReason::init(Reason::Runtime, None)),
            )),
        }
    }
}

impl FromValue for f64 {
    fn from_value(v: Value) -> Result<Self, Error> {
        match v {
            Value::Float(f) => Ok(f),
            other => Err(Error::init(
                format!("expected float, got {}", other.type_name()),
                None,
                Some(ErrorReason::init(Reason::Runtime, None)),
            )),
        }
    }
}

impl FromValue for String {
    fn from_value(v: Value) -> Result<Self, Error> {
        match v {
            Value::String(s) => Ok(s),
            other => Err(Error::init(
                format!("expected string, got {}", other.type_name()),
                None,
                Some(ErrorReason::init(Reason::Runtime, None)),
            )),
        }
    }
}

impl FromValue for bool {
    fn from_value(v: Value) -> Result<Self, Error> {
        match v {
            Value::Bool(b) => Ok(b),
            other => Err(Error::init(
                format!("expected bool, got {}", other.type_name()),
                None,
                Some(ErrorReason::init(Reason::Runtime, None)),
            )),
        }
    }
}

impl FromValue for char {
    fn from_value(v: Value) -> Result<Self, Error> {
        match v {
            Value::Char(c) => Ok(c),
            other => Err(Error::init(
                format!("expected char, got {}", other.type_name()),
                None,
                Some(ErrorReason::init(Reason::Runtime, None)),
            )),
        }
    }
}

impl<T: FromValue> FromValue for Vec<T> {
    fn from_value(v: Value) -> Result<Self, Error> {
        match v {
            Value::Values { items, .. } => items.into_iter().map(T::from_value).collect(),
            other => Err(Error::init(
                format!("expected array, got {}", other.type_name()),
                None,
                Some(ErrorReason::init(Reason::Runtime, None)),
            )),
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

impl<T: IntoValue + ValueType> IntoValue for Vec<T> {
    fn into_value(self) -> Value {
        Value::Values {
            items_type: T::type_annotation(),
            items: self.into_iter().map(T::into_value).collect(),
        }
    }
}

pub trait IntoNativeFn<Args> {
    fn into_native(self) -> NativeFn;
}

impl<F, R> IntoNativeFn<()> for F
where
    F: Fn(&mut Evaluator) -> R + Send + Sync + 'static,
    R: IntoValue,
{
    fn into_native(self) -> NativeFn {
        Arc::new(
            move |rt: &mut Evaluator, args: Vec<Value>| -> Result<Value, Error> {
                if !args.is_empty() {
                    return Err(Error::init(
                        format!("expected 0 argument(s), got {}", args.len()),
                        None,
                        Some(ErrorReason::init(Reason::Runtime, None)),
                    ));
                }
                Ok(self(rt).into_value())
            },
        )
    }
}

pub struct Fallible<T>(std::marker::PhantomData<T>);

macro_rules! impl_into_native_fn {
    ($count:literal, $(($ty:ident, $var:ident)),+) => {
        impl<F, R, $($ty),+> IntoNativeFn<($($ty,)+)> for F
        where
            F: Fn(&mut Evaluator, $($ty),+) -> R + Send + Sync + 'static,
            R: IntoValue,
            $($ty: FromValue),+
        {
            fn into_native(self) -> NativeFn {
                Arc::new(move |rt: &mut Evaluator, args: Vec<Value>| -> Result<Value, Error> {
                    if args.len() != $count {
                        return Err(Error::init(
                            format!("expected {} argument(s), got {}", $count, args.len()),
                            None, Some(ErrorReason::init(Reason::Runtime, None)),
                        ));
                    }
                    let mut iter = args.into_iter();
                    $(let $var = <$ty>::from_value(iter.next().unwrap())?;)+
                    Ok(self(rt, $($var),+).into_value())
                })
            }
        }

        impl<F, R, $($ty),+> IntoNativeFn<(Fallible<($($ty,)+)>,)> for F
        where
            F: Fn(&mut Evaluator, $($ty),+) -> Result<R, Error> + Send + Sync + 'static,
            R: IntoValue,
            $($ty: FromValue),+
        {
            fn into_native(self) -> NativeFn {
                Arc::new(move |rt: &mut Evaluator, args: Vec<Value>| -> Result<Value, Error> {
                    if args.len() != $count {
                        return Err(Error::init(
                            format!("expected {} argument(s), got {}", $count, args.len()),
                            None, Some(ErrorReason::init(Reason::Runtime, None)),
                        ));
                    }
                    let mut iter = args.into_iter();
                    $(let $var = <$ty>::from_value(iter.next().unwrap())?;)+
                    Ok(self(rt, $($var),+)?.into_value())
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
impl_into_native_fn!(
    6,
    (A1, a1),
    (A2, a2),
    (A3, a3),
    (A4, a4),
    (A5, a5),
    (A6, a6)
);
impl_into_native_fn!(
    7,
    (A1, a1),
    (A2, a2),
    (A3, a3),
    (A4, a4),
    (A5, a5),
    (A6, a6),
    (A7, a7)
);
impl_into_native_fn!(
    8,
    (A1, a1),
    (A2, a2),
    (A3, a3),
    (A4, a4),
    (A5, a5),
    (A6, a6),
    (A7, a7),
    (A8, a8)
);
impl_into_native_fn!(
    9,
    (A1, a1),
    (A2, a2),
    (A3, a3),
    (A4, a4),
    (A5, a5),
    (A6, a6),
    (A7, a7),
    (A8, a8),
    (A9, a9)
);
impl_into_native_fn!(
    10,
    (A1, a1),
    (A2, a2),
    (A3, a3),
    (A4, a4),
    (A5, a5),
    (A6, a6),
    (A7, a7),
    (A8, a8),
    (A9, a9),
    (A10, a10)
);
