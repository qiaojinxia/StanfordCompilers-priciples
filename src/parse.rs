use crate::{lexers};
#[macro_use]
use crate::{is_keywords};
use crate::define::*;
use crate::ast::*;

#[derive(Debug)]
pub(crate) struct Parser<'a> {
    scan_token:usize,
    curr_token:usize,
    tokens:Vec<Vec<&'a str>>,
}

impl Parser<'a>{
    pub(crate) fn new(mut tokens:Vec<Vec<&str>>) -> Option<Parser> {
        let parse = Parser{
            scan_token: 0,
            curr_token:0,
            tokens:tokens,
        };
        Some(parse)
    }
    pub(crate) fn next_token(&mut self) -> Option<Vec<&str>> {
        let token  = self.tokens.get(self.scan_token);
        self.scan_token +=1;
        match token {
            None => {return None}
            Some(val) => {
                return Some(token.unwrap().clone());
            }
        }

    }


}



