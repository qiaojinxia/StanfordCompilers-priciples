use crate::{lexers};
#[macro_use]
use crate::{is_keywords};
use crate::define::*;


#[derive(Debug)]
struct Parser<'a> {
    scan_token:usize,
    curr_token:usize,
    tokens:Vec<Vec<&'a str>>,
}


impl Parser<'static>{
    fn new(mut tokens:Vec<Vec<&str>>) -> Option<Parser> {
        let parse = Parser{
            scan_token: 0,
            curr_token:0,
            tokens:tokens,
        };
        Some(parse)
    }
    fn next_token(&mut self) -> Option<Vec<&'static str>> {
        let token  = self.tokens.get(self.scan_token);
        self.scan_token +=1;
        match token {
            None => {return None}
            Some(val) => {
                // if val[0] == SEMICOLON{ break }
                statement.push(val.clone());
            }
        }
        Some(token.unwrap().clone())
    }
}

pub fn parse_program(tokens:Vec<Vec<&'static str>>) {
    let mut parse = Parser::new(tokens);
        match parse {
            None => {}
            Some(  ref mut x1) => {
                loop{
                    match x1.next_token() {
                        None =>{break},
                        Some(token ) =>{
                            if token[0] == KEYWORDS {
                                match token[1] {
                                    "var" => {  }
                                    KEYWORDS =>{}

                                    _ => {}
                                    }
                                    println!("{:?}",token);
                            }
                        }
                    }
                }
            }
        }
}


