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

#[test]
fn test_tokenize_simple_program() {
    let input = "int main() { return 0; }";
    let tokens = Tokenizer::new(input).tokenize().unwrap();

    assert_eq!(tokens.len(), 10);
    assert_eq!(tokens[0].token_type,TokenType::Value);
    assert_eq!(tokens[0].lexeme,"int");

    assert_eq!(tokens[1].token_type,TokenType::Value);
    assert_eq!(tokens[1].lexeme,"main");

    assert_eq!(tokens[2].token_type,TokenType::Punctuation);
    assert_eq!(tokens[2].lexeme,"(");

    assert_eq!(tokens[3].token_type,TokenType::Punctuation);
    assert_eq!(tokens[3].lexeme,")");

    assert_eq!(tokens[4].token_type,TokenType::Punctuation);
    assert_eq!(tokens[4].lexeme,"{");

    assert_eq!(tokens[5].token_type,TokenType::Value);
    assert_eq!(tokens[5].lexeme,"return");

    assert_eq!(tokens[6].token_type,TokenType::Value);
    assert_eq!(tokens[6].lexeme,"0");

    assert_eq!(tokens[7].token_type,TokenType::Punctuation);
    assert_eq!(tokens[7].lexeme,";");

    assert_eq!(tokens[8].token_type,TokenType::Punctuation);
    assert_eq!(tokens[8].lexeme,"}");

    assert_eq!(tokens[9].token_type,TokenType::EOF);
}