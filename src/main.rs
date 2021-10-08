const CODE: &str = "
var b3 = 2;
a = 1 + ( b3 + 4);
return a;
";

static NUM: &'static str = "NUM";
static ID: &'static str = "ID";
static KEYWORDS: &'static str = "KEYWORDS";
static LPAREN: &'static str = "LPAREN";
static RPAREN: &'static str = "RPAREN";
static SEMICOLON: &'static str = "SEMICOLON";
static WHITESPACE: &'static str = "WHITESPACE";
static PLUS: &'static str = "PLUS";
static ASSIGN: &'static str = "ASSIGN";

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
    let mut current_token: Vec<&str> = Vec::new();
    let mut state = S::Start;
    let mut index = 0;
    let mut lookup = 0;
    let new_code = CODE.as_bytes();
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
                        current_token =  vec!(WHITESPACE," ");
                        state = S::Done;
                    } else if isValidId!(char) {
                        state = S::ID;
                    } else if char == "=" {
                        current_token =  vec!(ASSIGN,"=");
                        state = S::Done;
                    } else if char == "+" {
                        current_token =  vec!(PLUS,"+");
                        state = S::Done;
                    } else if char == ";" {
                        current_token =  vec!(SEMICOLON,";");
                        state = S::Done;
                    }else if char == "(" {
                        current_token =  vec!(LPAREN,"(");
                        state = S::Done;
                    }else if char == ")" {
                        current_token =  vec!(RPAREN,")");
                        state = S::Done;
                    };
                }
                S::Done => continue,
                S::Num => {
                    if isDigit!(char) {
                        state = S::Num;
                    } else {
                        current_token =  vec!(NUM,&CODE.clone()[index..lookup - 1]);
                        lookup -= 1;
                        state = S::Done;
                    }
                }
                S::ID => {
                    if isValidId!(char) {
                        state = S::ID;
                    } else {
                        let temp_token = &CODE.clone()[index..lookup - 1];
                        lookup -= 1;
                        if isKeywords!(&temp_token) {
                            current_token =  vec!(KEYWORDS,temp_token);
                        } else {
                            current_token =  vec!(ID,temp_token);
                        }
                        state = S::Done;
                    }
                }
            }
        }
        println!("{:?}", current_token);
        current_token = Vec::new();
        index = lookup;
        state = S::Start;
    }
}
