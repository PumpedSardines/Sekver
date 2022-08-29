use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Copy)]
pub enum TokenKey {
    // Keywords
    Return,
    While,
    True,
    False,
    If,
    Function,
    Mixin,
    Variable,
    Constant,
    Import,
    From,
    Export,

    // Generic
    ParentheseLeft,
    ParentheseRight,
    BracketRight,
    BracketLeft,
    CurlyBraceRight,
    CurlyBraceLeft,
    DoubleColon,
    SemiColon,
    Colon,
    Comma,

    // Expression related
    Addition,
    Subtraction,
    Multiplication,
    Division,
    Assign,
    Power,
    AdditionAssign,
    SubtractAssign,
    Equals,
    NotEquals,
    GreaterThan,
    GreaterThanOrEquals,
    LessThan,
    LessThanOrEquals,
    Not,
    And,
    Or,

    // Values
    PrimitiveNumber,
    PrimitiveString,
    Keyword,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct Token {
    pub key: TokenKey,
    pub ln: usize,
    pub col: usize,
    pub raw: Option<String>,
}
