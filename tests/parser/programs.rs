use rl_lang::{
    ast::{
        nodes::{Expression, ExpressionKind},
        statements::{Statement, StatementKind, TypeAnnotation},
    },
    lexer::tokentypes::TokenType,
    utils::span::Span,
};

use crate::common;

#[test]
fn integration_math_print() {
    let statements = common::parse(
        "get println from std::display
get pow from std::math
get std::math::consts::PI

CONST float pi = PI()
dec float x = pow(pi, pi)

println(x, y)",
    );
    let expected = vec![
        Statement::new(
            StatementKind::Import {
                names: vec!["println".to_string()],
                path: vec!["std".to_string(), "display".to_string()],
            },
            Span::new(0, 29),
        ),
        Statement::new(
            StatementKind::Expression(Expression::new(
                ExpressionKind::Integer(0),
                Span::new(29, 30),
            )),
            Span::new(29, 30),
        ),
        Statement::new(
            StatementKind::Import {
                names: vec!["pow".to_string()],
                path: vec!["std".to_string(), "math".to_string()],
            },
            Span::new(30, 52),
        ),
        Statement::new(
            StatementKind::Expression(Expression::new(
                ExpressionKind::Integer(0),
                Span::new(52, 53),
            )),
            Span::new(52, 53),
        ),
        Statement::new(
            StatementKind::Import {
                names: vec!["PI".to_string()],
                path: vec!["std".to_string(), "math".to_string(), "consts".to_string()],
            },
            Span::new(53, 78),
        ),
        Statement::new(
            StatementKind::Expression(Expression::new(
                ExpressionKind::Integer(0),
                Span::new(78, 79),
            )),
            Span::new(78, 79),
        ),
        Statement::new(
            StatementKind::Expression(Expression::new(
                ExpressionKind::Integer(0),
                Span::new(79, 80),
            )),
            Span::new(79, 80),
        ),
        Statement::new(
            StatementKind::ConstantDeclaration {
                name: "pi".to_string(),
                type_annotation: TypeAnnotation::CFloat,
                value: Expression::new(
                    ExpressionKind::Call {
                        path: vec!["PI".to_string()],
                        args: vec![],
                    },
                    Span::new(97, 101),
                ),
            },
            Span::new(80, 101),
        ),
        Statement::new(
            StatementKind::Expression(Expression::new(
                ExpressionKind::Integer(0),
                Span::new(101, 102),
            )),
            Span::new(101, 102),
        ),
        Statement::new(
            StatementKind::VariableDeclaration {
                name: "x".to_string(),
                type_annotation: TypeAnnotation::Float,
                value: Expression::new(
                    ExpressionKind::Call {
                        path: vec!["pow".to_string()],
                        args: vec![
                            Expression::new(
                                ExpressionKind::Identifier("pi".to_string()),
                                Span::new(120, 122),
                            ),
                            Expression::new(
                                ExpressionKind::Identifier("pi".to_string()),
                                Span::new(124, 126),
                            ),
                        ],
                    },
                    Span::new(116, 127),
                ),
            },
            Span::new(102, 127),
        ),
        Statement::new(
            StatementKind::Expression(Expression::new(
                ExpressionKind::Integer(0),
                Span::new(127, 128),
            )),
            Span::new(127, 128),
        ),
        Statement::new(
            StatementKind::Expression(Expression::new(
                ExpressionKind::Integer(0),
                Span::new(128, 129),
            )),
            Span::new(128, 129),
        ),
        Statement::new(
            StatementKind::Expression(Expression::new(
                ExpressionKind::Call {
                    path: vec!["println".to_string()],
                    args: vec![
                        Expression::new(
                            ExpressionKind::Identifier("x".to_string()),
                            Span::new(137, 138),
                        ),
                        Expression::new(
                            ExpressionKind::Identifier("y".to_string()),
                            Span::new(140, 141),
                        ),
                    ],
                },
                Span::new(129, 142),
            )),
            Span::new(129, 142),
        ),
    ];
    assert_eq!(statements, expected);
}

#[test]
fn integration_geometry() {
    let statements = common::parse(
        "get println from std::display
get sin, cos, hypot, is_prime, log2, factorial from std::math
get PI, TAU, PHI from std::math::consts

dec float angle = PI()
angle *= PHI()

dec float opp = sin(angle)
dec float adj = cos(angle)
dec float hyp = hypot(opp, adj)

dec bool is_right = hyp == 1.0
dec bool not_right = !is_right

dec int n = 7
dec bool prime = is_prime(n)
dec float fact = factorial(n)
dec float lg = log2(fact)

lg -= 1.0

dec bool big = lg > TAU()

println(hyp, is_right, prime, big)",
    );
    let expected = vec![
        Statement::new(
            StatementKind::Import {
                names: vec!["println".to_string()],
                path: vec!["std".to_string(), "display".to_string()],
            },
            Span::new(0, 29),
        ),
        Statement::new(
            StatementKind::Expression(Expression::new(
                ExpressionKind::Integer(0),
                Span::new(29, 30),
            )),
            Span::new(29, 30),
        ),
        Statement::new(
            StatementKind::Import {
                names: vec![
                    "sin".to_string(),
                    "cos".to_string(),
                    "hypot".to_string(),
                    "is_prime".to_string(),
                    "log2".to_string(),
                    "factorial".to_string(),
                ],
                path: vec!["std".to_string(), "math".to_string()],
            },
            Span::new(30, 91),
        ),
        Statement::new(
            StatementKind::Expression(Expression::new(
                ExpressionKind::Integer(0),
                Span::new(91, 92),
            )),
            Span::new(91, 92),
        ),
        Statement::new(
            StatementKind::Import {
                names: vec!["PI".to_string(), "TAU".to_string(), "PHI".to_string()],
                path: vec!["std".to_string(), "math".to_string(), "consts".to_string()],
            },
            Span::new(92, 131),
        ),
        Statement::new(
            StatementKind::Expression(Expression::new(
                ExpressionKind::Integer(0),
                Span::new(131, 132),
            )),
            Span::new(131, 132),
        ),
        Statement::new(
            StatementKind::Expression(Expression::new(
                ExpressionKind::Integer(0),
                Span::new(132, 133),
            )),
            Span::new(132, 133),
        ),
        Statement::new(
            StatementKind::VariableDeclaration {
                name: "angle".to_string(),
                type_annotation: TypeAnnotation::Float,
                value: Expression::new(
                    ExpressionKind::Call {
                        path: vec!["PI".to_string()],
                        args: vec![],
                    },
                    Span::new(151, 155),
                ),
            },
            Span::new(133, 155),
        ),
        Statement::new(
            StatementKind::Expression(Expression::new(
                ExpressionKind::Integer(0),
                Span::new(155, 156),
            )),
            Span::new(155, 156),
        ),
        Statement::new(
            StatementKind::Expression(Expression::new(
                ExpressionKind::Assign {
                    name: "angle".to_string(),
                    value: Box::new(Expression::new(
                        ExpressionKind::Binary {
                            left: Box::new(Expression::new(
                                ExpressionKind::Identifier("angle".to_string()),
                                Span::new(156, 161),
                            )),
                            operator: TokenType::Star,
                            right: Box::new(Expression::new(
                                ExpressionKind::Call {
                                    path: vec!["PHI".to_string()],
                                    args: vec![],
                                },
                                Span::new(165, 170),
                            )),
                        },
                        Span::new(156, 170),
                    )),
                },
                Span::new(156, 170),
            )),
            Span::new(156, 170),
        ),
        Statement::new(
            StatementKind::Expression(Expression::new(
                ExpressionKind::Integer(0),
                Span::new(170, 171),
            )),
            Span::new(170, 171),
        ),
        Statement::new(
            StatementKind::Expression(Expression::new(
                ExpressionKind::Integer(0),
                Span::new(171, 172),
            )),
            Span::new(171, 172),
        ),
        Statement::new(
            StatementKind::VariableDeclaration {
                name: "opp".to_string(),
                type_annotation: TypeAnnotation::Float,
                value: Expression::new(
                    ExpressionKind::Call {
                        path: vec!["sin".to_string()],
                        args: vec![Expression::new(
                            ExpressionKind::Identifier("angle".to_string()),
                            Span::new(192, 197),
                        )],
                    },
                    Span::new(188, 198),
                ),
            },
            Span::new(172, 198),
        ),
        Statement::new(
            StatementKind::Expression(Expression::new(
                ExpressionKind::Integer(0),
                Span::new(198, 199),
            )),
            Span::new(198, 199),
        ),
        Statement::new(
            StatementKind::VariableDeclaration {
                name: "adj".to_string(),
                type_annotation: TypeAnnotation::Float,
                value: Expression::new(
                    ExpressionKind::Call {
                        path: vec!["cos".to_string()],
                        args: vec![Expression::new(
                            ExpressionKind::Identifier("angle".to_string()),
                            Span::new(219, 224),
                        )],
                    },
                    Span::new(215, 225),
                ),
            },
            Span::new(199, 225),
        ),
        Statement::new(
            StatementKind::Expression(Expression::new(
                ExpressionKind::Integer(0),
                Span::new(225, 226),
            )),
            Span::new(225, 226),
        ),
        Statement::new(
            StatementKind::VariableDeclaration {
                name: "hyp".to_string(),
                type_annotation: TypeAnnotation::Float,
                value: Expression::new(
                    ExpressionKind::Call {
                        path: vec!["hypot".to_string()],
                        args: vec![
                            Expression::new(
                                ExpressionKind::Identifier("opp".to_string()),
                                Span::new(248, 251),
                            ),
                            Expression::new(
                                ExpressionKind::Identifier("adj".to_string()),
                                Span::new(253, 256),
                            ),
                        ],
                    },
                    Span::new(242, 257),
                ),
            },
            Span::new(226, 257),
        ),
        Statement::new(
            StatementKind::Expression(Expression::new(
                ExpressionKind::Integer(0),
                Span::new(257, 258),
            )),
            Span::new(257, 258),
        ),
        Statement::new(
            StatementKind::Expression(Expression::new(
                ExpressionKind::Integer(0),
                Span::new(258, 259),
            )),
            Span::new(258, 259),
        ),
        Statement::new(
            StatementKind::VariableDeclaration {
                name: "is_right".to_string(),
                type_annotation: TypeAnnotation::Bool,
                value: Expression::new(
                    ExpressionKind::Binary {
                        left: Box::new(Expression::new(
                            ExpressionKind::Identifier("hyp".to_string()),
                            Span::new(279, 282),
                        )),
                        operator: TokenType::Compare,
                        right: Box::new(Expression::new(
                            ExpressionKind::Float(1.0),
                            Span::new(286, 289),
                        )),
                    },
                    Span::new(279, 289),
                ),
            },
            Span::new(259, 289),
        ),
        Statement::new(
            StatementKind::Expression(Expression::new(
                ExpressionKind::Integer(0),
                Span::new(289, 290),
            )),
            Span::new(289, 290),
        ),
        Statement::new(
            StatementKind::VariableDeclaration {
                name: "not_right".to_string(),
                type_annotation: TypeAnnotation::Bool,
                value: Expression::new(
                    ExpressionKind::Unary {
                        operator: TokenType::Bang,
                        operand: Box::new(Expression::new(
                            ExpressionKind::Identifier("is_right".to_string()),
                            Span::new(312, 320),
                        )),
                    },
                    Span::new(311, 320),
                ),
            },
            Span::new(290, 320),
        ),
        Statement::new(
            StatementKind::Expression(Expression::new(
                ExpressionKind::Integer(0),
                Span::new(320, 321),
            )),
            Span::new(320, 321),
        ),
        Statement::new(
            StatementKind::Expression(Expression::new(
                ExpressionKind::Integer(0),
                Span::new(321, 322),
            )),
            Span::new(321, 322),
        ),
        Statement::new(
            StatementKind::VariableDeclaration {
                name: "n".to_string(),
                type_annotation: TypeAnnotation::Int,
                value: Expression::new(ExpressionKind::Integer(7), Span::new(334, 335)),
            },
            Span::new(322, 335),
        ),
        Statement::new(
            StatementKind::Expression(Expression::new(
                ExpressionKind::Integer(0),
                Span::new(335, 336),
            )),
            Span::new(335, 336),
        ),
        Statement::new(
            StatementKind::VariableDeclaration {
                name: "prime".to_string(),
                type_annotation: TypeAnnotation::Bool,
                value: Expression::new(
                    ExpressionKind::Call {
                        path: vec!["is_prime".to_string()],
                        args: vec![Expression::new(
                            ExpressionKind::Identifier("n".to_string()),
                            Span::new(362, 363),
                        )],
                    },
                    Span::new(353, 364),
                ),
            },
            Span::new(336, 364),
        ),
        Statement::new(
            StatementKind::Expression(Expression::new(
                ExpressionKind::Integer(0),
                Span::new(364, 365),
            )),
            Span::new(364, 365),
        ),
        Statement::new(
            StatementKind::VariableDeclaration {
                name: "fact".to_string(),
                type_annotation: TypeAnnotation::Float,
                value: Expression::new(
                    ExpressionKind::Call {
                        path: vec!["factorial".to_string()],
                        args: vec![Expression::new(
                            ExpressionKind::Identifier("n".to_string()),
                            Span::new(392, 393),
                        )],
                    },
                    Span::new(382, 394),
                ),
            },
            Span::new(365, 394),
        ),
        Statement::new(
            StatementKind::Expression(Expression::new(
                ExpressionKind::Integer(0),
                Span::new(394, 395),
            )),
            Span::new(394, 395),
        ),
        Statement::new(
            StatementKind::VariableDeclaration {
                name: "lg".to_string(),
                type_annotation: TypeAnnotation::Float,
                value: Expression::new(
                    ExpressionKind::Call {
                        path: vec!["log2".to_string()],
                        args: vec![Expression::new(
                            ExpressionKind::Identifier("fact".to_string()),
                            Span::new(415, 419),
                        )],
                    },
                    Span::new(410, 420),
                ),
            },
            Span::new(395, 420),
        ),
        Statement::new(
            StatementKind::Expression(Expression::new(
                ExpressionKind::Integer(0),
                Span::new(420, 421),
            )),
            Span::new(420, 421),
        ),
        Statement::new(
            StatementKind::Expression(Expression::new(
                ExpressionKind::Integer(0),
                Span::new(421, 422),
            )),
            Span::new(421, 422),
        ),
        Statement::new(
            StatementKind::Expression(Expression::new(
                ExpressionKind::Assign {
                    name: "lg".to_string(),
                    value: Box::new(Expression::new(
                        ExpressionKind::Binary {
                            left: Box::new(Expression::new(
                                ExpressionKind::Identifier("lg".to_string()),
                                Span::new(422, 424),
                            )),
                            operator: TokenType::Minus,
                            right: Box::new(Expression::new(
                                ExpressionKind::Float(1.0),
                                Span::new(428, 431),
                            )),
                        },
                        Span::new(422, 431),
                    )),
                },
                Span::new(422, 431),
            )),
            Span::new(422, 431),
        ),
        Statement::new(
            StatementKind::Expression(Expression::new(
                ExpressionKind::Integer(0),
                Span::new(431, 432),
            )),
            Span::new(431, 432),
        ),
        Statement::new(
            StatementKind::Expression(Expression::new(
                ExpressionKind::Integer(0),
                Span::new(432, 433),
            )),
            Span::new(432, 433),
        ),
        Statement::new(
            StatementKind::VariableDeclaration {
                name: "big".to_string(),
                type_annotation: TypeAnnotation::Bool,
                value: Expression::new(
                    ExpressionKind::Binary {
                        left: Box::new(Expression::new(
                            ExpressionKind::Identifier("lg".to_string()),
                            Span::new(448, 450),
                        )),
                        operator: TokenType::Greater,
                        right: Box::new(Expression::new(
                            ExpressionKind::Call {
                                path: vec!["TAU".to_string()],
                                args: vec![],
                            },
                            Span::new(453, 458),
                        )),
                    },
                    Span::new(448, 458),
                ),
            },
            Span::new(433, 458),
        ),
        Statement::new(
            StatementKind::Expression(Expression::new(
                ExpressionKind::Integer(0),
                Span::new(458, 459),
            )),
            Span::new(458, 459),
        ),
        Statement::new(
            StatementKind::Expression(Expression::new(
                ExpressionKind::Integer(0),
                Span::new(459, 460),
            )),
            Span::new(459, 460),
        ),
        Statement::new(
            StatementKind::Expression(Expression::new(
                ExpressionKind::Call {
                    path: vec!["println".to_string()],
                    args: vec![
                        Expression::new(
                            ExpressionKind::Identifier("hyp".to_string()),
                            Span::new(468, 471),
                        ),
                        Expression::new(
                            ExpressionKind::Identifier("is_right".to_string()),
                            Span::new(473, 481),
                        ),
                        Expression::new(
                            ExpressionKind::Identifier("prime".to_string()),
                            Span::new(483, 488),
                        ),
                        Expression::new(
                            ExpressionKind::Identifier("big".to_string()),
                            Span::new(490, 493),
                        ),
                    ],
                },
                Span::new(460, 494),
            )),
            Span::new(460, 494),
        ),
    ];
    assert_eq!(statements, expected);
}

#[test]
fn integration_fn_array() {
    let statements = common::parse(
        "get println from std::display
get is_prime, factorial, fibonacci from std::math

dec int n = 10
dec arr[fn] ops = [factorial, fibonacci]

dec int i = 0
for [int i = 0, i < 2, i += 1] {
    dec fn op = ops[i]
    dec int j = 1
    while (j < n) {
        dec int result = op(j)
        if (is_prime(result)) {
            println(result)
            j += 1
        } else {
            break
        }
    }
    i += 1
}",
    );
    let expected = vec![
        Statement::new(
            StatementKind::Import {
                names: vec!["println".to_string()],
                path: vec!["std".to_string(), "display".to_string()],
            },
            Span::new(0, 29),
        ),
        Statement::new(
            StatementKind::Expression(Expression::new(ExpressionKind::Integer(0), Span::new(29, 30))),
            Span::new(29, 30),
        ),
        Statement::new(
            StatementKind::Import {
                names: vec!["is_prime".to_string(), "factorial".to_string(), "fibonacci".to_string()],
                path: vec!["std".to_string(), "math".to_string()],
            },
            Span::new(30, 79),
        ),
        Statement::new(
            StatementKind::Expression(Expression::new(ExpressionKind::Integer(0), Span::new(79, 80))),
            Span::new(79, 80),
        ),
        Statement::new(
            StatementKind::Expression(Expression::new(ExpressionKind::Integer(0), Span::new(80, 81))),
            Span::new(80, 81),
        ),
        Statement::new(
            StatementKind::VariableDeclaration {
                name: "n".to_string(),
                type_annotation: TypeAnnotation::Int,
                value: Expression::new(ExpressionKind::Integer(10), Span::new(93, 95)),
            },
            Span::new(81, 95),
        ),
        Statement::new(
            StatementKind::Expression(Expression::new(ExpressionKind::Integer(0), Span::new(95, 96))),
            Span::new(95, 96),
        ),
        Statement::new(
            StatementKind::Array {
                name: "ops".to_string(),
                type_annotation: TypeAnnotation::Fn,
                value: vec![
                    Expression::new(ExpressionKind::Identifier("factorial".to_string()), Span::new(115, 124)),
                    Expression::new(ExpressionKind::Identifier("fibonacci".to_string()), Span::new(126, 135)),
                ],
            },
            Span::new(96, 136),
        ),
        Statement::new(
            StatementKind::Expression(Expression::new(ExpressionKind::Integer(0), Span::new(136, 137))),
            Span::new(136, 137),
        ),
        Statement::new(
            StatementKind::Expression(Expression::new(ExpressionKind::Integer(0), Span::new(137, 138))),
            Span::new(137, 138),
        ),
        Statement::new(
            StatementKind::VariableDeclaration {
                name: "i".to_string(),
                type_annotation: TypeAnnotation::Int,
                value: Expression::new(ExpressionKind::Integer(0), Span::new(150, 151)),
            },
            Span::new(138, 151),
        ),
        Statement::new(
            StatementKind::Expression(Expression::new(ExpressionKind::Integer(0), Span::new(151, 152))),
            Span::new(151, 152),
        ),
        Statement::new(
            StatementKind::For {
                initializer: Box::new(Statement::new(
                    StatementKind::VariableDeclaration {
                        name: "i".to_string(),
                        type_annotation: TypeAnnotation::Int,
                        value: Expression::new(ExpressionKind::Integer(0), Span::new(165, 166)),
                    },
                    Span::new(157, 166),
                )),
                condition: Expression::new(
                    ExpressionKind::Binary {
                        left: Box::new(Expression::new(ExpressionKind::Identifier("i".to_string()), Span::new(168, 169))),
                        operator: TokenType::Less,
                        right: Box::new(Expression::new(ExpressionKind::Integer(2), Span::new(172, 173))),
                    },
                    Span::new(168, 173),
                ),
                increment: Expression::new(
                    ExpressionKind::Assign {
                        name: "i".to_string(),
                        value: Box::new(Expression::new(
                            ExpressionKind::Binary {
                                left: Box::new(Expression::new(ExpressionKind::Identifier("i".to_string()), Span::new(175, 176))),
                                operator: TokenType::Plus,
                                right: Box::new(Expression::new(ExpressionKind::Integer(1), Span::new(180, 181))),
                            },
                            Span::new(175, 181),
                        )),
                    },
                    Span::new(175, 181),
                ),
                body: vec![
                    Statement::new(
                        StatementKind::VariableDeclaration {
                            name: "op".to_string(),
                            type_annotation: TypeAnnotation::Fn,
                            value: Expression::new(
                                ExpressionKind::Index {
                                    target: Box::new(Expression::new(ExpressionKind::Identifier("ops".to_string()), Span::new(201, 204))),
                                    index: Box::new(Expression::new(ExpressionKind::Identifier("i".to_string()), Span::new(205, 206))),
                                },
                                Span::new(201, 207),
                            ),
                        },
                        Span::new(189, 207),
                    ),
                    Statement::new(
                        StatementKind::VariableDeclaration {
                            name: "j".to_string(),
                            type_annotation: TypeAnnotation::Int,
                            value: Expression::new(ExpressionKind::Integer(1), Span::new(224, 225)),
                        },
                        Span::new(212, 225),
                    ),
                    Statement::new(
                        StatementKind::While {
                            condition: Expression::new(
                                ExpressionKind::Grouping(Box::new(Expression::new(
                                    ExpressionKind::Binary {
                                        left: Box::new(Expression::new(ExpressionKind::Identifier("j".to_string()), Span::new(237, 238))),
                                        operator: TokenType::Less,
                                        right: Box::new(Expression::new(ExpressionKind::Identifier("n".to_string()), Span::new(241, 242))),
                                    },
                                    Span::new(237, 242),
                                ))),
                                Span::new(236, 243),
                            ),
                            body: vec![
                                Statement::new(
                                    StatementKind::VariableDeclaration {
                                        name: "result".to_string(),
                                        type_annotation: TypeAnnotation::Int,
                                        value: Expression::new(
                                            ExpressionKind::Call {
                                                path: vec!["op".to_string()],
                                                args: vec![Expression::new(ExpressionKind::Identifier("j".to_string()), Span::new(274, 275))],
                                            },
                                            Span::new(271, 276),
                                        ),
                                    },
                                    Span::new(254, 276),
                                ),
                                Statement::new(
                                    StatementKind::Conditional {
                                        if_branch: Box::new(Statement::new(
                                            StatementKind::ConditionalBranch {
                                                condition: Some(Expression::new(
                                                    ExpressionKind::Grouping(Box::new(Expression::new(
                                                        ExpressionKind::Call {
                                                            path: vec!["is_prime".to_string()],
                                                            args: vec![Expression::new(ExpressionKind::Identifier("result".to_string()), Span::new(298, 304))],
                                                        },
                                                        Span::new(289, 305),
                                                    ))),
                                                    Span::new(288, 306),
                                                )),
                                                body: vec![
                                                    Statement::new(
                                                        StatementKind::Expression(Expression::new(
                                                            ExpressionKind::Call {
                                                                path: vec!["println".to_string()],
                                                                args: vec![Expression::new(ExpressionKind::Identifier("result".to_string()), Span::new(329, 335))],
                                                            },
                                                            Span::new(321, 336),
                                                        )),
                                                        Span::new(321, 336),
                                                    ),
                                                    Statement::new(
                                                        StatementKind::Expression(Expression::new(
                                                            ExpressionKind::Assign {
                                                                name: "j".to_string(),
                                                                value: Box::new(Expression::new(
                                                                    ExpressionKind::Binary {
                                                                        left: Box::new(Expression::new(ExpressionKind::Identifier("j".to_string()), Span::new(349, 350))),
                                                                        operator: TokenType::Plus,
                                                                        right: Box::new(Expression::new(ExpressionKind::Integer(1), Span::new(354, 355))),
                                                                    },
                                                                    Span::new(349, 355),
                                                                )),
                                                            },
                                                            Span::new(349, 355),
                                                        )),
                                                        Span::new(349, 355),
                                                    ),
                                                ],
                                            },
                                            Span::new(285, 365),
                                        )),
                                        elseif_branch: Some(vec![]),
                                        else_branch: Some(Box::new(Statement::new(
                                            StatementKind::ConditionalBranch {
                                                condition: None,
                                                body: vec![Statement::new(
                                                    StatementKind::Break,
                                                    Span::new(385, 390),
                                                )],
                                            },
                                            Span::new(366, 400),
                                        ))),
                                    },
                                    Span::new(285, 400),
                                ),
                            ],
                        },
                        Span::new(230, 406),
                    ),
                    Statement::new(
                        StatementKind::Expression(Expression::new(
                            ExpressionKind::Assign {
                                name: "i".to_string(),
                                value: Box::new(Expression::new(
                                    ExpressionKind::Binary {
                                        left: Box::new(Expression::new(ExpressionKind::Identifier("i".to_string()), Span::new(411, 412))),
                                        operator: TokenType::Plus,
                                        right: Box::new(Expression::new(ExpressionKind::Integer(1), Span::new(416, 417))),
                                    },
                                    Span::new(411, 417),
                                )),
                            },
                            Span::new(411, 417),
                        )),
                        Span::new(411, 417),
                    ),
                ],
            },
            Span::new(152, 419),
        ),
    ];
    assert_eq!(statements, expected);
}
