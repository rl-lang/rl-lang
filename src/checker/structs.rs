use std::collections::HashMap;

use crate::{
    ast::statements::TypeAnnotation,
    interpreter::native::Module,
    utils::{errors::Error, source::SourceFile},
};

pub struct TypeChecker {
    // list of scopes
    pub scopes: Vec<HashMap<String, ScopeItem>>,
    // source file if exists for ariadne
    pub source_file: Option<SourceFile>,
    // for the stdlib modules to populate the stdlib functions name
    pub root_module: Module,
    // all errors found in same file
    pub errors: Vec<Error>,
    // for lambdas and functions to check what type should be returned
    pub return_type_stack: Vec<TypeAnnotation>,
    // for loops to know what level of depth in loop the checker at
    pub loop_depth: u32,
    // functions name to use in check_call_path
    pub stdlib_fn_names: std::collections::HashSet<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ScopeItem {
    // item type
    pub type_annotation: CheckType,
    // weather is it a constant or variable
    pub is_const: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub enum CheckType {
    // types that can be known during checks
    Known(TypeAnnotation),
    Function {
        params: Vec<TypeAnnotation>,
        return_type: TypeAnnotation,
    },
    // types that cannot be known during checks
    Unknown,
}
