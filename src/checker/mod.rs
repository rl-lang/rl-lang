pub mod statements;
pub mod structs;
pub mod types;

use crate::{
    ast::statements::Statement,
    interpreter::evaluator::Evaluator,
    utils::{
        errors::{Error, Reason},
        source::SourceFile,
        span::Span,
    },
};
use std::collections::HashMap;
pub use structs::TypeChecker;

impl Default for TypeChecker {
    fn default() -> Self {
        Self::new()
    }
}

impl TypeChecker {
    pub fn new() -> Self {
        // getting all stdlib modules
        let root_module = Evaluator::default().with_stdlib().root_module;
        let mut stdlib_fn_names = std::collections::HashSet::new();
        collect_fn_names(&root_module, &mut stdlib_fn_names);

        Self {
            scopes: vec![HashMap::new()],
            source_file: None,
            root_module: Evaluator::default().with_stdlib().root_module,
            errors: Vec::new(),
            return_type_stack: Vec::new(),
            loop_depth: 0,
            stdlib_fn_names,
        }
    }

    // functions for source file for ariadne
    pub fn with_source_file(mut self, file: SourceFile) -> Self {
        self.source_file = Some(file);
        self
    }
    pub fn set_source_file(&mut self, file: SourceFile) {
        self.source_file = Some(file);
    }

    // runs check on every ast statement in the list and returns errors as list
    pub fn check(&mut self, statements: &[Statement]) -> &[Error] {
        for statement in statements {
            self.check_statement(statement);
        }
        &self.errors
    }

    // transforms arguments into Error type
    // for message it accepts str and String types
    pub fn err(&self, message: impl Into<String>, span: Span) -> Error {
        let err = Error::at(Reason::Compile, message, span);
        match &self.source_file {
            Some(file) => err.with_source_file(file),
            None => err,
        }
    }

    // transforms the arguments into Error type via err() functions
    // and pushes the error to the errors field
    pub fn error(&mut self, message: impl Into<String>, span: Span) {
        let err = self.err(message, span);
        self.errors.push(err);
    }
}

// collects the stdlib functions names for recgonizing
// the functions
fn collect_fn_names(
    module: &crate::interpreter::native::Module,
    out: &mut std::collections::HashSet<String>,
) {
    for name in module.functions.keys() {
        out.insert(name.clone());
    }
    for sub in module.submodules.values() {
        collect_fn_names(sub, out);
    }
}
