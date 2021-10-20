use std::fmt::{Display, Formatter};

trait Statement:Node{
    fn statementNode();
}

trait Expression:Node{
    fn expressionNode();
}
trait Node:Display{
    fn TokenLiteral() -> String;
}

struct Program{
    Statements:Vec<Statement>,
}

impl Display for Program {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl Node for Program{
    fn TokenLiteral() -> String {
        println!("{}",1);
    }
}