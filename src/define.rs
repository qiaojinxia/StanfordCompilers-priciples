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
var b3 = 2 + 3 * 2;
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
            TokenType::PLUS => 0,
            TokenType::MINUS => 0,
            TokenType::ASTERISK => 1,
            TokenType::SLASH => 1,
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

pub(crate) type ParserExpressFunc = fn(_:&mut Parser,leftE:Option<Box<dyn E>>) -> Option<Box<dyn E>>;
pub(crate) type ParserStatementFunc = fn(_:&mut Parser) -> Option<Box<dyn S>>;


impl Parser {
    pub(crate) fn new(scanner:TokenScaner) -> Self{
        Parser { statement_func_map: hashmap!(), express_func_map: hashmap!(), scaner: scanner }
    }
    pub(crate) fn register_statement(&mut self, token_type:KeyWord, func:ParserStatementFunc){
            self.statement_func_map.insert(token_type.call(), func);
    }
    pub(crate) fn register_express(&mut self, token_type:TokenType, func:ParserExpressFunc){
        self.express_func_map.insert(token_type.call(), func);
    }

    pub(crate) fn get_express(&self, token_type:TokenType) -> Option<&ParserExpressFunc> {
        self.express_func_map.get(token_type.call())
    }

    pub(crate) fn get_statement(&mut self, key_word:&'static str) -> Option<&ParserStatementFunc> {
        self.statement_func_map.get(key_word)
    }

    pub(crate) fn exec(&mut self){
        loop {
            let token = self.scaner.next_token().unwrap();
            if token.t_type == EOF{
                break
            }
            let key_word = token.literal;
            let wrap_exec_func = self.get_statement(key_word);
            match wrap_exec_func{
                None => { println!("{} Not Implement!",key_word)}
                Some(exec) => {
                     exec(self);
                }
            }

        }
    }
}

pub(crate) fn func_parser_var(parser:&mut Parser) -> Option<Box<dyn S>> {
    let mut _statment = VarStatements{
        m_type: NType::Int,
        identifier: None,
        init: None
    };
    _statment.identifier = func_parser_id(parser,None);
    let assign_token = parser.scaner.next_token().unwrap();
    if assign_token.t_type != TokenType::ASSIGN{
        panic!("友情提示:行:{} 期望 = 找到 {}!",assign_token.line,assign_token.literal)
    }
    _statment.init = parser_express(parser,0);
    Some(Box::new(_statment))
}

pub(crate) fn parser_operator_express(parser:&mut Parser,left_e:Option<Box<dyn E>>) -> Option<Box<dyn E>>{
    let token = parser.scaner.next_token().unwrap();
    let mut _express = OperatorExpress{
        left: left_e,
        Operator: "".to_string(),
        right: None
    };
    let token_priority = token.t_type.priority();
    _express.right = parser_express(parser,token_priority);
    Some(Box::new(_express))
}


pub(crate) fn parser_express(parser:&mut Parser,precedence:i32) -> Option<Box<dyn E>> {
    let mut token = parser.scaner.peek().unwrap().t_type;
    let left_express = parser.get_express(token).unwrap();
    let left_t = left_express(parser,None);
    match parser.scaner.peek(){
        None => {return left_t;}
        Some(token) => {
            if token.literal == SEMICOLON.call(){
                return left_t;
            }
        }
    }
    token = parser.scaner.peek().unwrap().t_type;
    let right_e = parser.get_express(token).unwrap();
    right_e(parser,left_t)
}


pub(crate) fn parser_literal(parser:&mut Parser,left_e:Option<Box<dyn E>>) -> Option<Box<dyn E>> {
    let mut literal = Literal{
        m_type: NType::Int,
        value: "".to_string()
    };
    let token = parser.scaner.next_token().unwrap();
    literal.value = String::from(token.literal);
    Some(Box::new(literal))
}

pub(crate) fn func_parser_id(parser:&mut Parser,left_e:Option<Box<dyn E>>) -> Option<Box<dyn E>> {
    let token = parser.scaner.next_token().unwrap();
    if token.t_type != TokenType::ID{
        panic!("友情提示:行:{} 变量名错误啦!",token.line)
    }
    let _express = ID{
        name: token.literal.parse().unwrap(),
        xtype: NType::None,
    };
    Some(Box::new(_express))
}



