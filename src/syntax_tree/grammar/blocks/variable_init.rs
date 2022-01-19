use super::*;

pub fn eval(tokens: &[Token]) -> Result<Block, ParsingError> {
    if tokens.len() < 6 {
        return Err(ParsingError {
            ln: tokens[0].ln,
            col: tokens[0].col,
            reason: "Invalid variable initalization",
        });
    }

    let order = [
        TokenKey::Variable,
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

    return Ok(Block::VariableInit(VariableInit {
        col: tokens[0].col,
        ln: tokens[0].ln,
        tpe,
        name: String::from(tokens[1].raw.as_ref().unwrap()),
        value: match tpe {
            Types::Matrix => matrix_init_eval(set_tokens)?,
            Types::List => list_init_eval(set_tokens)?,
            _ => VariableInitValues::Expression(expression::eval(set_tokens)?),
        },
    }));
}

fn list_init_eval(tokens: &[Token]) -> Result<VariableInitValues, ParsingError> {
    if tokens[0].key != TokenKey::BracketLeft {
        return Err(ParsingError {
            ln: tokens[0].ln,
            col: tokens[0].col,
            reason: "Invalid token",
        });
    }

    if tokens[tokens.len() - 1].key != TokenKey::BracketRight {
        return Err(ParsingError {
            ln: tokens[tokens.len() - 1].ln,
            col: tokens[tokens.len() - 1].col,
            reason: "Invalid token",
        });
    }

    return Ok(VariableInitValues::ListInit(ListInit {
        col: tokens[0].col,
        ln: tokens[0].ln,
        length: expression::eval(&tokens[1..(tokens.len() - 1)])?,
    }));
}

fn matrix_init_eval(tokens: &[Token]) -> Result<VariableInitValues, ParsingError> {
    if tokens[0].key != TokenKey::BracketLeft {
        return Err(ParsingError {
            ln: tokens[0].ln,
            col: tokens[0].col,
            reason: "Invalid token",
        });
    }

    if tokens[tokens.len() - 1].key != TokenKey::BracketRight {
        return Err(ParsingError {
            ln: tokens[tokens.len() - 1].ln,
            col: tokens[tokens.len() - 1].col,
            reason: "Invalid token",
        });
    }

    let mut p_count = 0;
    let mut b_count = 0;
    let mut comma_i = 0;
    // Ignore first and last
    for i in 1..tokens.len() {
        let token = &tokens[i];

        if token.key == TokenKey::Comma && p_count == 0 && b_count == 0 {
            return Ok(VariableInitValues::MatrixInit(MatrixInit {
                col: tokens[0].col,
                ln: tokens[0].ln,
                width: expression::eval(&tokens[1..i])?,
                height: expression::eval(&tokens[(i + 1)..(tokens.len() - 1)])?,
            }));
        }

        match token.key {
            TokenKey::BracketLeft => b_count += 1,
            TokenKey::BracketRight => b_count -= 1,
            TokenKey::ParentheseLeft => p_count += 1,
            TokenKey::ParentheseRight => p_count -= 1,
            _ => (),
        }

        if p_count < 0 || b_count < 0 {
            return Err(ParsingError {
                ln: tokens[0].ln,
                col: tokens[0].col,
                reason: "Unexpected token",
            });
        }
    }

    return Err(ParsingError {
        ln: tokens[0].ln,
        col: tokens[0].col,
        reason: "Couldn't find ',' in matrix initialization",
    });
}
