//! Grammar (Not sophisticated, just gets stuff done!)
//!
//! Also, I don't remember the correct terminology, will fix when I revisit
//! parsing and stuff...
//!
//! spawn box(1, 1, 1, 1, 1, 1)
//! spawn box
//! spawn <primitive>(<arguments>)
//! spawn <primitive>
//! quit
//!
//! give ak47
//! give <weapon>
//!
//! client.fps_max = 180
//!
//! EXPRESSION := COMMAND IDENTIFIER ("(" PARAMETER ")")?
//!     | ASSIGNMENT
//!
//! COMMAND := "spawn" | "quit" | "give"
//! PARAMETER := "" | LITERAL | PARAMETER + ","
//! 
//! ASSIGNMENT = (IDENTIFIER ".")? IDENTIFIER "=" ASSIGNMENT | STRING | NUMBER
//!     | BOOLEAN
//!

use std::rc::Rc;

use super::{
    types::{
        Token, TokenType, AST, CommandAST, Literal, AssignmentIdentifier,
    },
    scanner::Scanner
};

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    pub fn new(source: String) -> Self {
        let tokens = Scanner::new(source).scan_tokens();

        // for (i, token) in tokens.iter().enumerate() {
        //     println!("token[{}]: {}", i, token);
        // }

        Parser {
            tokens,
            current: 0,
        }
    }

    fn peek(&self) -> Token {
        self.tokens[self.current].clone()
    }

    fn previous(&self) -> Token {
        self.tokens[self.current - 1].clone()
    }

    fn is_at_end(&self) -> bool {
        self.peek().token_type == TokenType::EOF
    }

    fn check(&self, token_type: &TokenType) -> bool {
        if self.is_at_end() {
            return false;
        }

        self.peek().token_type == *token_type
    }

    fn advance(&mut self) -> Token {
        if !self.is_at_end() {
            self.current += 1;
        }

        self.previous()
    }

    fn match_with_type(&mut self, token_types: &[TokenType]) -> bool {
        for tt in token_types.iter() {
            if self.check(tt) {
                self.advance();
                return true;
            }
        }

        return false;
    }

    fn parse_command(&mut self) -> AST {
        let name = self.previous().lexeme;
        let mut identifier: String = String::default();
        let mut parameters: Vec<Literal> = vec![];

        if self.match_with_type(&[ TokenType::Identifier ]) {
            identifier = self.previous().lexeme;

            if self.match_with_type(&[ TokenType::LeftParen ]) {
                while !self.match_with_type(&[ TokenType::RightParen ])
                    && !self.is_at_end() {
                    if self.peek().token_type == TokenType::Comma {
                        self.advance();
                        continue;
                    }

                    // For now, accepting only numbers as parameters!
                    if self.peek().token_type == TokenType::Number 
                        || self.peek().token_type == TokenType::STRING {
                        parameters.push(self.peek().literal);
                    }

                    self.advance();
                }
            }
        }

        let command_ast = CommandAST { name, identifier, parameters };

        return AST::Command(command_ast);
    }

    fn parse_assignment(&mut self) -> AST {
        let mut assignment_identifier: AssignmentIdentifier =
            AssignmentIdentifier::None;

        while !self.match_with_type(&[ TokenType::Equals ])
            && !self.is_at_end()
        {
            let token = self.advance();
            
            if token.token_type == TokenType::Dot {
                self.advance();
                continue;
            }

            if assignment_identifier == AssignmentIdentifier::None
                && !token.lexeme.is_empty() {
                assignment_identifier = AssignmentIdentifier::Property(token.lexeme);
            } else {
                let super_property_name = match assignment_identifier {
                    AssignmentIdentifier::Property(s) => s,
                    _ => String::default(),
                };

                let new_assignment_identifier = AssignmentIdentifier::Property(token.lexeme);

                assignment_identifier = AssignmentIdentifier::SuperProperty(
                    super_property_name, Rc::new(new_assignment_identifier),
                );
            }
        }

        AST::None
    }

    pub fn parse(mut self) -> AST {
        if self.match_with_type(&[ TokenType::Spawn, TokenType::Quit ]) {
            return self.parse_command();
        }

        if self.match_with_type(&[ TokenType::Identifier ]) {
            return self.parse_assignment();
        }

        AST::None
    }
}

