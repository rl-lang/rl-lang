//! Variable resolution pass - runs between parsing and evaluation.
//!
//! The resolver walks the AST and transforms unresolved name references
//! into slot-indexed lookups (`depth`, `slot`), eliminating string-based
//! name lookups at runtime.
//!
//! - `depth` - how many scopes up from the current scope the variable lives
//! - `slot`  - the index of the variable within that scope's slot array
//!
//! Unresolved `Identifier` nodes become `ResolvedIdentifier { depth, slot }`.
//! Unresolved `Assign` nodes become `ResolvedAssign { depth, slot, value }`.
//! Function and lambda bodies are resolved in their own pushed scope.
//! Import statements are read from disk, lexed, parsed, and resolved inline.

use rl_ast::{Ast, statements::Statement};
use std::collections::{HashMap, HashSet};
use std::path::PathBuf;

mod contracts;
mod expressions;
mod statements;

/// Walks the AST and resolves all name references to `(depth, slot)` pairs.
pub struct Resolver {
    /// Stack of scopes, each scope being an ordered list of declared names.
    /// Index in the list is the slot number; distance from the top is the depth.
    scopes: Vec<Vec<String>>,
    pub current_dir: std::path::PathBuf,
    pub ast_arena: Ast,
    /// Canonical paths of files currently being resolved, from the entry
    /// file down to whatever `get` statement is on the stack right now.
    /// Guards against `A imports B imports A` recursing forever - each
    /// `ImportFile`/`ImportFileNamed` pushes its canonical path before
    /// recursing into the body and pops it after. Reporting the cycle as
    /// an error is the checker's job; the resolver just needs to not
    /// blow the stack, so a repeat here silently stops the recursion.
    importing: HashSet<PathBuf>,
    /// Caches the merged (parsed + arena-remapped, but not yet slot-resolved)
    /// statements for each canonical file path, so a module imported from
    /// several call sites is only read/lexed/parsed/merged once. Each call
    /// site still clones its own copy and resolves it independently, since
    /// slot numbers depend on the importing scope, not the file.
    import_cache: HashMap<PathBuf, Vec<Statement>>,
}

impl Default for Resolver {
    fn default() -> Self {
        Self::new()
    }
}

impl Resolver {
    /// Creates a new [`Resolver`] with a single empty global scope.
    pub fn new() -> Self {
        Self {
            scopes: vec![vec![]],
            current_dir: std::path::PathBuf::new(),
            ast_arena: Ast::new(),
            importing: HashSet::new(),
            import_cache: HashMap::new(),
        }
    }

    /// Pushes a new empty scope onto the scope stack.
    pub fn push_scope(&mut self) {
        self.scopes.push(vec![]);
    }

    /// Pops the innermost scope from the stack.
    pub fn pop_scope(&mut self) {
        self.scopes.pop();
    }

    /// Declares a name in the current scope and returns its slot index.
    ///
    /// The slot is the position of the name within the current scope frame,
    /// used later by the evaluator for direct indexed access.
    pub fn declare(&mut self, name: String) -> usize {
        let frame = self.scopes.last_mut().unwrap();
        let slot = frame.len();
        frame.push(name);
        slot
    }

    /// Searches for `name` by walking scopes from innermost to outermost.
    ///
    /// Returns `Some((depth, slot))` where `depth` is the number of scopes
    /// above the current one (0 = current), and `slot` is the index within
    /// that scope. Returns `None` if the name is not declared in any scope.
    pub fn resolve_name(&self, name: &str) -> Option<(usize, usize)> {
        for (depth, frame) in self.scopes.iter().rev().enumerate() {
            if let Some(slot) = frame.iter().rposition(|n| n == name) {
                return Some((depth, slot));
            }
        }
        None
    }

    /// Names declared in the persistent global scope (`scopes[0]`), in slot
    /// order. The global scope survives across [`resolve_program`] calls, so
    /// a REPL that keeps one [`Resolver`] alive can use this to expose
    /// user-defined names for tab-completion and to seed the VM compiler's
    /// global slot counter.
    ///
    /// [`resolve_program`]: crate::Resolver::resolve_program
    pub fn global_names(&self) -> &[String] {
        &self.scopes[0]
    }

    /// Number of names currently declared in the persistent global scope.
    /// The next top-level declaration made through [`resolve_program`] gets
    /// exactly this as its global slot.
    pub fn global_slot_count(&self) -> usize {
        self.scopes[0].len()
    }

    /// Truncates the persistent global scope back to `len` names.
    ///
    /// Used by the VM REPL: if resolution declares globals but compilation
    /// subsequently fails, the chunk never runs, so those slots never get set
    /// in the VM. Rolling the global scope back keeps the resolver's slot
    /// count in sync with what actually executed.
    pub fn truncate_global_scope(&mut self, len: usize) {
        self.scopes[0].truncate(len);
    }
}
