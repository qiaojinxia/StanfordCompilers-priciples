pub mod parse;
pub mod define;


use regex::Regex;

#[macro_export]
macro_rules! isKeywords {
    ( $x:expr ) => {{
        let arr: [&str; 4] = ["function", "return", "if", "var"];
        arr.contains($x)
    }};
}
#[macro_export]
macro_rules! isDigit {
    ( $x:expr ) => {{
        let re = Regex::new(r"\d").unwrap();
        re.is_match($x)
    }};
}
#[macro_export]
macro_rules! isValidId {
    ( $x:expr ) => {{
        let re = Regex::new(r"[A-Za-z0-9]").unwrap();
        re.is_match($x)
    }};
}
#[macro_export]
macro_rules! isBlank {
    ( $x:expr ) => {{
        let re = Regex::new(r"(\s|\t|\n)").unwrap();
        re.is_match($x)
    }};
}

#[derive(Debug, Eq, PartialEq)]
enum S {
    Start,
    Done,
    Num,
    ID,
}

fn main() {
    let mut current_token: Vec<Vec<&str>> = Vec::new();
    let mut state = S::Start;
    let mut index = 0;
    let mut lookup = 0;
    let new_code = define::CODE.as_bytes();
    while lookup < new_code.len() {
        while state != S::Done {
            let tmp = String::from(new_code[lookup] as char);
            lookup += 1;
            let char = tmp.as_str();
            match state {
                S::Start => {
                    if isDigit!(char) {
                        state = S::Num;
                    } else if isBlank!(char) {
                        // current_token.push(vec!(WHITESPACE," "));
                        state = S::Done;
                    } else if isValidId!(char) {
                        state = S::ID;
                    } else if char == "=" {
                        current_token.push(vec!(define::ASSIGN,"="));
                        state = S::Done;
                    } else if char == "+" {
                        current_token.push(vec!(define::PLUS,"+"));
                        state = S::Done;
                    } else if char == ";" {
                        current_token.push(vec!(define::SEMICOLON,";"));
                        state = S::Done;
                    }else if char == "(" {
                        current_token.push(vec!(define::LPAREN,"("));
                        state = S::Done;
                    }else if char == ")" {
                        current_token.push(vec!(define::RPAREN,")"));
                        state = S::Done;
                    };
                }
                S::Done => continue,
                S::Num => {
                    if isDigit!(char) {
                        state = S::Num;
                    } else {
                        current_token.push(vec!(define::NUM,&define::CODE.clone()[index..lookup - 1]));
                        lookup -= 1;
                        state = S::Done;
                    }
                }
                S::ID => {
                    if isValidId!(char) {
                        state = S::ID;
                    } else {
                        let temp_token = &define::CODE.clone()[index..lookup - 1];
                        lookup -= 1;
                        if isKeywords!(&temp_token) {
                            current_token.push(vec!(define::KEYWORDS,temp_token));
                        } else {
                            current_token.push(vec!(define::ID,temp_token));
                        }
                        state = S::Done;
                    }
                }
            }
        }
        index = lookup;
        state = S::Start;
    }
    println!("{:?}", current_token);
    parse::parseTokens(current_token);
}
