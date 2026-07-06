//! Helper methods on [`CheckType`] and [`ScopeItem`], plus private type-matching utilities.

use crate::{
    ast::statements::TypeAnnotation,
    checker::structs::{CheckType, ScopeItem},
};

impl ScopeItem {
    pub fn new(type_annotation: CheckType, is_const: bool) -> Self {
        Self {
            type_annotation,
            is_const,
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
                    || const_matches(a, b)
                    || record_matches(a, b)
                    || enum_matches(a, b)
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
        TypeAnnotation::Tuple(inner) => TypeAnnotation::CTuple(inner),
        TypeAnnotation::Error => TypeAnnotation::CError,
        TypeAnnotation::Result(inner) => TypeAnnotation::CResult(inner),
        TypeAnnotation::Record(name) => TypeAnnotation::CRecord(name),
        TypeAnnotation::Enum(name) => TypeAnnotation::CEnum(name),
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
    )
}
