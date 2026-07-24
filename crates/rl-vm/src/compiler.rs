use std::rc::Rc;

use crate::chunk::{Chunk, OpCode};
use crate::native::Module;
use crate::stdlib;
use crate::values::{VmFunction, VmValue};
use rl_ast::statements::MatchPattern;
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
    /// will consume and discard the Compiler
    pub fn compile(mut self, statements: &[Statement]) -> Result<Chunk, CompileError> {
        self.compile_body(statements)?;
        let end_span = statements.last().map(|s| s.span).unwrap_or_default();
        self.chunk.write_op(OpCode::Return, end_span);
        Ok(self.chunk)
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
                    let StatementKind::ResolvedFunctionDeclaration { name, params, body, .. } =
                        &m.kind
                    else {
                        continue;
                    };
                    let func_chunk = Self::compile_function_chunk(
                        self.ast,
                        body,
                        params.len(),
                        self.source.clone(),
                    )?;
                    let func = VmValue::Function(Rc::new(VmFunction {
                        name: name.clone(),
                        arity: params.len(),
                        chunk: func_chunk,
                    }));
                    let func_idx = self.chunk.add_constant(func);
                    let key = format!("{record}::{name}");
                    let key_idx = self.chunk.add_constant(VmValue::Str(Rc::from(key.as_str())));

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
                name,
                slot,
                params,
                body,
                ..
            } => {
                let func_chunk = Self::compile_function_chunk(
                    self.ast,
                    body,
                    params.len(),
                    self.source.clone(),
                )?;
                let func = VmValue::Function(Rc::new(VmFunction {
                    name: name.clone(),
                    arity: params.len(),
                    chunk: func_chunk,
                }));
                self.emit_const(func, span);
                self.chunk.write_op(OpCode::DefineLocal, span);
                self.chunk.write_u16(*slot as u16, span);
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

            StatementKind::Import { names, path } => {
                let mut module = &self.stdlib;
                for seg in path {
                    module = module.submodules.get(seg).ok_or_else(|| {
                        self.err(format!("unknown module '{}'", path.join("::")), span)
                    })?;
                }

                let fns: Vec<_> = names
                    .iter()
                    .map(|name| {
                        module.functions.get(name).cloned().ok_or_else(|| {
                            self.err(
                                format!("'{}' is not defined in '{}'", name, path.join("::")),
                                span,
                            )
                        })
                    })
                    .collect::<Result<_, CompileError>>()?;

                for (name, f) in names.iter().zip(fns) {
                    self.stdlib.functions.insert(name.clone(), f);
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
            ExpressionKind::Float(v) => self.emit_const(VmValue::Float(*v), span),
            ExpressionKind::Bool(v) => self.emit_const(VmValue::Bool(*v), span),
            ExpressionKind::Byte(v) => self.emit_const(VmValue::Byte(*v), span),
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
                        let key_idx =
                            self.chunk.add_constant(VmValue::Str(Rc::from(key.as_str())));
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
                if method.len() != 1 {
                    return Err(self.err(
                        "namespaced method calls (`value.module::method(...)`) are not yet \
                         supported by the vm compiler",
                        span,
                    ));
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
        self.chunk.write_op(OpCode::PushScope, span);
        self.scope_bases.push(self.next_slot);

        self.compile_expr(value)?;
        let _slot = self.next_slot;
        self.next_slot += 1;
        self.chunk.write_op(OpCode::DefineLocal, span);
        self.chunk.write_u16(_slot, span);

        let mut end_jumps = Vec::new();
        for (pattern, body) in arms {
            let next_arm_jump = match pattern {
                MatchPattern::Wildcard => None,
                MatchPattern::Literal(expr) => {
                    self.compile_expr(*expr)?;
                    self.chunk.write_op(OpCode::GetLocal, span);
                    self.chunk.write_u16(_slot, span);
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

        self.chunk.write_op(OpCode::PopScope, span);
        self.next_slot = self.scope_bases.pop().unwrap();
        Ok(())
    }

    fn compile_function_chunk(
        ast: &Ast,
        body: &[Statement],
        param_count: usize,
        source: Option<SourceFile>,
    ) -> Result<Chunk, CompileError> {
        let mut sub = Compiler::new(ast);
        sub.source = source;
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
        source: Option<SourceFile>,
    ) -> Result<Chunk, CompileError> {
        let mut sub = Compiler::new(ast);
        sub.source = source;
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
