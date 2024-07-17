use std::fmt::{ Display, Formatter, Result };
use std::rc::Rc;

#[derive(Clone)]
pub enum Literal {
    String(String),
    Boolean(bool),
    Number(f32),
    None,
}

#[derive(PartialEq, Eq, Clone)]
pub enum TokenType {
    // Literals
    Identifier, STRING, Number,

    // Symbols
    LeftParen, RightParen,
    Dot, Comma, Equals, True, False, Value,

    // Keywords
    Spawn, Quit, Clear,

    EOF,
}

impl Display for TokenType {
    fn fmt(&self, f: &mut Formatter) -> Result {
        let to_print = match self {
            Self::Identifier => "IDENTIFIER",
            Self::STRING => "STRING",
            Self::Number => "NUMBER",
            Self::LeftParen => "LEFT_PARET",
            Self::RightParen => "RIGHT_PARET",
            Self::Dot=> "DOT",
            Self::Comma => "COMMA",
            Self::Equals => "EQUALS",
            Self::True => "TRUE",
            Self::False => "FALSE",
            Self::Value => "VALUE",
            Self::Spawn => "SPAWN",
            Self::Quit => "QUIT",
            Self::Clear => "CLEAR",
            Self::EOF => "EOF",
        };

        write!(f, "{}", to_print)
    }
}

#[derive(Clone)]
pub struct Token {
    pub token_type: TokenType,
    pub lexeme: String,
    pub literal: Literal,

    /// line number
    pub ln: u16,

    /// column
    pub col: u16,
}

impl Display for Token {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(
            f,
            "Token \"{}\" at line {} column {} of type: {}",
            self.lexeme, self.ln, self.col, self.token_type
        )
    }
}

pub enum AST {
    Command(CommandAST),
    Assignment(AssignmentAST),
    None,
}

pub struct CommandAST {
    /// Name of the command e.g.: `spawn`
    pub name: String,
    pub identifier: String,
    pub parameters: Vec<Literal>,
}

/// Consider something like: `client.fps`
///
/// Here, `client` would be a "Super Property", `fps` would be a "Property".
#[derive(PartialEq, Eq)]
pub enum AssignmentIdentifier {
    SuperProperty(String, Rc<AssignmentIdentifier>),
    Property(String),
    None,
}

pub struct AssignmentAST {
    /// The LHS of the equals sign
    pub identifier: AssignmentIdentifier,

    /// The RHS of the equals sign
    pub literal: Literal,
}

impl Display for AST {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self {
            AST::Command(c) => {

                let mut params = String::default();

                for i in c.parameters.iter() {
                    params = format!("{}, {}", params, i);
                }

                write!(
                    f,
                    "AST::Command(name: \"{}\", identifier: \"{}\", parameters: [{}]",
                    c.name, c.identifier, params
                )
            },

            AST::Assignment(_a) => {
                write!(f, "AST::Assignment")
            }

            AST::None => {
                write!(f, "AST::None")
            }
        }
    }
}

impl Display for Literal {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self {
            Self::String(s) => {
                write!(f, "Literal (type: \"STRING\" value=\"{}\")", s)
            }

            Self::Number(n) => {
                write!(f, "Literal (type: \"NUMBER\" value=\"{}\")", n)
            }

            Self::Boolean(b) => {
                write!(f, "Literal (type: \"BOOLEAN\" value=\"{}\")", b)
            }

            Self::None => {
                write!(f, "Literal (type: \"NONE\")")
            }
        }
    }
}

// pub trait ASTVisitor {
//     fn visitCommandAST(command: &CommandAST) -> Literal;
//     fn visitAssignmentAST(assignment: &AssignmentAST) -> Literal;
// }
// 
// pub trait AST {
//     fn acceptVisitor(&self, visitor: impl ASTVisitor) -> Literal;
// }
// 
// pub struct CommandAST;
// impl AST for CommandAST {
//     fn acceptVisitor(&self, visitor: impl ASTVisitor) -> Literal {
//         visitor.visitCommandAST(&self);
//     }
// }
// 
// pub struct AssignmentAST;
// impl AST for AssignmentAST {
//     fn acceptVisitor(&self, visitor: ASTVisitor<Literal>) -> Literal {
//         visitor.visitAssignmentAST(&self);
//     }
// }

