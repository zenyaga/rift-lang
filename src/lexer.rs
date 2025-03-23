use std::io::{BufReader, BufRead};

#[derive(Debug, Clone)]
pub struct Token {
    pub kind: String,
    pub value: String,
}

pub fn tokenize(input: &str) -> Vec<Token> {
    let mut tokens = Vec::new();
    let reader = BufReader::new(input.as_bytes());
    let mut buffer = String::new();

    for line in reader.lines() {
        let line = line.unwrap();
        let mut chars = line.chars().peekable();
        while let Some(&c) = chars.peek() {
            match c {
                ' ' | '\t' | '\n' => {
                    if !buffer.is_empty() {
                        tokens.push(identify_token(&buffer));
                        buffer.clear();
                    }
                    chars.next();
                }
                '{' | '}' | ';' | '=' | ',' => {
                    if !buffer.is_empty() {
                        tokens.push(identify_token(&buffer));
                        buffer.clear();
                    }
                    tokens.push(Token { kind: "symbol".to_string(), value: c.to_string() });
                    chars.next();
                }
                '"' => {
                    if !buffer.is_empty() {
                        tokens.push(identify_token(&buffer));
                        buffer.clear();
                    }
                    buffer.push(chars.next().unwrap());
                    while let Some(&next) = chars.peek() {
                        if next == '"' { break; }
                        buffer.push(chars.next().unwrap());
                    }
                    buffer.push(chars.next().unwrap());
                    tokens.push(Token { kind: "string".to_string(), value: buffer[1..buffer.len()-1].to_string() });
                    buffer.clear();
                }
                _ => {
                    buffer.push(chars.next().unwrap());
                }
            }
        }
        if !buffer.is_empty() {
            tokens.push(identify_token(&buffer));
            buffer.clear();
        }
    }
    tokens
}

fn identify_token(word: &str) -> Token {
    match word {
        "@rift" | "@fuse" | "@task" | "@target" | "@deploy" | "let" | "call" | "if" | "else" | "while" => 
            Token { kind: "keyword".to_string(), value: word.to_string() },
        _ if word.chars().all(|c| c.is_digit(10)) => 
            Token { kind: "number".to_string(), value: word.to_string() },
        _ => Token { kind: "identifier".to_string(), value: word.to_string() },
    }
}