

mod lexers;
mod parse;
#[macro_use]
mod macros;
mod define;
mod ast;

use crate::define::{Parser, func_parser_var, func_parser_id, TokenType};
use crate::parse::TokenScaner;

fn main() {
    let tokens = lexers::analysis();
    let tokenscanner = TokenScaner::new(tokens).unwrap();
    let mut parser = Parser::new(tokenscanner);
    parser.register_statement(TokenType::VAR,func_parser_var);
    parser.register_express(TokenType::ID ,func_parser_id);
    parser.exec();

}
