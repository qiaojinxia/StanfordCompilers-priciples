mod lexers;
mod parse;
#[macro_use]
mod macros;
mod ast;
mod define;
use crate::define::{KeyWord, TokenType};
use crate::parse::{func_parser_id, func_parser_var, parser_literal, parser_operator_express, parser_semicolon, Parser, TokenScaner, parser_lparen_express};

fn main() {
    let tokens = lexers::analysis();
    let token_scanner = TokenScaner::new(tokens).unwrap();
    let mut parser = Parser::new(token_scanner);
    parser.register_statement(KeyWord::VAR, func_parser_var);
    parser.register_express(TokenType::ID, func_parser_id);
    parser.register_express(TokenType::ASTERISK, parser_operator_express);
    parser.register_express(TokenType::PLUS, parser_operator_express);
    parser.register_express(TokenType::SLASH, parser_operator_express);
    parser.register_express(TokenType::NUM, parser_literal);
    parser.register_express(TokenType::SEMICOLON, parser_semicolon);
    parser.register_express(TokenType::LPAREN, parser_lparen_express);
    parser.exec();
}
