use rustyline::Editor;
use std::collections::HashMap;
use tokio::task;
use std::sync::Arc;
use tokio::sync::RwLock;

mod lexer;
mod parser;
mod interpreter;

use lexer::tokenize;
use parser::parse;
use interpreter::{Environment, interpret};

#[tokio::main]
async fn main() {
    println!("Rift v2.0.0 - Code Fusion Powerhouse by Zen");
    let mut rl = Editor::<()>::new().unwrap();
    let env = Arc::new(RwLock::new(Environment {
        variables: HashMap::new(),
        rifts: HashMap::new(),
        tasks: HashMap::new(),
        artifact_cache: HashMap::new(),
        target_lang: None,
    }));

    loop {
        match rl.readline("rift> ") {
            Ok(line) => {
                rl.add_history_entry(line.as_str()).unwrap();
                let tokens = tokenize(&line);
                match parse(&tokens) {
                    Ok(ast) => {
                        let env_clone = Arc::clone(&env);
                        match task::spawn(async move {
                            let mut env_guard = env_clone.write().await;
                            interpret(&ast, &mut env_guard).await
                        }).await {
                            Ok(Ok(_)) => println!("Ok"),
                            Ok(Err(e)) => println!("Error: {}", e),
                            Err(e) => println!("Task failed: {}", e),
                        }
                    }
                    Err(e) => println!("Parse error: {}", e),
                }
            }
            Err(rustyline::error::ReadlineError::Interrupted) => {
                println!("Interrupted (Ctrl+C) - exiting...");
                break;
            }
            Err(rustyline::error::ReadlineError::Eof) => {
                println!("End of input (Ctrl+D) - exiting...");
                break;
            }
            Err(e) => {
                println!("Input error: {}", e);
                break;
            }
        }
    }
}

#[derive(Debug, Clone)]
pub enum AST {
    Program(Vec<AST>),
    Rift(String, Vec<AST>),
    Fuse(String, String),
    Task(String, Vec<AST>),
    Target(String),
    Deploy(String, HashMap<String, String>),
    Let(String, Box<AST>),
    Call(String, Vec<AST>),
    If(Box<AST>, Vec<AST>, Vec<AST>),
    While(Box<AST>, Vec<AST>),
    Number(i32),
    String(String),
    Identifier(String),
}