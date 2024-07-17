use super::types::{ Token, TokenType, Literal };

pub fn get_keyword(input: &str) -> Result<TokenType, String> {
    match input {
        "spawn" => Ok(TokenType::Spawn),
        // "client" => Ok(TokenType::Client),
        "quit" => Ok(TokenType::Quit),

        _ => Err(String::from("Token not found"))
    }
}

pub struct Scanner {
    pub source: String,
    pub tokens: Vec<Token>,
    pub start: usize,
    pub current: usize,
    pub line: u16,
    pub col: u16,
}

impl Scanner {
    pub fn new(source: String) -> Self { Self {
        source, tokens: vec![], start: 0, current: 0, line: 0, col: 0
    } }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    fn advance(&mut self) -> char {
        let c = self.source.chars().nth(self.current).unwrap();

        self.current += 1;

        c
    }

    fn peek(&self) -> char {
        if self.is_at_end() {
            return '\0';
        }

        self.source.chars().nth(self.current).unwrap()
    }

    fn peek_next(&self) -> char {
        if self.current + 1 >= self.source.len() {
            return '\0';
        }

        self.source.chars().nth(self.current + 1).unwrap()
    }

    fn string(&mut self) {
        while self.peek() != '"' && !self.is_at_end() {
            if self.peek() == '\n' {
                self.line += 1;
            }

            self.advance();
        }

        if self.is_at_end() {
            // TODO: some error mechanism here...
            return;
        }

        self.advance();

        self.add_token_with_literal(
            TokenType::STRING,
            Literal::String(
                self.source.chars().skip(self.start + 1)
                    .take(self.current - self.start - 2)
                    .collect()
            ),
        );
    }

    fn is_digit(&self, c: char) -> bool { c >= '0' && c <= '9' }

    fn is_alpha(&self, c: char) -> bool {
        (c >= 'a' && c <= 'z') || (c >= 'A' && c <= 'Z') || c == '_'
    }

    fn is_alpha_numeric(&self, c: char) -> bool {
        self.is_alpha(c) || self.is_digit(c)
    }

    fn number(&mut self) {
        if self.peek() == '-' {
            self.advance();
        }

        while self.is_digit(self.peek()) {
            self.advance();
        }

        if self.peek() == '.' && self.is_digit(self.peek_next()) {
            self.advance();

            while self.is_digit(self.peek()) {
                self.advance();
            }
        }

        let number_text = self.source.chars().skip(self.start)
            .take(self.current - self.start).collect::<String>();

        let number: f32 = match number_text.parse::<f32>() {
            Ok(num) => num,
            Err(_) => {
                match number_text.parse::<i64>() {
                    Ok(num) => num as f32,
                    Err(e) => {
                        println!("Error while parsing number \"{}\": {}", number_text, e);
                        
                        0.0
                    }
                }
            }
        };

        self.add_token_with_literal(
            TokenType::Number,
            Literal::Number(number),
        );
    }
    
    fn identifier(&mut self) {
        while self.is_alpha_numeric(self.peek()) {
            self.advance();
        }

        let text: String = self.source.chars().skip(self.start)
            .take(self.current - self.start).collect();

        match text.as_str() {
            "spawn" => {
                self.add_token_with_literal(
                    TokenType::Spawn,
                    Literal::String(text)
                )
            }

            "quit" => {
                self.add_token_with_literal(
                    TokenType::Quit,
                    Literal::String(text)
                )
            }

            _ => {
                self.add_token_with_literal(
                    TokenType::Identifier,
                    Literal::String(text)
                )
            }
        }
    }

    fn add_token_with_literal(&mut self, token_type: TokenType, literal: Literal) {
        self.tokens.push(Token {
            token_type,
            lexeme: self.source.chars().skip(self.start)
                .take(self.current - self.start).collect(),
            literal,
            ln: self.line,
            col: 0,
        });
    }

    fn add_token(&mut self, token_type: TokenType) {
        self.tokens.push(Token {
            token_type,
            lexeme: self.source.chars().skip(self.start).take(1)
                .collect(),
            literal: Literal::None,
            ln: self.line,
            col: 0,
        });
    }

    pub fn scan_token(&mut self) {
        let character = self.advance();

        match character {
            '(' => { self.add_token(TokenType::LeftParen); }
            ')' => { self.add_token(TokenType::RightParen); }
            '.' => { self.add_token(TokenType::Dot); }
            ',' => { self.add_token(TokenType::Comma); }
            '=' => { self.add_token(TokenType::Equals); }
            '"' => { self.string(); }
            '\n' => { self.line += 1; self.col = 0; }
            '\r' | ' ' | '\t' => {}
            '-' => {
                if self.is_digit(self.peek()) {
                    self.number();
                }
            }

            _ => {
                if self.is_digit(character) {
                    self.number();
                } else if self.is_alpha(character) {
                    self.identifier();
                } else {
                    println!("Error while parsing, unexpected character at position: {}", self.col);
                }
            }
        }
    }

    // Returns tokens and consumes itself ( :( ).
    pub fn scan_tokens(mut self) -> Vec<Token> {
        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token();
        }

        self.tokens.push(Token {
            token_type: TokenType::EOF,
            lexeme: String::default(),
            literal: Literal::None,
            ln: self.line,
            col: self.col,
        });

        self.tokens
    }
}

