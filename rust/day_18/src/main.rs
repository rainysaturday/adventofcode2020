
#[derive(Debug, Clone)]
enum Token {
    Value(i64),
    ParenOpen,
    ParenClose,
    OperAdd,
    OperMul
}

fn tokenize(line: &str) -> Vec<Token> {
    line.replace("(", "( ").replace(")", " )").split(" ").map(|t| 
        match t.trim() {
            "(" => Token::ParenOpen,
            ")" => Token::ParenClose,
            "+" => Token::OperAdd,
            "*" => Token::OperMul,
            v => Token::Value(v.parse::<i64>().expect(&format!("Unable to parse value {}", v))),
        }
    ).collect::<Vec<Token>>()
}

fn find_expr(tokens: &[Token], start: usize) -> (Option<&[Token]>, usize) {
    match tokens[start] {
        Token::ParenOpen => {
            let mut level = 0;
            for i in start..tokens.len() {
                match tokens[i] {
                    Token::ParenOpen => level = level + 1,
                    Token::ParenClose => {
                        level = level - 1;
                        if level == 0 {
                            return (Some(&tokens[start+1..i]), i + 1);
                        }
                    },
                    _ => ()
                };
            }
            panic!("Failed to find closing paren for tokens");
        },
        Token::ParenClose => (None, start + 1),
        Token::Value(_) => (Some(&tokens[start..start+1]), start + 1),
        _ => panic!("Shoulnd not search expressions for add or mul")
    }
}

fn eval(tokens: &[Token], add_has_prio: bool) -> Option<i64> {
    if tokens.len() == 0 {
        return None;
    }

    if tokens.len() == 1 {
        match tokens[0] {
            Token::Value(v) => return Some(v),
            _ => panic!("Token length 1 and it is not a value, bad expression")
        }
    }

    let mut pos = 0;
    let mut resolved: Vec<Token> = Vec::new();      // Flatten sub-expressions to a single value
    while pos < tokens.len() {
        if pos != 0 {
            resolved.push(tokens[pos].clone());
            pos = pos + 1;
        }
        let (expr, new_pos) = find_expr(tokens, pos);
        if let Some(sub_tokens) = expr {
            let value = eval(sub_tokens, add_has_prio).expect("Should be able to evaluate");
            resolved.push(Token::Value(value));
        }
        pos = new_pos;
    }

    let mut pos = 0;
    while pos < resolved.len() {
        match resolved[pos] {
            Token::OperAdd => {
                if let Token::Value(p) = resolved[pos - 1] {
                    if let Token::Value(n) = resolved[pos + 1] {
                        resolved[pos - 1] = Token::Value(p + n);
                        resolved.remove(pos);
                        resolved.remove(pos);
                    } else {
                        panic!("Missing next value");
                    }
                } else {
                    panic!("Missing prev value");
                }
            },
            Token::OperMul => {
                if add_has_prio {
                    pos = pos + 1;  // Skip muls for now
                } else {
                    if let Token::Value(p) = resolved[pos - 1] {
                        if let Token::Value(n) = resolved[pos + 1] {
                            resolved[pos - 1] = Token::Value(p * n);
                            resolved.remove(pos);
                            resolved.remove(pos);
                        } else {
                            panic!("Missing next value");
                        }
                    } else {
                        panic!("Missing prev value");
                    }
                }
            },
            _ => pos = pos + 1
        }
    }

    // Mul any value together if we have many
    Some(resolved.iter().map(|t| if let Token::Value(v) = t { v } else { &1 }).fold(1, |a, b| a * b))
}

fn main() {
    let input = include_str!("../../../input18");

    println!("Part1 sum {}", input.split("\n")
        .map(|line| eval(&tokenize(line), false).expect(&format!("Failed to evaluate expression {}", line)))
        .sum::<i64>());
    println!("Part2 sum {}", input.split("\n")
        .map(|line| eval(&tokenize(line), true).expect(&format!("Failed to evaluate expression {}", line)))
        .sum::<i64>());
}
