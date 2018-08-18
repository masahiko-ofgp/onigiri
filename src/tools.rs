use std::str::FromStr;

// Create i32 from chars.
pub struct Ni32{pub attr: i32}

impl Ni32 {
    pub fn new(attr: &Vec<char>) -> Ni32 {
        let num_chars: Vec<String> = attr.iter()
            .map(|ref n| n.to_string()).collect();
        
        let num = &num_chars.concat();
        Ni32 {attr: i32::from_str(&num).unwrap()}
    }
}

// Create string literal from chars.
pub struct Literal{pub attr: String}

impl Literal {
    pub fn new(attr: &Vec<char>) -> Literal {
        let str_chars: Vec<String> = attr.iter()
            .map(|ref s| s.to_string()).collect();
        
        let lit = &str_chars.concat();
        
        Literal {attr: lit.to_string()}
    }
}
