use anyhow::anyhow;
use crate::compiler::firelang_lexer::lexer::{Token, TokenKind};
#[derive(Debug,PartialEq, Eq, PartialOrd, Ord,Clone)]
pub enum Type {
    Int,
    Char,
    Float,
    //TODO: Add more type because I am not ensure.
}
impl TryFrom<Token> for Type {
    type Error = anyhow::Error;
    fn try_from(value: Token) -> Result<Self, Self::Error> {
        if value.kind != TokenKind::Ident {
            return Err(anyhow!("Error: Cannot transform an un-ident token into type at line {} column {}",value.line,value.column));
        }
        if value.content.parse::<i128>().is_ok() {
            return Ok(Self::Int);
        }
        else if value.content.parse::<f64>().is_ok() {
            return Ok(Self::Float);
        }
        else if value.content.len() == 1{
            return Ok(Self::Char);
        }
        Err(anyhow!("Error: Undefined token at line {} column {}",value.line,value.column))
    }
}

pub enum BinaryOperator {
    Addition,
    Subtraction,
    Multiplication,
    Division,
    Modulus,
    LessThan,
    LessThanOrEqual,
    GreaterThan,
    GreaterThanOrEqual,
    Equal,
    NotEqual,
    And,
    Or,
    AddAssign,
    SubtractAssign,
    MultiplyAssign,
    DivideAssign,
}
impl TryFrom<TokenKind> for BinaryOperator {
    type Error = anyhow::Error;
    fn try_from(token: TokenKind) -> Result<BinaryOperator, Self::Error> {
        match token {
            TokenKind::Star => Ok(BinaryOperator::Multiplication),
            TokenKind::Slash => Ok(BinaryOperator::Division),
            TokenKind::Plus => Ok(BinaryOperator::Addition),
            TokenKind::Minus => Ok(BinaryOperator::Subtraction),
            TokenKind::Percent => Ok(BinaryOperator::Modulus),
            TokenKind::And => Ok(BinaryOperator::And),
            TokenKind::Or => Ok(BinaryOperator::Or),
            //TODO : dectect content
            other => Err(anyhow!(
                "Token {:?} cannot be converted into a BinaryOperator",
                other
            )),
        }
    }
}