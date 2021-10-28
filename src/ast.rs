use std::fmt::{Display, Formatter};
use crate::define::NType;

// use std::fmt::{Display, Formatter};
// use crate::parse::Parser;
// use crate::define::{KType, ID, ASSIGN};
//
pub(crate) trait E:Display{

}

pub(crate) trait S:Display{

}

pub(crate) struct VarStatements{
    pub(crate) m_type:NType,
    pub(crate) identifier:Option<Box<dyn E>>,
    pub(crate) init:Option<Box<dyn E>>,
}

impl Display for VarStatements {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl S for VarStatements {
    
}

//
// type ParserFunc = fn(_:Parser,) -> E;
//
//
// pub(crate) struct Express {
//   left:Option<Box<E>>,
//   right:Option<Box<E>>,
// }
//
// impl Display for Express {
//     fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
//         write!(f, "{} ", "")
//     }
// }
//
//
// impl E for Statement {
//     fn Parse(&mut self , parser: &mut Parser)  {
//         let token = parser.cur_token();
//         match token {
//             Some(token) => {
//                 match token[0] {
//                     KEYWORDS=>{
//                         match token[1]{
//                             "var" =>{
//                                 let mut  node = VarStatements::new();
//                                 node.Parse(parser);
//                                 self.left = Some(node);
//                             }
//                             "if" =>{ println!("if")}
//                             "return" =>{ println!("return")}
//                             &_ => {}
//                         }
//                     }
//                 }
//             }
//             None => {return}
//         }
//     }
// }
//
//
// impl Statement{
//
// }
//
// impl Display for Statement {
//     fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
//         todo!()
//     }
// }
//
// impl E for Express {
//     fn Parse(&mut self , parser: &mut Parser)  {
//         let token = parser.cur_token();
//            match token {
//                Some(token) => {
//                    match token[0] {
//                        "ID"=>{
//                            let mut  node = ID::new();
//                            node.Parse(parser);
//                            parser.next_token();
//                            self.left = Some(node);
//                        },
//                        "NUM" =>{
//                            let mut  node = Literal::new();
//                            node.Parse(parser);
//                            self.left = Some(node);
//                            return
//                        }
//                        "SEMICOLON" =>{
//                            return
//                        }
//                        "INFIX" =>{
//                            let mut  node = InfixExpression::new();
//                            node.Parse(parser);
//                            self.left = Some(node);
//                            return
//                        }
//                        _ => {}
//                    }
//                    }
//                None => {return}
//            }
//         let mut rightExpress = Express::new();
//         rightExpress.Parse(parser);
//         self.right = Some(rightExpress);
//     }
// }
//
//
//
pub(crate) struct ID{
    pub(crate) name:String,
    pub(crate) xtype:NType,
}

impl Display for ID {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f,"type:Identifier \n name:{}",self.name)
    }
}

impl E for ID{

}
//
// impl ID{
//     pub(crate) fn new() -> Box<E> {
//         Box::new(ID{ name: "".to_string(), xtype: KType::ID })
//     }
// }
//
// struct VarStatements{
//     mType:KType,
//     Identifier:Option<Box<E>>,
//     init:Option<Box<E>>,
// }
// //
// // impl Display for VarStatements {
// //     fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
// //         todo!()
// //     }
// // }
// //
// //
//
// impl VarStatements{
//     pub(crate) fn new() -> Box<E> {
//         Box::new(VarStatements{ mType: KType::VariableDeclarator, Identifier: None, init: None })
//     }
// }
//
//
struct Literal {
    m_type:NType,
    value:String,
}

impl Literal{
    pub(crate) fn new() -> Box<dyn E> {
        Box::new(Literal{ m_type: NType::None, value: "".to_string() })
    }
}

impl Display for Literal {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f,"type:Literal \n value:{}",self.value)
    }
}

impl E for Literal {

}
//
// impl Display for VarStatements {
//     fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
//         let x = self;
//         write!(f,"type:VariableDeclarator \n   id:{} init:{}",x.Identifier.as_ref().unwrap(),x)
//     }
// }
//
// impl E for VarStatements {
//     fn Parse(&mut self, parser: &mut Parser) {
//         let mut node =  Express::new();
//         parser.next_token();
//         node.Parse(parser);
//         self.Identifier = Some(node);
//         let assign = parser.next_token().unwrap();
//         if assign[0] != ASSIGN{
//             panic!("expected next token to be =!")
//         }
//         let mut express = Express::new();
//         express.Parse(parser);
//         self.init = Some(express);
//         parser.end_statement();
//     }
// }
//
// struct InfixExpression {
//     left:Option<Box<E>>,
//     right:Option<Box<E>>,
//     operator:String,
// }
//
// impl InfixExpression{
//     pub(crate) fn new() -> Box<E> {
//         Box::new(InfixExpression{
//             left: None,
//             right: None,
//             operator: "".to_string()
//         })
//     }
// }
//
// impl Display for InfixExpression {
//     fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
//         todo!()
//     }
// }
//
// impl E for InfixExpression {
//     fn Parse(&mut self, parser: &mut Parser) {
//         parser.prev();
//         let mut l_node = Express::new();
//         l_node.Parse(parser);
//         self.left = Some(node);
//         let mut r_node = Express::new();
//         r_node.Parse(parser);
//
//
//     }
// }
//
