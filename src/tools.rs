use std::str::FromStr;

// Helper function.
// Convert from Vec<char> to String.
// e.g.) vec!['1', '2', '3'] => String "123"
pub fn chars_to_string(chars: &Vec<char>) -> String {
    let vec_str: Vec<String> = chars.iter()
        .map(|ref v| v.to_string()).collect();
    let result = vec_str.concat();
    result
}

// Create u8 from chars.
pub struct Nu8{pub attr: u8}

impl Nu8 {
    pub fn new(attr: &Vec<char>) -> Nu8 {
        let n = chars_to_string(&attr);
        Nu8 {attr: u8::from_str(&n).unwrap()}
    }
}

// Create i32 from chars.
pub struct Ni32{pub attr: i32}

impl Ni32 {
    pub fn new(attr: &Vec<char>) -> Ni32 {
        let n = chars_to_string(&attr);
        Ni32 {attr: i32::from_str(&n).unwrap()}
    }
}

// Create i64 from chars.
pub struct Ni64{pub attr: i64}

impl Ni64 {
    pub fn new(attr: &Vec<char>) -> Ni64 {
        let n = chars_to_string(&attr);
        Ni64 {attr: i64::from_str(&n).unwrap()}
    }
}

// Create string literal from chars.
pub struct Literal{pub attr: String}

impl Literal {
    pub fn new(attr: &Vec<char>) -> Literal {
        Literal {attr: chars_to_string(&attr)}
    }
}
