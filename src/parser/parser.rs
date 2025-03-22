use crate::parser::AstNode::{Atom, List};
use crate::tokenizer::{Token, TokenType};
use std::collections::HashMap;
use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub enum AstNode {
    Atom(Token),
    List(Vec<AstNode>),
}

impl Display for AstNode {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Atom(token) => write!(f, "{}", token.lexeme),
            List(l) => {
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
                        current_statement.push(Atom(new_token));
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
                            root_list.push(List(current_statement));
                            current_statement = Vec::new();
                            self.advance();
                        }
                        _ => {
                            // todo: consider returning error
                        }
                    },
                    TokenType::EOF => {
                        root_list.push(List(current_statement));
                        current_statement = Vec::new();
                        self.advance();
                    }
                }
            }
        }
        Ok(List(root_list))
    }

    pub fn parse(&mut self) -> Result<AstNode, String> {
        let mut root_list: Vec<AstNode> = Vec::new();
        let mut current_statement: Vec<AstNode> = Vec::new();

        while !self.is_at_end() {
            if let Some(t) = self.peek() {
                match &t.token_type {
                    TokenType::Value => {
                        let new_token = Token::new(TokenType::Value, t.lexeme.to_string());
                        current_statement.push(Atom(new_token));
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
                            root_list.push(List(current_statement));
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

        Ok(List(root_list))
    }
}

// shunting yard algorithm
fn convert_to_rpn(statement: Vec<AstNode>) -> Result<Vec<AstNode>, String> {
    let mut precedence: HashMap<&str, u32> = HashMap::new();
    precedence.insert("+", 1);
    precedence.insert("-", 1);
    precedence.insert("*", 2);
    precedence.insert("/", 2);
    precedence.insert("=", 0);

    let mut output: Vec<AstNode> = Vec::new();
    let mut operator_stack: Vec<AstNode> = Vec::new();

    for ast_node in statement {
        match ast_node {
            Atom(token) => {
                // if known operator
                if let Some(current_op_precedence) = precedence.get(&token.lexeme.as_str()) {
                    while let Some(Atom(op)) = operator_stack.last() {
                        let op_precedence =
                            precedence.get(op.lexeme.as_str()).unwrap_or_else(|| &0);
                        if op_precedence < current_op_precedence {
                            break;
                        }
                        let op = operator_stack.pop().unwrap();
                        output.push(op);
                    }
                    operator_stack.push(Atom(token));
                } else {
                    output.push(Atom(token));
                }
            }
            List(_) => {
                output.push(ast_node);
            }
        }
    }

    while !operator_stack.is_empty() {
        if let Some(op) = operator_stack.pop() {
            output.push(op);
        } else {
            break;
        }
    }
    Ok(output)
}

#[cfg(test)]
mod test {
    use crate::parser::parser::convert_to_rpn;
    use crate::parser::AstNode::{Atom, List};
    use crate::tokenizer::{Token, TokenType};

    #[test]
    fn convert_to_rpn_test_variable_declaration() {
        let statement = vec![
            Atom(Token::new(TokenType::Value, "int".to_string())),
            Atom(Token::new(TokenType::Value, "a".to_string())),
            Atom(Token::new(TokenType::Value, "=".to_string())),
            Atom(Token::new(TokenType::Value, "1".to_string())),
        ];
        let converted_statement = convert_to_rpn(statement).unwrap();
        println!("{}", List(converted_statement));
    }

    #[test]
    fn convert_to_rpn_test_variable_expression() {
        // int a = 10 + 33 * 7
        let statement = vec![
            Atom(Token::new(TokenType::Value, "int".to_string())),
            Atom(Token::new(TokenType::Value, "a".to_string())),
            Atom(Token::new(TokenType::Value, "=".to_string())),
            Atom(Token::new(TokenType::Value, "10".to_string())),
            Atom(Token::new(TokenType::Value, "+".to_string())),
            Atom(Token::new(TokenType::Value, "33".to_string())),
            Atom(Token::new(TokenType::Value, "*".to_string())),
            Atom(Token::new(TokenType::Value, "7".to_string())),
        ];
        let converted_statement = convert_to_rpn(statement).unwrap();
        println!("{}", List(converted_statement));
    }
}
