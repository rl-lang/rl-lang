pub mod operators;
pub mod scope;
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
            hovers: Vec::new(),
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
    // same as error() but with helper
    pub fn error_with_help(&mut self, message: impl Into<String>, span: Span, help: Option<&str>) {
        let mut err = self.err(message, span);
        if let Some(h) = help {
            err = err.with_help(format!("did you mean `{}`?", h));
        }
        self.errors.push(err);
    }

    // adds markdown hover text for a source span
    pub fn push_hover(&mut self, span: Span, text: impl Into<String>) {
        self.hovers.push((span, text.into()));
    }

    // using find_fn_doc() in `crate::docs` find the current docs
    // for the function and add the markdown hover for the span of fn
    fn push_stdlib_hover(&mut self, path: &[String], span: Span) {
        let fn_name = match path.last() {
            Some(n) => n.as_str(),
            None => return,
        };
        // get the module to handle std::io::print()
        // and get print from std::io
        let module = if path.len() >= 2 {
            Some(path[path.len() - 2].as_str())
        } else {
            None
        };

        let text = match crate::docs::find_fn_doc(module, fn_name)
            .or_else(|| crate::docs::find_fn_doc(None, fn_name))
        {
            Some((std_entry, func)) => format!(
                "```rl\nstd::{}::{}\n```\n{}",
                std_entry.name, func.signature, func.description
            ),
            None => format!("```rl\nfn {}(..)\n```\nstdlib function", fn_name),
        };

        self.push_hover(span, text);
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
