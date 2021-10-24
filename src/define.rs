pub const CODE: &str = "
var b3 = 2;
";


pub static NUM: &'static str = "NUM";
pub static ID: &'static str = "ID";
pub static KEYWORDS: &'static str = "KEYWORDS";
pub static LPAREN: &'static str = "LPAREN";
pub static RPAREN: &'static str = "RPAREN";
pub static SEMICOLON: &'static str = "SEMICOLON";
pub static WHITESPACE: &'static str = "WHITESPACE";
pub static NewLine: &'static str = "NewLine";
pub static PLUS: &'static str = "PLUS";
pub static ASSIGN: &'static str = "ASSIGN";
pub static RETURN: &'static str = "return";


pub static VAR: &'static str = "var";

pub enum KType{
    VariableDeclarator,
    ID,
    Literal,
}

pub enum NType{
    Int,
    Float,
    Strings,
}