use std::rc::Rc;

use crate::chunk::{Chunk, OpCode};
use crate::native::Module;
use crate::stdlib;
use crate::values::{VmFunction, VmValue};
use rl_ast::statements::{FunctionAttribute, MatchPattern, TypeAnnotation};
use rl_ast::{
    Ast, ExprId, nodes::ExpressionKind, statements::Statement, statements::StatementKind,
};
use rl_lexer::tokentypes::TokenType;
use rl_utils::errors::{Error, Reason};
use rl_utils::source::SourceFile;
use rl_utils::span::Span;

/// Errors raised while compiling an AST down to bytecode.
///
/// This is a plain alias over the shared [`Error`] type used everywhere
/// else in the pipeline (lexer/parser/checker/interpreter), so `rl-vm`
/// diagnostics get the same ariadne-rendered source snippets instead of
/// the bare-string errors it used to produce.
pub type CompileError = Error;

enum ContinueTarget {
    Backward(usize),
    #[allow(unused)]
    Forward,
}

/// Entry-point functions discovered by [`Compiler::scan_entry_points`], with
/// the resolver-assigned global slots that the compiled chunk stores them at.
struct EntryPoint {
    span: Span,
    slot: u16,
    tests: Vec<(u16, Span)>,
    inits: Vec<(u16, Span, Option<u32>)>,
    finals: Vec<(u16, Span, Option<u32>)>,
}

/// Whether a top-level statement runs in entry mode. Mirrors the filter in the
/// interpreter's `evaluate_program`: only declarations and imports execute;
/// control flow and expression statements are skipped.
fn is_program_setup_statement(kind: &StatementKind) -> bool {
    matches!(
        kind,
        StatementKind::ResolvedImportFile { .. }
            | StatementKind::ResolvedFunctionDeclaration { .. }
            | StatementKind::FunctionDeclaration { .. }
            | StatementKind::Import { .. }
            | StatementKind::ImportFile { .. }
            | StatementKind::ImportFileNamed { .. }
            | StatementKind::ResolvedVariableDeclaration { .. }
            | StatementKind::ResolvedConstantDeclaration { .. }
            | StatementKind::ResolvedArray { .. }
            | StatementKind::ResolvedConstantArray { .. }
            | StatementKind::ResolvedMap { .. }
            | StatementKind::ResolvedConstantMap { .. }
            | StatementKind::ResolvedSet { .. }
            | StatementKind::ResolvedConstantSet { .. }
            | StatementKind::TagDeclaration { .. }
            | StatementKind::RecordDeclaration { .. }
            | StatementKind::ResolvedImplBlock { .. }
    )
}

/// Orders `init`/`final` calls for emission: numbered priorities run first in
/// ascending order, unnumbered ones run last in declaration order.
fn sort_entry_calls_by_priority(
    mut functions: Vec<(u16, Span, Option<u32>)>,
) -> Vec<(u16, Span, Option<u32>)> {
    functions.sort_by_key(|(_, _, priority)| (priority.is_none(), priority.unwrap_or(0)));
    functions
}

/// Numeric type codes for `OpCode::Cast`, matching the operand written by
/// the compiler and read by the `Cast` handler in `vm_logic.rs`. Mirrors the
/// numeric `TypeAnnotation` targets the interpreter's `evaluator.rs` casts to.
struct CastTarget;
impl CastTarget {
    const INT: u16 = 0;
    const FLOAT: u16 = 1;
    const UINT: u16 = 2;
    const SFLOAT: u16 = 3;
    const SUINT: u16 = 4;
    const SINT: u16 = 5;
    const BBYTE: u16 = 6;
    const BSBYTE: u16 = 7;
    const BYTE: u16 = 8;
    const SBYTE: u16 = 9;
}

struct LoopCtx {
    continue_target: ContinueTarget,
    continue_jumps: Vec<usize>,
    break_jumps: Vec<usize>,
    scope_depth: u16,
}

pub struct Compiler<'a> {
    ast: &'a Ast,
    chunk: Chunk,
    next_slot: u16,
    scope_bases: Vec<u16>,
    stdlib: Module,
    loop_stack: Vec<LoopCtx>,
    source: Option<SourceFile>,
}

impl<'a> Compiler<'a> {
    pub fn new(ast: &'a Ast) -> Self {
        Self {
            ast,
            chunk: Chunk::new(),
            next_slot: 0,
            scope_bases: Vec::new(),
            stdlib: stdlib::root(),
            loop_stack: Vec::new(),
            source: None,
        }
    }

    /// Attaches the original source text so compile errors can render
    /// ariadne source snippets instead of a bare message.
    pub fn with_source_file(mut self, source: SourceFile) -> Self {
        self.source = Some(source);
        self
    }

    /// Replaces the stdlib module tree this compiler resolves imports and
    /// `std::` calls against. The REPL passes the previous session's module
    /// here so `get x from std::io` bindings survive across inputs (the
    /// compiler mutates its own `stdlib` in place when it compiles an
    /// [`StatementKind::Import`]).
    pub fn with_stdlib(mut self, stdlib: Module) -> Self {
        self.stdlib = stdlib;
        self
    }

    /// The stdlib module tree this compiler uses, after any imports it
    /// compiled have been folded in. Used by the REPL to persist imports
    /// across inputs.
    pub fn stdlib(&self) -> &Module {
        &self.stdlib
    }

    /// Seeds the global slot counter, so a REPL input can continue assigning
    /// globals from where the persistent resolver's global scope left off.
    pub fn with_global_slot_base(mut self, base: u16) -> Self {
        self.next_slot = base;
        self
    }

    /// Builds a [`Reason::Compile`] error anchored at `span`, with source
    /// attached when known.
    fn err(&self, message: impl Into<String>, span: Span) -> CompileError {
        let err = Error::at(Reason::Compile, message, span);
        match &self.source {
            Some(file) => err.with_source_file(file),
            None => err,
        }
    }

    fn resolve(&self, depth: usize, slot: usize) -> Option<u16> {
        if depth == self.scope_bases.len() {
            return None;
        }
        let base = self.scope_bases[self.scope_bases.len() - 1 - depth];
        Some(base + slot as u16)
    }

    /// Entry function
    /// returns compiled Chunk
    /// stops on first error
    ///
    /// Programs without an entry point compile in bare script mode: every
    /// top-level statement runs top to bottom. Programs with an entry point
    /// (`!#[entry]`, or plain `main` as a fallback) compile only their
    /// declarations and imports, then orchestrate calls in the same order the
    /// interpreter uses: tests -> inits -> entry -> finals.
    pub fn compile(&mut self, statements: &[Statement]) -> Result<Chunk, CompileError> {
        match self.scan_entry_points(statements)? {
            Some(entry) => {
                for stmt in statements {
                    if is_program_setup_statement(&stmt.kind) {
                        self.compile_statement(stmt)?;
                    }
                }
                self.emit_entry_point_calls(entry)?;
            }
            None => {
                self.compile_body(statements)?;
            }
        }

        let end_span = statements.last().map(|s| s.span).unwrap_or_default();
        self.chunk.write_op(OpCode::Return, end_span);
        Ok(std::mem::take(&mut self.chunk))
    }

    /// Scans the program for an explicit `!#[entry]` (falling back to a bare
    /// `main`) and collects the tests/inits/finals that entry mode must run
    /// around it. Returns `None` for script-mode programs.
    ///
    /// # Errors
    /// Returns `Err` when more than one `!#[entry]` function is declared,
    /// mirroring the interpreter's `evaluate_program`.
    fn scan_entry_points(
        &self,
        statements: &[Statement],
    ) -> Result<Option<EntryPoint>, CompileError> {
        let mut explicit_entry: Option<(Span, u16)> = None;
        let mut main_entry: Option<(Span, u16)> = None;
        let mut inits: Vec<(u16, Span, Option<u32>)> = vec![];
        let mut finals: Vec<(u16, Span, Option<u32>)> = vec![];
        let mut tests: Vec<(u16, Span)> = vec![];

        for statement in statements {
            let StatementKind::ResolvedFunctionDeclaration {
                name,
                attribute,
                slot,
                ..
            } = &statement.kind
            else {
                continue;
            };
            let slot = *slot as u16;

            match attribute {
                Some(FunctionAttribute::Entry) => {
                    if explicit_entry.is_some() {
                        return Err(self.err("multiple !#[entry] functions found", statement.span));
                    }
                    explicit_entry = Some((statement.span, slot));
                }
                Some(FunctionAttribute::Test) => tests.push((slot, statement.span)),
                Some(FunctionAttribute::Init(priority)) => {
                    inits.push((slot, statement.span, *priority))
                }
                Some(FunctionAttribute::Final(priority)) => {
                    finals.push((slot, statement.span, *priority))
                }
                None if name == "main" => main_entry = Some((statement.span, slot)),
                _ => {}
            }
        }

        let Some((span, slot)) = explicit_entry.or(main_entry) else {
            return Ok(None);
        };

        Ok(Some(EntryPoint {
            span,
            slot,
            tests,
            inits,
            finals,
        }))
    }

    /// Emits the entry-mode orchestration calls: every test and init/final is
    /// called and its result discarded, the entry function's result is left
    /// on the stack so `run_and_return` surfaces it as the program's value.
    fn emit_entry_point_calls(&mut self, entry: EntryPoint) -> Result<(), CompileError> {
        let EntryPoint {
            span,
            slot,
            tests,
            inits,
            finals,
        } = entry;

        for (test_slot, test_span) in tests {
            self.emit_entry_call(test_slot, test_span, true)?;
        }
        for (init_slot, init_span, _) in sort_entry_calls_by_priority(inits) {
            self.emit_entry_call(init_slot, init_span, true)?;
        }
        self.emit_entry_call(slot, span, false)?;
        for (final_slot, final_span, _) in sort_entry_calls_by_priority(finals) {
            self.emit_entry_call(final_slot, final_span, true)?;
        }
        Ok(())
    }

    /// Emits a zero-argument call to the function stored at global `slot`,
    /// optionally popping its (discarded) result off the stack.
    fn emit_entry_call(
        &mut self,
        slot: u16,
        span: Span,
        discard: bool,
    ) -> Result<(), CompileError> {
        self.chunk.write_op(OpCode::GetGlobal, span);
        self.chunk.write_u16(slot, span);
        self.chunk.write_op(OpCode::Call, span);
        self.chunk.write_u16(0, span);
        if discard {
            self.chunk.write_op(OpCode::Pop, span);
        }
        Ok(())
    }

    /// Statement entry function
    fn compile_statement(&mut self, stmt: &Statement) -> Result<(), CompileError> {
        let span = stmt.span;
        match &stmt.kind {
            StatementKind::ResolvedVariableDeclaration { value, .. }
            | StatementKind::ResolvedConstantDeclaration { value, .. }
            | StatementKind::ResolvedArray { value, .. }
            | StatementKind::ResolvedConstantArray { value, .. }
            | StatementKind::ResolvedMap { value, .. }
            | StatementKind::ResolvedConstantMap { value, .. }
            | StatementKind::ResolvedSet { value, .. }
            | StatementKind::ResolvedConstantSet { value, .. } => {
                self.compile_expr(*value)?;
                let slot = self.next_slot;
                self.next_slot += 1;
                self.chunk.write_op(OpCode::DefineLocal, span);
                self.chunk.write_u16(slot, span);
                Ok(())
            }

            StatementKind::VariableDeclaration { .. }
            | StatementKind::ConstantDeclaration { .. }
            | StatementKind::Array { .. }
            | StatementKind::ConstantArray { .. }
            | StatementKind::Map { .. }
            | StatementKind::ConstantMap { .. }
            | StatementKind::Set { .. }
            | StatementKind::ConstantSet { .. }
            | StatementKind::DestructureDeclaration { .. }
            | StatementKind::ImportFile { .. }
            | StatementKind::ImportFileNamed { .. } => Err(self.err(
                "unresolved declaration reached the compiler - run the resolver pass first",
                span,
            )),

            StatementKind::RecordDeclaration { .. } | StatementKind::TagDeclaration { .. } => {
                Ok(())
            }

            StatementKind::ResolvedImplBlock { record, methods } => {
                for m in methods {
                    let StatementKind::ResolvedFunctionDeclaration {
                        name, params, body, ..
                    } = &m.kind
                    else {
                        continue;
                    };
                    let func_chunk = Self::compile_function_chunk(
                        self.ast,
                        body,
                        params.len(),
                        self.stdlib.clone(),
                        self.source.clone(),
                    )?;
                    let func = VmValue::Function(Rc::new(VmFunction {
                        name: name.clone(),
                        arity: params.len(),
                        chunk: func_chunk,
                    }));
                    let func_idx = self.chunk.add_constant(func);
                    let key = format!("{record}::{name}");
                    let key_idx = self
                        .chunk
                        .add_constant(VmValue::Str(Rc::from(key.as_str())));

                    self.chunk.write_op(OpCode::RegisterMethod, span);
                    self.chunk.write_u16(key_idx, span);
                    self.chunk.write_u16(func_idx, span);
                }
                Ok(())
            }

            StatementKind::While { condition, body } => {
                let loop_start = self.chunk.code.len();
                self.compile_expr(*condition)?;
                let exit_jump = self.emit_jump(OpCode::JumpIfFalse, span);

                self.loop_stack.push(LoopCtx {
                    continue_target: ContinueTarget::Backward(loop_start),
                    continue_jumps: Vec::new(),
                    break_jumps: Vec::new(),
                    scope_depth: self.scope_bases.len() as u16,
                });
                // resolver unconditionally pushes a scope for `while` bodies
                self.compile_block(body, true, span)?;
                let ctx = self.loop_stack.pop().unwrap();

                self.emit_loop(loop_start, span);
                self.patch_jump(exit_jump);
                for pos in ctx.break_jumps {
                    self.patch_jump(pos);
                }
                Ok(())
            }

            StatementKind::Loop(body) => {
                let loop_start = self.chunk.code.len();

                self.loop_stack.push(LoopCtx {
                    continue_target: ContinueTarget::Backward(loop_start),
                    continue_jumps: Vec::new(),
                    break_jumps: Vec::new(),
                    scope_depth: self.scope_bases.len() as u16,
                });
                self.compile_block(body, true, span)?;
                let ctx = self.loop_stack.pop().unwrap();

                self.emit_loop(loop_start, span);
                for pos in ctx.break_jumps {
                    self.patch_jump(pos);
                }
                Ok(())
            }

            StatementKind::ResolvedFor {
                initializer,
                condition,
                increment,
                body,
            } => {
                // The resolver never pushes a scope around a C-style for loop
                // (see rl-resolver/src/statements.rs), so the initializer's
                // variable - and anything the body declares - lives directly
                // in the enclosing frame, with no PushScope/PopScope pair.
                self.compile_statement(initializer)?;

                let loop_start = self.chunk.code.len();
                self.compile_expr(*condition)?;
                let exit_jump = self.emit_jump(OpCode::JumpIfFalse, span);

                // `continue` must still run the increment before re-checking
                // the condition, so it can't jump straight back to
                // `loop_start` like `while` does - it jumps forward to just
                // before the increment instead.
                self.loop_stack.push(LoopCtx {
                    continue_target: ContinueTarget::Forward,
                    continue_jumps: Vec::new(),
                    break_jumps: Vec::new(),
                    scope_depth: self.scope_bases.len() as u16,
                });
                // no extra scope for the body either, matching the interpreter
                self.compile_block(body, false, span)?;
                let ctx = self.loop_stack.pop().unwrap();

                // `continue` jumps land here, right before the increment.
                for pos in ctx.continue_jumps {
                    self.patch_jump(pos);
                }
                self.compile_expr_statement(*increment)?;
                self.emit_loop(loop_start, span);

                self.patch_jump(exit_jump);
                for pos in ctx.break_jumps {
                    self.patch_jump(pos);
                }
                Ok(())
            }

            StatementKind::ResolvedForRange {
                slot, range, body, ..
            } => {
                let items = match &range.kind {
                    StatementKind::Range(items) => items.clone(),
                    _ => {
                        return Err(self.err("for-range: expected a range statement", range.span));
                    }
                };

                let mut break_jumps = Vec::new();
                for item in items {
                    let pre_depth = self.scope_bases.len() as u16;

                    self.chunk.write_op(OpCode::PushScope, span);
                    self.scope_bases.push(self.next_slot);

                    self.emit_const(VmValue::Int(item), span);
                    let loop_var_slot = self.next_slot + *slot as u16;
                    self.next_slot = loop_var_slot + 1;
                    self.emit_define_slot(loop_var_slot, span);

                    self.loop_stack.push(LoopCtx {
                        continue_target: ContinueTarget::Forward,
                        continue_jumps: Vec::new(),
                        break_jumps: Vec::new(),
                        scope_depth: pre_depth,
                    });
                    self.compile_block(body, false, span)?;
                    let ctx = self.loop_stack.pop().unwrap();

                    self.next_slot = self.scope_bases.pop().unwrap();
                    self.chunk.write_op(OpCode::PopScope, span);

                    for pos in ctx.continue_jumps {
                        self.patch_jump(pos);
                    }
                    break_jumps.extend(ctx.break_jumps);
                }
                for pos in break_jumps {
                    self.patch_jump(pos);
                }
                Ok(())
            }

            StatementKind::ResolvedForEach {
                slot,
                iterable,
                body,
                ..
            } => {
                let hidden_base = self.next_slot;
                self.next_slot += 2;
                let arr_slot = hidden_base;
                let idx_slot = hidden_base + 1;
                let hidden_global = self.scope_bases.is_empty();

                self.compile_expr(*iterable)?;
                self.emit_define_slot(arr_slot, span);

                self.emit_const(VmValue::Int(0), span);
                self.emit_define_slot(idx_slot, span);

                let loop_start = self.chunk.code.len();
                self.emit_get_slot(idx_slot, hidden_global, span);
                self.emit_get_slot(arr_slot, hidden_global, span);
                self.chunk.write_op(OpCode::ArrLen, span);
                self.chunk.write_op(OpCode::Less, span);
                let exit_jump = self.emit_jump(OpCode::JumpIfFalse, span);

                let pre_depth = self.scope_bases.len() as u16;
                self.chunk.write_op(OpCode::PushScope, span);
                self.scope_bases.push(self.next_slot);

                self.emit_get_slot(arr_slot, hidden_global, span);
                self.emit_get_slot(idx_slot, hidden_global, span);
                self.chunk.write_op(OpCode::Index, span);
                let loop_var_slot = self.next_slot + *slot as u16;
                self.next_slot = loop_var_slot + 1;
                self.emit_define_slot(loop_var_slot, span);

                self.loop_stack.push(LoopCtx {
                    continue_target: ContinueTarget::Forward,
                    continue_jumps: Vec::new(),
                    break_jumps: Vec::new(),
                    scope_depth: pre_depth,
                });
                self.compile_block(body, false, span)?;
                let ctx = self.loop_stack.pop().unwrap();

                self.next_slot = self.scope_bases.pop().unwrap();
                self.chunk.write_op(OpCode::PopScope, span);

                for pos in ctx.continue_jumps {
                    self.patch_jump(pos);
                }
                self.emit_get_slot(idx_slot, hidden_global, span);
                self.emit_const(VmValue::Int(1), span);
                self.chunk.write_op(OpCode::Add, span);
                self.emit_set_slot(idx_slot, hidden_global, span);

                self.emit_loop(loop_start, span);
                self.patch_jump(exit_jump);
                for pos in ctx.break_jumps {
                    self.patch_jump(pos);
                }

                self.next_slot = hidden_base;
                Ok(())
            }

            StatementKind::Break => {
                if self.loop_stack.is_empty() {
                    return Err(self.err("`break` outside of a loop", span));
                }
                let target_depth = self.loop_stack.last().unwrap().scope_depth;
                let current_depth = self.scope_bases.len() as u16;
                for _ in target_depth..current_depth {
                    self.chunk.write_op(OpCode::PopScope, span);
                }
                let pos = self.emit_jump(OpCode::Jump, span);
                self.loop_stack.last_mut().unwrap().break_jumps.push(pos);
                Ok(())
            }

            StatementKind::Continue => {
                if self.loop_stack.is_empty() {
                    return Err(self.err("`continue` outside of a loop", span));
                }
                let target_depth = self.loop_stack.last().unwrap().scope_depth;
                let current_depth = self.scope_bases.len() as u16;
                for _ in target_depth..current_depth {
                    self.chunk.write_op(OpCode::PopScope, span);
                }
                match self.loop_stack.last().unwrap().continue_target {
                    ContinueTarget::Backward(target) => self.emit_loop(target, span),
                    ContinueTarget::Forward => {
                        let pos = self.emit_jump(OpCode::Jump, span);
                        self.loop_stack.last_mut().unwrap().continue_jumps.push(pos);
                    }
                }
                Ok(())
            }

            StatementKind::Conditional {
                if_branch,
                else_branch,
            } => self.compile_conditional(if_branch, else_branch.as_deref(), span),

            StatementKind::Expression(id) => self.compile_expr_statement(*id),

            StatementKind::Match { value, arms } => self.compile_match(*value, arms, span),

            StatementKind::ResolvedDestructureDeclaration { slots, value, .. } => {
                self.compile_destructure(*value, slots, span)
            }

            StatementKind::ResolvedFunctionDeclaration {
                name, params, body, ..
            } => {
                let func_chunk = Self::compile_function_chunk(
                    self.ast,
                    body,
                    params.len(),
                    self.stdlib.clone(),
                    self.source.clone(),
                )?;
                let func = VmValue::Function(Rc::new(VmFunction {
                    name: name.clone(),
                    arity: params.len(),
                    chunk: func_chunk,
                }));
                let slot = self.next_slot;
                self.next_slot += 1;
                let func_idx = self.chunk.add_constant(func);
                self.chunk.write_op(OpCode::Const, span);
                self.chunk.write_u16(func_idx, span);
                self.chunk.write_op(OpCode::DefineLocal, span);
                self.chunk.write_u16(slot, span);
                // Register the function as a method-call fallback so
                // `value.name(...)` calls it with the receiver as its first
                // argument (mirrors the interpreter's `fn_names`).
                let key_idx = self
                    .chunk
                    .add_constant(VmValue::Str(Rc::from(name.as_str())));
                self.chunk.write_op(OpCode::RegisterUserMethod, span);
                self.chunk.write_u16(key_idx, span);
                self.chunk.write_u16(func_idx, span);
                Ok(())
            }

            StatementKind::Return(expr_opt) => {
                match expr_opt {
                    Some(e) => self.compile_expr(*e)?,
                    None => self.emit_const(VmValue::Null, span),
                }
                self.chunk.write_op(OpCode::Return, span);
                Ok(())
            }

            StatementKind::Import { names, wildcard, path } => {
                let mut module = &self.stdlib;
                for seg in path {
                    module = module.submodules.get(seg).ok_or_else(|| {
                        self.err(format!("unknown module '{}'", path.join("::")), span)
                    })?;
                }

                if *wildcard {
                    let fns: Vec<_> = module.functions.iter().map(|(n, f)| (n.clone(), f.clone())).collect();
                    for (name, f) in fns {
                        self.stdlib.functions.insert(name.clone(), f.clone());
                        let key_idx = self
                            .chunk
                            .add_constant(VmValue::Str(Rc::from(name.as_str())));
                        let value_idx = self.chunk.add_constant(VmValue::Native(f));
                        self.chunk.write_op(OpCode::RegisterStdlibMethod, span);
                        self.chunk.write_u16(key_idx, span);
                        self.chunk.write_u16(value_idx, span);
                    }
                } else {
                    let fns: Vec<_> = names
                        .iter()
                        .map(|(name, _alias)| {
                            module.functions.get(name).cloned().ok_or_else(|| {
                                self.err(
                                    format!("'{}' is not defined in '{}'", name, path.join("::")),
                                    span,
                                )
                            })
                        })
                        .collect::<Result<_, CompileError>>()?;

                    for ((name, _alias), f) in names.iter().zip(fns) {
                        let effective_name = _alias.as_deref().unwrap_or(name);
                        self.stdlib.functions.insert(effective_name.to_string(), f.clone());
                        let key_idx = self
                            .chunk
                            .add_constant(VmValue::Str(Rc::from(effective_name)));
                        let value_idx = self.chunk.add_constant(VmValue::Native(f));
                        self.chunk.write_op(OpCode::RegisterStdlibMethod, span);
                        self.chunk.write_u16(key_idx, span);
                        self.chunk.write_u16(value_idx, span);
                    }
                }

                Ok(())
            }

            StatementKind::ResolvedImportFile { body, .. } => {
                for stmt in body {
                    self.compile_statement(stmt)?;
                }
                Ok(())
            }

            other => Err(self.err(
                format!("statement kind not yet supported by the vm compiler: {other:?}"),
                span,
            )),
        }
    }

    fn compile_conditional(
        &mut self,
        if_branch: &Statement,
        else_branch: Option<&Statement>,
        span: Span,
    ) -> Result<(), CompileError> {
        let StatementKind::ConditionalBranch {
            condition,
            body,
            needs_scope,
        } = &if_branch.kind
        else {
            return Err(self.err(
                "malformed if-branch reached the vm compiler",
                if_branch.span,
            ));
        };
        let condition = condition
            .ok_or_else(|| self.err("if-branch is missing its condition", if_branch.span))?;

        self.compile_expr(condition)?;
        let else_jump = self.emit_jump(OpCode::JumpIfFalse, span);
        self.compile_block(body, *needs_scope, span)?;

        let end_jump = if else_branch.is_some() {
            Some(self.emit_jump(OpCode::Jump, span))
        } else {
            None
        };
        self.patch_jump(else_jump);

        if let Some(else_stmt) = else_branch {
            match &else_stmt.kind {
                StatementKind::Conditional {
                    if_branch,
                    else_branch,
                } => self.compile_conditional(if_branch, else_branch.as_deref(), span)?,
                StatementKind::ConditionalBranch {
                    body, needs_scope, ..
                } => {
                    self.compile_block(body, *needs_scope, span)?;
                }
                other => {
                    return Err(self.err(
                        format!("unexpected else-branch kind: {other:?}"),
                        else_stmt.span,
                    ));
                }
            }
        }

        if let Some(end_jump) = end_jump {
            self.patch_jump(end_jump);
        }
        Ok(())
    }

    fn compile_body(&mut self, statements: &[Statement]) -> Result<(), CompileError> {
        let trailing_expr = matches!(
            statements.last().map(|s| &s.kind),
            Some(StatementKind::Expression(_))
        );
        for (i, stmt) in statements.iter().enumerate() {
            if let StatementKind::Expression(id) = &stmt.kind {
                if trailing_expr && i == statements.len() - 1 {
                    self.compile_expr(*id)?;
                } else {
                    self.compile_expr_statement(*id)?;
                }
            } else {
                self.compile_statement(stmt)?;
            }
        }
        Ok(())
    }

    /// Compiles a `{ }` body. Every expression statement inside a block is
    /// always discarded (Pop) - only the top-level program's trailing
    /// expression statement keeps its value.
    fn compile_block(
        &mut self,
        body: &[Statement],
        needs_scope: bool,
        span: Span,
    ) -> Result<(), CompileError> {
        if needs_scope {
            self.chunk.write_op(OpCode::PushScope, span);
            self.scope_bases.push(self.next_slot);
        }
        for stmt in body {
            if let StatementKind::Expression(id) = &stmt.kind {
                self.compile_expr_statement(*id)?;
            } else {
                self.compile_statement(stmt)?;
            }
        }
        if needs_scope {
            self.chunk.write_op(OpCode::PopScope, span);
            self.next_slot = self.scope_bases.pop().unwrap();
        }
        Ok(())
    }

    /// Expression statement entry function
    fn compile_expr_statement(&mut self, id: ExprId) -> Result<(), CompileError> {
        let span = self.ast.exprs.get(id).span;
        self.compile_expr(id)?;
        self.chunk.write_op(OpCode::Pop, span);
        Ok(())
    }

    /// Expression entry function
    fn compile_expr(&mut self, id: ExprId) -> Result<(), CompileError> {
        let expr = self.ast.exprs.get(id);
        let span = expr.span;
        match &expr.kind {
            ExpressionKind::Null => self.emit_const(VmValue::Null, span),
            ExpressionKind::Integer(v) => self.emit_const(VmValue::Int(*v), span),
            ExpressionKind::UInt(v) => self.emit_const(VmValue::UInt(*v), span),
            ExpressionKind::SInt(v) => self.emit_const(VmValue::SInt(*v), span),
            ExpressionKind::SUInt(v) => self.emit_const(VmValue::SUInt(*v), span),
            ExpressionKind::BByte(v) => self.emit_const(VmValue::BByte(*v), span),
            ExpressionKind::BSByte(v) => self.emit_const(VmValue::BSByte(*v), span),
            ExpressionKind::Byte(v) => self.emit_const(VmValue::Byte(*v), span),
            ExpressionKind::SByte(v) => self.emit_const(VmValue::SByte(*v), span),
            ExpressionKind::Float(v) => self.emit_const(VmValue::Float(*v), span),
            ExpressionKind::SFloat(v) => self.emit_const(VmValue::SFloat(*v), span),
            ExpressionKind::Bool(v) => self.emit_const(VmValue::Bool(*v), span),
            ExpressionKind::Character(v) => self.emit_const(VmValue::Char(*v), span),
            ExpressionKind::String(v) => self.emit_const(VmValue::Str(Rc::from(v.as_str())), span),

            ExpressionKind::Grouping(inner) => self.compile_expr(*inner)?,

            ExpressionKind::Unary { operator, operand } => {
                self.compile_expr(*operand)?;
                match operator {
                    TokenType::Minus => self.chunk.write_op(OpCode::Negate, span),
                    TokenType::Bang => self.chunk.write_op(OpCode::Not, span),
                    other => {
                        return Err(self.err(
                            format!("unsupported unary operator in vm compiler: {other:?}"),
                            span,
                        ));
                    }
                }
            }

            ExpressionKind::Binary {
                left,
                operator,
                right,
            } => {
                match operator {
                    TokenType::And => return self.compile_logical(*left, *right, span, true),
                    TokenType::Or => return self.compile_logical(*left, *right, span, false),
                    _ => {}
                }
                self.compile_expr(*left)?;
                self.compile_expr(*right)?;
                let op = match operator {
                    TokenType::Plus => OpCode::Add,
                    TokenType::Minus => OpCode::Sub,
                    TokenType::Star => OpCode::Mul,
                    TokenType::Slash => OpCode::Div,
                    TokenType::Compare => OpCode::Eq,
                    TokenType::BangEqual => OpCode::NotEq,
                    TokenType::Less => OpCode::Less,
                    TokenType::LessEqual => OpCode::LessEq,
                    TokenType::Greater => OpCode::Greater,
                    TokenType::GreaterEqual => OpCode::GreaterEq,
                    other => {
                        return Err(self.err(
                            format!("unsupported binary operator in vm compiler: {other:?}"),
                            span,
                        ));
                    }
                };
                self.chunk.write_op(op, span);
            }

            ExpressionKind::ResolvedIdentifier { depth, slot, .. } => {
                match self.resolve(*depth, *slot) {
                    Some(s) => {
                        self.chunk.write_op(OpCode::GetLocal, span);
                        self.chunk.write_u16(s, span);
                    }

                    None => {
                        self.chunk.write_op(OpCode::GetGlobal, span);
                        self.chunk.write_u16(*slot as u16, span);
                    }
                }
            }

            ExpressionKind::ResolvedAssign {
                depth, slot, value, ..
            } => {
                self.compile_expr(*value)?;
                match self.resolve(*depth, *slot) {
                    Some(s) => {
                        self.chunk.write_op(OpCode::SetLocal, span);
                        self.chunk.write_u16(s, span);
                    }

                    None => {
                        self.chunk.write_op(OpCode::SetGlobal, span);
                        self.chunk.write_u16(*slot as u16, span);
                    }
                }
            }

            ExpressionKind::Call { path, args } => {
                match self.stdlib.resolve(path) {
                    Some(native) => self.emit_const(VmValue::Native(native), span),
                    None if path.len() == 2 => {
                        // `Record::method` associated function, e.g.
                        // `Point::new(1, 2)` - resolved at runtime against
                        // the `impl_methods` table (see `LookupAssoc`),
                        // since the compiler doesn't track record impls.
                        let key = format!("{}::{}", path[0], path[1]);
                        let key_idx = self
                            .chunk
                            .add_constant(VmValue::Str(Rc::from(key.as_str())));
                        self.chunk.write_op(OpCode::LookupAssoc, span);
                        self.chunk.write_u16(key_idx, span);
                    }
                    None => {
                        return Err(
                            self.err(format!("undefined function {}", path.join("::")), span)
                        );
                    }
                }
                for arg in args {
                    self.compile_expr(*arg)?;
                }
                self.chunk.write_op(OpCode::Call, span);
                self.chunk.write_u16(args.len() as u16, span);
            }

            ExpressionKind::MethodCall {
                caller,
                method,
                args,
            } => {
                if method.len() > 1 {
                    let native = self.stdlib.resolve(method).ok_or_else(|| {
                        self.err(format!("undefined function {}", method.join("::")), span)
                    })?;
                    self.emit_const(VmValue::Native(native), span); // callee
                    self.compile_expr(*caller)?; // receiver = arg 1
                    for arg in args {
                        self.compile_expr(*arg)?;
                    }
                    self.chunk.write_op(OpCode::Call, span);
                    self.chunk.write_u16((args.len() + 1) as u16, span);
                    return Ok(());
                }
                // Instance method dispatch, e.g. `point.magnitude()`. The
                // record type is only known at runtime, so the caller is
                // compiled first and `LookupMethod` resolves against it
                // there (see the `LookupMethod` handler in `vm_logic.rs`),
                // inserting the resolved function below it on the stack so
                // it lines up with `OpCode::Call`'s `[callee, args...]`
                // layout, with `self` as the first argument.
                self.compile_expr(*caller)?;
                let name_idx = self
                    .chunk
                    .add_constant(VmValue::Str(Rc::from(method[0].as_str())));
                self.chunk.write_op(OpCode::LookupMethod, span);
                self.chunk.write_u16(name_idx, span);

                for arg in args {
                    self.compile_expr(*arg)?;
                }
                self.chunk.write_op(OpCode::Call, span);
                self.chunk.write_u16((args.len() + 1) as u16, span);
            }

            ExpressionKind::CallExpr { callee, args } => {
                self.compile_expr(*callee)?;
                for arg in args {
                    self.compile_expr(*arg)?;
                }
                self.chunk.write_op(OpCode::Call, span);
                self.chunk.write_u16(args.len() as u16, span);
            }

            ExpressionKind::OkLiteral(inner) => {
                self.compile_expr(*inner)?;
                self.chunk.write_op(OpCode::Ok, span);
            }

            ExpressionKind::ErrLiteral(inner) => {
                self.compile_expr(*inner)?;
                self.chunk.write_op(OpCode::Err, span);
            }

            ExpressionKind::Propagate(inner) => {
                self.compile_expr(*inner)?;
                self.chunk.write_op(OpCode::Propagate, span);
            }

            ExpressionKind::ErrorLiteral(inner) => {
                self.compile_expr(*inner)?;
                self.chunk.write_op(OpCode::Error, span);
            }

            ExpressionKind::ArrayLiteral(items) => {
                for item in items {
                    self.compile_expr(*item)?;
                }
                self.chunk.write_op(OpCode::BuildArr, span);
                self.chunk.write_u16(items.len() as u16, span);
            }

            ExpressionKind::TupleLiteral(items) => {
                for item in items {
                    self.compile_expr(*item)?;
                }
                self.chunk.write_op(OpCode::BuildTuple, span);
                self.chunk.write_u16(items.len() as u16, span);
            }

            ExpressionKind::SetLiteral(items) => {
                for item in items {
                    self.compile_expr(*item)?;
                }
                self.chunk.write_op(OpCode::BuildSet, span);
                self.chunk.write_u16(items.len() as u16, span);
            }

            ExpressionKind::MapLiteral(items) => {
                for (key, value) in items {
                    self.compile_expr(*key)?;
                    self.compile_expr(*value)?;
                }
                self.chunk.write_op(OpCode::BuildMap, span);
                self.chunk.write_u16(items.len() as u16, span);
            }

            ExpressionKind::Index { target, index } => {
                self.compile_expr(*target)?;
                self.compile_expr(*index)?;
                self.chunk.write_op(OpCode::Index, span);
            }

            ExpressionKind::IndexAssign {
                target,
                index,
                value,
            } => {
                let ExpressionKind::ResolvedIdentifier { depth, slot, .. } =
                    &self.ast.exprs.get(*target).kind
                else {
                    return Err(self.err(
                        "vm compiler only supports index-assignment on a plain variable \
                         (e.g. `arr[i] = x`), not a chained or computed target",
                        self.ast.exprs.get(*target).span,
                    ));
                };
                let (depth, slot) = (*depth, *slot);
                let resolved = self.resolve(depth, slot);

                // push current array
                match resolved {
                    Some(s) => {
                        self.chunk.write_op(OpCode::GetLocal, span);
                        self.chunk.write_u16(s, span);
                    }
                    None => {
                        self.chunk.write_op(OpCode::GetGlobal, span);
                        self.chunk.write_u16(slot as u16, span);
                    }
                }
                self.compile_expr(*index)?;
                self.compile_expr(*value)?;
                self.chunk.write_op(OpCode::ArrSet, span);
                // write the updated array back; result stays on the stack as the expr's value
                match resolved {
                    Some(s) => {
                        self.chunk.write_op(OpCode::SetLocal, span);
                        self.chunk.write_u16(s, span);
                    }
                    None => {
                        self.chunk.write_op(OpCode::SetGlobal, span);
                        self.chunk.write_u16(slot as u16, span);
                    }
                }
            }

            ExpressionKind::StructLiteral { name, fields } => {
                let field_names: Vec<VmValue> = fields
                    .iter()
                    .map(|(fname, _)| VmValue::Str(Rc::from(fname.as_str())))
                    .collect();
                for (_, value_id) in fields {
                    self.compile_expr(*value_id)?;
                }
                let name_idx = self
                    .chunk
                    .add_constant(VmValue::Str(Rc::from(name.as_str())));
                let fields_idx = self.chunk.add_constant(VmValue::Arr(Rc::new(field_names)));
                self.chunk.write_op(OpCode::BuildRecord, span);
                self.chunk.write_u16(name_idx, span);
                self.chunk.write_u16(fields_idx, span);
                self.chunk.write_u16(fields.len() as u16, span);
            }

            ExpressionKind::FieldAccess { target, field } => {
                self.compile_expr(*target)?;
                let field_idx = self
                    .chunk
                    .add_constant(VmValue::Str(Rc::from(field.as_str())));
                self.chunk.write_op(OpCode::FieldGet, span);
                self.chunk.write_u16(field_idx, span);
            }

            ExpressionKind::FieldAssign {
                target,
                field,
                value,
            } => {
                self.compile_expr(*target)?;
                self.compile_expr(*value)?;
                let field_idx = self
                    .chunk
                    .add_constant(VmValue::Str(Rc::from(field.as_str())));
                self.chunk.write_op(OpCode::FieldSet, span);
                self.chunk.write_u16(field_idx, span);
            }

            ExpressionKind::EnumVariant { enum_name, variant } => {
                self.emit_const(
                    VmValue::Tag {
                        name: Rc::from(enum_name.as_str()),
                        variant: Rc::from(variant.as_str()),
                    },
                    span,
                );
            }

            ExpressionKind::Cast { value, target_type } => {
                self.compile_expr(*value)?;
                let code = match target_type {
                    TypeAnnotation::Int => CastTarget::INT,
                    TypeAnnotation::UInt => CastTarget::UINT,
                    TypeAnnotation::SInt => CastTarget::SINT,
                    TypeAnnotation::SUInt => CastTarget::SUINT,
                    TypeAnnotation::Float => CastTarget::FLOAT,
                    TypeAnnotation::SFloat => CastTarget::SFLOAT,
                    TypeAnnotation::Byte => CastTarget::BYTE,
                    TypeAnnotation::SByte => CastTarget::SBYTE,
                    TypeAnnotation::BByte => CastTarget::BBYTE,
                    TypeAnnotation::BSByte => CastTarget::BSBYTE,
                    other => {
                        return Err(
                            self.err(format!("unsupported cast target type {other:?}"), span)
                        );
                    }
                };
                self.chunk.write_op(OpCode::Cast, span);
                self.chunk.write_u16(code, span);
            }

            ExpressionKind::ResolvedLambda {
                params,
                body,
                capture_depth,
                ..
            } => {
                let param_count = params.len();
                let (capture_start, captured_scope_bases, outer_next_slot): (u16, &[u16], u16) =
                    if self.scope_bases.is_empty() {
                        (0, &[], 0)
                    } else {
                        let cap_depth = (*capture_depth).min(self.scope_bases.len());
                        let capture_start = if cap_depth == 0 {
                            self.next_slot
                        } else {
                            self.scope_bases[self.scope_bases.len() - cap_depth]
                        };
                        let captured_scope_bases =
                            &self.scope_bases[self.scope_bases.len() - cap_depth..];
                        (capture_start, captured_scope_bases, self.next_slot)
                    };
                let chunk = Self::compile_closure_chunk(
                    self.ast,
                    body,
                    param_count,
                    captured_scope_bases,
                    outer_next_slot,
                    self.stdlib.clone(),
                    self.source.clone(),
                )?;

                let func = VmValue::Function(Rc::new(VmFunction {
                    name: "<lambda>".to_string(),
                    arity: param_count,
                    chunk,
                }));
                let const_idx = self.chunk.add_constant(func);
                self.chunk.write_op(OpCode::BuildClosure, span);
                self.chunk.write_u16(const_idx, span);
                self.chunk.write_u16(capture_start, span);
            }

            ExpressionKind::Identifier(name) => {
                return Err(self.err(format!("undefined variable '{}'", name), span));
            }

            other => {
                return Err(self.err(
                    format!("expression kind not yet supported by the vm compiler: {other:?}"),
                    span,
                ));
            }
        }
        Ok(())
    }

    /// Helper function that adds the value into Chunk constants
    fn emit_const(&mut self, value: VmValue, span: Span) {
        let idx = self.chunk.add_constant(value);
        self.chunk.write_op(OpCode::Const, span);
        self.chunk.write_u16(idx, span);
    }

    /// Emits `op` with a placeholder u16 operand; returns the byte offset
    /// of that operand so it can be filled in later via `patch_jump`.
    fn emit_jump(&mut self, op: OpCode, span: Span) -> usize {
        self.chunk.write_op(op, span);
        self.chunk.write_u16(0xFFFF, span);
        self.chunk.code.len() - 2
    }

    /// Backpatches a previously-emitted forward jump to land at the
    /// current end of the chunk.
    fn patch_jump(&mut self, operand_pos: usize) {
        let jump = self.chunk.code.len() - (operand_pos + 2);
        let bytes = (jump as u16).to_le_bytes();
        self.chunk.code[operand_pos] = bytes[0];
        self.chunk.code[operand_pos + 1] = bytes[1];
    }

    /// Emits a backward `Loop` jump targeting `loop_start`.
    fn emit_loop(&mut self, loop_start: usize, span: Span) {
        self.chunk.write_op(OpCode::Loop, span);
        let pos_after_operand = self.chunk.code.len() + 2;
        let offset = (pos_after_operand - loop_start) as u16;
        self.chunk.write_u16(offset, span);
    }

    fn compile_match(
        &mut self,
        value: ExprId,
        arms: &[(MatchPattern, Vec<Statement>)],
        span: Span,
    ) -> Result<(), CompileError> {
        let slot = self.next_slot;
        self.next_slot += 1;
        let vslot = slot;
        let is_global = self.scope_bases.is_empty();

        self.compile_expr(value)?;
        self.emit_define_slot(vslot, span);

        let mut end_jumps = Vec::new();
        for (pattern, body) in arms {
            let next_arm_jump = match pattern {
                MatchPattern::Wildcard => None,
                MatchPattern::Literal(expr) => {
                    self.compile_expr(*expr)?;
                    self.emit_get_slot(vslot, is_global, span);
                    self.chunk.write_op(OpCode::Eq, span);
                    Some(self.emit_jump(OpCode::JumpIfFalse, span))
                }
            };

            self.compile_block(body, true, span)?;
            end_jumps.push(self.emit_jump(OpCode::Jump, span));

            if let Some(j) = next_arm_jump {
                self.patch_jump(j);
            }
        }

        for j in end_jumps {
            self.patch_jump(j);
        }

        self.next_slot = slot;
        Ok(())
    }

    fn compile_function_chunk(
        ast: &Ast,
        body: &[Statement],
        param_count: usize,
        stdlib: Module,
        source: Option<SourceFile>,
    ) -> Result<Chunk, CompileError> {
        let mut sub = Compiler::new(ast);
        sub.source = source;
        sub.stdlib = stdlib;
        sub.scope_bases.push(0);
        sub.next_slot = param_count as u16;
        sub.compile_body(body)?;
        // implicit `return null` on fallthrough - anchored at the last
        // statement's span (or a dummy span for an empty body).
        let end_span = body.last().map(|s| s.span).unwrap_or_default();
        sub.chunk.write_op(OpCode::Return, end_span);
        Ok(sub.chunk)
    }

    fn compile_closure_chunk(
        ast: &Ast,
        body: &[Statement],
        param_count: usize,
        captured_scope_bases: &[u16],
        outer_next_slot: u16,
        stdlib: Module,
        source: Option<SourceFile>,
    ) -> Result<Chunk, CompileError> {
        let mut sub = Compiler::new(ast);
        sub.source = source;
        sub.stdlib = stdlib;
        sub.scope_bases = captured_scope_bases.to_vec();
        sub.scope_bases.push(outer_next_slot);
        sub.next_slot = outer_next_slot + param_count as u16;
        sub.compile_body(body)?;
        let end_span = body.last().map(|s| s.span).unwrap_or_default();
        sub.chunk.write_op(OpCode::Return, end_span);
        Ok(sub.chunk)
    }

    fn compile_destructure(
        &mut self,
        value: ExprId,
        slots: &[usize],
        span: Span,
    ) -> Result<(), CompileError> {
        if slots.is_empty() {
            self.compile_expr(value)?;
            self.chunk.write_op(OpCode::Pop, span);
            return Ok(());
        }

        let is_global = self.scope_bases.is_empty();

        let base = self.next_slot;
        self.next_slot += slots.len() as u16;

        self.compile_expr(value)?;
        self.emit_define_slot(base, span);

        for i in 1..slots.len() {
            self.emit_get_slot(base, is_global, span);
            self.emit_const(VmValue::Int(i as i64), span);
            self.chunk.write_op(OpCode::Index, span);
            self.emit_define_slot(base + i as u16, span);
        }

        self.emit_get_slot(base, is_global, span);
        self.emit_const(VmValue::Int(0), span);
        self.chunk.write_op(OpCode::Index, span);
        self.emit_define_slot(base, span);

        Ok(())
    }

    fn compile_logical(
        &mut self,
        left: ExprId,
        right: ExprId,
        span: Span,
        is_and: bool,
    ) -> Result<(), CompileError> {
        self.compile_expr(left)?;
        let branch = self.emit_jump(OpCode::JumpIfFalse, span);

        if is_and {
            self.compile_expr(right)?;
            self.emit_const(VmValue::Bool(true), span);
            self.chunk.write_op(OpCode::Eq, span);
        } else {
            self.emit_const(VmValue::Bool(true), span);
        }
        let end = self.emit_jump(OpCode::Jump, span);

        self.patch_jump(branch);
        if is_and {
            self.emit_const(VmValue::Bool(false), span);
        } else {
            self.compile_expr(right)?;
            self.emit_const(VmValue::Bool(true), span);
            self.chunk.write_op(OpCode::Eq, span);
        }
        self.patch_jump(end);
        Ok(())
    }

    /// Defines a raw, compiler-managed slot.
    fn emit_define_slot(&mut self, slot: u16, span: Span) {
        self.chunk.write_op(OpCode::DefineLocal, span);
        self.chunk.write_u16(slot, span);
    }

    /// Reads a raw, compiler-managed slot.
    fn emit_get_slot(&mut self, slot: u16, is_global: bool, span: Span) {
        if is_global {
            self.chunk.write_op(OpCode::GetGlobal, span);
        } else {
            self.chunk.write_op(OpCode::GetLocal, span);
        }
        self.chunk.write_u16(slot, span);
    }

    /// Writes a raw, compiler-managed slot and discards the leftover
    /// value that SetLocal/SetGlobal leave on the stack.
    fn emit_set_slot(&mut self, slot: u16, is_global: bool, span: Span) {
        if is_global {
            self.chunk.write_op(OpCode::SetGlobal, span);
        } else {
            self.chunk.write_op(OpCode::SetLocal, span);
        }
        self.chunk.write_u16(slot, span);
        self.chunk.write_op(OpCode::Pop, span);
    }
}
