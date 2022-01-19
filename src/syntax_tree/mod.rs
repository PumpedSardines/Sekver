use super::tokenize::*;
mod syntax;

pub mod grammar;

use syntax::*;

pub fn syntax_tree(tokens: Vec<Token>) -> Result<Vec<GlobalBlock>, ParsingError> {
    return Ok(grammar::eval(&tokens)?);
}