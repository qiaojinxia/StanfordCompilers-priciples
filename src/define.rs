use crate::{hashmap};
use crate::ast::{E, S, ID};
use crate::parse::{TokenScaner};
use std::collections::HashMap;
use crate::ast::{VarStatements};
use crate::define::TokenType::{EOF, SEMICOLON};
use std::fmt::{Display, Formatter};
use crate::lexers::Token;
use crate::ast::{OperatorExpress,Literal};


pub const CODE: &str = "
var b3 = 2 * 3 + 2;
";

pub static NEW_LINE: &'static str = "NewLine";
#[derive(Debug,Copy,Clone)]
pub enum KeyWord{
    VAR,
    RETURN,
    IF,
}

impl KeyWord{
    pub fn call(self) -> &'static str {
        match self {
            KeyWord::VAR => "var",
            KeyWord::RETURN => "return",
            KeyWord::IF => "if",
        }
    }
}



#[derive(Debug,Copy,Clone,Eq, PartialEq)]
pub enum TokenType {
    NUM,
    ID,
    MINUS,
    ASTERISK,
    PLUS,
    SLASH,
    KEYWORDS,
    LPAREN,
    RPAREN,
    SEMICOLON,
    WHITESPACE,
    INFIX,
    ASSIGN,
    EOF,
}

impl Display for TokenType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
       write!(f,"{:?}",self)
    }
}

impl TokenType{
    pub fn priority(self) -> i32 {
        match self {
            TokenType::NUM => 0,
            TokenType::ID => 0,
            TokenType::KEYWORDS => 0,
            TokenType::LPAREN => 0,
            TokenType::RPAREN => 0,
            TokenType::SEMICOLON => 0,
            TokenType::WHITESPACE => 0,
            TokenType::INFIX => 0,
            TokenType::ASSIGN => 0,
            TokenType::PLUS => 1,
            TokenType::MINUS => 0,
            TokenType::ASTERISK => 2,
            TokenType::SLASH => 2,
            TokenType::EOF => 0,
        }
    }
    pub fn call(self) -> &'static str {
        match self {
            TokenType::NUM => "None",
            TokenType::ID => "ID",
            TokenType::KEYWORDS => "KEYWORDS",
            TokenType::LPAREN => "LPAREN",
            TokenType::RPAREN => "RPAREN",
            TokenType::SEMICOLON => "SEMICOLON",
            TokenType::WHITESPACE => "WHITESPACE",
            TokenType::INFIX => "INFIX",
            TokenType::ASSIGN => "ASSIGN",
            TokenType::PLUS => "PLUS",
            TokenType::MINUS => "MINUS",
            TokenType::ASTERISK => "ASTERISK",
            TokenType::SLASH => "SLASH",
            TokenType::EOF => "EOF",
        }
    }
}

#[derive(Debug,Copy, Clone)]
pub enum NType{
    None,
    Int,
    Float,
    Strings,
}




