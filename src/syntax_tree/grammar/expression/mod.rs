use super::*;

pub fn parse_string(string: &str) -> String {
    let mut v = string.chars();
    v.next();
    v.next_back();
    return String::from(v.as_str());
}

#[derive(Debug, Clone)]
enum InterExpr {
    ArithmeticOperator(ArithmeticOperator),
    Expression(Expression),
}

// This function evaluate expressions.
pub fn eval(tokens: &[Token]) -> Result<Expression, ParsingError> {
    if tokens.len() == 1 {
        return eval_single(&tokens[0]);
    }

    let mut combo_blocks: Vec<InterExpr> = Vec::new();
    let mut p_count = 0;
    let mut p_start = 0;
    let mut p_func = false;
    let mut p_eval = false;

    let mut b_count = 0;
    let mut b_start = 0;
    let mut b_eval = false;

    // First loop trough all and evaluate function calls and parentheses

    for i in 0..tokens.len() {
        let token = &tokens[i];

        // Check if this keyword is for a function or list/matrix
        // Skip it, because it will be evaluated to a variable
        if token.key == TokenKey::Keyword
            && i != tokens.len() - 1
            && (tokens[i + 1].key == TokenKey::ParentheseLeft
                || tokens[i + 1].key == TokenKey::BracketLeft)
        {
            continue;
        }

        // Bracket eval
        if token.key == TokenKey::BracketLeft && !p_eval {
            if !p_eval && !b_eval {
                b_eval = true;
                b_start = i;
                if i == 0 || tokens[i - 1].key != TokenKey::Keyword {
                    return Err(ParsingError {
                        col: token.col,
                        ln: token.ln,
                        reason: "Unknown keyword",
                    });
                }
            }
            b_count += 1;
            continue;
        }

        if token.key == TokenKey::BracketRight && !p_eval {
            if !b_eval && !p_eval {
                return Err(ParsingError::from(token, "Unrecognized token"));
            }
            b_count -= 1;

            if b_count == 0 {
                let b_end = i;
                let in_peranthese = &tokens[(b_start + 1)..(b_end)];

                let func_tok = &tokens[b_start - 1];

                combo_blocks.push(InterExpr::Expression(eval_bracket_call(
                    func_tok,
                    in_peranthese,
                )?));
                b_eval = false;
            }
            continue;
        }

        // Parentheses evaluation
        if token.key == TokenKey::ParentheseLeft && !b_eval {
            if !p_eval {
                p_eval = true;
                p_start = i;
                if i != 0 && tokens[i - 1].key == TokenKey::Keyword {
                    p_func = true;
                }
            }
            p_count += 1;
            continue;
        }

        if token.key == TokenKey::ParentheseRight && !b_eval {
            if !p_eval {
                return Err(ParsingError::from(token, "Unrecognized token"));
            }
            p_count -= 1;

            if p_count == 0 {
                let paranthese_end = i;
                let in_peranthese = &tokens[(p_start + 1)..(paranthese_end)];

                if p_func {
                    let func_tok = &tokens[p_start - 1];
                    combo_blocks.push(InterExpr::Expression(Expression::FunctionCall(
                        FunctionCall {
                            ln: func_tok.ln,
                            col: func_tok.col,
                            name: func_tok.raw.as_ref().unwrap().clone(),
                            params: eval_func_args(in_peranthese)?,
                        },
                    )));
                } else {
                    combo_blocks.push(InterExpr::Expression(eval(in_peranthese)?));
                }

                p_eval = false;
            }
            continue;
        }

        if !p_eval && !b_eval {
            combo_blocks.push(eval_unknown_token(&token)?);
        }
    }

    // The combo blocks can now be evaluated in passes recursively
    let expression = eval_combo_blocks(&combo_blocks)?;

    return Ok(expression);
}

fn eval_combo_blocks(tokens: &[InterExpr]) -> Result<Expression, ParsingError> {
    if tokens.len() == 0 {
        panic!("single");
    }

    if tokens.len() == 1 {
        return match &tokens[0] {
            InterExpr::Expression(val) => Ok(val.clone()),
            InterExpr::ArithmeticOperator(val) => Err(ParsingError {
                col: val.col,
                ln: val.ln,
                reason: "Unexpected operator",
            }),
        };
    }

    // Denotes in what order operators take priority
    // Decreasing order
    const PASSES: &'static [&'static [ArithmeticOperators]] = &[
        &[
            ArithmeticOperators::Assign,
            ArithmeticOperators::AdditionAssign,
            ArithmeticOperators::SubtractAssign,
        ],
        &[ArithmeticOperators::And, ArithmeticOperators::Or],
        &[
            ArithmeticOperators::Equals,
            ArithmeticOperators::NotEquals,
            ArithmeticOperators::GreaterThan,
            ArithmeticOperators::GreaterThanOrEquals,
            ArithmeticOperators::LessThan,
            ArithmeticOperators::LessThanOrEquals,
        ],
        &[
            ArithmeticOperators::Addition,
            ArithmeticOperators::Subtraction,
        ],
        &[
            ArithmeticOperators::Multiplication,
            ArithmeticOperators::Division,
        ],
        &[ArithmeticOperators::Not],
    ];

    // This may seem inefficient
    // And it is...
    // But it still scales linearly
    // Since passes is a constant
    // So not as bad as first thought
    for current in PASSES {
        for i in (0..tokens.len()).rev() {
            let token = &tokens[i];

            let operator = match token {
                InterExpr::ArithmeticOperator(op) => op,
                _ => continue,
            };

            if i == 0 || i == tokens.len() - 1 {
                // Edge case for ! operator
                // For example !true
                if operator.operator == ArithmeticOperators::Not && i != tokens.len() - 1 {
                    match &tokens[i + 1] {
                        InterExpr::Expression(_) => (),
                        InterExpr::ArithmeticOperator(_) => {
                            return Err(ParsingError {
                                col: operator.col,
                                ln: operator.ln,
                                reason: "Can't negate operator",
                            });
                        }
                    }
                // Edge case for - operator
                // For example -3 + 4
                } else if operator.operator == ArithmeticOperators::Subtraction
                    && i != tokens.len() - 1
                {
                    match &tokens[i + 1] {
                        InterExpr::Expression(_) => (),
                        InterExpr::ArithmeticOperator(_) => {
                            return Err(ParsingError {
                                col: operator.col,
                                ln: operator.ln,
                                reason: "Can't negate operator",
                            });
                        }
                    }
                } else {
                    return Err(ParsingError {
                        ln: operator.ln,
                        col: operator.col,
                        reason: match i {
                            0 => "Can't start expression with a operator",
                            _ => "Can't end expression with a operator",
                        },
                    });
                }
            }

            if current.contains(&operator.operator) {
                // edge case for not operator
                if operator.operator == ArithmeticOperators::Not {
                    let new = [
                        &tokens[..i],
                        &[InterExpr::Expression(Expression::Not(Not {
                            ln: operator.ln,
                            col: operator.col,
                            val: Box::new(match tokens[i + 1].clone() {
                                InterExpr::Expression(val) => val,
                                InterExpr::ArithmeticOperator(_) => panic!("Shouldn't be possible"),
                            }),
                        }))],
                        &tokens[(i + 2)..],
                    ]
                    .concat();
                    return eval_combo_blocks(&new);
                }

                // Edgecase for subtract
                if operator.operator == ArithmeticOperators::Subtraction {
                    let run_special = i == 0
                        || match &tokens[i - 1] {
                            InterExpr::ArithmeticOperator(_) => true,
                            _ => false,
                        };

                    if run_special {
                        let new = [
                            &tokens[..i],
                            &[InterExpr::Expression(Expression::Negate(Negate {
                                ln: operator.ln,
                                col: operator.col,
                                val: Box::new(match tokens[i + 1].clone() {
                                    InterExpr::Expression(val) => val,
                                    InterExpr::ArithmeticOperator(_) => {
                                        panic!("Shouldn't be possible")
                                    }
                                }),
                            }))],
                            &tokens[(i + 2)..],
                        ]
                        .concat();
                        return eval_combo_blocks(&new);
                    }
                }

                // Default behaviour
                return Ok(Expression::Arithmetic(Arithmetic {
                    ln: operator.ln,
                    col: operator.col,
                    operator: operator.clone(),
                    a: Box::new(eval_combo_blocks(&tokens[..i])?),
                    b: Box::new(eval_combo_blocks(&tokens[(i + 1)..])?),
                }));
            }
        }
    }

    return Err(ParsingError {
        ln: 0,
        col: 0,
        reason: "any",
    });
}

fn eval_bracket_call(i_token: &Token, tokens: &[Token]) -> Result<Expression, ParsingError> {
    let mut p_count = 0;
    let mut b_count = 0;
    // Ignore first and last

    for i in 0..tokens.len() {
        let token = &tokens[i];

        if token.key == TokenKey::Comma && p_count == 0 && b_count == 0 {
            return Ok(Expression::MatrixCall(MatrixCall {
                col: tokens[0].col,
                ln: tokens[0].ln,
                params: Box::new((
                    expression::eval(&tokens[0..i])?,
                    expression::eval(&tokens[(i + 1)..tokens.len()])?,
                )),
                name: i_token.raw.as_ref().unwrap().clone(),
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

    return Ok(Expression::ListCall(ListCall {
        col: tokens[0].col,
        ln: tokens[0].ln,
        params: Box::new(expression::eval(&tokens[0..tokens.len()])?),
        name: i_token.raw.as_ref().unwrap().clone(),
    }));
}

fn eval_func_args(tokens: &[Token]) -> Result<Vec<Expression>, ParsingError> {
    if tokens.len() == 0 {
        return Ok(Vec::new());
    }

    let mut params: Vec<Expression> = Vec::new();

    let mut p_count = 0;
    let mut b_count = 0;
    let mut comma_i = 0;
    // Ignore first and last
    for i in 0..tokens.len() {
        let token = &tokens[i];

        if token.key == TokenKey::Comma && p_count == 0 && b_count == 0 {
            params.push(expression::eval(&tokens[comma_i..i])?);

            comma_i = i + 1;
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

    params.push(expression::eval(&tokens[comma_i..tokens.len()])?);

    return Ok(params);
}

fn eval_unknown_token(token: &Token) -> Result<InterExpr, ParsingError> {
    return match token.key {
        TokenKey::Addition => Ok(InterExpr::ArithmeticOperator(ArithmeticOperator {
            col: token.col,
            ln: token.ln,
            operator: ArithmeticOperators::Addition,
        })),
        TokenKey::Subtraction => Ok(InterExpr::ArithmeticOperator(ArithmeticOperator {
            col: token.col,
            ln: token.ln,
            operator: ArithmeticOperators::Subtraction,
        })),
        TokenKey::Multiplication => Ok(InterExpr::ArithmeticOperator(ArithmeticOperator {
            col: token.col,
            ln: token.ln,
            operator: ArithmeticOperators::Multiplication,
        })),
        TokenKey::Division => Ok(InterExpr::ArithmeticOperator(ArithmeticOperator {
            col: token.col,
            ln: token.ln,
            operator: ArithmeticOperators::Division,
        })),
        TokenKey::Assign => Ok(InterExpr::ArithmeticOperator(ArithmeticOperator {
            col: token.col,
            ln: token.ln,
            operator: ArithmeticOperators::Assign,
        })),
        TokenKey::AdditionAssign => Ok(InterExpr::ArithmeticOperator(ArithmeticOperator {
            col: token.col,
            ln: token.ln,
            operator: ArithmeticOperators::AdditionAssign,
        })),
        TokenKey::SubtractAssign => Ok(InterExpr::ArithmeticOperator(ArithmeticOperator {
            col: token.col,
            ln: token.ln,
            operator: ArithmeticOperators::SubtractAssign,
        })),
        TokenKey::Equals => Ok(InterExpr::ArithmeticOperator(ArithmeticOperator {
            col: token.col,
            ln: token.ln,
            operator: ArithmeticOperators::Equals,
        })),
        TokenKey::NotEquals => Ok(InterExpr::ArithmeticOperator(ArithmeticOperator {
            col: token.col,
            ln: token.ln,
            operator: ArithmeticOperators::NotEquals,
        })),
        TokenKey::GreaterThan => Ok(InterExpr::ArithmeticOperator(ArithmeticOperator {
            col: token.col,
            ln: token.ln,
            operator: ArithmeticOperators::GreaterThan,
        })),
        TokenKey::GreaterThanOrEquals => Ok(InterExpr::ArithmeticOperator(ArithmeticOperator {
            col: token.col,
            ln: token.ln,
            operator: ArithmeticOperators::GreaterThanOrEquals,
        })),
        TokenKey::LessThan => Ok(InterExpr::ArithmeticOperator(ArithmeticOperator {
            col: token.col,
            ln: token.ln,
            operator: ArithmeticOperators::LessThan,
        })),
        TokenKey::LessThanOrEquals => Ok(InterExpr::ArithmeticOperator(ArithmeticOperator {
            col: token.col,
            ln: token.ln,
            operator: ArithmeticOperators::LessThanOrEquals,
        })),
        TokenKey::Not => Ok(InterExpr::ArithmeticOperator(ArithmeticOperator {
            col: token.col,
            ln: token.ln,
            operator: ArithmeticOperators::Not,
        })),
        TokenKey::And => Ok(InterExpr::ArithmeticOperator(ArithmeticOperator {
            col: token.col,
            ln: token.ln,
            operator: ArithmeticOperators::And,
        })),
        TokenKey::Or => Ok(InterExpr::ArithmeticOperator(ArithmeticOperator {
            col: token.col,
            ln: token.ln,
            operator: ArithmeticOperators::Or,
        })),
        _ => Ok(InterExpr::Expression(eval_single(token)?)),
    };
}

fn eval_single(token: &Token) -> Result<Expression, ParsingError> {
    return match token.key {
        TokenKey::True => Ok(Expression::Boolean(Boolean {
            col: token.col,
            ln: token.ln,
            val: true,
        })),
        TokenKey::False => Ok(Expression::Boolean(Boolean {
            col: token.col,
            ln: token.ln,
            val: false,
        })),
        TokenKey::PrimitiveString => Ok(Expression::Str(Str {
            col: token.col,
            ln: token.ln,
            val: parse_string(token.raw.as_ref().unwrap()),
        })),
        TokenKey::PrimitiveNumber => Ok(Expression::Number(Number {
            col: token.col,
            ln: token.ln,
            num: token.raw.as_ref().unwrap().parse().unwrap(),
        })),
        TokenKey::Keyword => Ok(Expression::Variable(Variable {
            col: token.col,
            ln: token.ln,
            name: String::from(token.raw.as_ref().unwrap()),
        })),
        _ => Err(ParsingError {
            col: token.col,
            ln: token.ln,
            reason: "Unrecognized word",
        }),
    };
}
