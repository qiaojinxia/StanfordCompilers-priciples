mod lexers;
mod parse;
#[macro_use]
mod macros;
mod define;
mod ast;

fn main() {
    lexers::analysis();
}
