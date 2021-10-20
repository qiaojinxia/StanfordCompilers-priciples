use crate::{lexers};
#[macro_use]
use crate::{is_keywords};
use crate::define::*;

pub(crate) fn parse_statement(mut tokens:Vec<Vec<&str>>) -> Option<Vec<Vec<&str>>>{
    let mut result:Vec<Vec<&str>> = vec![];
    tokens.reverse();
    loop{
        let token = tokens.pop().unwrap();
        result.push( token.clone());
        if token[0] == SEMICOLON {
            break;
        }
    }
    if result.len() > 0{
        Some(result)
    }else{
        None
    }
}

pub(crate) fn parse(tokens:Vec<Vec<&str>>) {
    let buffer = parse_statement(tokens);
    let flag = true;
    match buffer {
        None => {}
        Some( ref x) => {
            if flag{
                let isok = Statement(x);
                assert_eq!(isok, false)
            }
        }
    }

}

fn Statement(tokens:&Vec<Vec<&str>>) -> bool{
    let mut  new_token = tokens.clone();
    match new_token.last() {
        None => {return false}
        Some(val) => {
            if val[0] == SEMICOLON{
                new_token.pop();
            }else{
                return false
            }
        }
    }
    Declaration(&new_token) | IfStatement(&new_token) | ReturnStatement(&new_token) | BlockStatement(&new_token)
}

fn Declaration(tokens:&Vec<Vec<&str>>) -> bool { FunctionDeclaration(tokens) | VariableDeclaration(tokens) }

fn FunctionDeclaration(tokens:&Vec<Vec<&str>>) -> bool { false }


fn SequenceExpression(tokens:&[Vec<&str>]) -> bool{
    if tokens.len() == 0{
        return true
    }
    tokens[0][0] == ID || SequenceExpression(&tokens[1..])
}

fn VariableDeclaration(tokens:&Vec<Vec<&str>>) -> bool {
    is_keywords!(&tokens[0][1])  &&  Assign(&tokens[1..])
}

fn Assign(tokens:&[Vec<&str>]) -> bool{
    tokens[0][0] == ID && tokens[1][0] == ASSIGN && P(&tokens[2..])
}

fn P(tokens:&[Vec<&str>]) -> bool {
    tokens[0][0] == NUM
}

fn ADDExpression(tokens:&Vec<Vec<&str>>) -> bool {
    MUlExpression(tokens) || ADDExpression(tokens) && tokens[1][0] == NUM && MUlExpression(tokens)
}

fn MUlExpression(tokens:&Vec<Vec<&str>>) -> bool {
    P(tokens) || MUlExpression(tokens) && tokens[1][0] == NUM
}

fn Expression(tokens: &Vec<Vec<&str>>) -> bool { false }

fn IfStatement(tokens: &Vec<Vec<&str>>) -> bool { false }

fn BlockStatement(tokens: &Vec<Vec<&str>>) -> bool { false }

fn ReturnStatement(tokens: &Vec<Vec<&str>>) -> bool { tokens.len() == 1 && tokens[0][1] == RETURN }
