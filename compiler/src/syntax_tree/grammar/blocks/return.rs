use super::*;

pub fn eval(tokens: &[Token]) -> Result<Block, ParsingError> {
    if tokens.len() == 2 {
        return Ok(Block::Return(Return {
            ln: tokens[0].ln,
            col: tokens[0].col,
            eval: None
        }));
    }

    return Ok(Block::Return(Return {
        ln: tokens[0].ln,
        col: tokens[0].col,
        eval: Some(expression::eval(&tokens[1..(tokens.len() - 1)])?)
    }));
}