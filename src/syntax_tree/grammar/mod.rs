use super::super::tokenize::*;
use super::syntax::*;

pub mod get_type;
pub mod blocks;
pub mod expression;

pub fn eval(tokens: &[Token]) -> Result<Vec<GlobalBlock>, ParsingError> {
    let mut i = 0;

    let mut global_blocks: Vec<GlobalBlock> = Vec::new();

    while i < tokens.len() {
        if tokens[i].key == TokenKey::Function {
            let mut curly_start = 0;

            for ii in i..tokens.len() {
                if tokens[ii].key == TokenKey::CurlyBraceLeft {
                    curly_start = ii;
                    break;
                }
            }

            if curly_start == 0 {
                return Err(ParsingError {
                    col: tokens[i].col,
                    ln: tokens[i].ln,
                    reason: "Can't finding starting parentheses for function"
                });
            }

            // To keep count on how many brackets we've looped trough all
            let mut curly_count = 1;
            let mut curly_end = 0;

            for ii in (curly_start + 1)..tokens.len() {
                if tokens[ii].key == TokenKey::CurlyBraceLeft {
                    curly_count += 1;
                    continue;
                }

                if tokens[ii].key == TokenKey::CurlyBraceRight {
                    curly_count -= 1;

                    if curly_count == 0 {
                        curly_end = ii;
                        break;
                    }
                }
            }

            if curly_end == 0 {
                return Err(ParsingError {
                    col: tokens[i].col,
                    ln: tokens[i].ln,
                    reason: "Can't finding ending parentheses for function"
                });
            }

            global_blocks.push(GlobalBlock::Function(func_eval(
                &tokens[i..=curly_end],
            )?));

            i = curly_end + 1;
        } else {
            return Err(ParsingError {
                col: tokens[i].col,
                ln: tokens[i].ln,
                reason: "Unrecognized word"
            });
        }
    }

    return Ok(global_blocks);
}

pub fn func_eval(tokens: &[Token]) -> Result<Function, ParsingError> {
    let mut name = String::new();

    for i in 0..tokens.len() {
        let token = &tokens[i];

        if i == 0 && token.key != TokenKey::Function {
            return Err(ParsingError{
                ln: token.ln,
                col: token.col,
                reason: "Expected keyword func"
            });
        }

        if i == 1 {
            if token.key != TokenKey::Keyword {
                return Err(ParsingError{
                    ln: token.ln,
                    col: token.col,
                    reason: "Expected name on function"
                });
            }

            name = String::from(token.raw.as_ref().unwrap());
        }

        if token.key == TokenKey::CurlyBraceLeft {
            return Ok(Function {
                col: tokens[0].col,
                ln: tokens[0].ln,
                ret: Types::Empty,
                params: Vec::new(),
                name,
                blocks: blocks::eval(&tokens[(i+1)..(tokens.len() - 1)])?
            });
        }
    }


    panic!("Couldn't evaluate function");
}
