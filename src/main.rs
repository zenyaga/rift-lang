use rustyline::Editor;
use std::collections::HashMap;
use tokio::task;
use std::sync::Arc;
use tokio::sync::RwLock;

mod error;
mod lexer;
mod parser;
mod interpreter;
mod executor;
mod transformer;
mod deployer;

use error::{Result, RiftError};
use lexer::tokenize;
use parser::parse;
use interpreter::{Environment, interpret};

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

#[tokio::main]
async fn main() -> Result<()> {
    println!("Rift v2.0.1 - Code Fusion Powerhouse by Zen");
    println!("Type 'help' for available commands, 'exit' to quit");
    
    let mut rl = Editor::<()>::new()
        .map_err(|e| RiftError::IoError(std::io::Error::new(
            std::io::ErrorKind::Other, 
            format!("Failed to initialize readline: {}", e)
        )))?;
    
    let env = Arc::new(RwLock::new(Environment::new()));

    // Load history if available
    if rl.load_history("rift_history.txt").is_err() {
        // History file doesn't exist yet, that's fine
    }

    loop {
        match rl.readline("rift> ") {
            Ok(line) => {
                let line = line.trim();
                
                // Handle special commands
                match line {
                    "help" => {
                        print_help();
                        continue;
                    }
                    "exit" | "quit" => {
                        println!("Goodbye!");
                        break;
                    }
                    "clear" => {
                        let mut env_guard = env.write().await;
                        env_guard.clear();
                        println!("Environment cleared");
                        continue;
                    }
                    "status" => {
                        let env_guard = env.read().await;
                        print_status(&env_guard);
                        continue;
                    }
                    "" => continue,
                    _ => {}
                }
                
                rl.add_history_entry(line).unwrap();
                
                // Parse and execute
                match execute_line(line, &env).await {
                    Ok(_) => println!("Ok"),
                    Err(e) => {
                        eprintln!("Error: {}", e);
                        
                        // Provide helpful suggestions based on error type
                        match &e {
                            RiftError::UnsupportedLanguage(lang) => {
                                eprintln!("Hint: Supported languages are: python, javascript, go, java, cpp, php, rust");
                            }
                            RiftError::ParseError(_) => {
                                eprintln!("Hint: Check syntax. Use 'help' for examples");
                            }
                            _ => {}
                        }
                    }
                }
            }
            Err(rustyline::error::ReadlineError::Interrupted) => {
                println!("Use 'exit' to quit");
                continue;
            }
            Err(rustyline::error::ReadlineError::Eof) => {
                println!("Goodbye!");
                break;
            }
            Err(e) => {
                eprintln!("Input error: {}", e);
                break;
            }
        }
    }

    // Save history
    if let Err(e) = rl.save_history("rift_history.txt") {
        eprintln!("Warning: Could not save history: {}", e);
    }

    Ok(())
}

async fn execute_line(line: &str, env: &Arc<RwLock<Environment>>) -> Result<()> {
    let tokens = tokenize(line)?;
    let ast = parse(&tokens)?;
    
    let env_clone = Arc::clone(env);
    let result = task::spawn(async move {
        let mut env_guard = env_clone.write().await;
        interpret(&ast, &mut env_guard).await
    }).await;
    
    match result {
        Ok(Ok(_)) => Ok(()),
        Ok(Err(e)) => Err(e),
        Err(e) => Err(RiftError::ExecutionError {
            language: "runtime".to_string(),
            message: format!("Task execution failed: {}", e),
        }),
    }
}

fn print_help() {
    println!(r#"
Rift v2.0.1 Commands:

Basic Commands:
  @rift name {{ ... }}           - Create a new rift (project)
  @fuse "lang" {{ "code" }}      - Add code in specified language
  @task name {{ ... }}           - Create a transformation task
  @target "lang"                 - Set target language for transformation
  @deploy "target" {{ ... }}     - Deploy to specified target
  call name;                     - Execute a rift or task
  let var = value;               - Set a variable

Flow Control:
  if condition {{ ... }}         - Conditional execution
  while condition {{ ... }}      - Loop execution

Utility Commands:
  help                           - Show this help
  status                         - Show environment status
  clear                          - Clear all rifts and variables
  exit/quit                      - Exit Rift

Example Usage:
  @rift hello {{ @fuse "python" {{ "print('Hello, World!')" }} }}
  call hello;
  
  @task optimize {{ @target "rust" call optimize with hello; }}
  call optimize;

Supported Languages:
  python, javascript, go, java, cpp, php, rust

Deployment Targets:
  local, ethereum, solana, aws
"#);
}

fn print_status(env: &Environment) {
    println!("Environment Status:");
    println!("  Rifts: {}", env.rifts.len());
    println!("  Tasks: {}", env.tasks.len());
    println!("  Variables: {}", env.variables.len());
    println!("  Cache entries: {}", env.artifact_cache.len());
    
    if let Some(target) = &env.target_lang {
        println!("  Target language: {}", target);
    }
    
    if !env.rifts.is_empty() {
        println!("  Available rifts: {}", env.rifts.keys().collect::<Vec<_>>().join(", "));
    }
    
    if !env.tasks.is_empty() {
        println!("  Available tasks: {}", env.tasks.keys().collect::<Vec<_>>().join(", "));
    }
}