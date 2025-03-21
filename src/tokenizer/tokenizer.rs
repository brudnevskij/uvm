// TODO: consider replacing Value with Identifier, Number and Keyword.
#[derive(Debug, PartialEq)]
pub enum TokenType {
    Value,
    Punctuation,
    EOF,
}

#[derive(Debug, PartialEq)]
pub struct Token {
    pub token_type: TokenType,
    pub lexeme: String,
}

impl Token {
    pub fn new(token_type: TokenType, lexeme: String) -> Token {
        Token { token_type, lexeme }
    }
}

pub struct Tokenizer<'a> {
    input: &'a str,
    position: usize,
    column: usize,
    line: usize,
}

impl<'a> Tokenizer<'a> {
    pub fn new(input: &'a str) -> Tokenizer<'a> {
        Tokenizer {
            input,
            position: 0,
            column: 1,
            line: 0,
        }
    }

    fn is_at_end(&self) -> bool {
        self.position >= self.input.len()
    }

    fn peek(&self) -> Option<char> {
        self.input[self.position..].chars().next()
    }

    fn advance(&mut self) -> Option<char> {
        if self.is_at_end() {
            return None;
        }

        let c = self.peek()?;
        self.position += c.len_utf8();
        self.column += 1;
        if c == '\n' {
            self.line += 1;
            self.column = 1;
        }
        Some(c)
    }

    fn tokenize_value(&mut self) -> Token {
        let start = self.position;
        while let Some(c) = self.peek() {
            if !c.is_alphanumeric() && c != '_' {
                break;
            }
            self.advance();
        }

        let lexeme = &self.input[start..self.position];
        Token::new(TokenType::Value, lexeme.to_string())
    }

    pub fn tokenize(&mut self) -> Result<Vec<Token>, String> {
        let mut tokens: Vec<Token> = Vec::new();
        while !self.is_at_end() {
            if let Some(c) = self.peek() {
                match c {
                    ' ' | '\t' | '\n' | '\r' => {
                        self.advance();
                    }
                    '{' | '}' | ';' | '(' | ')' => {
                        let t = Token::new(TokenType::Punctuation, c.to_string());
                        tokens.push(t);
                        self.advance();
                    }
                    'a'..='z' | 'A'..='Z' | '_' | '0'..='9' => {
                        tokens.push(self.tokenize_value());
                    }
                    _ => {
                        return Err(format!(
                            "Unexpected character {}, at line {} column {}",
                            c, self.line, self.column
                        ));
                    }
                }
            }
        }
        tokens.push(Token::new(TokenType::EOF, String::new()));
        Ok(tokens)
    }
}
