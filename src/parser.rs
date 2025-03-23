use crate::{lexer::Token, AST};
use std::collections::HashMap;

pub fn parse(tokens: &[Token]) -> Result<AST, String> {
    let mut pos = 0;
    let mut nodes = Vec::new();

    while pos < tokens.len() {
        match tokens[pos].value.as_str() {
            "@rift" => {
                pos += 1;
                let name = tokens[pos].value.clone();
                pos += 2; // Skip name and {
                let body = parse_block(tokens, &mut pos)?;
                nodes.push(AST::Rift(name, body));
            }
            "@fuse" => {
                pos += 1;
                let lang = tokens[pos].value.clone();
                pos += 2; // Skip lang and {
                let code = tokens[pos].value.clone();
                pos += 2; // Skip code and }
                nodes.push(AST::Fuse(lang, code));
            }
            "@task" => {
                pos += 1;
                let name = tokens[pos].value.clone();
                pos += 2; // Skip name and {
                let body = parse_block(tokens, &mut pos)?;
                nodes.push(AST::Task(name, body));
            }
            "@target" => {
                pos += 1;
                let lang = tokens[pos].value.clone();
                pos += 1; // Skip lang
                nodes.push(AST::Target(lang));
            }
            "@deploy" => {
                pos += 1;
                let target = tokens[pos].value.clone();
                pos += 2; // Skip target and {
                let config = parse_config(tokens, &mut pos)?;
                nodes.push(AST::Deploy(target, config));
            }
            "let" => {
                pos += 1;
                let name = tokens[pos].value.clone();
                pos += 2; // Skip name and =
                let value = parse_expression(tokens, &mut pos)?;
                pos += 1; // Skip ;
                nodes.push(AST::Let(name, Box::new(value)));
            }
            "call" => {
                pos += 1;
                let name = tokens[pos].value.clone();
                pos += 1; // Skip name
                let args = if pos < tokens.len() && tokens[pos].value != ";" {
                    parse_args(tokens, &mut pos)?
                } else {
                    Vec::new()
                };
                pos += 1; // Skip ;
                nodes.push(AST::Call(name, args));
            }
            "if" => {
                pos += 1;
                let condition = parse_expression(tokens, &mut pos)?;
                pos += 1; // Skip {
                let then_body = parse_block(tokens, &mut pos)?;
                let mut else_body = Vec::new();
                if pos < tokens.len() && tokens[pos].value == "else" {
                    pos += 2; // Skip else {
                    else_body = parse_block(tokens, &mut pos)?;
                }
                nodes.push(AST::If(Box::new(condition), then_body, else_body));
            }
            "while" => {
                pos += 1;
                let condition = parse_expression(tokens, &mut pos)?;
                pos += 1; // Skip {
                let body = parse_block(tokens, &mut pos)?;
                nodes.push(AST::While(Box::new(condition), body));
            }
            _ => return Err(format!("Unexpected token: {}", tokens[pos].value)),
        }
    }
    Ok(AST::Program(nodes))
}

fn parse_block(tokens: &[Token], pos: &mut usize) -> Result<Vec<AST>, String> {
    let mut body = Vec::new();
    while *pos < tokens.len() && tokens[*pos].value != "}" {
        body.push(parse_single(tokens, pos)?);
        *pos += 1;
    }
    Ok(body)
}

fn parse_single(tokens: &[Token], pos: &mut usize) -> Result<AST, String> {
    match tokens[*pos].value.as_str() {
        "@fuse" => {
            *pos += 1;
            let lang = tokens[*pos].value.clone();
            *pos += 2; // Skip lang and {
            let code = tokens[*pos].value.clone();
            *pos += 2; // Skip code and }
            Ok(AST::Fuse(lang, code))
        }
        "@target" => {
            *pos += 1;
            let lang = tokens[*pos].value.clone();
            *pos += 1; // Skip lang
            Ok(AST::Target(lang))
        }
        "let" => {
            *pos += 1;
            let name = tokens[*pos].value.clone();
            *pos += 2; // Skip name and =
            let value = parse_expression(tokens, pos)?;
            Ok(AST::Let(name, Box::new(value)))
        }
        "call" => {
            *pos += 1;
            let name = tokens[*pos].value.clone();
            *pos += 1; // Skip name
            let args = if *pos < tokens.len() && tokens[*pos].value != ";" {
                parse_args(tokens, pos)?
            } else {
                Vec::new()
            };
            Ok(AST::Call(name, args))
        }
        _ => parse_expression(tokens, pos),
    }
}

fn parse_expression(tokens: &[Token], pos: &mut usize) -> Result<AST, String> {
    let token = &tokens[*pos];
    *pos += 1;
    match token.kind.as_str() {
        "number" => Ok(AST::Number(token.value.parse().unwrap())),
        "string" => Ok(AST::String(token.value.clone())),
        "identifier" => Ok(AST::Identifier(token.value.clone())),
        _ => Err(format!("Invalid expression: {}", token.value)),
    }
}

fn parse_args(tokens: &[Token], pos: &mut usize) -> Result<Vec<AST>, String> {
    let mut args = Vec::new();
    while *pos < tokens.len() && tokens[*pos].value != ";" {
        args.push(parse_expression(tokens, pos)?);
        if *pos < tokens.len() && tokens[*pos].value == "," { *pos += 1; }
    }
    Ok(args)
}

fn parse_config(tokens: &[Token], pos: &mut usize) -> Result<HashMap<String, String>, String> {
    let mut config = HashMap::new();
    while *pos < tokens.len() && tokens[*pos].value != "}" {
        let key = tokens[*pos].value.clone();
        *pos += 2; // Skip key and =
        let value = tokens[*pos].value.clone();
        *pos += 2; // Skip value and ;
        config.insert(key, value);
    }
    Ok(config)
}