use crate::tokenizer::tokenizer::TokenType;
use super::*;

#[test]
fn test_tokenize() {
    let input = "value";
    let tokens = Tokenizer::new(input).tokenize().unwrap();

    assert_eq!(tokens.len(), 2);
    assert_eq!(tokens[0].token_type, TokenType::Value);
    assert_eq!(tokens[1].token_type, TokenType::EOF);
    assert_eq!(tokens[0].lexeme, "value");
}