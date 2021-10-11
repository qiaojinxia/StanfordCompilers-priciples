use crate::define;


pub(crate) fn parse_statement(mut tokens:Vec<Vec<&str>>) -> Option<Vec<Vec<&str>>>{
    let mut result:Vec<Vec<&str>> = vec![];
    tokens.reverse();
    loop{
        let token = tokens.pop().unwrap();
        result.push( token.clone());
        if token[0] == define::SEMICOLON {
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
    while buffer != None   && flag{
        // let isok = Statement(buffer.unwrap());
        // println!("{}",isok)
    }
}

fn Statement(tokens:Vec<Vec<&str>>) -> bool{
    true
}

fn IfStatement() -> bool { true }

