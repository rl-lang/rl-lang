use crate::{
    ast::statements::TypeAnnotation,
    checker::structs::{CheckType, TypeChecker},
    lexer::tokentypes::TokenType,
    utils::span::Span,
};

impl TypeChecker {
    pub fn check_unary_operator(
        &mut self,
        operand: CheckType,
        _operand_span: Span,
        op: &TokenType,
        span: Span,
    ) -> CheckType {
        if operand.is_unknown() {
            return CheckType::Unknown;
        }
        match op {
            // is it corrent bang unary?
            TokenType::Bang => match &operand {
                CheckType::Known(TypeAnnotation::Bool | TypeAnnotation::CBool) => {
                    CheckType::Known(TypeAnnotation::Bool)
                }
                _ => {
                    self.error(format!("type mismatch on !: got {}", operand.info()), span);
                    CheckType::Unknown
                }
            },
            // is it correect minus unary?
            TokenType::Minus => match &operand {
                CheckType::Known(TypeAnnotation::Int | TypeAnnotation::CInt) => {
                    CheckType::Known(TypeAnnotation::Int)
                }
                CheckType::Known(TypeAnnotation::Float | TypeAnnotation::CFloat) => {
                    CheckType::Known(TypeAnnotation::Float)
                }
                _ => {
                    self.error(
                        format!("type mismatch on unary -: got {}", operand.info()),
                        span,
                    );
                    CheckType::Unknown
                }
            },
            // undefined unary
            _ => {
                self.error(format!("unknown unary operator {:?}", op), span);
                CheckType::Unknown
            }
        }
    }
}
