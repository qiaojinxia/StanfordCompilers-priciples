use regex::Regex;
use crate::define::CODE;
use crate::define::{SEMICOLON, PLUS, ASSIGN, LPAREN, RPAREN, NUM, KEYWORDS, ID};
use crate::parse;
#[macro_use]
use crate::{is_keywords,is_valid_id,is_digit,is_blank,is_new_line};

#[derive(Debug, Eq, PartialEq)]
enum S {
    Start,
    Done,
    Num,
    ID,
}

pub fn analysis(){
    let mut current_token: Vec<Vec<&str>> = Vec::new();
    let mut state = S::Start;
    let mut index = 0;
    let mut lookup = 0;
    let new_code = CODE.as_bytes();
    let mut cur_line = 1;
    while lookup < new_code.len() {
        while state != S::Done {
            let tmp = String::from(new_code[lookup] as char);
            lookup += 1;
            let char = tmp.as_str();
            match state {
                S::Start => {
                    if is_digit!(char) {
                        state = S::Num;
                    } else if is_new_line!(char) {
                        cur_line +=1;
                        state = S::Done;
                    }else if is_blank!(char) {
                        state = S::Done;
                    } else if is_valid_id!(char) {
                        state = S::ID;
                    } else if char == "=" {
                        current_token.push(vec!(ASSIGN, "="));
                        state = S::Done;
                    } else if char == "+" {
                        current_token.push(vec!(PLUS, "+"));
                        state = S::Done;
                    } else if char == ";" {
                        current_token.push(vec!(SEMICOLON, ";"));
                        state = S::Done;
                    }else if char == "(" {
                        current_token.push(vec!(LPAREN, "("));
                        state = S::Done;
                    }else if char == ")" {
                        current_token.push(vec!(RPAREN, ")"));
                        state = S::Done;
                    };
                }
                S::Done => continue,
                S::Num => {
                    if is_digit!(char) {
                        state = S::Num;
                    } else {
                        current_token.push(vec!(NUM, &CODE.clone()[index..lookup - 1]));
                        lookup -= 1;
                        state = S::Done;
                    }
                }
                S::ID => {
                    if is_valid_id!(char) {
                        state = S::ID;
                    } else {
                        let temp_token = &CODE.clone()[index..lookup - 1];
                        lookup -= 1;
                        if is_keywords!(&temp_token) {
                            current_token.push(vec!(KEYWORDS, temp_token));
                        } else {
                            current_token.push(vec!(ID, temp_token));
                        }
                        state = S::Done;
                    }
                }
            }
        }
        index = lookup;
        state = S::Start;
    }
    parse::parse_program(current_token);
}