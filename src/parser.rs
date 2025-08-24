use crate::{lexer::{Token, TokenKind}, AST, error::{Result, RiftError}};
use std::collections::HashMap;

pub struct Parser {
    tokens: Vec<Token>,
    pos: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, pos: 0 }
    }
    
    pub fn parse(&mut self) -> Result<AST> {
        let mut nodes = Vec::new();
        
        while !self.is_at_end() {
            // Skip comments
            if self.current_token_is(TokenKind::Comment) {
                self.advance();
                continue;
            }
            
            match self.parse_statement() {
                Ok(node) => nodes.push(node),
                Err(e) => return Err(self.error_with_context(format!("Parse error: {}", e))),
            }
        }
        
        Ok(AST::Program(nodes))
    }
    
    fn parse_statement(&mut self) -> Result<AST> {
        if self.is_at_end() {
            return Err(RiftError::ParseError("Unexpected end of input".to_string()));
        }
        
        match self.current().value.as_str() {
            "@rift" => self.parse_rift(),
            "@fuse" => self.parse_fuse(),
            "@task" => self.parse_task(),
            "@target" => self.parse_target(),
            "@deploy" => self.parse_deploy(),
            "let" => self.parse_let(),
            "call" => self.parse_call(),
            "if" => self.parse_if(),
            "while" => self.parse_while(),
            _ => Err(RiftError::ParseError(format!(
                "Unexpected token: '{}' at line {}, column {}",
                self.current().value, self.current().line, self.current().column
            ))),
        }
    }
    
    fn parse_rift(&mut self) -> Result<AST> {
        self.consume_keyword("@rift")?;
        
        let name = self.consume_identifier("Expected rift name")?;
        self.consume_symbol("{", "Expected '{' after rift name")?;
        
        let body = self.parse_block()?;
        
        Ok(AST::Rift(name, body))
    }
    
    fn parse_fuse(&mut self) -> Result<AST> {
        self.consume_keyword("@fuse")?;
        
        let lang = self.consume_string("Expected language string after @fuse")?;
        self.consume_symbol("{", "Expected '{' after language")?;
        
        let code = self.consume_string("Expected code string in fuse block")?;
        
        self.consume_symbol("}", "Expected '}' after code")?;
        
        Ok(AST::Fuse(lang, code))
    }
    
    fn parse_task(&mut self) -> Result<AST> {
        self.consume_keyword("@task")?;
        
        let name = self.consume_identifier("Expected task name")?;
        self.consume_symbol("{", "Expected '{' after task name")?;
        
        let body = self.parse_block()?;
        
        Ok(AST::Task(name, body))
    }
    
    fn parse_target(&mut self) -> Result<AST> {
        self.consume_keyword("@target")?;
        
        let lang = self.consume_string("Expected language string after @target")?;
        
        Ok(AST::Target(lang))
    }
    
    fn parse_deploy(&mut self) -> Result<AST> {
        self.consume_keyword("@deploy")?;
        
        let target = self.consume_string("Expected target string after @deploy")?;
        self.consume_symbol("{", "Expected '{' after deploy target")?;
        
        let config = self.parse_config()?;
        
        Ok(AST::Deploy(target, config))
    }
    
    fn parse_let(&mut self) -> Result<AST> {
        self.consume_keyword("let")?;
        
        let name = self.consume_identifier("Expected variable name after 'let'")?;
        self.consume_symbol("=", "Expected '=' after variable name")?;
        
        let value = self.parse_expression()?;
        
        self.consume_symbol(";", "Expected ';' after let statement")?;
        
        Ok(AST::Let(name, Box::new(value)))
    }
    
    fn parse_call(&mut self) -> Result<AST> {
        self.consume_keyword("call")?;
        
        let name = self.consume_identifier("Expected function name after 'call'")?;
        let mut args = Vec::new();
        
        // Parse optional arguments
        while !self.is_at_end() && !self.current_token_value_is(";") {
            if self.current_token_value_is("with") {
                self.advance(); // consume 'with'
            }
            
            args.push(self.parse_expression()?);
            
            if self.current_token_value_is(",") {
                self.advance(); // consume comma
            } else {
                break;
            }
        }
        
        self.consume_symbol(";", "Expected ';' after call statement")?;
        
        Ok(AST::Call(name, args))
    }
    
    fn parse_if(&mut self) -> Result<AST> {
        self.consume_keyword("if")?;
        
        let condition = self.parse_expression()?;
        
        self.consume_symbol("{", "Expected '{' after if condition")?;
        let then_body = self.parse_block_content()?;
        
        let mut else_body = Vec::new();
        
        if !self.is_at_end() && self.current_token_value_is("else") {
            self.advance(); // consume 'else'
            self.consume_symbol("{", "Expected '{' after 'else'")?;
            else_body = self.parse_block_content()?;
        }
        
        Ok(AST::If(Box::new(condition), then_body, else_body))
    }
    
    fn parse_while(&mut self) -> Result<AST> {
        self.consume_keyword("while")?;
        
        let condition = self.parse_expression()?;
        
        self.consume_symbol("{", "Expected '{' after while condition")?;
        let body = self.parse_block_content()?;
        
        Ok(AST::While(Box::new(condition), body))
    }
    
    fn parse_block(&mut self) -> Result<Vec<AST>> {
        let body = self.parse_block_content()?;
        Ok(body)
    }
    
    fn parse_block_content(&mut self) -> Result<Vec<AST>> {
        let mut body = Vec::new();
        
        while !self.is_at_end() && !self.current_token_value_is("}") {
            // Skip comments in blocks
            if self.current_token_is(TokenKind::Comment) {
                self.advance();
                continue;
            }
            
            body.push(self.parse_statement()?);
        }
        
        self.consume_symbol("}", "Expected '}' to close block")?;
        
        Ok(body)
    }
    
    fn parse_expression(&mut self)