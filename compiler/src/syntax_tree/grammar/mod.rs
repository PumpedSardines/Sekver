use super::super::tokenize::*;
use super::syntax::*;

pub mod blocks;
pub mod expression;
pub mod get_type;

pub fn eval(tokens: &[Token]) -> Result<Vec<GlobalBlock>, ParsingError> {
    let mut i = 0;

    let mut global_blocks: Vec<GlobalBlock> = Vec::new();
    let mut is_exp = false;

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
                    reason: "Can't finding starting parentheses for function",
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
                    reason: "Can't finding ending parentheses for function",
                });
            }

            global_blocks.push(GlobalBlock::Function(func_eval(
                &tokens[i..=curly_end],
                is_exp,
            )?));
            is_exp = false;

            i = curly_end + 1;
        } else if tokens[i].key == TokenKey::Constant {
            let mut semicolon_index = 0;

            for ii in i..tokens.len() {
                if tokens[ii].key == TokenKey::SemiColon {
                    semicolon_index = ii;
                    break;
                }
            }

            if semicolon_index == 0 {
                return Err(ParsingError {
                    col: tokens[i].col,
                    ln: tokens[i].ln,
                    reason: "Can't finding ending semicolon for init constant statement",
                });
            }

            global_blocks.push(GlobalBlock::ConstantInit(constant_eval(
                &tokens[i..=semicolon_index],
                is_exp,
            )?));

            is_exp = false;

            i = semicolon_index + 1;
        } else if tokens[i].key == TokenKey::Import {
            if is_exp {
                return Err(ParsingError {
                    col: tokens[i].col,
                    ln: tokens[i].ln,
                    reason: "Unexpected token exp",
                });
            }
            let mut semicolon_index = 0;

            for ii in i..tokens.len() {
                if tokens[ii].key == TokenKey::SemiColon {
                    semicolon_index = ii;
                    break;
                }
            }

            if semicolon_index == 0 {
                return Err(ParsingError {
                    col: tokens[i].col,
                    ln: tokens[i].ln,
                    reason: "Can't finding ending semicolon for import statement",
                });
            }

            global_blocks.push(GlobalBlock::Import(import_eval(
                &tokens[i..=semicolon_index],
            )?));

            i = semicolon_index + 1;
        } else if tokens[i].key == TokenKey::Export {
            if is_exp {
                return Err(ParsingError {
                    col: tokens[i].col,
                    ln: tokens[i].ln,
                    reason: "Unexpected token exp",
                });
            }

            is_exp = true;
            i += 1;
        } else {
            return Err(ParsingError {
                col: tokens[i].col,
                ln: tokens[i].ln,
                reason: "Unrecognized word",
            });
        }
    }

    return Ok(global_blocks);
}

pub fn func_eval(tokens: &[Token], exp: bool) -> Result<Function, ParsingError> {
    let mut name = String::new();

    for i in 0..tokens.len() {
        let token = &tokens[i];

        if i == 0 && token.key != TokenKey::Function {
            return Err(ParsingError {
                ln: token.ln,
                col: token.col,
                reason: "Expected keyword fnc",
            });
        }

        if i == 1 {
            if token.key != TokenKey::Keyword {
                return Err(ParsingError {
                    ln: token.ln,
                    col: token.col,
                    reason: "Expected name on function",
                });
            }

            name = String::from(token.raw.as_ref().unwrap());
        }

        if token.key == TokenKey::CurlyBraceLeft {
            return Ok(Function {
                col: tokens[0].col,
                ln: tokens[0].ln,
                ret: Types::Empty,
                exp: exp,
                params: Vec::new(),
                name,
                blocks: blocks::eval(&tokens[(i + 1)..(tokens.len() - 1)])?,
            });
        }
    }

    panic!("Couldn't evaluate function");
}

pub fn import_eval(tokens: &[Token]) -> Result<Import, ParsingError> {
    if tokens.len() != 5 {
        return Err(ParsingError {
            ln: tokens[0].ln,
            col: tokens[0].col,
            reason: "Too many tokens in keyword import block",
        });
    }

    if tokens[0].key != TokenKey::Import {
        return Err(ParsingError {
            ln: tokens[0].ln,
            col: tokens[0].col,
            reason: "Expected keyword imp",
        });
    }

    if tokens[1].key != TokenKey::Keyword {
        return Err(ParsingError {
            ln: tokens[1].ln,
            col: tokens[1].col,
            reason: "Expected a keyword",
        });
    }

    if tokens[4].key != TokenKey::SemiColon {
        return Err(ParsingError {
            ln: tokens[2].ln,
            col: tokens[2].col,
            reason: "Expected keyword ;",
        });
    }

    let expression = expression::eval(&tokens[3..4])?;

    if let Expression::Str(value) = expression {
        return Ok(Import {
            col: tokens[0].col,
            ln: tokens[0].ln,
            namespace: tokens[1].clone().raw.unwrap(),
            from: value.val,
        });
    }

    panic!("Couldn't evaluate function");
}

pub fn constant_eval(tokens: &[Token], exp: bool) -> Result<ConstantInit, ParsingError> {
    if tokens.len() < 6 {
        return Err(ParsingError {
            ln: tokens[0].ln,
            col: tokens[0].col,
            reason: "Invalid variable initalization",
        });
    }

    let order = [
        TokenKey::Constant,
        TokenKey::Keyword,
        TokenKey::Colon,
        TokenKey::Keyword,
        TokenKey::Assign,
    ];

    for (i, keyword) in order.iter().enumerate() {
        if tokens[i].key != *keyword {
            return Err(ParsingError {
                ln: tokens[i].ln,
                col: tokens[i].col,
                reason: "Invalid word",
            });
        }
    }

    let tpe = get_type::eval(&tokens[3])?;
    let set_tokens = &tokens[5..(tokens.len() - 1)];

    let invalid_type = match tpe {
        Types::Matrix => true,
        Types::List => true,
        _ => false,
    };

    if invalid_type {
        return Err(ParsingError {
            ln: tokens[3].ln,
            col: tokens[3].col,
            reason: "Invalid type for a constant variable",
        });
    }

    return Ok(ConstantInit {
        col: tokens[0].col,
        ln: tokens[0].ln,
        tpe,
        exp: exp,
        name: String::from(tokens[1].raw.as_ref().unwrap()),
        value: expression::eval(set_tokens)?,
    });
}
