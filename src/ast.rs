

use std::fmt::{Display, Formatter};
use crate::parse::Parser;
use crate::define::{KType, ID, ASSIGN};

pub(crate) trait Node:Display{
    fn Parse( &mut self,_:&mut Parser);
}


pub(crate) struct Express {
  left:Option<Box<Node>>,
  right:Option<Box<Node>>,
}

impl Display for Express {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} ", "")
    }
}

impl Express {
    pub(crate) fn new() -> Box<Express> {
       Box::new(Express{ left:None, right: None })
    }
}

impl Node for Express {
    fn Parse(&mut self , parser: &mut Parser)  {
        let token = parser.cur_token();
           match token {
               Some(token) => {
                   match token[0] {
                       "ID"=>{
                           let mut  node = ID::new();
                           node.Parse(parser);
                           self.left = Some(node);
                           return
                       },
                       "NUM" =>{
                           let mut  node = Literal::new();
                           node.Parse(parser);
                           self.left = Some(node);
                           return
                       }
                       "SEMICOLON" =>{
                           return
                       }
                       KEYWORDS=>{
                           match token[1]{
                               "var" =>{
                                   let mut  node = VarStatements::new();
                                   node.Parse(parser);
                                   self.left = Some(node);
                               }
                               "if" =>{ println!("if")}
                               "return" =>{ println!("return")}
                               &_ => {}
                       }
                   }

               }
                   }
               None => {return}
           }
        let mut rightExpress = Express::new();
        rightExpress.Parse(parser);
        self.right = Some(rightExpress);
    }
}



struct ID{
    name:String,
    xtype:KType,
}

impl Display for ID {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f,"type:Identifier \n name:{}",self.name)
    }
}

impl Node for ID{
    fn Parse(&mut self, parser: &mut Parser) {
        let variabaleName = parser.next_token().unwrap();
        if variabaleName[0] != ID {
            panic!("Error Must Be ID!")
        }
        self.name = variabaleName[1].to_string();
        parser.end_token();
    }
}

impl ID{
    pub(crate) fn new() -> Box<Node> {
        Box::new(ID{ name: "".to_string(), xtype: KType::ID })
    }
}

struct VarStatements{
    mType:KType,
    Identifier:Option<Box<Node>>,
    init:Option<Box<Node>>,
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
        Box::new(VarStatements{ mType: KType::VariableDeclarator, Identifier: None, init: None })
    }
}


struct Literal {
    mType:KType,
    value:String,
}

impl Literal{
    pub(crate) fn new() -> Box<Node> {
        Box::new(Literal{ mType: KType::Literal, value: "".to_string() })
    }
}

impl Display for Literal {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f,"type:Literal \n value:{}",self.value)
    }
}

impl Node for Literal {
    fn Parse(&mut self, parser: &mut Parser) {
        let token = parser.next_token().unwrap();
        self.value = token[1].to_string();
        parser.end_token();
    }
}

impl Display for VarStatements {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let x = self;
        write!(f,"type:VariableDeclarator \n   id:{} init:{}",x.Identifier.as_ref().unwrap(),x)
    }
}

impl Node for VarStatements {
    fn Parse(&mut self, parser: &mut Parser) {
        let mut node =  Express::new();
        parser.next_token();
        node.Parse(parser);
        self.Identifier = Some(node);
        let assign = parser.next_token().unwrap();
        if assign[0] != ASSIGN{
            panic!("let express error,need =!")
        }
        let mut express = Express::new();
        express.Parse(parser);
        parser.end_token();
    }
}


