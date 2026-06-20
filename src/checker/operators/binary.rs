use crate::{
    ast::statements::TypeAnnotation,
    checker::{
        operators::op_str,
        structs::{CheckType, TypeChecker},
    },
    lexer::tokentypes::TokenType,
    utils::span::Span,
};

impl TypeChecker {
    pub fn check_binary_operator(
        &mut self,
        left: CheckType,
        right: CheckType,
        op: &TokenType,
        span: Span,
    ) -> CheckType {
        // if any of sides is unknown then it is unknown
        if left.is_unknown() || right.is_unknown() {
            return CheckType::Unknown;
        }

        match op {
            // arithmetic check if both same type or not
            TokenType::Plus | TokenType::Minus | TokenType::Star | TokenType::Slash => {
                match (&left, &right) {
                    (
                        CheckType::Known(TypeAnnotation::Int | TypeAnnotation::CInt),
                        CheckType::Known(TypeAnnotation::Int | TypeAnnotation::CInt),
                    ) => CheckType::Known(TypeAnnotation::Int),
                    (
                        CheckType::Known(TypeAnnotation::Float | TypeAnnotation::CFloat),
                        CheckType::Known(TypeAnnotation::Float | TypeAnnotation::CFloat),
                    ) => CheckType::Known(TypeAnnotation::Float),
                    _ => {
                        self.error(
                            format!(
                                "type mismatch on {}: got {} and {}",
                                op_str(op),
                                left.info(),
                                right.info()
                            ),
                            span,
                        );
                        CheckType::Unknown
                    }
                }
            }

            // comparisons should be same type
            TokenType::Less
            | TokenType::Greater
            | TokenType::LessEqual
            | TokenType::GreaterEqual => match (&left, &right) {
                (
                    CheckType::Known(TypeAnnotation::Int | TypeAnnotation::CInt),
                    CheckType::Known(TypeAnnotation::Int | TypeAnnotation::CInt),
                )
                | (
                    CheckType::Known(TypeAnnotation::Float | TypeAnnotation::CFloat),
                    CheckType::Known(TypeAnnotation::Float | TypeAnnotation::CFloat),
                ) => CheckType::Known(TypeAnnotation::Bool),
                _ => {
                    self.error(
                        format!(
                            "type mismatch on {}: got {} and {}",
                            op_str(op),
                            left.info(),
                            right.info()
                        ),
                        span,
                    );
                    CheckType::Unknown
                }
            },

            // equality should be between same types
            TokenType::Compare | TokenType::BangEqual => {
                let ok = match (&left, &right) {
                    (
                        CheckType::Known(TypeAnnotation::Int | TypeAnnotation::CInt),
                        CheckType::Known(TypeAnnotation::Int | TypeAnnotation::CInt),
                    )
                    | (
                        CheckType::Known(TypeAnnotation::Float | TypeAnnotation::CFloat),
                        CheckType::Known(TypeAnnotation::Float | TypeAnnotation::CFloat),
                    )
                    | (
                        CheckType::Known(TypeAnnotation::String | TypeAnnotation::CString),
                        CheckType::Known(TypeAnnotation::String | TypeAnnotation::CString),
                    )
                    | (
                        CheckType::Known(TypeAnnotation::Char | TypeAnnotation::CChar),
                        CheckType::Known(TypeAnnotation::Char | TypeAnnotation::CChar),
                    )
                    | (
                        CheckType::Known(TypeAnnotation::Bool | TypeAnnotation::CBool),
                        CheckType::Known(TypeAnnotation::Bool | TypeAnnotation::CBool),
                    ) => true,
                    _ => false,
                };
                if !ok {
                    self.error(
                        format!(
                            "type mismatch on {}: got {} and {}",
                            op_str(op),
                            left.info(),
                            right.info()
                        ),
                        span,
                    );
                }
                CheckType::Known(TypeAnnotation::Bool)
            }

            // unknown operator
            _ => {
                self.error(format!("unknown binary operator {:?}", op), span);
                CheckType::Unknown
            }
        }
    }
}
