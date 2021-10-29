use crate::lexers::{Token};
#[macro_use]

#[derive(Debug)]
pub(crate) struct TokenScaner {
    scan_token:usize,
    curr_token:usize,
    tokens:Vec<Token>,
}

impl TokenScaner{
    pub(crate) fn new(_tokens:Vec<Token>) -> Option<TokenScaner> {
        let parse = TokenScaner {
            scan_token: 0,
            curr_token:0,
            tokens:_tokens,
        };
        Some(parse)
    }
    pub(crate) fn next_token(&mut self) -> Option<&Token> {
        let token  = self.tokens.get(self.scan_token);
        self.scan_token +=1;
        token

    }
    pub(crate) fn peek(&self) -> Option<&Token> {
        self.tokens.get(self.scan_token)
    }
    pub(crate) fn end_statement(&mut self)  {
        self.curr_token  = self.scan_token;
    }

}




