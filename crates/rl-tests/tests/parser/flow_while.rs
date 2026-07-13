use rl_ast::nodes::ExpressionKind;
use rl_utils::span::Span;

use crate::{assert_while, common};

#[test]
fn while_loop() {
    assert_while!(
        "while (true) {0}",
        condition: ExpressionKind::Bool(true), Span::new(7, 11), grouped: Span::new(6, 12),
        body_expr: ExpressionKind::Integer(0), Span::new(14, 15), Span::new(14, 15),
        span: Span::new(0, 16),
    );
}
