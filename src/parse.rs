use crate::define;

pub(crate) fn parseTokens(mut tokens:Vec<Vec<&str>>) -> Vec<Vec<&str>>{
    let mut result:Vec<Vec<&str>> = vec![];
    loop{
        let token = tokens.pop().unwrap();
        result.push( token.clone());
        if token[0] == define::SEMICOLON {
            break;
        }
    }
    result
}