/// Span::new shorthand
#[macro_export]
macro_rules! spnn {
    ($ss:expr, $se:expr) => {
        $crate::utils::span::Span::new($ss, $se)
    };
}

/// The newline/statement-separator filler (Expression(Integer(0)))
#[macro_export]
macro_rules! nl {
    ($ast:expr, $s:expr, $e:expr) => {{
        let __ast = &mut *$ast;
        let val = ast.alloc_expr(
            $crate::ast::nodes::ExpressionKind::Integer(0),
            $crate::utils::span::Span::new($s, $e),
        );
        __ast.alloc_stmt(
            $crate::ast::statements::StatementKind::Expression(__val),
            $crate::utils::span::Span::new($s, $e),
        )
    }};
}

/// Identifier expression: id!(ast, "x", 132, 133)
#[macro_export]
macro_rules! id {
    ($ast:expr, $name:expr, $s:expr, $e:expr) => {
        $ast.alloc_expr(
            $crate::ast::nodes::ExpressionKind::Identifier($name.to_string()),
            $crate::utils::span::Span::new($s, $e),
        )
    };
}

/// Integer literal expression: int!(ast, 7, 329, 330)
#[macro_export]
macro_rules! int {
    ($ast:expr, $val:expr, $s:expr, $e:expr) => {
        $ast.alloc_expr(
            $crate::ast::nodes::ExpressionKind::Integer($val),
            $crate::utils::span::Span::new($s, $e),
        )
    };
}

/// Float literal expression: flt!(ast, 1.0, 281, 284)
#[macro_export]
macro_rules! flt {
    ($ast:expr, $val:expr, $s:expr, $e:expr) => {
        $ast.alloc_expr(
            $crate::ast::nodes::ExpressionKind::Float($val),
            $crate::utils::span::Span::new($s, $e),
        )
    };
}

/// Call expression: call!(ast, ["pow"], [pi_id, pi_id], 111, 122)
#[macro_export]
macro_rules! call {
    ($ast:expr, [$($p:expr),+ $(,)?], [$($arg:expr),* $(,)?], $s:expr, $e:expr) => {
        $ast.alloc_expr(
            $crate::ast::nodes::ExpressionKind::Call {
                path: vec![$($p.to_string()),+],
                args: vec![$($arg),*],
            },
            $crate::utils::span::Span::new($s, $e),
        )
    };
}

/// Binary expression: bin!(ast, left_id, TokenType::Plus, right_id, 170, 176)
#[macro_export]
macro_rules! bin {
    ($ast:expr, $left:expr, $op:expr, $right:expr, $s:expr, $e:expr) => {
        $ast.alloc_expr(
            $crate::ast::nodes::ExpressionKind::Binary {
                left: $left,
                operator: $op,
                right: $right,
            },
            $crate::utils::span::Span::new($s, $e),
        )
    };
}

/// Unary expression: un!(ast, TokenType::Bang, operand_id, 306, 315)
#[macro_export]
macro_rules! un {
    ($ast:expr, $op:expr, $operand:expr, $s:expr, $e:expr) => {
        $ast.alloc_expr(
            $crate::ast::nodes::ExpressionKind::Unary {
                operator: $op,
                operand: $operand,
            },
            $crate::utils::span::Span::new($s, $e),
        )
    };
}

/// Import statement: import!(ast, ["println"], ["std", "io"], 0, 24)
#[macro_export]
macro_rules! import {
    ($ast:expr, [$($name:expr),+ $(,)?], [$($p:expr),+ $(,)?], $s:expr, $e:expr) => {
        $ast.alloc_stmt(
            $crate::ast::statements::StatementKind::Import {
                names: vec![$($name.to_string()),+],
                path: vec![$($p.to_string()),+],
            },
            $crate::utils::span::Span::new($s, $e),
        )
    };
}

/// Variable declaration: vardecl!(ast, "x", TypeAnnotation::Float, value_id, 97, 122)
#[macro_export]
macro_rules! vardecl {
    ($ast:expr, $name:expr, $ty:expr, $val:expr, $s:expr, $e:expr) => {
        $ast.alloc_stmt(
            $crate::ast::statements::StatementKind::VariableDeclaration {
                name: $name.to_string(),
                type_annotation: $ty,
                value: $val,
            },
            $crate::utils::span::Span::new($s, $e),
        )
    };
}

/// Constant declaration: constdecl!(ast, "pi", TypeAnnotation::CFloat, value_id, 75, 96)
#[macro_export]
macro_rules! constdecl {
    ($ast:expr, $name:expr, $ty:expr, $val:expr, $s:expr, $e:expr) => {
        $ast.alloc_stmt(
            $crate::ast::statements::StatementKind::ConstantDeclaration {
                name: $name.to_string(),
                type_annotation: $ty,
                value: $val,
            },
            $crate::utils::span::Span::new($s, $e),
        )
    };
}

/// Bare expression statement: exprstmt!(ast, call_id, 124, 137)
#[macro_export]
macro_rules! exprstmt {
    ($ast:expr, $expr:expr, $s:expr, $e:expr) => {
        $ast.alloc_stmt(
            $crate::ast::statements::StatementKind::Expression($expr),
            $crate::utils::span::Span::new($s, $e),
        )
    };
}

/// Index expression: idx!(ast, target_id, index_id, 196, 202)
#[macro_export]
macro_rules! idx {
    ($ast:expr, $target:expr, $index:expr, $s:expr, $e:expr) => {
        $ast.alloc_expr(
            $crate::ast::nodes::ExpressionKind::Index {
                target: $target,
                index: $index,
            },
            $crate::utils::span::Span::new($s, $e),
        )
    };
}

/// Grouping/parenthesized expression: grp!(ast, inner_id, 231, 238)
#[macro_export]
macro_rules! grp {
    ($ast:expr, $inner:expr, $s:expr, $e:expr) => {
        $ast.alloc_expr(
            $crate::ast::nodes::ExpressionKind::Grouping($inner),
            $crate::utils::span::Span::new($s, $e),
        )
    };
}

/// Bare assign expression (no statement wrapper) - for for-loop increments etc.
#[macro_export]
macro_rules! assignexpr {
    ($ast:expr, $name:expr, $val:expr, $s:expr, $e:expr) => {
        $ast.alloc_expr(
            $crate::ast::nodes::ExpressionKind::Assign {
                name: $name.to_string(),
                value: $val,
            },
            $crate::utils::span::Span::new($s, $e),
        )
    };
}

/// Array declaration: arraydecl!(ast, "ops", TypeAnnotation::Fn, [id1, id2], 91, 131)
#[macro_export]
macro_rules! arraydecl {
    ($ast:expr, $name:expr, $ty:expr, [$($v:expr),* $(,)?], $s:expr, $e:expr) => {
        $ast.alloc_stmt(
            $crate::ast::statements::StatementKind::Array {
                name: $name.to_string(),
                type_annotation: $ty,
                value: vec![$($v),*],
            },
            $crate::utils::span::Span::new($s, $e),
        )
    };
}

/// Assignment as an expression statement: assign!(ast, "lg", value_id, 417, 426)
#[macro_export]
macro_rules! assign {
    ($ast:expr, $name:expr, $val:expr, $s:expr, $e:expr) => {{
        let __ast = &mut *$ast;
        let val = $crate::assignexpr!(ast, $name, $val, $s, $e);
        $crate::exprstmt!(ast, val, $s, $e)
    }};
}
