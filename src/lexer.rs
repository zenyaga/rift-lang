use crate::error::{Result, RiftError};

#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    pub kind: TokenKind,
    pub value: String,
    pub line: usize,
    pub column: usize,
}

#[derive(Debug, Clone, PartialEq)]
pub enum TokenKind {
    Keyword,
    Identifier,
    String,
    Number,
    Symbol,
    Comment,
}

pub fn tokenize(input: &str) -> Result<Vec<Token>> {
    if input.trim().is_empty() {
        return Ok(vec![]);
    }
    
    let mut tokens = Vec::new();
    let mut chars = input.char_indices().peekable();
    let mut line = 1;
    let mut column = 1;
    
    while let Some((pos, ch)) = chars.next() {
        match ch {
            // Skip whitespace
            ' ' | '\t' => {
                column += 1;
                continue;
            }
            '\n' | '\r' => {
                line += 1;
                column = 1;
                continue;
            }
            
            // Comments
            '/' if chars.peek().map(|(_, c)| *c) == Some('/') => {
                chars.next(); // consume second '/'
                column += 2;
                
                let mut comment = String::new();
                while let Some((_, ch)) = chars.peek() {
                    if *ch == '\n' || *ch == '\r' {
                        break;
                    }
                    comment.push(*ch);
                    chars.next();
                    column += 1;
                }
                
                tokens.push(Token {
                    kind: TokenKind::Comment,
                    value: comment,
                    line,
                    column: column - comment.len(),
                });
            }
            
            // Symbols
            '{' | '}' | ';' | '=' | ',' | '(' | ')' => {
                tokens.push(Token {
                    kind: TokenKind::Symbol,
                    value: ch.to_string(),
                    line,
                    column,
                });
                column += 1;
            }
            
            // String literals
            '"' => {
                let start_column = column;
                column += 1; // opening quote
                
                let mut string_value = String::new();
                let mut escaped = false;
                
                while let Some((_, ch)) = chars.next() {
                    column += 1;
                    
                    if escaped {
                        match ch {
                            'n' => string_value.push('\n'),
                            't' => string_value.push('\t'),
                            'r' => string_value.push('\r'),
                            '\\' => string_value.push('\\'),
                            '"' => string_value.push('"'),
                            _ => {
                                string_value.push('\\');
                                string_value.push(ch);
                            }
                        }
                        escaped = false;
                    } else if ch == '\\' {
                        escaped = true;
                    } else if ch == '"' {
                        break;
                    } else {
                        string_value.push(ch);
                    }
                }
                
                tokens.push(Token {
                    kind: TokenKind::String,
                    value: string_value,
                    line,
                    column: start_column,
                });
            }
            
            // Numbers
            '0'..='9' => {
                let start_column = column;
                let mut number = String::new();
                number.push(ch);
                column += 1;
                
                // Collect remaining digits
                while let Some((_, next_ch)) = chars.peek() {
                    if next_ch.is_ascii_digit() || *next_ch == '.' {
                        number.push(*next_ch);
                        chars.next();
                        column += 1;
                    } else {
                        break;
                    }
                }
                
                tokens.push(Token {
                    kind: TokenKind::Number,
                    value: number,
                    line,
                    column: start_column,
                });
            }
            
            // Identifiers and keywords
            ch if ch.is_alphabetic() || ch == '@' || ch == '_' => {
                let start_column = column;
                let mut identifier = String::new();
                identifier.push(ch);
                column += 1;
                
                // Collect remaining alphanumeric characters
                while let Some((_, next_ch)) = chars.peek() {
                    if next_ch.is_alphanumeric() || *next_ch == '_' {
                        identifier.push(*next_ch);
                        chars.next();
                        column += 1;
                    } else {
                        break;
                    }
                }
                
                let kind = if is_keyword(&identifier) {
                    TokenKind::Keyword
                } else {
                    TokenKind::Identifier
                };
                
                tokens.push(Token {
                    kind,
                    value: identifier,
                    line,
                    column: start_column,
                });
            }
            
            // Unexpected character
            _ => {
                return Err(RiftError::ParseError(format!(
                    "Unexpected character '{}' at line {}, column {}",
                    ch, line, column
                )));
            }
        }
    }
    
    Ok(tokens)
}

fn is_keyword(word: &str) -> bool {
    matches!(
        word,
        "@rift" | "@fuse" | "@task" | "@target" | "@deploy" 
        | "let" | "call" | "if" | "else" | "while" 
        | "with" | "optimize"
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_tokenization() {
        let input = "@rift test { @fuse \"python\" { \"print('hello')\" } }";
        let tokens = tokenize(input).unwrap();
        
        assert_eq!(tokens.len(), 9);
        assert_eq!(tokens[0].value, "@rift");
        assert_eq!(tokens[0].kind, TokenKind::Keyword);
        assert_eq!(tokens[1].value, "test");
        assert_eq!(tokens[1].kind, TokenKind::Identifier);
    }

    #[test]
    fn test_string_escaping() {
        let input = r#""hello\nworld""#;
        let tokens = tokenize(input).unwrap();
        
        assert_eq!(tokens.len(), 1);
        assert_eq!(tokens[0].value, "hello\nworld");
        assert_eq!(tokens[0].kind, TokenKind::String);
    }

    #[test]
    fn test_numbers() {
        let input = "123 45.67";
        let tokens = tokenize(input).unwrap();
        
        assert_eq!(tokens.len(), 2);
        assert_eq!(tokens[0].value, "123");
        assert_eq!(tokens[0].kind, TokenKind::Number);
        assert_eq!(tokens[1].value, "45.67");
        assert_eq!(tokens[1].kind, TokenKind::Number);
    }

    #[test]
    fn test_comments() {
        let input = "test // this is a comment\n@rift";
        let tokens = tokenize(input).unwrap();
        
        assert_eq!(tokens.len(), 3);
        assert_eq!(tokens[0].value, "test");
        assert_eq!(tokens[1].kind, TokenKind::Comment);
        assert_eq!(tokens[2].value, "@rift");
    }

    #[test]
    fn test_error_handling() {
        let input = "test $ invalid";
        let result = tokenize(input);
        assert!(result.is_err());
    }
}