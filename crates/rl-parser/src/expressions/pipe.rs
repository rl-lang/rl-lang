use crate::parser_logic::Parser;
use rl_ast::{ExprId, nodes::ExpressionKind};
use rl_lexer::tokentypes::TokenType;
use rl_utils::errors::Error;

impl Parser {
    /// Parses the `|>` pipe operator.
    ///
    /// - `a |> f(args)` desugars to `a.f(args)` - the left-hand side becomes the
    ///   receiver of the method call.
    /// - `a |> obj.method(args)` desugars to `obj.method(a, args)` - the LHS is
    ///   injected as the first argument.
    pub fn parse_pipe(&mut self) -> Result<ExprId, Error> {
        let mut left = self.parse_logical()?;
        while self.match_type(&[TokenType::Pipe]) {
            while self.match_type(&[TokenType::Newline]) {}
            let right = self.parse_primary()?;
            let left_id = self.ast_arena.exprs.get(left);
            let right_id = self.ast_arena.exprs.get(right);
            let span = left_id.span.join(right_id.span);

            match &right_id.kind {
                // a |> f(args)  ->  a.f(args)
                ExpressionKind::Call { path, args } => {
                    let method = path.clone();
                    let args = args.clone();
                    left = self.ast_arena.alloc_expr(
                        ExpressionKind::MethodCall {
                            caller: left,
                            method,
                            args,
                        },
                        span,
                    );
                }
                // a |> obj.method(args)  ->  obj.method(a, args)
                ExpressionKind::MethodCall {
                    caller,
                    method,
                    args,
                } => {
                    let caller = *caller;
                    let method = method.clone();
                    let mut new_args = vec![left];
                    new_args.extend(args);
                    left = self.ast_arena.alloc_expr(
                        ExpressionKind::MethodCall {
                            caller,
                            method,
                            args: new_args,
                        },
                        span,
                    );
                }
                _ => {
                    return Err(self.err(
                        "right-hand side of `|>` must be a function or method call",
                        span,
                    ));
                }
            }
        }
        Ok(left)
    }
}
