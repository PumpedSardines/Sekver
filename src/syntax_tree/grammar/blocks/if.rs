use super::*;

pub fn eval(tokens: &[Token]) -> Result<Block, ParsingError> {
    let mut curly_start = 0;
    let mut expr = None;

    for i in 0..tokens.len() {
        if tokens[i].key == TokenKey::CurlyBraceLeft {
            curly_start = i + 1;

            expr = Some(expression::eval(&tokens[1..i])?);
            break;
        }
    }

    let expr = match expr {
        Some(v) => v,
        None => {
            return Err(ParsingError {
                col: tokens[0].col,
                ln: tokens[0].ln,
                reason: "Couldn't find start '{' for if",
            });
        }
    };

    let mut curly_count = 1;
    for i in curly_start..tokens.len() {
        if tokens[i].key == TokenKey::CurlyBraceLeft {
            curly_count += 1;
        }

        if tokens[i].key == TokenKey::CurlyBraceRight {
            curly_count -= 1;

            if curly_count == 0 {
                let curly_end = i;

                return Ok(Block::If(If {
                    col: tokens[0].col,
                    ln: tokens[0].ln,
                    eval: expr,
                    blocks: super::eval(&tokens[curly_start..curly_end])?,
                }));
            }
        }
    }

    return Err(ParsingError {
        col: tokens[0].col,
        ln: tokens[0].ln,
        reason: "Couldn't find ending '}' for if",
    });
}
