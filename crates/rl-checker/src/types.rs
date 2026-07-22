//! Helper methods on [`CheckType`] and [`ScopeItem`], plus private type-matching utilities.

use std::{collections::HashMap, rc::Rc};

use crate::structs::{CheckType, ScopeItem};
use rl_ast::statements::TypeAnnotation;
use rl_utils::span::Span;

impl ScopeItem {
    pub fn new(type_annotation: CheckType, is_const: bool, decl_span: Span) -> Self {
        Self {
            type_annotation,
            is_const,
            decl_span,
        }
    }
}

// helper functions for CheckType
impl CheckType {
    /// Constructs a [`CheckType::Known`] from a [`TypeAnnotation`].
    pub fn known(ty: TypeAnnotation) -> Self {
        CheckType::Known(ty)
    }

    /// Returns `true` if this type is [`TypeAnnotation::Null`].
    pub fn is_null(&self) -> bool {
        matches!(self, CheckType::Known(TypeAnnotation::Null))
    }

    /// Returns `true` if this type is [`CheckType::Unknown`].
    pub fn is_unknown(&self) -> bool {
        matches!(self, CheckType::Unknown)
    }

    /// Converts a mutable `Known` type to its `const` variant (e.g. `Int` -> `CInt`).
    pub fn into_const(self) -> Self {
        match self {
            CheckType::Known(ty) => CheckType::Known(const_variant(ty)),
            other => other,
        }
    }

    /// Returns `true` if `self` is compatible with `expected`.
    ///
    /// Compatibility rules:
    /// - Either side being `Unknown` always matches (avoids cascading errors)
    /// - `Null` matches anything (represents the absence of a value)
    /// - `Function { .. }` matches `Known(Fn)` and vice versa
    /// - Two `Function` types match only if params and return type are identical
    /// - Two `Known` types match if equal, or via [`null_array_elision`] or [`const_matches`]
    pub fn matches(&self, expected: &CheckType) -> bool {
        match (self, expected) {
            // if any side is [`CheckType::Unknown`] returns true
            (CheckType::Unknown, _) | (_, CheckType::Unknown) => true,

            // if item type is [`TypeAnnotation::Null`] will return true
            // to represent the absence of value
            (CheckType::Known(TypeAnnotation::Null), _) => true,

            (_, CheckType::Known(TypeAnnotation::Generic(_)))
            | (CheckType::Known(TypeAnnotation::Generic(_)), _) => true,

            // matches different functions types and returns true
            (CheckType::Function { .. }, CheckType::Known(TypeAnnotation::Fn))
            | (CheckType::Known(TypeAnnotation::Fn), CheckType::Function { .. }) => true,

            // matches [`CheckType::Function`] fields values with each other
            // returns true if all match
            (
                CheckType::Function {
                    params: p1,
                    return_type: r1,
                },
                CheckType::Function {
                    params: p2,
                    return_type: r2,
                },
            ) => p1 == p2 && r1 == r2,

            // checks the TypeAnnotation and compare then returns [`bool`]
            (CheckType::Known(a), CheckType::Known(b)) => {
                a == b
                    || null_array_elision(a, b)
                    || null_map_elision(a, b)
                    || const_matches(a, b)
                    || record_matches(a, b)
                    || enum_matches(a, b)
                    || set_matches(a, b)
            }

            _ => false,
        }
    }

    /// Returns a human-readable description of this type for error messages.
    pub fn info(&self) -> String {
        match self {
            CheckType::Known(ty) => format!("{:?}", ty),
            CheckType::Function {
                params,
                return_type,
            } => format!(
                "fn({}) -> {:?}",
                params
                    .iter()
                    .map(|p| format!("{:?}", p))
                    .collect::<Vec<_>>()
                    .join(", "),
                return_type
            ),
            CheckType::Unknown => "unknown".to_string(),
        }
    }
}

/// Returns `true` if two array types are compatible when either inner type is `Null`
/// (i.e. an empty array `[]` is compatible with any typed array).
fn null_array_elision(a: &TypeAnnotation, b: &TypeAnnotation) -> bool {
    match (a, b) {
        (
            TypeAnnotation::Array(x) | TypeAnnotation::CArray(x),
            TypeAnnotation::Array(y) | TypeAnnotation::CArray(y),
        ) => **x == TypeAnnotation::Null || **y == TypeAnnotation::Null || null_array_elision(x, y),

        (
            TypeAnnotation::Tuple(a) | TypeAnnotation::CTuple(a),
            TypeAnnotation::Tuple(b) | TypeAnnotation::CTuple(b),
        ) => a.iter().zip(b.iter()).all(|(x, y)| {
            *x == TypeAnnotation::Null || *y == TypeAnnotation::Null || null_array_elision(x, y)
        }),
        _ => false,
    }
}

fn null_map_elision(a: &TypeAnnotation, b: &TypeAnnotation) -> bool {
    match (a, b) {
        (
            TypeAnnotation::Map(ak, av) | TypeAnnotation::CMap(ak, av),
            TypeAnnotation::Map(bk, bv) | TypeAnnotation::CMap(bk, bv),
        ) => {
            (**ak == TypeAnnotation::Null
                || **bk == TypeAnnotation::Null
                || null_array_elision(ak, bk)
                || null_map_elision(ak, bk))
                && (**av == TypeAnnotation::Null
                    || **bv == TypeAnnotation::Null
                    || null_array_elision(av, bv)
                    || null_map_elision(av, bv))
        }
        _ => false,
    }
}

/// Converts a mutable [`TypeAnnotation`] to its immutable (`C`-prefixed) variant.
fn const_variant(ty: TypeAnnotation) -> TypeAnnotation {
    match ty {
        TypeAnnotation::Int => TypeAnnotation::CInt,
        TypeAnnotation::Float => TypeAnnotation::CFloat,
        TypeAnnotation::Bool => TypeAnnotation::CBool,
        TypeAnnotation::String => TypeAnnotation::CString,
        TypeAnnotation::Byte => TypeAnnotation::CByte,
        TypeAnnotation::Char => TypeAnnotation::CChar,
        TypeAnnotation::Array(inner) => TypeAnnotation::CArray(inner),
        TypeAnnotation::Map(key, value) => TypeAnnotation::CMap(key, value),
        TypeAnnotation::Tuple(inner) => TypeAnnotation::CTuple(inner),
        TypeAnnotation::Error => TypeAnnotation::CError,
        TypeAnnotation::Result(inner) => TypeAnnotation::CResult(inner),
        TypeAnnotation::Record(name) => TypeAnnotation::CRecord(name),
        TypeAnnotation::Enum(name) => TypeAnnotation::CEnum(name),
        TypeAnnotation::Set(items) => TypeAnnotation::CSet(items),
        other => other,
    }
}

/// Returns `true` if `a` and `b` are the same named record, regardless of
/// `Record`/`CRecord` (mutable/const) mismatch.
fn record_matches(a: &TypeAnnotation, b: &TypeAnnotation) -> bool {
    matches!(
        (a, b),
        (TypeAnnotation::Record(x) | TypeAnnotation::CRecord(x),
         TypeAnnotation::Record(y) | TypeAnnotation::CRecord(y)) if x == y
    )
}

/// Returns `true` if `a` and `b` are the same named tag (enum), regardless of
/// `Enum`/`CEnum` (mutable/const) mismatch.
fn enum_matches(a: &TypeAnnotation, b: &TypeAnnotation) -> bool {
    matches!(
        (a, b),
        (TypeAnnotation::Enum(x) | TypeAnnotation::CEnum(x),
         TypeAnnotation::Enum(y) | TypeAnnotation::CEnum(y)) if x == y
    )
}

fn set_matches(a: &TypeAnnotation, b: &TypeAnnotation) -> bool {
    matches!(
        (a, b),
        (TypeAnnotation::Set(x) | TypeAnnotation::CSet(x),
         TypeAnnotation::Set(y) | TypeAnnotation::CSet(y)) if x == y
    )
}

/// Returns `true` if `a` is the const variant of `b` or vice versa (e.g. `CInt` <-> `Int`).
fn const_matches(a: &TypeAnnotation, b: &TypeAnnotation) -> bool {
    matches!(
        (a, b),
        (TypeAnnotation::CString, TypeAnnotation::String)
            | (TypeAnnotation::CInt, TypeAnnotation::Int)
            | (TypeAnnotation::CFloat, TypeAnnotation::Float)
            | (TypeAnnotation::CBool, TypeAnnotation::Bool)
            | (TypeAnnotation::CByte, TypeAnnotation::Byte)
            | (TypeAnnotation::CChar, TypeAnnotation::Char)
            | (TypeAnnotation::CTuple(_), TypeAnnotation::Tuple(_))
            | (TypeAnnotation::Tuple(_), TypeAnnotation::CTuple(_))
            | (TypeAnnotation::CError, TypeAnnotation::Error)
            | (TypeAnnotation::Error, TypeAnnotation::CError)
            | (TypeAnnotation::CResult(_), TypeAnnotation::Result(_))
            | (TypeAnnotation::Result(_), TypeAnnotation::CResult(_))
            | (TypeAnnotation::CMap(_, _), TypeAnnotation::Map(_, _))
            | (TypeAnnotation::Map(_, _), TypeAnnotation::CMap(_, _))
            | (TypeAnnotation::Set(_), TypeAnnotation::CSet(_))
            | (TypeAnnotation::CSet(_), TypeAnnotation::Set(_))
            | (TypeAnnotation::Array(_), TypeAnnotation::CArray(_))
            | (TypeAnnotation::CArray(_), TypeAnnotation::Array(_))
    )
}

pub fn has_generic(ty: &TypeAnnotation) -> bool {
    match ty {
        TypeAnnotation::Generic(_) => true,
        TypeAnnotation::Array(inner)
        | TypeAnnotation::CArray(inner)
        | TypeAnnotation::Set(inner)
        | TypeAnnotation::CSet(inner)
        | TypeAnnotation::Result(inner)
        | TypeAnnotation::CResult(inner) => has_generic(inner),
        TypeAnnotation::Map(k, v) | TypeAnnotation::CMap(k, v) => has_generic(k) || has_generic(v),
        TypeAnnotation::Tuple(items) | TypeAnnotation::CTuple(items) => {
            items.iter().any(has_generic)
        }
        _ => false,
    }
}

pub fn unify(
    expected: &TypeAnnotation,
    actual: &TypeAnnotation,
    bindings: &mut HashMap<String, TypeAnnotation>,
) -> bool {
    match (expected, actual) {
        (TypeAnnotation::Generic(name), _) => match bindings.get(name) {
            Some(bound) => {
                CheckType::Known(bound.clone()).matches(&CheckType::Known(actual.clone()))
            }
            None => {
                bindings.insert(name.clone(), actual.clone());
                true
            }
        },
        (
            TypeAnnotation::Array(e) | TypeAnnotation::CArray(e),
            TypeAnnotation::Array(a) | TypeAnnotation::CArray(a),
        ) => unify(e, a, bindings),
        (
            TypeAnnotation::Set(e) | TypeAnnotation::CSet(e),
            TypeAnnotation::Set(a) | TypeAnnotation::CSet(a),
        ) => unify(e, a, bindings),
        (
            TypeAnnotation::Result(e) | TypeAnnotation::CResult(e),
            TypeAnnotation::Result(a) | TypeAnnotation::CResult(a),
        ) => unify(e, a, bindings),
        (
            TypeAnnotation::Map(ek, ev) | TypeAnnotation::CMap(ek, ev),
            TypeAnnotation::Map(ak, av) | TypeAnnotation::CMap(ak, av),
        ) => unify(ek, ak, bindings) && unify(ev, av, bindings),
        (
            TypeAnnotation::Tuple(e) | TypeAnnotation::CTuple(e),
            TypeAnnotation::Tuple(a) | TypeAnnotation::CTuple(a),
        ) => e.len() == a.len() && e.iter().zip(a.iter()).all(|(e, a)| unify(e, a, bindings)),
        (e, a) => CheckType::Known(e.clone()).matches(&CheckType::Known(a.clone())),
    }
}

pub fn substitute(
    ty: &TypeAnnotation,
    bindings: &HashMap<String, TypeAnnotation>,
) -> TypeAnnotation {
    match ty {
        TypeAnnotation::Generic(name) => bindings.get(name).cloned().unwrap_or_else(|| ty.clone()),
        TypeAnnotation::Array(inner) => {
            TypeAnnotation::Array(Box::new(substitute(inner, bindings)))
        }
        TypeAnnotation::CArray(inner) => {
            TypeAnnotation::CArray(Box::new(substitute(inner, bindings)))
        }
        TypeAnnotation::Set(inner) => TypeAnnotation::Set(Box::new(substitute(inner, bindings))),
        TypeAnnotation::CSet(inner) => TypeAnnotation::CSet(Box::new(substitute(inner, bindings))),
        TypeAnnotation::Result(inner) => {
            TypeAnnotation::Result(Box::new(substitute(inner, bindings)))
        }
        TypeAnnotation::CResult(inner) => {
            TypeAnnotation::CResult(Box::new(substitute(inner, bindings)))
        }
        TypeAnnotation::Map(k, v) => TypeAnnotation::Map(
            Box::new(substitute(k, bindings)),
            Box::new(substitute(v, bindings)),
        ),
        TypeAnnotation::CMap(k, v) => TypeAnnotation::CMap(
            Box::new(substitute(k, bindings)),
            Box::new(substitute(v, bindings)),
        ),
        TypeAnnotation::Tuple(items) => TypeAnnotation::Tuple(Rc::new(
            items.iter().map(|t| substitute(t, bindings)).collect(),
        )),
        TypeAnnotation::CTuple(items) => TypeAnnotation::CTuple(Rc::new(
            items.iter().map(|t| substitute(t, bindings)).collect(),
        )),
        other => other.clone(),
    }
}
