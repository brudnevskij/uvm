use crate::tokenizer::{Token, TokenType};
use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub enum AstNode {
    Atom(Token),
    List(Vec<AstNode>),
}

impl Display for AstNode {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            AstNode::Atom(token) => write!(f, "{}", token.lexeme),
            AstNode::List(l) => {
                write!(f, "(")?;
                for (i, node) in l.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}", node)?;
                }
                write!(f, ")")
            }
        }?;
        Ok(())
    }
}

pub struct Parser<'a> {
    tokens: &'a [Token],
    position: usize,
}

impl<'a> Parser<'a> {
    pub fn new(tokens: &'a [Token]) -> Self {
        Self {
            tokens,
            position: 0,
        }
    }

    fn is_at_end(&self) -> bool {
        self.position >= self.tokens.len()
    }

    fn peek(&self) -> Option<&Token> {
        if self.is_at_end() {
            return None;
        }
        Some(&self.tokens[self.position])
    }

    fn advance(&mut self) -> Option<&Token> {
        if self.is_at_end() {
            return None;
        }
        let token = &self.tokens[self.position];
        self.position += 1;
        Some(token)
    }

    fn parse_list(&mut self) -> Result<AstNode, String> {
        let mut root_list: Vec<AstNode> = Vec::new();
        let mut current_statement: Vec<AstNode> = Vec::new();

        loop {
            if let Some(token) = self.peek() {
                match token.token_type {
                    TokenType::Value => {
                        let new_token = Token::new(TokenType::Value, token.lexeme.clone());
                        current_statement.push(AstNode::Atom(new_token));
                        self.advance();
                    }
                    TokenType::Punctuation => match token.lexeme.as_str() {
                        "(" | "{" => {
                            self.advance();
                            let nested_list = self.parse_list()?;
                            current_statement.push(nested_list);
                        }
                        ")" | "}" => {
                            root_list.append(&mut current_statement);
                            self.advance();
                            break;
                        }
                        ";" => {
                            root_list.push(AstNode::List(current_statement));
                            current_statement = Vec::new();
                            self.advance();
                        }
                        _ => {
                            // todo: consider returning error
                        }
                    },
                    TokenType::EOF => {
                        root_list.push(AstNode::List(current_statement));
                        current_statement = Vec::new();
                        self.advance();
                    }
                }
            }
        }
        Ok(AstNode::List(root_list))
    }

    pub fn parse(&mut self) -> Result<AstNode, String> {
        let mut root_list: Vec<AstNode> = Vec::new();
        let mut current_statement: Vec<AstNode> = Vec::new();

        while !self.is_at_end() {
            if let Some(t) = self.peek() {
                match &t.token_type {
                    TokenType::Value => {
                        let new_token = Token::new(TokenType::Value, t.lexeme.to_string());
                        current_statement.push(AstNode::Atom(new_token));
                        self.advance();
                    }
                    TokenType::Punctuation => match t.lexeme.as_str() {
                        "(" | "{" => {
                            self.advance();
                            let nested_list = self.parse_list()?;
                            current_statement.push(nested_list);
                        }
                        ";" => {
                            // saving statement as sublist after termination
                            root_list.push(AstNode::List(current_statement));
                            current_statement = Vec::new();
                            self.advance();
                        }
                        _ => {
                            // todo: consider returning error
                        }
                    },
                    TokenType::EOF => {
                        root_list.append(&mut current_statement);
                        current_statement = Vec::new();
                        self.advance();
                    }
                }
            }
        }

        Ok(AstNode::List(root_list))
    }
}
