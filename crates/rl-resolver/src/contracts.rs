//! Contract desugaring (#348): `requires`/`ensures` clauses, parameter
//! refinements, and `ret` binding lower to plain `if` guards, `dec` temps,
//! and `return err(..)` / abort calls BEFORE resolution, so the checker and
//! every backend see only ordinary code.
//!
//! Shapes (with `MSG` the clause message expression and `PRED` its predicate):
//!
//! - Entry checks (param refinements, then `requires`, in order):
//!   `if (!(PRED)) { return err(MSG) }` for `result` functions,
//!   `if (!(PRED)) { std::res::result_unwrap(err(MSG)) }` otherwise.
//!   The abort call never returns (misuse aborts), so the bare form needs
//!   no early return.
//! - `return EXPR` under `ensures` becomes:
//!   `dec __ensure_N = EXPR`
//!   `if (std::res::is_err(__ensure_N)) { return __ensure_N }` (result only)
//!   `dec ret = INNER` (`INNER` unwraps for `result`, else the temp itself)
//!   `if (!(ENS)) { <violation> }` per clause, then `return __ensure_N`.
//!   `ret` resolves to the `dec` like any variable, so no expression walk
//!   is needed; the sequence is atomic (no user code interleaves), making
//!   the shadow sound.
//!
//! Contracts need an explicit `-> T` for `Err`-valued violations: an
//! unannotated (`Null`) return desugars to the bare (abort) form, so
//! contract failures abort instead of returning `Err`. Annotate
//! `-> result[T]` when callers must receive the failure as a value.

use rl_ast::{
    Ast, ExprId,
    nodes::ExpressionKind,
    statements::{
        ContractClause, Param, RefineOp, RefineOperand, Statement,
        StatementKind, TypeAnnotation,
    },
};
use rl_lexer::tokentypes::TokenType;
use rl_utils::span::Span;

struct Ctx<'a> {
    arena: &'a mut Ast,
    span: Span,
    is_result: bool,
    temp_counter: u64,
}

fn stmt(kind: StatementKind, span: Span) -> Statement {
    Statement::new(kind, span)
}

impl<'a> Ctx<'a> {
    fn expr(&mut self, kind: ExpressionKind) -> ExprId {
        self.arena.alloc_expr(kind, self.span)
    }

    fn ident(&mut self, name: &str) -> ExprId {
        let kind = ExpressionKind::Identifier(name.to_string());
        self.expr(kind)
    }

    fn str_lit(&mut self, s: &str) -> ExprId {
        let kind = ExpressionKind::String(s.to_string());
        self.expr(kind)
    }

    fn err_lit(&mut self, msg: ExprId) -> ExprId {
        let kind = ExpressionKind::ErrLiteral(msg);
        self.expr(kind)
    }

    /// `std::res::result_unwrap(x)` as an expression.
    fn unwrap_call(&mut self, x: ExprId) -> ExprId {
        let kind = ExpressionKind::Call {
            path: vec!["std".to_string(), "res".to_string(), "result_unwrap".to_string()],
            args: vec![x],
        };
        self.expr(kind)
    }

    /// `std::res::is_err(x)` as an expression.
    fn is_err_call(&mut self, x: ExprId) -> ExprId {
        let kind = ExpressionKind::Call {
            path: vec!["std".to_string(), "res".to_string(), "is_err".to_string()],
            args: vec![x],
        };
        self.expr(kind)
    }

    /// The clause message expression, or a synthesized default.
    fn msg_expr(&mut self, clause: &ContractClause, default: &str) -> ExprId {
        match clause.message {
            Some(id) => id,
            None => self.str_lit(default),
        }
    }

    /// The violation for a failed predicate: `return err(msg)` for `result`
    /// functions, an aborting `result_unwrap(err(msg))` statement otherwise.
    fn violation(&mut self, msg: ExprId) -> Statement {
        if self.is_result {
            let err = self.err_lit(msg);
            stmt(StatementKind::Return(Some(err)), self.span)
        } else {
            let err = self.err_lit(msg);
            let abort = self.unwrap_call(err);
            stmt(StatementKind::Expression(abort), self.span)
        }
    }

    /// `if (!(pred)) { violation }`. The predicate is deep-cloned per
    /// guard: resolution rewrites nodes in place, so a clause condition
    /// spliced into several guards must resolve separately at each site.
    fn guard(&mut self, pred: ExprId, msg: ExprId) -> Statement {
        let pred = deep_clone_expr(self, pred);        let neg = self.expr(ExpressionKind::Unary {
            operator: TokenType::Bang,
            operand: pred,
        });
        let violation = self.violation(msg);
        let branch = stmt(
            StatementKind::ConditionalBranch {
                condition: Some(neg),
                body: vec![violation],
                needs_scope: false,
            },
            self.span,
        );
        stmt(
            StatementKind::Conditional {
                if_branch: Box::new(branch),
                else_branch: None,
            },
            self.span,
        )
    }

    /// Wraps one `return <value>` under `ensures` (see module docs).
    fn wrap_return(&mut self, value: ExprId, ensures: &[ContractClause]) -> Vec<Statement> {
        let n = self.temp_counter;
        self.temp_counter += 1;
        let temp = format!("__ensure_ret_{n}");
        let mut seq = vec![stmt(
            StatementKind::VariableDeclaration {
                name: temp.clone(),
                type_annotation: TypeAnnotation::Infer,
                value,
                unit_annotation: None,
                item_attributes: Vec::new(),
            },
            self.span,
        )];
        // `result` functions skip `ensures` on `Err` values (plan #348).
        if self.is_result {
            let temp_id = self.ident(&temp);
            let check = self.is_err_call(temp_id);
            let branch = stmt(
                StatementKind::ConditionalBranch {
                    condition: Some(check),
                    body: vec![stmt(
                        StatementKind::Return(Some(self.ident(&temp))),
                        self.span,
                    )],
                    needs_scope: false,
                },
                self.span,
            );
            seq.push(stmt(
                StatementKind::Conditional {
                    if_branch: Box::new(branch),
                    else_branch: None,
                },
                self.span,
            ));
        }
        let inner = if self.is_result {
            let temp_id = self.ident(&temp);
            self.unwrap_call(temp_id)
        } else {
            self.ident(&temp)
        };
        seq.push(stmt(
            StatementKind::VariableDeclaration {
                name: "ret".to_string(),
                type_annotation: TypeAnnotation::Infer,
                value: inner,
                unit_annotation: None,
                item_attributes: Vec::new(),
            },
            self.span,
        ));
        for clause in ensures {
            let msg = self.msg_expr(clause, "ensures violated");
            let guard = self.guard(clause.condition, msg);
            seq.push(guard);
        }
        seq.push(stmt(
            StatementKind::Return(Some(self.ident(&temp))),
            self.span,
        ));
        seq
    }

    fn refine_predicate(&mut self, param: &str, op: RefineOp, operand: &RefineOperand) -> ExprId {
        let token = match op {
            RefineOp::Gt => TokenType::Greater,
            RefineOp::Ge => TokenType::GreaterEqual,
            RefineOp::Lt => TokenType::Less,
            RefineOp::Le => TokenType::LessEqual,
            RefineOp::Eq => TokenType::Compare,
            RefineOp::Ne => TokenType::BangEqual,
        };
        let right = match operand {
            RefineOperand::Integer(v) => self.expr(ExpressionKind::Integer(*v)),
            RefineOperand::Str(s) => self.str_lit(s),
            RefineOperand::Bool(b) => self.expr(ExpressionKind::Bool(*b)),
            RefineOperand::Param(p) => self.ident(p),
        };
        let left = self.ident(param);
        self.expr(ExpressionKind::Binary {
            left,
            operator: token,
            right,
        })
    }
}

fn refine_op_str(op: RefineOp) -> &'static str {
    match op {
        RefineOp::Gt => ">",
        RefineOp::Ge => ">=",
        RefineOp::Lt => "<",
        RefineOp::Le => "<=",
        RefineOp::Eq => "==",
        RefineOp::Ne => "!=",
    }
}

fn operand_str(operand: &RefineOperand) -> String {
    match operand {
        RefineOperand::Integer(v) => v.to_string(),
        RefineOperand::Str(s) => format!("\"{s}\""),
        RefineOperand::Bool(b) => b.to_string(),
        RefineOperand::Param(p) => p.clone(),
    }
}

/// Deep-clones an expression subtree into fresh arena ids. Required because
/// [`crate::Resolver::resolve_expression`] rewrites nodes in place: a clause
/// condition spliced into several guards must resolve separately at each
/// site (depths differ per scope), so sharing one id bakes the first
/// site's resolution into all later uses.
fn deep_clone_expr(ctx: &mut Ctx, id: ExprId) -> ExprId {
    use ExpressionKind as K;
    let (kind, span) = {
        let node = ctx.arena.exprs.get(id);
        (node.kind.clone(), node.span)
    };
    let cloned = match kind {
        K::Integer(v) => K::Integer(v),
        K::SInt(v) => K::SInt(v),
        K::UInt(v) => K::UInt(v),
        K::SUInt(v) => K::SUInt(v),
        K::Byte(v) => K::Byte(v),
        K::SByte(v) => K::SByte(v),
        K::BByte(v) => K::BByte(v),
        K::BSByte(v) => K::BSByte(v),
        K::String(s) => K::String(s),
        K::Bool(b) => K::Bool(b),
        K::Float(v) => K::Float(v),
        K::SFloat(v) => K::SFloat(v),
        K::Character(c) => K::Character(c),
        K::Null => K::Null,
        K::Identifier(name) => K::Identifier(name),
        K::ResolvedIdentifier { name, depth, slot } => {
            K::ResolvedIdentifier { name, depth, slot }
        }
        K::Binary { left, operator, right } => {
            let left = deep_clone_expr(ctx, left);
            let right = deep_clone_expr(ctx, right);
            K::Binary { left, operator, right }
        }
        K::Unary { operator, operand } => {
            let operand = deep_clone_expr(ctx, operand);
            K::Unary { operator, operand }
        }
        K::Grouping(inner) => K::Grouping(deep_clone_expr(ctx, inner)),
        K::ArrayLiteral(items) => {
            K::ArrayLiteral(items.into_iter().map(|i| deep_clone_expr(ctx, i)).collect())
        }
        K::MapLiteral(pairs) => K::MapLiteral(
            pairs
                .into_iter()
                .map(|(k, v)| (deep_clone_expr(ctx, k), deep_clone_expr(ctx, v)))
                .collect(),
        ),
        K::SetLiteral(items) => {
            K::SetLiteral(items.into_iter().map(|i| deep_clone_expr(ctx, i)).collect())
        }
        K::TupleLiteral(items) => {
            K::TupleLiteral(items.into_iter().map(|i| deep_clone_expr(ctx, i)).collect())
        }
        K::Assign { name, value } => {
            let value = deep_clone_expr(ctx, value);
            K::Assign { name, value }
        }
        K::ResolvedAssign { name, depth, slot, value } => {
            let value = deep_clone_expr(ctx, value);
            K::ResolvedAssign { name, depth, slot, value }
        }
        K::Call { path, args } => K::Call {
            path,
            args: args.into_iter().map(|a| deep_clone_expr(ctx, a)).collect(),
        },
        K::CallExpr { callee, args } => {
            let callee = deep_clone_expr(ctx, callee);
            K::CallExpr {
                callee,
                args: args.into_iter().map(|a| deep_clone_expr(ctx, a)).collect(),
            }
        }
        K::MethodCall { caller, method, args } => {
            let caller = deep_clone_expr(ctx, caller);
            K::MethodCall {
                caller,
                method,
                args: args.into_iter().map(|a| deep_clone_expr(ctx, a)).collect(),
            }
        }
        K::Index { target, index } => {
            let target = deep_clone_expr(ctx, target);
            let index = deep_clone_expr(ctx, index);
            K::Index { target, index }
        }
        K::IndexAssign { target, index, value } => {
            let target = deep_clone_expr(ctx, target);
            let index = deep_clone_expr(ctx, index);
            let value = deep_clone_expr(ctx, value);
            K::IndexAssign { target, index, value }
        }
        K::Cast { value, target_type } => {
            let value = deep_clone_expr(ctx, value);
            K::Cast { value, target_type }
        }
        K::Is { value, target_type } => {
            let value = deep_clone_expr(ctx, value);
            K::Is { value, target_type }
        }
        K::ErrorLiteral(inner) => K::ErrorLiteral(deep_clone_expr(ctx, inner)),
        K::OkLiteral(inner) => K::OkLiteral(deep_clone_expr(ctx, inner)),
        K::ErrLiteral(inner) => K::ErrLiteral(deep_clone_expr(ctx, inner)),
        K::Propagate(inner) => K::Propagate(deep_clone_expr(ctx, inner)),
        K::StructLiteral { name, fields } => K::StructLiteral {
            name,
            fields: fields
                .into_iter()
                .map(|(f, v)| (f, deep_clone_expr(ctx, v)))
                .collect(),
        },
        K::FieldAccess { target, field } => {
            let target = deep_clone_expr(ctx, target);
            K::FieldAccess { target, field }
        }
        K::FieldAssign { target, field, value } => {
            let target = deep_clone_expr(ctx, target);
            let value = deep_clone_expr(ctx, value);
            K::FieldAssign { target, field, value }
        }
        K::EnumVariant { enum_name, variant } => K::EnumVariant { enum_name, variant },
        // Lambdas and closures capture by scope: cloning their bodies here
        // would detach them from the resolution context. Contract
        // conditions containing lambdas are rejected by the checker
        // instead (functions are compared by identity, never by value).
        K::Lambda { .. } | K::ResolvedLambda { .. } => {
            return id;
        }
    };
    ctx.arena.alloc_expr(cloned, span)
}

/// Transforms `return` statements anywhere in `stmts` except inside nested
/// function or lambda boundaries (their own contracts desugar separately).
/// Never descends into expressions, so lambdas in expression position are
/// untouched by construction.
fn transform_returns(ctx: &mut Ctx, stmts: Vec<Statement>, ensures: &[ContractClause]) -> Vec<Statement> {
    stmts
        .into_iter()
        .flat_map(|s| transform_statement(ctx, s, ensures))
        .collect()
}

fn transform_branch(ctx: &mut Ctx, branch: Statement, ensures: &[ContractClause]) -> Statement {
    let span = branch.span;
    match branch.kind {
        StatementKind::ConditionalBranch { condition, body, .. } => stmt(
            StatementKind::ConditionalBranch {
                condition,
                body: transform_returns(ctx, body, ensures),
                needs_scope: false,
            },
            span,
        ),
        other => stmt(other, span),
    }
}

fn transform_statement(ctx: &mut Ctx, s: Statement, ensures: &[ContractClause]) -> Vec<Statement> {
    let span = s.span;
    match s.kind {
        StatementKind::Return(Some(value)) => ctx.wrap_return(value, ensures),
        StatementKind::Return(None) => {
            let null = ctx.expr(ExpressionKind::Null);
            ctx.wrap_return(null, ensures)
        }
        StatementKind::Conditional { if_branch, else_branch } => vec![stmt(
            StatementKind::Conditional {
                if_branch: Box::new(transform_branch(ctx, *if_branch, ensures)),
                else_branch: else_branch.map(|b| Box::new(transform_branch(ctx, *b, ensures))),
            },
            span,
        )],
        StatementKind::While { condition, body } => vec![stmt(
            StatementKind::While {
                condition,
                body: transform_returns(ctx, body, ensures),
            },
            span,
        )],
        StatementKind::Loop(body) => {
            vec![stmt(StatementKind::Loop(transform_returns(ctx, body, ensures)), span)]
        }
        StatementKind::For {
            initializer,
            condition,
            increment,
            body,
        } => vec![stmt(
            StatementKind::For {
                initializer,
                condition,
                increment,
                body: transform_returns(ctx, body, ensures),
            },
            span,
        )],
        StatementKind::ForRange {
            variable,
            range,
            body,
        } => vec![stmt(
            StatementKind::ForRange {
                variable,
                range,
                body: transform_returns(ctx, body, ensures),
            },
            span,
        )],
        StatementKind::ForEach {
            variable,
            iterable,
            body,
        } => vec![stmt(
            StatementKind::ForEach {
                variable,
                iterable,
                body: transform_returns(ctx, body, ensures),
            },
            span,
        )],
        StatementKind::Match { value, arms } => vec![stmt(
            StatementKind::Match {
                value,
                arms: arms
                    .into_iter()
                    .map(|(pat, body)| (pat, transform_returns(ctx, body, ensures)))
                    .collect(),
            },
            span,
        )],
        // Nested functions, lambdas (inside expressions, never descended
        // into), and all other statements pass through untouched.
        other => vec![stmt(other, span)],
    }
}

/// Entry point: desugars contracts on one function body. No-op without
/// contracts. Unannotated (`Null`) returns take the bare (abort) form.
pub fn desugar_fn_contracts(
    arena: &mut Ast,
    params: &[Param],
    return_type: &TypeAnnotation,
    body: Vec<Statement>,
    requires: &[ContractClause],
    ensures: &[ContractClause],
    span: Span,
) -> Vec<Statement> {
    let has_contracts = !requires.is_empty()
        || !ensures.is_empty()
        || params.iter().any(|p| p.refinement.is_some());
    if !has_contracts {
        return body;
    }
    let is_result = matches!(
        return_type,
        TypeAnnotation::Result(_) | TypeAnnotation::CResult(_)
    );
    let mut ctx = Ctx {
        arena,
        span,
        is_result,
        temp_counter: 0,
    };
    let mut out = Vec::new();
    // Entry checks: parameter refinements first, then `requires`, in order.
    for param in params {
        if let Some(refinement) = &param.refinement {
            let pred = ctx.refine_predicate(&param.param_name, refinement.op, &refinement.operand);
            let msg = ctx.str_lit(&format!(
                "refinement failed: {} {} {}",
                param.param_name,
                refine_op_str(refinement.op),
                operand_str(&refinement.operand)
            ));
            out.push(ctx.guard(pred, msg));
        }
    }
    for clause in requires {
        let msg = ctx.msg_expr(clause, "requires violated");
        out.push(ctx.guard(clause.condition, msg));
    }
    // `ensures` wraps every return; a trailing bare expression counts as
    // one. Only pop when the last statement really is a trailing
    // expression - otherwise the wrapped `return` would reorder ahead of
    // its own temporaries.
    let mut body = if ensures.is_empty() {
        body
    } else {
        transform_returns(&mut ctx, body, ensures)
    };
    let trailing = !ensures.is_empty()
        && matches!(
            body.last().map(|s| &s.kind),
            Some(StatementKind::Expression(_))
        );
    if trailing
        && let Some(last) = body.pop()
        && let StatementKind::Expression(value) = last.kind
    {
        out.extend(ctx.wrap_return(value, ensures));
    }
    out.extend(body);
    out
}
