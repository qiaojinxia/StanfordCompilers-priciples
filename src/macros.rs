#[macro_export]
macro_rules! is_keywords {
    ( $x:expr ) => {{
        let arr: [&str; 4] = ["function", "return", "if", "var"];
        arr.contains($x)
    }};
}
#[macro_export]
macro_rules! is_digit {
    ( $x:expr ) => {{
        let re = Regex::new(r"\d").unwrap();
        re.is_match($x)
    }};
}
#[macro_export]
macro_rules! is_valid_id {
    ( $x:expr ) => {{
        let re = Regex::new(r"[A-Za-z0-9]").unwrap();
        re.is_match($x)
    }};
}
#[macro_export]
macro_rules! is_new_line {
    ( $x:expr ) => {{
        let re = Regex::new(r"(\t|\n)").unwrap();
        re.is_match($x)
    }};
}

#[macro_export]
macro_rules! is_blank {
    ( $x:expr ) => {{
        let re = Regex::new(r"(\s)").unwrap();
        re.is_match($x)
    }};
}
