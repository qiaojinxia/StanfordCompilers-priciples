

use std::fmt::{Display, Formatter};
use crate::parse::Parser;

pub(crate) trait Node{
    fn Parse( self,_:&mut Parser);
}


pub(crate) struct Express {
  left:Option<Box<Node>>,
  right:Option<Box<Node>>,
}

impl Display for Express {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "({})", self)
    }
}

impl Express {
    pub(crate) fn new() -> Box<Express> {
       Box::new(Express{ left:None, right: None })
    }
}

impl Node for Express {
    fn Parse(mut self , parser: &mut Parser)  {
       loop {
           let token = parser.next_token();
           let mut node = None;
           match token {
               Some(token) => {
                   match token[1]{
                       "var" =>{
                           node = Some(VarStatements::new());
                       }
                       "if" =>{ println!("if")}
                       "return" =>{ println!("return")}
                       &_ => {}
                   }
               }
               None => {break}
           }
           match self.left {
               None => { self.left = node}
               Some(_) => {self.right = node}
           }
       }
    }
}





struct VarStatements{
    variableName:String,
    right:Option<Box<Node>>,
}
//
// impl Display for VarStatements {
//     fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
//         todo!()
//     }
// }
//
//

impl VarStatements{
    pub(crate) fn new() -> Box<Node> {
        Box::new(VarStatements{ variableName: "".to_string(), right: None })
    }
}

impl Node for VarStatements {
    fn Parse(self, _: &mut Parser) {
        todo!()
    }
}


