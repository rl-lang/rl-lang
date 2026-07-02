use rl_lang::{
    arraydecl, assign, assignexpr,
    ast::statements::{StatementKind, TypeAnnotation},
    bin, call, constdecl, exprstmt, flt, grp, id, idx, import, int,
    lexer::tokentypes::TokenType,
    nl, sn, un,
    utils::span::Span,
    vardecl,
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
        import!(["println"], ["std", "io"], 0, 24),
        nl!(24, 25),
        import!(["pow"], ["std", "math"], 25, 47),
        nl!(47, 48),
        import!(["PI"], ["std", "math", "consts"], 48, 73),
        nl!(73, 74),
        nl!(74, 75),
        constdecl!(
            "pi",
            TypeAnnotation::CFloat,
            call!(["PI"], [], 92, 96),
            75,
            96
        ),
        nl!(96, 97),
        vardecl!(
            "x",
            TypeAnnotation::Float,
            call!(
                ["pow"],
                [id!("pi", 115, 117), id!("pi", 119, 121)],
                111,
                122
            ),
            97,
            122
        ),
        nl!(122, 123),
        nl!(123, 124),
        exprstmt!(
            call!(
                ["println"],
                [id!("x", 132, 133), id!("y", 135, 136)],
                124,
                137
            ),
            124,
            137
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
        import!(["println"], ["std", "io"], 0, 24),
        nl!(24, 25),
        import!(
            ["sin", "cos", "hypot", "is_prime", "log2", "factorial"],
            ["std", "math"],
            25,
            86
        ),
        nl!(86, 87),
        import!(["PI", "TAU", "PHI"], ["std", "math", "consts"], 87, 126),
        nl!(126, 127),
        nl!(127, 128),
        vardecl!(
            "angle",
            TypeAnnotation::Float,
            call!(["PI"], [], 146, 150),
            128,
            150
        ),
        nl!(150, 151),
        assign!(
            "angle",
            bin!(
                id!("angle", 151, 156),
                TokenType::Star,
                call!(["PHI"], [], 160, 165),
                151,
                165
            ),
            151,
            165
        ),
        nl!(165, 166),
        nl!(166, 167),
        vardecl!(
            "opp",
            TypeAnnotation::Float,
            call!(["sin"], [id!("angle", 187, 192)], 183, 193),
            167,
            193
        ),
        nl!(193, 194),
        vardecl!(
            "adj",
            TypeAnnotation::Float,
            call!(["cos"], [id!("angle", 214, 219)], 210, 220),
            194,
            220
        ),
        nl!(220, 221),
        vardecl!(
            "hyp",
            TypeAnnotation::Float,
            call!(
                ["hypot"],
                [id!("opp", 243, 246), id!("adj", 248, 251)],
                237,
                252
            ),
            221,
            252
        ),
        nl!(252, 253),
        nl!(253, 254),
        vardecl!(
            "is_right",
            TypeAnnotation::Bool,
            bin!(
                id!("hyp", 274, 277),
                TokenType::Compare,
                flt!(1.0, 281, 284),
                274,
                284
            ),
            254,
            284
        ),
        nl!(284, 285),
        vardecl!(
            "not_right",
            TypeAnnotation::Bool,
            un!(TokenType::Bang, id!("is_right", 307, 315), 306, 315),
            285,
            315
        ),
        nl!(315, 316),
        nl!(316, 317),
        vardecl!("n", TypeAnnotation::Int, int!(7, 329, 330), 317, 330),
        nl!(330, 331),
        vardecl!(
            "prime",
            TypeAnnotation::Bool,
            call!(["is_prime"], [id!("n", 357, 358)], 348, 359),
            331,
            359
        ),
        nl!(359, 360),
        vardecl!(
            "fact",
            TypeAnnotation::Float,
            call!(["factorial"], [id!("n", 387, 388)], 377, 389),
            360,
            389
        ),
        nl!(389, 390),
        vardecl!(
            "lg",
            TypeAnnotation::Float,
            call!(["log2"], [id!("fact", 410, 414)], 405, 415),
            390,
            415
        ),
        nl!(415, 416),
        nl!(416, 417),
        assign!(
            "lg",
            bin!(
                id!("lg", 417, 419),
                TokenType::Minus,
                flt!(1.0, 423, 426),
                417,
                426
            ),
            417,
            426
        ),
        nl!(426, 427),
        nl!(427, 428),
        vardecl!(
            "big",
            TypeAnnotation::Bool,
            bin!(
                id!("lg", 443, 445),
                TokenType::Greater,
                call!(["TAU"], [], 448, 453),
                443,
                453
            ),
            428,
            453
        ),
        nl!(453, 454),
        nl!(454, 455),
        exprstmt!(
            call!(
                ["println"],
                [
                    id!("hyp", 463, 466),
                    id!("is_right", 468, 476),
                    id!("prime", 478, 483),
                    id!("big", 485, 488)
                ],
                455,
                489
            ),
            455,
            489
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
        import!(["println"], ["std", "io"], 0, 24),
        nl!(24, 25),
        import!(
            ["is_prime", "factorial", "fibonacci"],
            ["std", "math"],
            25,
            74
        ),
        nl!(74, 75),
        nl!(75, 76),
        vardecl!("n", TypeAnnotation::Int, int!(10, 88, 90), 76, 90),
        nl!(90, 91),
        arraydecl!(
            "ops",
            TypeAnnotation::Fn,
            [id!("factorial", 110, 119), id!("fibonacci", 121, 130)],
            91,
            131
        ),
        nl!(131, 132),
        nl!(132, 133),
        vardecl!("i", TypeAnnotation::Int, int!(0, 145, 146), 133, 146),
        nl!(146, 147),
        sn!(
            StatementKind::For {
                initializer: Box::new(vardecl!(
                    "i",
                    TypeAnnotation::Int,
                    int!(0, 160, 161),
                    152,
                    161
                )),
                condition: bin!(
                    id!("i", 163, 164),
                    TokenType::Less,
                    int!(2, 167, 168),
                    163,
                    168
                ),
                increment: assignexpr!(
                    "i",
                    bin!(
                        id!("i", 170, 171),
                        TokenType::Plus,
                        int!(1, 175, 176),
                        170,
                        176
                    ),
                    170,
                    176
                ),
                body: vec![
                    vardecl!(
                        "op",
                        TypeAnnotation::Fn,
                        idx!(id!("ops", 196, 199), id!("i", 200, 201), 196, 202),
                        184,
                        202
                    ),
                    vardecl!("j", TypeAnnotation::Int, int!(1, 219, 220), 207, 220),
                    sn!(
                        StatementKind::While {
                            condition: grp!(
                                bin!(
                                    id!("j", 232, 233),
                                    TokenType::Less,
                                    id!("n", 236, 237),
                                    232,
                                    237
                                ),
                                231,
                                238
                            ),
                            body: vec![
                                vardecl!(
                                    "_resul",
                                    TypeAnnotation::Int,
                                    call!(["op"], [id!("j", 269, 270)], 266, 271),
                                    249,
                                    271
                                ),
                                sn!(
                                    StatementKind::Conditional {
                                        if_branch: Box::new(sn!(
                                            StatementKind::ConditionalBranch {
                                                condition: Some(grp!(
                                                    call!(
                                                        ["is_prime"],
                                                        [id!("_resul", 293, 299)],
                                                        284,
                                                        300
                                                    ),
                                                    283,
                                                    301
                                                )),
                                                body: vec![
                                                    exprstmt!(
                                                        call!(
                                                            ["println"],
                                                            [id!("_resul", 324, 330)],
                                                            316,
                                                            331
                                                        ),
                                                        316,
                                                        331
                                                    ),
                                                    assign!(
                                                        "j",
                                                        bin!(
                                                            id!("j", 344, 345),
                                                            TokenType::Plus,
                                                            int!(1, 349, 350),
                                                            344,
                                                            350
                                                        ),
                                                        344,
                                                        350
                                                    ),
                                                ],
                                            },
                                            Span::new(280, 360)
                                        )),
                                        else_branch: Some(Box::new(sn!(
                                            StatementKind::ConditionalBranch {
                                                condition: None,
                                                body: vec![sn!(
                                                    StatementKind::Break,
                                                    Span::new(380, 385)
                                                )],
                                            },
                                            Span::new(361, 395)
                                        ))),
                                    },
                                    Span::new(280, 395)
                                ),
                            ],
                        },
                        Span::new(225, 401)
                    ),
                    assign!(
                        "i",
                        bin!(
                            id!("i", 406, 407),
                            TokenType::Plus,
                            int!(1, 411, 412),
                            406,
                            412
                        ),
                        406,
                        412
                    ),
                ],
            },
            Span::new(147, 414)
        ),
    ];
    assert_eq!(statements, expected);
}
