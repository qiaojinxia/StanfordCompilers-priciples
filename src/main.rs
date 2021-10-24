#![feature(in_band_lifetimes)]

mod lexers;
mod parse;
#[macro_use]
mod macros;
mod define;
mod ast;

fn main() {
    lexers::analysis();
}
