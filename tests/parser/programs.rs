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
        "get println from std::io
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
                path: vec!["std".to_string(), "io".to_string()],
            },
            Span::new(0, 24),
        ),
        Statement::new(
            StatementKind::Expression(Expression::new(
                ExpressionKind::Integer(0),
                Span::new(24, 25),
            )),
            Span::new(24, 25),
        ),
        Statement::new(
            StatementKind::Import {
                names: vec!["pow".to_string()],
                path: vec!["std".to_string(), "math".to_string()],
            },
            Span::new(25, 47),
        ),
        Statement::new(
            StatementKind::Expression(Expression::new(
                ExpressionKind::Integer(0),
                Span::new(47, 48),
            )),
            Span::new(47, 48),
        ),
        Statement::new(
            StatementKind::Import {
                names: vec!["PI".to_string()],
                path: vec!["std".to_string(), "math".to_string(), "consts".to_string()],
            },
            Span::new(48, 73),
        ),
        Statement::new(
            StatementKind::Expression(Expression::new(
                ExpressionKind::Integer(0),
                Span::new(73, 74),
            )),
            Span::new(73, 74),
        ),
        Statement::new(
            StatementKind::Expression(Expression::new(
                ExpressionKind::Integer(0),
                Span::new(74, 75),
            )),
            Span::new(74, 75),
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
                    Span::new(92, 96),
                ),
            },
            Span::new(75, 96),
        ),
        Statement::new(
            StatementKind::Expression(Expression::new(
                ExpressionKind::Integer(0),
                Span::new(96, 97),
            )),
            Span::new(96, 97),
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
                                Span::new(115, 117),
                            ),
                            Expression::new(
                                ExpressionKind::Identifier("pi".to_string()),
                                Span::new(119, 121),
                            ),
                        ],
                    },
                    Span::new(111, 122),
                ),
            },
            Span::new(97, 122),
        ),
        Statement::new(
            StatementKind::Expression(Expression::new(
                ExpressionKind::Integer(0),
                Span::new(122, 123),
            )),
            Span::new(122, 123),
        ),
        Statement::new(
            StatementKind::Expression(Expression::new(
                ExpressionKind::Integer(0),
                Span::new(123, 124),
            )),
            Span::new(123, 124),
        ),
        Statement::new(
            StatementKind::Expression(Expression::new(
                ExpressionKind::Call {
                    path: vec!["println".to_string()],
                    args: vec![
                        Expression::new(
                            ExpressionKind::Identifier("x".to_string()),
                            Span::new(132, 133),
                        ),
                        Expression::new(
                            ExpressionKind::Identifier("y".to_string()),
                            Span::new(135, 136),
                        ),
                    ],
                },
                Span::new(124, 137),
            )),
            Span::new(124, 137),
        ),
    ];
    assert_eq!(statements, expected);
}

#[test]
fn integration_geometry() {
    let statements = common::parse(
        "get println from std::io
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
                path: vec!["std".to_string(), "io".to_string()],
            },
            Span::new(0, 24),
        ),
        Statement::new(
            StatementKind::Expression(Expression::new(
                ExpressionKind::Integer(0),
                Span::new(24, 25),
            )),
            Span::new(24, 25),
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
            Span::new(25, 86),
        ),
        Statement::new(
            StatementKind::Expression(Expression::new(
                ExpressionKind::Integer(0),
                Span::new(86, 87),
            )),
            Span::new(86, 87),
        ),
        Statement::new(
            StatementKind::Import {
                names: vec!["PI".to_string(), "TAU".to_string(), "PHI".to_string()],
                path: vec!["std".to_string(), "math".to_string(), "consts".to_string()],
            },
            Span::new(87, 126),
        ),
        Statement::new(
            StatementKind::Expression(Expression::new(
                ExpressionKind::Integer(0),
                Span::new(126, 127),
            )),
            Span::new(126, 127),
        ),
        Statement::new(
            StatementKind::Expression(Expression::new(
                ExpressionKind::Integer(0),
                Span::new(127, 128),
            )),
            Span::new(127, 128),
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
                    Span::new(146, 150),
                ),
            },
            Span::new(128, 150),
        ),
        Statement::new(
            StatementKind::Expression(Expression::new(
                ExpressionKind::Integer(0),
                Span::new(150, 151),
            )),
            Span::new(150, 151),
        ),
        Statement::new(
            StatementKind::Expression(Expression::new(
                ExpressionKind::Assign {
                    name: "angle".to_string(),
                    value: Box::new(Expression::new(
                        ExpressionKind::Binary {
                            left: Box::new(Expression::new(
                                ExpressionKind::Identifier("angle".to_string()),
                                Span::new(151, 156),
                            )),
                            operator: TokenType::Star,
                            right: Box::new(Expression::new(
                                ExpressionKind::Call {
                                    path: vec!["PHI".to_string()],
                                    args: vec![],
                                },
                                Span::new(160, 165),
                            )),
                        },
                        Span::new(151, 165),
                    )),
                },
                Span::new(151, 165),
            )),
            Span::new(151, 165),
        ),
        Statement::new(
            StatementKind::Expression(Expression::new(
                ExpressionKind::Integer(0),
                Span::new(165, 166),
            )),
            Span::new(165, 166),
        ),
        Statement::new(
            StatementKind::Expression(Expression::new(
                ExpressionKind::Integer(0),
                Span::new(166, 167),
            )),
            Span::new(166, 167),
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
                            Span::new(187, 192),
                        )],
                    },
                    Span::new(183, 193),
                ),
            },
            Span::new(167, 193),
        ),
        Statement::new(
            StatementKind::Expression(Expression::new(
                ExpressionKind::Integer(0),
                Span::new(193, 194),
            )),
            Span::new(193, 194),
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
                            Span::new(214, 219),
                        )],
                    },
                    Span::new(210, 220),
                ),
            },
            Span::new(194, 220),
        ),
        Statement::new(
            StatementKind::Expression(Expression::new(
                ExpressionKind::Integer(0),
                Span::new(220, 221),
            )),
            Span::new(220, 221),
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
                                Span::new(243, 246),
                            ),
                            Expression::new(
                                ExpressionKind::Identifier("adj".to_string()),
                                Span::new(248, 251),
                            ),
                        ],
                    },
                    Span::new(237, 252),
                ),
            },
            Span::new(221, 252),
        ),
        Statement::new(
            StatementKind::Expression(Expression::new(
                ExpressionKind::Integer(0),
                Span::new(252, 253),
            )),
            Span::new(252, 253),
        ),
        Statement::new(
            StatementKind::Expression(Expression::new(
                ExpressionKind::Integer(0),
                Span::new(253, 254),
            )),
            Span::new(253, 254),
        ),
        Statement::new(
            StatementKind::VariableDeclaration {
                name: "is_right".to_string(),
                type_annotation: TypeAnnotation::Bool,
                value: Expression::new(
                    ExpressionKind::Binary {
                        left: Box::new(Expression::new(
                            ExpressionKind::Identifier("hyp".to_string()),
                            Span::new(274, 277),
                        )),
                        operator: TokenType::Compare,
                        right: Box::new(Expression::new(
                            ExpressionKind::Float(1.0),
                            Span::new(281, 284),
                        )),
                    },
                    Span::new(274, 284),
                ),
            },
            Span::new(254, 284),
        ),
        Statement::new(
            StatementKind::Expression(Expression::new(
                ExpressionKind::Integer(0),
                Span::new(284, 285),
            )),
            Span::new(284, 285),
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
                            Span::new(307, 315),
                        )),
                    },
                    Span::new(306, 315),
                ),
            },
            Span::new(285, 315),
        ),
        Statement::new(
            StatementKind::Expression(Expression::new(
                ExpressionKind::Integer(0),
                Span::new(315, 316),
            )),
            Span::new(315, 316),
        ),
        Statement::new(
            StatementKind::Expression(Expression::new(
                ExpressionKind::Integer(0),
                Span::new(316, 317),
            )),
            Span::new(316, 317),
        ),
        Statement::new(
            StatementKind::VariableDeclaration {
                name: "n".to_string(),
                type_annotation: TypeAnnotation::Int,
                value: Expression::new(ExpressionKind::Integer(7), Span::new(329, 330)),
            },
            Span::new(317, 330),
        ),
        Statement::new(
            StatementKind::Expression(Expression::new(
                ExpressionKind::Integer(0),
                Span::new(330, 331),
            )),
            Span::new(330, 331),
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
                            Span::new(357, 358),
                        )],
                    },
                    Span::new(348, 359),
                ),
            },
            Span::new(331, 359),
        ),
        Statement::new(
            StatementKind::Expression(Expression::new(
                ExpressionKind::Integer(0),
                Span::new(359, 360),
            )),
            Span::new(359, 360),
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
                            Span::new(387, 388),
                        )],
                    },
                    Span::new(377, 389),
                ),
            },
            Span::new(360, 389),
        ),
        Statement::new(
            StatementKind::Expression(Expression::new(
                ExpressionKind::Integer(0),
                Span::new(389, 390),
            )),
            Span::new(389, 390),
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
                            Span::new(410, 414),
                        )],
                    },
                    Span::new(405, 415),
                ),
            },
            Span::new(390, 415),
        ),
        Statement::new(
            StatementKind::Expression(Expression::new(
                ExpressionKind::Integer(0),
                Span::new(415, 416),
            )),
            Span::new(415, 416),
        ),
        Statement::new(
            StatementKind::Expression(Expression::new(
                ExpressionKind::Integer(0),
                Span::new(416, 417),
            )),
            Span::new(416, 417),
        ),
        Statement::new(
            StatementKind::Expression(Expression::new(
                ExpressionKind::Assign {
                    name: "lg".to_string(),
                    value: Box::new(Expression::new(
                        ExpressionKind::Binary {
                            left: Box::new(Expression::new(
                                ExpressionKind::Identifier("lg".to_string()),
                                Span::new(417, 419),
                            )),
                            operator: TokenType::Minus,
                            right: Box::new(Expression::new(
                                ExpressionKind::Float(1.0),
                                Span::new(423, 426),
                            )),
                        },
                        Span::new(417, 426),
                    )),
                },
                Span::new(417, 426),
            )),
            Span::new(417, 426),
        ),
        Statement::new(
            StatementKind::Expression(Expression::new(
                ExpressionKind::Integer(0),
                Span::new(426, 427),
            )),
            Span::new(426, 427),
        ),
        Statement::new(
            StatementKind::Expression(Expression::new(
                ExpressionKind::Integer(0),
                Span::new(427, 428),
            )),
            Span::new(427, 428),
        ),
        Statement::new(
            StatementKind::VariableDeclaration {
                name: "big".to_string(),
                type_annotation: TypeAnnotation::Bool,
                value: Expression::new(
                    ExpressionKind::Binary {
                        left: Box::new(Expression::new(
                            ExpressionKind::Identifier("lg".to_string()),
                            Span::new(443, 445),
                        )),
                        operator: TokenType::Greater,
                        right: Box::new(Expression::new(
                            ExpressionKind::Call {
                                path: vec!["TAU".to_string()],
                                args: vec![],
                            },
                            Span::new(448, 453),
                        )),
                    },
                    Span::new(443, 453),
                ),
            },
            Span::new(428, 453),
        ),
        Statement::new(
            StatementKind::Expression(Expression::new(
                ExpressionKind::Integer(0),
                Span::new(453, 454),
            )),
            Span::new(453, 454),
        ),
        Statement::new(
            StatementKind::Expression(Expression::new(
                ExpressionKind::Integer(0),
                Span::new(454, 455),
            )),
            Span::new(454, 455),
        ),
        Statement::new(
            StatementKind::Expression(Expression::new(
                ExpressionKind::Call {
                    path: vec!["println".to_string()],
                    args: vec![
                        Expression::new(
                            ExpressionKind::Identifier("hyp".to_string()),
                            Span::new(463, 466),
                        ),
                        Expression::new(
                            ExpressionKind::Identifier("is_right".to_string()),
                            Span::new(468, 476),
                        ),
                        Expression::new(
                            ExpressionKind::Identifier("prime".to_string()),
                            Span::new(478, 483),
                        ),
                        Expression::new(
                            ExpressionKind::Identifier("big".to_string()),
                            Span::new(485, 488),
                        ),
                    ],
                },
                Span::new(455, 489),
            )),
            Span::new(455, 489),
        ),
    ];
    assert_eq!(statements, expected);
}

#[test]
fn integration_fn_array() {
    let statements = common::parse(
        "get println from std::io
get is_prime, factorial, fibonacci from std::math

dec int n = 10
dec arr[fn] ops = [factorial, fibonacci]

dec int i = 0
for [int i = 0, i < 2, i += 1] {
    dec fn op = ops[i]
    dec int j = 1
    while (j < n) {
        dec int _resul = op(j)
        if (is_prime(_resul)) {
            println(_resul)
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
                    path: vec!["std".to_string(), "io".to_string()],
                },
                Span::new(0, 24),
            ),
            Statement::new(
                StatementKind::Expression(Expression::new(ExpressionKind::Integer(0), Span::new(24, 25))),
                Span::new(24, 25),
            ),
            Statement::new(
                StatementKind::Import {
                    names: vec!["is_prime".to_string(), "factorial".to_string(), "fibonacci".to_string()],
                    path: vec!["std".to_string(), "math".to_string()],
                },
                Span::new(25, 74),
            ),
            Statement::new(
                StatementKind::Expression(Expression::new(ExpressionKind::Integer(0), Span::new(74, 75))),
                Span::new(74, 75),
            ),
            Statement::new(
                StatementKind::Expression(Expression::new(ExpressionKind::Integer(0), Span::new(75, 76))),
                Span::new(75, 76),
            ),
            Statement::new(
                StatementKind::VariableDeclaration {
                    name: "n".to_string(),
                    type_annotation: TypeAnnotation::Int,
                    value: Expression::new(ExpressionKind::Integer(10), Span::new(88, 90)),
                },
                Span::new(76, 90),
            ),
            Statement::new(
                StatementKind::Expression(Expression::new(ExpressionKind::Integer(0), Span::new(90, 91))),
                Span::new(90, 91),
            ),
            Statement::new(
                StatementKind::Array {
                    name: "ops".to_string(),
                    type_annotation: TypeAnnotation::Fn,
                    value: vec![
                        Expression::new(ExpressionKind::Identifier("factorial".to_string()), Span::new(110, 119)),
                        Expression::new(ExpressionKind::Identifier("fibonacci".to_string()), Span::new(121, 130)),
                    ],
                },
                Span::new(91, 131),
            ),
            Statement::new(
                StatementKind::Expression(Expression::new(ExpressionKind::Integer(0), Span::new(131, 132))),
                Span::new(131, 132),
            ),
            Statement::new(
                StatementKind::Expression(Expression::new(ExpressionKind::Integer(0), Span::new(132, 133))),
                Span::new(132, 133),
            ),
            Statement::new(
                StatementKind::VariableDeclaration {
                    name: "i".to_string(),
                    type_annotation: TypeAnnotation::Int,
                    value: Expression::new(ExpressionKind::Integer(0), Span::new(145, 146)),
                },
                Span::new(133, 146),
            ),
            Statement::new(
                StatementKind::Expression(Expression::new(ExpressionKind::Integer(0), Span::new(146, 147))),
                Span::new(146, 147),
            ),
            Statement::new(
                StatementKind::For {
                    initializer: Box::new(Statement::new(
                        StatementKind::VariableDeclaration {
                            name: "i".to_string(),
                            type_annotation: TypeAnnotation::Int,
                            value: Expression::new(ExpressionKind::Integer(0), Span::new(160, 161)),
                        },
                        Span::new(152, 161),
                    )),
                    condition: Expression::new(
                        ExpressionKind::Binary {
                            left: Box::new(Expression::new(ExpressionKind::Identifier("i".to_string()), Span::new(163, 164))),
                            operator: TokenType::Less,
                            right: Box::new(Expression::new(ExpressionKind::Integer(2), Span::new(167, 168))),
                        },
                        Span::new(163, 168),
                    ),
                    increment: Expression::new(
                        ExpressionKind::Assign {
                            name: "i".to_string(),
                            value: Box::new(Expression::new(
                                ExpressionKind::Binary {
                                    left: Box::new(Expression::new(ExpressionKind::Identifier("i".to_string()), Span::new(170, 171))),
                                    operator: TokenType::Plus,
                                    right: Box::new(Expression::new(ExpressionKind::Integer(1), Span::new(175, 176))),
                                },
                                Span::new(170, 176),
                            )),
                        },
                        Span::new(170, 176),
                    ),
                    body: vec![
                        Statement::new(
                            StatementKind::VariableDeclaration {
                                name: "op".to_string(),
                                type_annotation: TypeAnnotation::Fn,
                                value: Expression::new(
                                    ExpressionKind::Index {
                                        target: Box::new(Expression::new(ExpressionKind::Identifier("ops".to_string()), Span::new(196, 199))),
                                        index: Box::new(Expression::new(ExpressionKind::Identifier("i".to_string()), Span::new(200, 201))),
                                    },
                                    Span::new(196, 202),
                                ),
                            },
                            Span::new(184, 202),
                        ),
                        Statement::new(
                            StatementKind::VariableDeclaration {
                                name: "j".to_string(),
                                type_annotation: TypeAnnotation::Int,
                                value: Expression::new(ExpressionKind::Integer(1), Span::new(219, 220)),
                            },
                            Span::new(207, 220),
                        ),
                        Statement::new(
                            StatementKind::While {
                                condition: Expression::new(
                                    ExpressionKind::Grouping(Box::new(Expression::new(
                                        ExpressionKind::Binary {
                                            left: Box::new(Expression::new(ExpressionKind::Identifier("j".to_string()), Span::new(232, 233))),
                                            operator: TokenType::Less,
                                            right: Box::new(Expression::new(ExpressionKind::Identifier("n".to_string()), Span::new(236, 237))),
                                        },
                                        Span::new(232, 237),
                                    ))),
                                    Span::new(231, 238),
                                ),
                                body: vec![
                                    Statement::new(
                                        StatementKind::VariableDeclaration {
                                            name: "_resul".to_string(),
                                            type_annotation: TypeAnnotation::Int,
                                            value: Expression::new(
                                                ExpressionKind::Call {
                                                    path: vec!["op".to_string()],
                                                    args: vec![Expression::new(ExpressionKind::Identifier("j".to_string()), Span::new(269, 270))],
                                                },
                                                Span::new(266, 271),
                                            ),
                                        },
                                        Span::new(249, 271),
                                    ),
                                    Statement::new(
                                        StatementKind::Conditional {
                                            if_branch: Box::new(Statement::new(
                                                StatementKind::ConditionalBranch {
                                                    condition: Some(Expression::new(
                                                        ExpressionKind::Grouping(Box::new(Expression::new(
                                                            ExpressionKind::Call {
                                                                path: vec!["is_prime".to_string()],
                                                                args: vec![Expression::new(ExpressionKind::Identifier("_resul".to_string()), Span::new(293, 299))],
                                                            },
                                                            Span::new(284, 300),
                                                        ))),
                                                        Span::new(283, 301),
                                                    )),
                                                    body: vec![
                                                        Statement::new(
                                                            StatementKind::Expression(Expression::new(
                                                                ExpressionKind::Call {
                                                                    path: vec!["println".to_string()],
                                                                    args: vec![Expression::new(ExpressionKind::Identifier("_resul".to_string()), Span::new(324, 330))],
                                                                },
                                                                Span::new(316, 331),
                                                            )),
                                                            Span::new(316, 331),
                                                        ),
                                                        Statement::new(
                                                            StatementKind::Expression(Expression::new(
                                                                ExpressionKind::Assign {
                                                                    name: "j".to_string(),
                                                                    value: Box::new(Expression::new(
                                                                        ExpressionKind::Binary {
                                                                            left: Box::new(Expression::new(ExpressionKind::Identifier("j".to_string()), Span::new(344, 345))),
                                                                            operator: TokenType::Plus,
                                                                            right: Box::new(Expression::new(ExpressionKind::Integer(1), Span::new(349, 350))),
                                                                        },
                                                                        Span::new(344, 350),
                                                                    )),
                                                                },
                                                                Span::new(344, 350),
                                                            )),
                                                            Span::new(344, 350),
                                                        ),
                                                    ],
                                                },
                                                Span::new(280, 360),
                                            )),

                                            else_branch: Some(Box::new(Statement::new(
                                                StatementKind::ConditionalBranch {
                                                    condition: None,
                                                    body: vec![Statement::new(
                                                        StatementKind::Break,
                                                        Span::new(380, 385),
                                                    )],
                                                },
                                                Span::new(361, 395),
                                            ))),
                                        },
                                        Span::new(280, 395),
                                    ),
                                ],
                            },
                            Span::new(225, 401),
                        ),
                        Statement::new(
                            StatementKind::Expression(Expression::new(
                                ExpressionKind::Assign {
                                    name: "i".to_string(),
                                    value: Box::new(Expression::new(
                                        ExpressionKind::Binary {
                                            left: Box::new(Expression::new(ExpressionKind::Identifier("i".to_string()), Span::new(406, 407))),
                                            operator: TokenType::Plus,
                                            right: Box::new(Expression::new(ExpressionKind::Integer(1), Span::new(411, 412))),
                                        },
                                        Span::new(406, 412),
                                    )),
                                },
                                Span::new(406, 412),
                            )),
                            Span::new(406, 412),
                        ),
                    ],
                },
                Span::new(147, 414),
            ),
        ];
    assert_eq!(statements, expected);
}
