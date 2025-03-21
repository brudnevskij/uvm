use super::{AstNode, Parser};
use crate::tokenizer::{Token, TokenType};

#[test]
fn test_parser() {
    let tokens = vec![
        Token::new(TokenType::Value, "int".to_string()),
        Token::new(TokenType::Value, "main".to_string()),
        Token::new(TokenType::Punctuation, "(".to_string()),
        Token::new(TokenType::Punctuation, ")".to_string()),
        Token::new(TokenType::Punctuation, "{".to_string()),
        Token::new(TokenType::Value, "return".to_string()),
        Token::new(TokenType::Value, "0".to_string()),
        Token::new(TokenType::Punctuation, ";".to_string()),
        Token::new(TokenType::Punctuation, "}".to_string()),
        Token::new(TokenType::EOF, String::new()),
    ];

    let ast = Parser::new(&tokens).parse().unwrap();
    println!("{}", ast);
}
