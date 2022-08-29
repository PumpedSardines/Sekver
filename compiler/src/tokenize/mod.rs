mod tokens;
use regex::Regex;
pub use tokens::*;

pub fn tokenize(code: &str) -> Result<Vec<Token>, String> {
    let mut tokens: Vec<Token> = vec![];

    let special_char_combos = [
        "===", "!==", "+=", "-=", "==", "!=", ">=", "::", "<=", "&&", "//", "**", "||", "(", ")",
        "[", "]", "{", "}", ";", ":", ",", "+", "-", "*", "/", "=", ">", "<", "!",
    ];

    let lines = code.split('\n');

    for (ln, line) in lines.enumerate() {
        // Every single line is evaluated independently

        let letters: Vec<char> = line.chars().collect();
        let mut is_str = false;
        let mut is_esc = false;
        let mut str_ln = 0;
        let mut cur_str = String::new();

        let mut col = 0;
        'main: while col < letters.len() {
            if letters[col] == '"' {
                cur_str.push_str(&letters[col].to_string());
                if is_str {
                    if !is_esc {
                        // What to happen when a string is finnished
                        is_str = false;
                        tokens.push(get_token(&cur_str, str_ln, col));
                        cur_str = String::new();
                        col += 1;
                        continue;
                    }
                } else {
                    is_str = true;
                    str_ln = ln;
                }
                col += 1;
                continue;
            }

            if is_str {
                if letters[col] == '\\' && !is_esc {
                    is_esc = true;
                    col += 1;
                    continue;
                }

                cur_str.push_str(&letters[col].to_string());
                col += 1;
                is_esc = false;
                continue;
            }

            if letters[col] == ' '
                || letters[col] == '\n'
                || letters[col] == '\r'
                || letters[col] == '\t'
            {
                col += 1;
                continue;
            }

            // This nested loop seems incredibly inefficient
            // But both these loops are fixed in amount of iterations
            // So this still scales linearly
            for special_char_combo in special_char_combos {
                let mut does_match = true;
                let len = special_char_combo.len();

                for i in 0..len {
                    let let_i = col + i;

                    if let_i >= letters.len() {
                        does_match = false;
                        break;
                    }

                    if letters[let_i] != special_char_combo.chars().nth(i).unwrap() {
                        does_match = false;
                        break;
                    }
                }

                if does_match && special_char_combo == "//" {
                    break 'main;
                }

                if does_match {
                    tokens.push(get_token(special_char_combo, ln, col));
                    col += len;
                    continue 'main;
                }
            }

            let re_allowed_chars = Regex::new("^[A-Za-z0-9_\\.]$").unwrap();

            let mut i = 0;
            let mut cur_word = String::new();
            loop {
                let let_i = col + i;

                if let_i >= letters.len() {
                    if cur_word.trim() != "" {
                        tokens.push(get_token(&cur_word, ln, col));
                    }
                    if i == 0 {
                        panic!("Unrecognized character at line: {}, column: {}", ln, col)
                    }
                    col += i;
                    break;
                }

                if !re_allowed_chars.is_match(&letters[let_i].to_string()) {
                    if cur_word.trim() != "" {
                        tokens.push(get_token(&cur_word, ln, col));
                    }
                    if i == 0 {
                        panic!("Unrecognized character at line: {}, column: {}", ln, col)
                    }
                    col += i;
                    break;
                }

                cur_word.push_str(&letters[let_i].to_string());

                i += 1;
            }
        }

        if is_str {
            panic!("String doesn't have ending \" on line {}", ln);
        }
    }

    return Ok(tokens);
}

fn get_token(raw: &str, ln: usize, col: usize) -> Token {
    let ln = ln + 1;
    let col = col + 1;

    return match raw {
        "{" => Token {
            ln,
            col,
            raw: None,
            key: TokenKey::CurlyBraceLeft,
        },
        "}" => Token {
            ln,
            col,
            raw: None,
            key: TokenKey::CurlyBraceRight,
        },
        "(" => Token {
            ln,
            col,
            raw: None,
            key: TokenKey::ParentheseLeft,
        },
        ")" => Token {
            ln,
            col,
            raw: None,
            key: TokenKey::ParentheseRight,
        },
        "[" => Token {
            ln,
            col,
            raw: None,
            key: TokenKey::BracketLeft,
        },
        "]" => Token {
            ln,
            col,
            raw: None,
            key: TokenKey::BracketRight,
        },
        ";" => Token {
            ln,
            col,
            raw: None,
            key: TokenKey::SemiColon,
        },
        ":" => Token {
            ln,
            col,
            raw: None,
            key: TokenKey::Colon,
        },
        "," => Token {
            ln,
            col,
            raw: None,
            key: TokenKey::Comma,
        },
        "+" => Token {
            ln,
            col,
            raw: None,
            key: TokenKey::Addition,
        },
        "**" => Token {
            ln,
            col,
            raw: None,
            key: TokenKey::Power,
        },
        "-" => Token {
            ln,
            col,
            raw: None,
            key: TokenKey::Subtraction,
        },
        "/" => Token {
            ln,
            col,
            raw: None,
            key: TokenKey::Division,
        },
        "*" => Token {
            ln,
            col,
            raw: None,
            key: TokenKey::Multiplication,
        },
        "=" => Token {
            ln,
            col,
            raw: None,
            key: TokenKey::Assign,
        },
        "+=" => Token {
            ln,
            col,
            raw: None,
            key: TokenKey::AdditionAssign,
        },
        "-=" => Token {
            ln,
            col,
            raw: None,
            key: TokenKey::SubtractAssign,
        },
        "==" | "===" => Token {
            ln,
            col,
            raw: None,
            key: TokenKey::Equals,
        },
        "!=" | "!==" => Token {
            ln,
            col,
            raw: None,
            key: TokenKey::NotEquals,
        },
        "::" => Token {
            ln,
            col,
            raw: None,
            key: TokenKey::DoubleColon,
        },
        ">" => Token {
            ln,
            col,
            raw: None,
            key: TokenKey::GreaterThan,
        },
        ">=" => Token {
            ln,
            col,
            raw: None,
            key: TokenKey::GreaterThanOrEquals,
        },
        "<" => Token {
            ln,
            col,
            raw: None,
            key: TokenKey::LessThan,
        },
        "<=" => Token {
            ln,
            col,
            raw: None,
            key: TokenKey::LessThanOrEquals,
        },
        "&&" => Token {
            ln,
            col,
            raw: None,
            key: TokenKey::And,
        },
        "||" => Token {
            ln,
            col,
            raw: None,
            key: TokenKey::Or,
        },
        "!" => Token {
            ln,
            col,
            raw: None,
            key: TokenKey::Not,
        },
        "fnc" => Token {
            ln,
            col,
            raw: None,
            key: TokenKey::Function,
        },
        "mxn" => Token {
            ln,
            col,
            raw: None,
            key: TokenKey::Mixin,
        },
        "var" => Token {
            ln,
            col,
            raw: None,
            key: TokenKey::Variable,
        },
        "cst" => Token {
            ln,
            col,
            raw: None,
            key: TokenKey::Constant,
        },
        "while" => Token {
            ln,
            col,
            raw: None,
            key: TokenKey::While,
        },
        "imp" => Token {
            ln,
            col,
            raw: None,
            key: TokenKey::Import,
        },
        "frm" => Token {
            ln,
            col,
            raw: None,
            key: TokenKey::From,
        },
        "exp" => Token {
            ln,
            col,
            raw: None,
            key: TokenKey::Export,
        },
        "if" => Token {
            ln,
            col,
            raw: None,
            key: TokenKey::If,
        },
        "ret" => Token {
            ln,
            col,
            raw: None,
            key: TokenKey::Return,
        },
        "true" => Token {
            ln,
            col,
            raw: None,
            key: TokenKey::True,
        },
        "false" => Token {
            ln,
            col,
            raw: None,
            key: TokenKey::False,
        },
        raw => {
            let re_number = Regex::new(r"^\d*\.?\d+$").unwrap();
            let re_string = Regex::new("^\".+\"$").unwrap();

            if re_number.is_match(raw) {
                return Token {
                    ln,
                    col,
                    raw: Some(String::from(raw)),
                    key: TokenKey::PrimitiveNumber,
                };
            }

            if re_string.is_match(raw) {
                return Token {
                    ln,
                    col,
                    raw: Some(String::from(raw)),
                    key: TokenKey::PrimitiveString,
                };
            }

            return Token {
                ln,
                col,
                raw: Some(String::from(raw)),
                key: TokenKey::Keyword,
            };
        }
    };
}
