/// Wraps a StatementKind + span into a Statement
#[macro_export]
macro_rules! sn {
    ($e:expr, $s:expr) => {
        $crate::ast::statements::Statement::new($e, $s)
    };
}

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
    ($s:expr, $e:expr) => {
        $crate::sn!(
            $crate::ast::statements::StatementKind::Expression(
                $crate::ast::nodes::Expression::new(
                    $crate::ast::nodes::ExpressionKind::Integer(0),
                    $crate::utils::span::Span::new($s, $e),
                )
            ),
            $crate::utils::span::Span::new($s, $e)
        )
    };
}

/// Generic expression builder: ex!(kind, start, end)
#[macro_export]
macro_rules! ex {
    ($kind:expr, $s:expr, $e:expr) => {
        $crate::ast::nodes::Expression::new($kind, $crate::utils::span::Span::new($s, $e))
    };
}

/// Identifier expression: id!("x", 132, 133)
#[macro_export]
macro_rules! id {
    ($name:expr, $s:expr, $e:expr) => {
        $crate::ast::nodes::Expression::new(
            $crate::ast::nodes::ExpressionKind::Identifier($name.to_string()),
            $crate::utils::span::Span::new($s, $e),
        )
    };
}

/// Integer literal expression: int!(7, 329, 330)
#[macro_export]
macro_rules! int {
    ($val:expr, $s:expr, $e:expr) => {
        $crate::ast::nodes::Expression::new(
            $crate::ast::nodes::ExpressionKind::Integer($val),
            $crate::utils::span::Span::new($s, $e),
        )
    };
}

/// Float literal expression: flt!(1.0, 281, 284)
#[macro_export]
macro_rules! flt {
    ($val:expr, $s:expr, $e:expr) => {
        $crate::ast::nodes::Expression::new(
            $crate::ast::nodes::ExpressionKind::Float($val),
            $crate::utils::span::Span::new($s, $e),
        )
    };
}

/// Call expression: call!(["pow"], [pi_expr, pi_expr], 111, 122)
#[macro_export]
macro_rules! call {
    ([$($p:expr),+ $(,)?], [$($arg:expr),* $(,)?], $s:expr, $e:expr) => {
        $crate::ast::nodes::Expression::new(
            $crate::ast::nodes::ExpressionKind::Call {
                path: vec![$($p.to_string()),+],
                args: vec![$($arg),*],
            },
            $crate::utils::span::Span::new($s, $e),
        )
    };
}

/// Binary expression: bin!(left_expr, TokenType::Plus, right_expr, 170, 176)
#[macro_export]
macro_rules! bin {
    ($left:expr, $op:expr, $right:expr, $s:expr, $e:expr) => {
        $crate::ast::nodes::Expression::new(
            $crate::ast::nodes::ExpressionKind::Binary {
                left: Box::new($left),
                operator: $op,
                right: Box::new($right),
            },
            $crate::utils::span::Span::new($s, $e),
        )
    };
}

/// Unary expression: un!(TokenType::Bang, operand_expr, 306, 315)
#[macro_export]
macro_rules! un {
    ($op:expr, $operand:expr, $s:expr, $e:expr) => {
        $crate::ast::nodes::Expression::new(
            $crate::ast::nodes::ExpressionKind::Unary {
                operator: $op,
                operand: Box::new($operand),
            },
            $crate::utils::span::Span::new($s, $e),
        )
    };
}

/// Import statement: import!(["println"], ["std", "io"], 0, 24)
#[macro_export]
macro_rules! import {
    ([$($name:expr),+ $(,)?], [$($p:expr),+ $(,)?], $s:expr, $e:expr) => {
        $crate::sn!(
            $crate::ast::statements::StatementKind::Import {
                names: vec![$($name.to_string()),+],
                path: vec![$($p.to_string()),+],
            },
            $crate::utils::span::Span::new($s, $e)
        )
    };
}

/// Variable declaration: vardecl!("x", TypeAnnotation::Float, value_expr, 97, 122)
#[macro_export]
macro_rules! vardecl {
    ($name:expr, $ty:expr, $val:expr, $s:expr, $e:expr) => {
        $crate::sn!(
            $crate::ast::statements::StatementKind::VariableDeclaration {
                name: $name.to_string(),
                type_annotation: $ty,
                value: $val,
            },
            $crate::utils::span::Span::new($s, $e)
        )
    };
}

/// Constant declaration: constdecl!("pi", TypeAnnotation::CFloat, value_expr, 75, 96)
#[macro_export]
macro_rules! constdecl {
    ($name:expr, $ty:expr, $val:expr, $s:expr, $e:expr) => {
        $crate::sn!(
            $crate::ast::statements::StatementKind::ConstantDeclaration {
                name: $name.to_string(),
                type_annotation: $ty,
                value: $val,
            },
            $crate::utils::span::Span::new($s, $e)
        )
    };
}

/// Bare expression statement: exprstmt!(call_expr, 124, 137)
#[macro_export]
macro_rules! exprstmt {
    ($expr:expr, $s:expr, $e:expr) => {
        $crate::sn!(
            $crate::ast::statements::StatementKind::Expression($expr),
            $crate::utils::span::Span::new($s, $e)
        )
    };
}

/// Index expression: idx!(target_expr, index_expr, 196, 202)
#[macro_export]
macro_rules! idx {
    ($target:expr, $index:expr, $s:expr, $e:expr) => {
        $crate::ast::nodes::Expression::new(
            $crate::ast::nodes::ExpressionKind::Index {
                target: Box::new($target),
                index: Box::new($index),
            },
            $crate::utils::span::Span::new($s, $e),
        )
    };
}

/// Grouping/parenthesized expression: grp!(inner_expr, 231, 238)
#[macro_export]
macro_rules! grp {
    ($inner:expr, $s:expr, $e:expr) => {
        $crate::ast::nodes::Expression::new(
            $crate::ast::nodes::ExpressionKind::Grouping(Box::new($inner)),
            $crate::utils::span::Span::new($s, $e),
        )
    };
}

/// Bare assign expression (no statement wrapper) — for `for`-loop increments etc.
#[macro_export]
macro_rules! assignexpr {
    ($name:expr, $val:expr, $s:expr, $e:expr) => {
        $crate::ast::nodes::Expression::new(
            $crate::ast::nodes::ExpressionKind::Assign {
                name: $name.to_string(),
                value: Box::new($val),
            },
            $crate::utils::span::Span::new($s, $e),
        )
    };
}

/// Array declaration: arraydecl!("ops", TypeAnnotation::Fn, [expr1, expr2], 91, 131)
#[macro_export]
macro_rules! arraydecl {
    ($name:expr, $ty:expr, [$($v:expr),* $(,)?], $s:expr, $e:expr) => {
        $crate::sn!(
            $crate::ast::statements::StatementKind::Array {
                name: $name.to_string(),
                type_annotation: $ty,
                value: vec![$($v),*],
            },
            $crate::utils::span::Span::new($s, $e)
        )
    };
}

/// Assignment as an expression statement: assign!("lg", value_expr, 417, 426)
#[macro_export]
macro_rules! assign {
    ($name:expr, $val:expr, $s:expr, $e:expr) => {
        $crate::exprstmt!($crate::assignexpr!($name, $val, $s, $e), $s, $e)
    };
}
