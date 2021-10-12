mod lexical_analysis;
mod parse;
#[macro_use]
mod macros;
mod define;

fn main() {
    lexical_analysis::analysis();
}
