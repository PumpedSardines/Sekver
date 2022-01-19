use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub enum Types {
    Boolean,
    String,
    Number,
    Matrix,
    List,
    Empty,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Number {
    pub col: usize,
    pub ln: usize,
    pub num: f64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Str {
    pub col: usize,
    pub ln: usize,
    pub val: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Boolean {
    pub col: usize,
    pub ln: usize,
    pub val: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Variable {
    pub col: usize,
    pub ln: usize,
    pub name: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FunctionCall {
    pub col: usize,
    pub ln: usize,
    pub params: Vec<Expression>,
    pub name: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MatrixCall {
    pub col: usize,
    pub ln: usize,
    pub params: Box<(Expression, Expression)>,
    pub name: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ListCall {
    pub col: usize,
    pub ln: usize,
    pub params: Box<Expression>,
    pub name: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub enum ArithmeticOperators {
    Assign,
    AdditionAssign,
    SubtractAssign,
    Addition,
    Subtraction,
    Multiplication,
    Division,
    Equals,
    NotEquals,
    GreaterThan,
    Not,
    GreaterThanOrEquals,
    LessThan,
    LessThanOrEquals,
    And,
    Or,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ArithmeticOperator {
    pub col: usize,
    pub ln: usize,
    pub operator: ArithmeticOperators
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MatrixInit {
    pub col: usize,
    pub ln: usize,
    pub width: Expression,
    pub height: Expression,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ListInit {
    pub col: usize,
    pub ln: usize,
    pub length: Expression,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Arithmetic {
    pub col: usize,
    pub ln: usize,
    pub operator: ArithmeticOperator,
    pub a: Box<Expression>,
    pub b: Box<Expression>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Not {
    pub col: usize,
    pub ln: usize,
    pub val: Box<Expression>, 
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Negate {
    pub col: usize,
    pub ln: usize,
    pub val: Box<Expression>, 
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Expression {
    Number(Number),
    Boolean(Boolean),
    Str(Str),
    Not(Not),
    Negate(Negate),
    Variable(Variable),
    FunctionCall(FunctionCall),
    MatrixCall(MatrixCall),
    ListCall(ListCall),
    Arithmetic(Arithmetic),
}

#[derive(Serialize, Deserialize, Debug)]
pub struct While {
    pub col: usize,
    pub ln: usize,
    pub eval: Expression,
    pub blocks: Vec<Block>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct If {
    pub col: usize,
    pub ln: usize,
    pub eval: Expression,
    pub blocks: Vec<Block>,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum VariableInitValues {
    Expression(Expression),
    MatrixInit(MatrixInit),
    ListInit(ListInit)
}

#[derive(Serialize, Deserialize, Debug)]
pub struct VariableInit {
    pub col: usize,
    pub ln: usize,
    pub value: VariableInitValues,
    pub tpe: Types,
    pub name: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct VariableSet {
    pub col: usize,
    pub ln: usize,
    pub value: Expression,
    pub name: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Return {
    pub col: usize,
    pub ln: usize,
    pub eval: Option<Expression>
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Block {
    VariableSet(VariableSet),
    VariableInit(VariableInit),
    Expression(Expression),
    While(While),
    Return(Return),
    If(If),
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Function {
    pub col: usize,
    pub ln: usize,
    pub ret: Types,
    pub params: Vec<Types>,
    pub name: String,
    pub blocks: Vec<Block>,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum GlobalBlock {
    Function(Function),
}

pub struct ParsingError {
    pub col: usize,
    pub ln: usize,
    pub reason: &'static str,
}

impl ParsingError {
    pub fn from(token: &super::Token, reason: &'static str) -> ParsingError {
        return ParsingError {
            col: token.col,
            ln: token.ln,
            reason
        };
    }
}