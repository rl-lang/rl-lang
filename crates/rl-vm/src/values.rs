use std::cell::RefCell;
use std::collections::{HashMap, HashSet};
use std::fmt;
use std::rc::Rc;

use rl_ast::statements::HandleKind;

use crate::Chunk;
use crate::native::NativeFn;
use crate::runtime::VmRuntime;
use rl_std_core::NativeHandle;

#[derive(Debug, Clone, PartialEq)]
pub enum VmValue {
    Null,
    Int(i64),
    UInt(u64),
    SInt(i32),
    SUInt(u32),
    BByte(u16),
    BSByte(i16),
    Byte(u8),
    SByte(i8),
    Float(f64),
    SFloat(f32),
    Bool(bool),
    Char(char),
    Str(Rc<str>),
    /// user defined function call
    Function(Rc<VmFunction>),
    /// std function call
    Native(VmNative),
    Ok(Box<VmValue>),
    Err(Box<VmValue>),
    Error(Box<VmValue>),
    Arr(Rc<Vec<VmValue>>),
    Tuple(Rc<Vec<VmValue>>),
    Set(Rc<RefCell<HashSet<VmMapKey>>>),
    Map(Rc<RefCell<HashMap<VmMapKey, VmValue>>>),
    Record {
        name: Rc<str>,
        fields: RecordFields,
    },
    Tag {
        name: Rc<str>,
        variant: Rc<str>,
    },
    Closure {
        func: Rc<VmFunction>,
        captured: Rc<Vec<VmValue>>,
        capture_start: u16,
    },
    /// An opaque resource id scoped to one stdlib module (`c`, `audio`, ...).
    Handle {
        kind: HandleKind,
        id: u64,
    },
}

// for some reason this works???
// need to look it up
pub type RecordField = Rc<RefCell<Vec<(Rc<str>, VmValue)>>>;

#[derive(Debug, Clone, PartialEq, Default)]
pub struct RecordFields(RecordField);

impl RecordFields {
    pub fn new(fields: Vec<(Rc<str>, VmValue)>) -> Self {
        Self(Rc::new(RefCell::new(fields)))
    }

    pub fn get(&self, name: &str) -> Option<VmValue> {
        self.0
            .borrow()
            .iter()
            .find(|(n, _)| &**n == name)
            .map(|(_, v)| v.clone())
    }

    pub fn set(&self, name: &str, value: VmValue) {
        let mut fields = self.0.borrow_mut();
        if let Some(entry) = fields.iter_mut().find(|(n, _)| &**n == name) {
            entry.1 = value;
        } else {
            fields.push((Rc::from(name), value));
        }
    }

    pub fn iter(&self) -> impl Iterator<Item = (Rc<str>, VmValue)> + '_ {
        self.0.borrow().clone().into_iter()
    }

    pub fn has(&self, name: &str) -> bool {
        self.0.borrow().iter().any(|(n, _)| &**n == name)
    }

    pub fn len(&self) -> usize {
        self.0.borrow().len()
    }

    pub fn is_empty(&self) -> bool {
        self.0.borrow().is_empty()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum VmMapKey {
    Int(i64),
    UInt(u64),
    SInt(i32),
    SUInt(u32),
    BByte(u16),
    BSByte(i16),
    Byte(u8),
    SByte(i8),
    Str(Rc<str>),
    Bool(bool),
    Char(char),
}

impl VmMapKey {
    pub fn from_value(v: &VmValue) -> Option<VmMapKey> {
        match v {
            VmValue::Int(i) => Some(VmMapKey::Int(*i)),
            VmValue::UInt(i) => Some(VmMapKey::UInt(*i)),
            VmValue::SInt(i) => Some(VmMapKey::SInt(*i)),
            VmValue::SUInt(i) => Some(VmMapKey::SUInt(*i)),
            VmValue::BByte(b) => Some(VmMapKey::BByte(*b)),
            VmValue::BSByte(b) => Some(VmMapKey::BSByte(*b)),
            VmValue::Byte(b) => Some(VmMapKey::Byte(*b)),
            VmValue::SByte(b) => Some(VmMapKey::SByte(*b)),
            VmValue::Str(s) => Some(VmMapKey::Str(s.clone())),
            VmValue::Bool(b) => Some(VmMapKey::Bool(*b)),
            VmValue::Char(c) => Some(VmMapKey::Char(*c)),
            _ => None,
        }
    }
    pub fn into_value(self) -> VmValue {
        match self {
            VmMapKey::Int(i) => VmValue::Int(i),
            VmMapKey::UInt(i) => VmValue::UInt(i),
            VmMapKey::SInt(i) => VmValue::SInt(i),
            VmMapKey::SUInt(i) => VmValue::SUInt(i),
            VmMapKey::BByte(b) => VmValue::BByte(b),
            VmMapKey::BSByte(b) => VmValue::BSByte(b),
            VmMapKey::Byte(b) => VmValue::Byte(b),
            VmMapKey::SByte(b) => VmValue::SByte(b),
            VmMapKey::Str(s) => VmValue::Str(s),
            VmMapKey::Bool(b) => VmValue::Bool(b),
            VmMapKey::Char(c) => VmValue::Char(c),
        }
    }
}

impl fmt::Display for VmValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            VmValue::Null => write!(f, "null"),
            VmValue::Int(i) => write!(f, "{}", i),
            VmValue::UInt(u) => write!(f, "{}", u),
            VmValue::SInt(i) => write!(f, "{}", i),
            VmValue::SUInt(u) => write!(f, "{}", u),
            VmValue::BByte(b) => write!(f, "{}", b),
            VmValue::BSByte(b) => write!(f, "{}", b),
            VmValue::Byte(b) => write!(f, "{}", b),
            VmValue::SByte(b) => write!(f, "{}", b),
            VmValue::Float(fl) => write!(f, "{}", fl),
            VmValue::SFloat(fl) => write!(f, "{}", fl),
            VmValue::Bool(b) => write!(f, "{}", b),
            VmValue::Char(c) => write!(f, "{}", c),
            VmValue::Str(s) => write!(f, "{}", s),
            VmValue::Function(func) => write!(f, "<fn {}/{}>", func.name, func.arity),
            VmValue::Native(func) => write!(f, "<native fn {}>", func.name()),
            VmValue::Ok(inner) => write!(f, "ok({})", inner),
            VmValue::Err(inner) => write!(f, "err({})", inner),
            VmValue::Error(inner) => write!(f, "error({})", inner),
            VmValue::Arr(items) => {
                write!(f, "[")?;
                for (i, item) in items.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}", item)?;
                }
                write!(f, "]")
            }
            VmValue::Tuple(items) => {
                write!(f, "(")?;
                for (i, item) in items.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}", item)?;
                }
                write!(f, ")")
            }
            VmValue::Set(items) => {
                write!(f, "{{")?;
                for (i, item) in items.borrow().iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}", item.clone().into_value())?;
                }
                write!(f, "}}")
            }
            VmValue::Map(entries) => {
                write!(f, "{{")?;
                for (i, (k, v)) in entries.borrow().iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}: {}", k.clone().into_value(), v)?;
                }
                write!(f, "}}")
            }
            VmValue::Record { name, fields } => {
                write!(f, "{} {{", name)?;
                for (i, (fname, fval)) in fields.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}: {}", fname, fval)?;
                }
                write!(f, "}}")
            }
            VmValue::Tag { name, variant } => write!(f, "{}.{}", name, variant),
            VmValue::Closure { func, .. } => write!(f, "<closure {}/{}>", func.name, func.arity),
            VmValue::Handle { kind, id } => write!(f, "<{:?} handle #{}>", kind, id),
        }
    }
}

impl VmValue {
    /// Human-readable type name used in error labels (e.g. "int", "bool").
    pub fn type_name(&self) -> &'static str {
        match self {
            VmValue::Null => "null",
            VmValue::Int(_) => "int",
            VmValue::UInt(_) => "uint",
            VmValue::SInt(_) => "small int",
            VmValue::SUInt(_) => "small uint",
            VmValue::BByte(_) => "big byte",
            VmValue::BSByte(_) => "big sbyte",
            VmValue::Byte(_) => "byte",
            VmValue::SByte(_) => "sbyte",
            VmValue::Float(_) => "float",
            VmValue::SFloat(_) => "small float",
            VmValue::Bool(_) => "bool",
            VmValue::Char(_) => "char",
            VmValue::Str(_) => "string",
            VmValue::Function(_) => "function",
            VmValue::Native(_) => "native function",
            VmValue::Ok(_) => "ok",
            VmValue::Err(_) => "err",
            VmValue::Error(_) => "error",
            VmValue::Arr(_) => "arr",
            VmValue::Tuple(_) => "tuple",
            VmValue::Set(_) => "set",
            VmValue::Map(_) => "map",
            VmValue::Record { .. } => "record",
            VmValue::Tag { .. } => "tag",
            VmValue::Closure { .. } => "closure",
            VmValue::Handle { kind, .. } => match kind {
                HandleKind::C => "c handle",
                HandleKind::Net => "net handle",
                HandleKind::Http => "http handle",
                HandleKind::Audio => "audio handle",
                HandleKind::Gui => "gui handle",
                HandleKind::File => "file handle",
                HandleKind::Buffer => "buffer handle",
            },
        }
    }

    pub fn is_ok(&self) -> bool {
        matches!(self, VmValue::Ok(_))
    }

    pub fn is_err(&self) -> bool {
        matches!(self, VmValue::Err(_))
    }
}

#[derive(Debug)]
pub struct VmFunction {
    pub name: String,
    pub arity: usize,
    pub chunk: Chunk,
}

// VmValue derives PartialEq, so VmFunction needs it too - compare by
// identity (name+arity) rather than deep-comparing bytecode
impl PartialEq for VmFunction {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name && self.arity == other.arity
    }
}

/// A native (Rust-implemented) function bound into `VmValue::Native`.
///
/// During the `rl-std` migration this is an enum: `Std` holds a thin
/// function-pointer descriptor from the shared stdlib; `Legacy` holds the old
/// `Rc<dyn Fn>` machinery for modules not yet moved. Once every module has
/// migrated, `Legacy` (and `VmNativeFn` below) are removed and this collapses
/// to a plain `NativeHandle<VmRuntime>`.
#[derive(Clone)]
pub enum VmNative {
    /// A shared-stdlib function (thin `fn` pointer, no allocation).
    Std(NativeHandle<VmRuntime>),
    /// A not-yet-migrated function using the old boxed-closure machinery.
    Legacy(Rc<VmNativeFn>),
}

impl VmNative {
    /// The function's name (used for resolution and equality).
    pub fn name(&self) -> &str {
        match self {
            VmNative::Std(h) => h.name,
            VmNative::Legacy(f) => &f.name,
        }
    }
}

impl fmt::Debug for VmNative {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "VmNative({})", self.name())
    }
}

impl PartialEq for VmNative {
    fn eq(&self, other: &Self) -> bool {
        self.name() == other.name()
    }
}

/// A native (Rust-implemented) function bound into `VmValue::Native` via the
/// legacy `Rc<dyn Fn>` machinery (kept until every module migrates to
/// `rl-std`).
pub struct VmNativeFn {
    pub name: String,
    pub func: NativeFn,
}

impl fmt::Debug for VmNativeFn {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "VmNativeFn({})", self.name)
    }
}

impl PartialEq for VmNativeFn {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}
