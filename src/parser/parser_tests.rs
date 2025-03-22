use super::{AstNode, Parser};
use crate::tokenizer::{Token, TokenType};

#[test]
fn test_parser_two_statements() {
    let tokens = vec![
        Token::new(TokenType::Value, "int".to_string()),
        Token::new(TokenType::Value, "main".to_string()),
        Token::new(TokenType::Punctuation, "(".to_string()),
        Token::new(TokenType::Punctuation, ")".to_string()),
        Token::new(TokenType::Punctuation, "{".to_string()),
        Token::new(TokenType::Value, "int".to_string()),
        Token::new(TokenType::Value, "a".to_string()),
        Token::new(TokenType::Value, "=".to_string()),
        Token::new(TokenType::Value, "10".to_string()),
        Token::new(TokenType::Punctuation, ";".to_string()),
        Token::new(TokenType::Value, "return".to_string()),
        Token::new(TokenType::Value, "0".to_string()),
        Token::new(TokenType::Punctuation, ";".to_string()),
        Token::new(TokenType::Punctuation, "}".to_string()),
        Token::new(TokenType::EOF, String::new()),
    ];

    let ast = Parser::new(&tokens).parse().unwrap();
    println!("{}", ast);

    // Expected structure: (int main () (int a = 10 return 0))
    if let AstNode::List(root) = ast {
        assert_eq!(root.len(), 4);  // int, main, params, and body list

        // Check function signature
        if let AstNode::Atom(token) = &root[0] {
            assert_eq!(token.lexeme, "int");
        }
        if let AstNode::Atom(token) = &root[1] {
            assert_eq!(token.lexeme, "main");
        }

        // Check args
        if let AstNode::List(list) = &root[2] {
            assert_eq!(list.len(), 0);
        }

        // Check body
        if let AstNode::List(body) = &root[3] {
            assert_eq!(body.len(), 2);  // Two statements

            // Check first statement: int a = 10
            if let AstNode::List(statement) = &body[0] {
                assert_eq!(statement.len(), 4);
                if let AstNode::Atom(token) = &statement[0] {
                    assert_eq!(token.lexeme, "int");
                }
                if let AstNode::Atom(token) = &statement[1] {
                    assert_eq!(token.lexeme, "a");
                }
                if let AstNode::Atom(token) = &statement[2] {
                    assert_eq!(token.lexeme, "=");
                }
                if let AstNode::Atom(token) = &statement[3] {
                    assert_eq!(token.lexeme, "10");
                }
            }

            // Check second statement: return 0
            if let AstNode::List(statement) = &body[1] {
                assert_eq!(statement.len(), 2);
                if let AstNode::Atom(token) = &statement[0] {
                    assert_eq!(token.lexeme, "return");
                }
                if let AstNode::Atom(token) = &statement[1] {
                    assert_eq!(token.lexeme, "0");
                }
            }
        }
    } else {
        panic!("Root node should be a List");
    }
}

#[test]
fn test_parser_simple() {
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

    // Expected structure: (int main () (return 0))
    if let AstNode::List(root) = ast {
        assert_eq!(root.len(), 4);  // int, main, and body list

        if let AstNode::Atom(token) = &root[0] {
            assert_eq!(token.lexeme, "int");
        }
        if let AstNode::Atom(token) = &root[1] {
            assert_eq!(token.lexeme, "main");
        }

        if let AstNode::List(list) = &root[2] {
            assert_eq!(list.len(), 0);
        }

        if let AstNode::List(body) = &root[3] {
            assert_eq!(body.len(), 1);  // One statement

            if let AstNode::List(statement) = &body[0] {
                assert_eq!(statement.len(), 2);
                if let AstNode::Atom(token) = &statement[0] {
                    assert_eq!(token.lexeme, "return");
                }
                if let AstNode::Atom(token) = &statement[1] {
                    assert_eq!(token.lexeme, "0");
                }
            }
        }
    } else {
        panic!("Root node should be a List");
    }
}

#[test]
fn test_parse_with_func_call() {
    let tokens = vec![
        Token::new(TokenType::Value, "int".to_string()),
        Token::new(TokenType::Value, "main".to_string()),
        Token::new(TokenType::Punctuation, "(".to_string()),
        Token::new(TokenType::Punctuation, ")".to_string()),
        Token::new(TokenType::Punctuation, "{".to_string()),
        Token::new(TokenType::Value, "int".to_string()),
        Token::new(TokenType::Value, "a".to_string()),
        Token::new(TokenType::Value, "=".to_string()),
        Token::new(TokenType::Value, "10".to_string()),
        Token::new(TokenType::Punctuation, ";".to_string()),
        Token::new(TokenType::Value, "int".to_string()),
        Token::new(TokenType::Value, "b".to_string()),
        Token::new(TokenType::Value, "=".to_string()),
        Token::new(TokenType::Value, "test".to_string()),
        Token::new(TokenType::Punctuation, "(".to_string()),
        Token::new(TokenType::Punctuation, ")".to_string()),
        Token::new(TokenType::Punctuation, ";".to_string()),
        Token::new(TokenType::Value, "return".to_string()),
        Token::new(TokenType::Value, "0".to_string()),
        Token::new(TokenType::Punctuation, ";".to_string()),
        Token::new(TokenType::Punctuation, "}".to_string()),
        Token::new(TokenType::EOF, String::new()),
    ];

    let ast = Parser::new(&tokens).parse().unwrap();
    println!("{}", ast);

    // Expected structure: (int main () (int a = 10 int b = (test) return 0))
    if let AstNode::List(root) = ast {
        assert_eq!(root.len(), 4);  // int, main, and body list

        if let AstNode::Atom(token) = &root[0] {
            assert_eq!(token.lexeme, "int");
        }
        if let AstNode::Atom(token) = &root[1] {
            assert_eq!(token.lexeme, "main");
        }

        if let AstNode::List(list) = &root[2] {
            assert_eq!(list.len(), 0);
        }
        if let AstNode::List(body) = &root[3] {
            assert_eq!(body.len(), 3);  // Three statements

            // First statement: int a = 10
            if let AstNode::List(statement) = &body[0] {
                assert_eq!(statement.len(), 4);
                if let AstNode::Atom(token) = &statement[0] {
                    assert_eq!(token.lexeme, "int");
                }
                if let AstNode::Atom(token) = &statement[1] {
                    assert_eq!(token.lexeme, "a");
                }
                if let AstNode::Atom(token) = &statement[2] {
                    assert_eq!(token.lexeme, "=");
                }
                if let AstNode::Atom(token) = &statement[3] {
                    assert_eq!(token.lexeme, "10");
                }
            }

            // Second statement: int b = test()
            if let AstNode::List(statement) = &body[1] {
                assert_eq!(statement.len(), 5);
                if let AstNode::Atom(token) = &statement[0] {
                    assert_eq!(token.lexeme, "int");
                }
                if let AstNode::Atom(token) = &statement[1] {
                    assert_eq!(token.lexeme, "b");
                }
                if let AstNode::Atom(token) = &statement[2] {
                    assert_eq!(token.lexeme, "=");
                }
                if let AstNode::Atom(func_name) = &statement[3] {
                    assert_eq!(func_name.lexeme, "test");
                }
                if let AstNode::List(args) = &statement[4] {
                    assert_eq!(args.len(), 0);
                }
            }

            // Third statement: return 0
            if let AstNode::List(statement) = &body[2] {
                assert_eq!(statement.len(), 2);
                if let AstNode::Atom(token) = &statement[0] {
                    assert_eq!(token.lexeme, "return");
                }
                if let AstNode::Atom(token) = &statement[1] {
                    assert_eq!(token.lexeme, "0");
                }
            }
        }
    } else {
        panic!("Root node should be a List");
    }
}
