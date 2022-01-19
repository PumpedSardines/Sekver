use super::*;

pub fn eval(token: &Token) -> Result<Types, ParsingError> {

    return match token.raw.as_ref().unwrap().as_str() {
        "str" => Ok(Types::String),
        "num" => Ok(Types::Number),
        "bln" => Ok(Types::Boolean),
        "mtx" => Ok(Types::Matrix),
        "lst" => Ok(Types::List),
        "emp" => Ok(Types::Empty),
        _ => Err(ParsingError {
            ln: token.ln,
            col: token.col,
            reason: "Unrecognized type"
        })
    };

}