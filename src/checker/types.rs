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
    /// returns [`CheckType::Known(type)`] if the item type can be determinated while being checked
    pub fn known(ty: TypeAnnotation) -> Self {
        CheckType::Known(ty)
    }

    /// returns true if the item type is [`TypeAnnotation::null`]
    pub fn is_null(&self) -> bool {
        matches!(self, CheckType::Known(TypeAnnotation::Null))
    }

    /// returns [`CheckType::Unknown`] for items type that cannot be determinated while being checked
    pub fn is_unknown(&self) -> bool {
        matches!(self, CheckType::Unknown)
    }

    /// converts the [`CheckType::Known`] variable variant to constant variant
    pub fn into_const(self) -> Self {
        match self {
            CheckType::Known(ty) => CheckType::Known(const_variant(ty)),
            other => other,
        }
    }

    /// returns [`bool`] weather true or false if the conditions for matching met
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
                a == b || null_array_elision(a, b) || const_matches(a, b)
            }
            _ => false,
        }
    }

    /// returns string value of the current item descriptions
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

/// matches mutable arrays and immmutables arrays both sides should match to return true
/// otherwise false
fn null_array_elision(a: &TypeAnnotation, b: &TypeAnnotation) -> bool {
    match (a, b) {
        (TypeAnnotation::Array(x), TypeAnnotation::Array(y))
        | (TypeAnnotation::CArray(x), TypeAnnotation::CArray(y)) => {
            **x == TypeAnnotation::Null || **y == TypeAnnotation::Null
        }
        _ => false,
    }
}

/// transforms the mutable [`TypeAnnotation`] to its immutable version
fn const_variant(ty: TypeAnnotation) -> TypeAnnotation {
    match ty {
        TypeAnnotation::Int => TypeAnnotation::CInt,
        TypeAnnotation::Float => TypeAnnotation::CFloat,
        TypeAnnotation::Bool => TypeAnnotation::CBool,
        TypeAnnotation::String => TypeAnnotation::CString,
        TypeAnnotation::Byte => TypeAnnotation::CByte,
        TypeAnnotation::Char => TypeAnnotation::CChar,
        TypeAnnotation::Array(inner) => TypeAnnotation::CArray(inner),
        other => other,
    }
}

/// compares two values (the immutable TypeAnnotation to its mutable value)
fn const_matches(a: &TypeAnnotation, b: &TypeAnnotation) -> bool {
    matches!(
        (a, b),
        (TypeAnnotation::CString, TypeAnnotation::String)
            | (TypeAnnotation::CInt, TypeAnnotation::Int)
            | (TypeAnnotation::CFloat, TypeAnnotation::Float)
            | (TypeAnnotation::CBool, TypeAnnotation::Bool)
            | (TypeAnnotation::CByte, TypeAnnotation::Byte)
            | (TypeAnnotation::CChar, TypeAnnotation::Char)
    )
}
