

mod lexers;
mod parse;
#[macro_use]
mod macros;
mod define;
mod ast;

use crate::define::{Parser, func_parser_var, func_parser_id, parser_literal, TokenType, KeyWord, parser_operator_express};
use crate::parse::TokenScaner;
use crate::define::TokenType::KEYWORDS;

fn main() {
    let tokens = lexers::analysis();
    let token_scanner = TokenScaner::new(tokens).unwrap();
    let mut parser = Parser::new(token_scanner);
    parser.register_statement(KeyWord::VAR,func_parser_var);
    parser.register_express(TokenType::ID ,func_parser_id);

    parser.register_express(TokenType::PLUS ,parser_operator_express);
    parser.register_express(TokenType::SLASH ,parser_operator_express);
    parser.register_express(TokenType::NUM ,parser_literal);

    parser.exec();

}
