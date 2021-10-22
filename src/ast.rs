use std::fmt::{Display, Formatter};

trait Statement:Node{

}

trait Expression:Node{

}

trait Node:Display{
    fn TokenLiteral() -> String;
}
//
// struct IfStatement{
//     Statements:Vec<Statement>,
// }
//
// struct VariableDeclaration{
//     baseInfo:BaseInfo,
//     id:Node,
//     init:Node,
// }
//

struct Token {

}
struct LetStatements{
    identifier:Node,
}


//
//
// //a
// struct Identifier{
//
// }
//
// impl Display for Identifier {
//     fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
//         todo!()
//     }
// }
//
// impl Node for Identifier {
//     fn TokenLiteral() -> String {
//         todo!()
//     }
// }
//
// //3
// struct Literal{
//     a:BaseExpression,
// }
//
// impl Display for Literal {
//     fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
//         todo!()
//     }
// }
//
// impl Node for Literal{
//     fn TokenLiteral() -> String {
//         todo!()
//     }
// }
//
//
//
// //{}
// struct BlockStatement{
//     baseInfo:BaseInfo,
//
// }
//
//
// // =
// struct AssignStatement{
//
//     left:Node,
//     right:Node,
//
// }
//
//
// struct BaseInfo{
//     etype:String,
//     start:int,
//     end:int,
// }
//
// //a == 3
// struct BinaryExpression{
//     baseInfo:BaseInfo,
//     left:Node,
//     right:Node,
// }
//
// struct Program{
//     Statements:Vec<Statement>,
// }
//
//
// impl Display for Program {
//     fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
//         todo!()
//     }
// }
//
//
// impl Node for Program{
//     fn TokenLiteral() -> String {
//         println!("{}",1);
//         println!("商业友谊是指客户与企业保持的一种社交关系,与服务人员保持商业友谊的客户往往比较看中自己与企业间的长期关系,客户吧服务人员看作是朋友,就往往比较容易产生依恋感,也会愿意与服务人员的企业保持长期联系。");
//
//     }
// }
