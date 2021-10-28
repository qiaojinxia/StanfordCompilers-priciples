use crate::{hashmap};
use crate::ast::{E, S, ID};
use crate::parse::{TokenScaner};
use std::collections::HashMap;
use crate::ast::{VarStatements};
use crate::define::TokenType::EOF;
use std::fmt::{Display, Formatter};


pub const CODE: &str = "
var b3 = 2 + 3 * 2;
";

pub static NEW_LINE: &'static str = "NewLine";
#[derive(Debug)]
pub enum KeyWord{
    VAR,
    RETURN,
    IF,
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
    RETURN,
    VAR,
    EOF,
}

impl Display for TokenType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
       write!(f,"{:?}",self)
    }
}

impl TokenType{
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
            TokenType::RETURN => "RETURN",
            TokenType::VAR => "VAR",
            TokenType::EOF => "EOF",
        }
    }
}

pub enum NType{
    None,
    Int,
    Float,
    Strings,
}

pub struct Parser {
    statement_func_map:HashMap<&'static str, ParserStatementFunc>,
    express_func_map:HashMap<&'static str, ParserExpressFunc>,
    scaner:TokenScaner,
}

pub(crate) type ParserExpressFunc = fn(_:&mut Parser) -> Option<Box<dyn E>>;
pub(crate) type ParserStatementFunc = fn(_:&mut Parser) -> Option<Box<dyn S>>;


impl Parser {
    pub(crate) fn new(scanner:TokenScaner) -> Self{
        Parser { statement_func_map: hashmap!(), express_func_map: hashmap!(), scaner: scanner }
    }
    pub(crate) fn register_statement(&mut self, token_type:TokenType, func:ParserStatementFunc){
            self.statement_func_map.insert(token_type.call(), func);
    }
    pub(crate) fn register_express(&mut self, token_type:TokenType, func:ParserExpressFunc){
        self.express_func_map.insert(token_type.call(), func);
    }

    pub(crate) fn get_express(&self, token_type:TokenType) -> Option<&ParserExpressFunc> {
        self.express_func_map.get(token_type.call())
    }

    pub(crate) fn get_statement(&mut self, token_type:TokenType) -> Option<&ParserStatementFunc> {
        self.statement_func_map.get(token_type.call())
    }

    pub(crate) fn exec(&mut self){
        loop {
            let token = self.scaner.next_token().unwrap();
            if token.t_type == EOF{
                break
            }
            let token_type = token.t_type;
            let wrap_exec_func = self.get_statement(token_type);
            match wrap_exec_func{
                None => { println!("{} Not Implement!",token_type)}
                Some(exec) => {
                    let aa = exec(self);
                }
            }

        }
    }
}

pub(crate) fn func_parser_var(parser:&mut Parser) -> Option<Box<dyn S>> {
    let _statment = VarStatements{
        m_type: NType::Int,
        identifier: None,
        init: None
    };
    let _token = parser.scaner.next_token();
    match  _token {
        None => {}
        Some(token) => {
            let t_type = token.t_type;
            let id_parse = parser.get_express(t_type).unwrap();
            id_parse(parser);
        }

    }
    None
}

pub(crate) fn func_parser_id(parser:&mut Parser) -> Option<Box<dyn E>> {
    let token = parser.scaner.next_token().unwrap();
    if token.t_type != TokenType::ID{
        panic!("友情提示:行:{} 变量名错误啦!",token.line)
    }
    let _express = ID{
        name: token.literal.parse().unwrap(),
        xtype: NType::None,
    };
    None
}



