

mod lexers;
mod parse;
#[macro_use]
mod macros;
mod define;
mod ast;


use crate::parse::{TokenScaner, func_parser_var, parser_operator_express, parser_literal, func_parser_id, Parser};
use crate::define::TokenType::KEYWORDS;
use crate::define::{TokenType, KeyWord};

fn main() {
    let tokens = lexers::analysis();
    let token_scanner = TokenScaner::new(tokens).unwrap();
    let mut parser = Parser::new(token_scanner);
    parser.register_statement(KeyWord::VAR,func_parser_var);
    parser.register_express(TokenType::ID ,func_parser_id);

    parser.register_express(
        TokenType::ASTERISK ,parser_operator_express);
    parser.register_express(TokenType::PLUS ,parser_operator_express);
    parser.register_express(TokenType::SLASH ,parser_operator_express);
    parser.register_express(TokenType::NUM ,parser_literal);
    parser.register_express(TokenType::SEMICOLON ,parser_literal);
    parser.exec();

}
