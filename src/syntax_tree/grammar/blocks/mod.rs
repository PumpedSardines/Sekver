use super::*;
mod variable_init;
mod r#while;
mod r#if;
mod r#return;

pub fn eval(tokens: &[Token]) -> Result<Vec<Block>, ParsingError> {
    let mut blocks = Vec::new();

    let mut i = 0;
    'main: while i < tokens.len() {
        let token = &tokens[i];

        if token.key == TokenKey::Variable {
            for ii in i..tokens.len() {
                if tokens[ii].key == TokenKey::SemiColon {
                    let block = variable_init::eval(&tokens[i..=ii])?;
                    blocks.push(block);
                    i = ii + 1;
                    continue 'main;
                }
            }
        }

        if token.key == TokenKey::Return {
            for ii in i..tokens.len() {
                if tokens[ii].key == TokenKey::SemiColon {
                    let block = r#return::eval(&tokens[i..=ii])?;
                    blocks.push(block);
                    i = ii + 1;
                    continue 'main;
                }
            }
        }

        if token.key == TokenKey::While {
            let mut curly_start = 0;
            for ii in i..tokens.len() {
                if tokens[ii].key == TokenKey::CurlyBraceLeft {
                    curly_start = ii + 1;
                    break;
                }
            }

            if curly_start == 0 {
                return Err(ParsingError{
                    col: token.col,
                    ln: token.ln,
                    reason: "Couldn't find starting '{' for if"
                });
            }

            let mut curly_count = 1;
            for ii in curly_start..tokens.len() {
                if tokens[ii].key == TokenKey::CurlyBraceLeft {
                    curly_count += 1;
                }

                if tokens[ii].key == TokenKey::CurlyBraceRight {
                    curly_count -= 1;

                    if curly_count == 0 {
                        let curly_end = ii;
                        let block = r#while::eval(&tokens[i..=curly_end])?;
                        blocks.push(block);
                        i = curly_end + 1;
                        continue 'main;
                    }
                }
            }
            return Err(ParsingError{
                col: token.col,
                ln: token.ln,
                reason: "Couldn't find ending '}' for while"
            });
        }

        if token.key == TokenKey::If {
            let mut curly_start = 0;
            for ii in i..tokens.len() {
                if tokens[ii].key == TokenKey::CurlyBraceLeft {
                    curly_start = ii + 1;
                    break;
                }
            }

            if curly_start == 0 {
                return Err(ParsingError{
                    col: token.col,
                    ln: token.ln,
                    reason: "Couldn't find starting '{' for if"
                });
            }

            let mut curly_count = 1;
            for ii in curly_start..tokens.len() {
                if tokens[ii].key == TokenKey::CurlyBraceLeft {
                    curly_count += 1;
                }

                if tokens[ii].key == TokenKey::CurlyBraceRight {
                    curly_count -= 1;

                    if curly_count == 0 {
                        let curly_end = ii;
                        let block = r#if::eval(&tokens[i..=curly_end])?;
                        blocks.push(block);
                        i = curly_end + 1;
                        continue 'main;
                    }
                }
            }

            return Err(ParsingError{
                col: token.col,
                ln: token.ln,
                reason: "Couldn't find ending '}' for if"
            });
        }

        for ii in i..tokens.len() {
            if tokens[ii].key == TokenKey::SemiColon {
                let block = Block::Expression(expression::eval(&tokens[i..ii])?);
                blocks.push(block);
                i = ii + 1;
                continue 'main;
            }
        }
    }

    return Ok(blocks);
}
