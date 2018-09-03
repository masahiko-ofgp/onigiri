//! `onigiri::tools` contains 2 functions and 14 structs.
//!
//! Sep 3 2018, I add new struct `Vvc`.
//! However, I didn't changed the previous functions.

use std::str::FromStr;


pub fn chars_to_string(chars: &Vec<char>) -> String {
    //! Convert from `Vec<char>` to `String`.
    //! This function is used in N* and Literal.
    //! ```
    //! let chars: Vec<char> = vec!['-', '1', '2', '3'];
    //! assert_eq!(
    //!     onigiri::tools::chars_to_string(&chars), 
    //!     "-123".to_string());
    //! ```
    let vec_str: Vec<String> = chars.iter()
        .map(|ref v| v.to_string()).collect();
    let result = vec_str.concat();
    result
}

pub fn create_vvchar(text: &String) -> Vec<Vec<char>>{
    //! This function convert from `String` to `Vec<Vec<char>>`.
    //! ```
    //! let text = "123 456".to_string();
    //! assert_eq!(
    //!     onigiri::tools::create_vvchar(&text),
    //!     vec![vec!['1','2','3'], vec!['4','5','6']]
    //! );
    //! ```
    let split_text: Vec<&str> = text.split_whitespace().collect();
    let vvchar: Vec<Vec<char>> = split_text.iter()
        .map(|&x| x.chars().collect()).collect();
    
    vvchar
}

// Vvc is abbreviation of Vec<Vec<char>>.
#[derive(Debug)]
pub struct Vvc {
    pub attr: Vec<Vec<char>>,
    count: usize
}

impl Vvc {
    pub fn new(attr: &String) -> Vvc {
        //! This function create `Vvc` from `String`.
        //! It is almost the same as `create_vvchar()`,
        //! but you can use `next()` and `nth()`.
        //! 
        //! ```
        //! let test_text = "-123".to_string();
        //! let mut new_vvc = onigiri::tools::Vvc::new(&test_text);
        //! assert_eq!(&new_vvc.attr, &vec![vec!['-','1','2','3']]);
        //! ```
        Vvc { attr: create_vvchar(&attr), count: 0 }
    }
}

// This iterator iterates over Vec<Vec<char>> converted to String.
impl Iterator for Vvc {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        //! ```
        //! let test_text = "-123 + 456".to_string();
        //! let mut new_vvc = onigiri::tools::Vvc::new(&test_text);
        //! assert_eq!(&new_vvc.next(), &Some("-123".to_string()));
        //! assert_eq!(&new_vvc.next(), &Some("+".to_string()));
        //! assert_eq!(&new_vvc.next(), &Some("456".to_string()));
        //! assert_eq!(&new_vvc.next(), &None);
        //! ```
        self.count += 1;

        if self.count <= self.attr.len() {
            Some(chars_to_string(&(self.attr[self.count - 1])))
        } else {
            None
        }
    }
    
    fn nth(&mut self, n: usize) -> Option<Self::Item> {
        //! ```
        //! let test_text = "-123 + 456".to_string();
        //! let mut new_vvc = onigiri::tools::Vvc::new(&test_text);
        //! assert_eq!(&new_vvc.nth(1), &Some("+".to_string()));
        //! assert_eq!(&new_vvc.nth(3), &None);
        //! ```
        if n < self.attr.len() {
            Some(chars_to_string(&(self.attr[n])))
        } else {
            None
        }
    }
}

// Create i8 from chars.
pub struct Ni8{pub attr: i8}

impl Ni8 {
    pub fn new(attr: &Vec<char>) -> Ni8 {
        let n = chars_to_string(&attr);
        Ni8 {attr: i8::from_str(&n).unwrap()}
    }
}

// Create u8 from chars.
pub struct Nu8{pub attr: u8}

impl Nu8 {
    pub fn new(attr: &Vec<char>) -> Nu8 {
        let n = chars_to_string(&attr);
        Nu8 {attr: u8::from_str(&n).unwrap()}
    }
}

// Create i16 from chars.
pub struct Ni16{pub attr: i16}

impl Ni16 {
    pub fn new(attr: &Vec<char>) -> Ni16 {
        let n = chars_to_string(&attr);
        Ni16 {attr: i16::from_str(&n).unwrap()}
    }
}

// Create u16 from chars.
pub struct Nu16{pub attr: u16}

impl Nu16 {
    pub fn new(attr: &Vec<char>) -> Nu16 {
        let n = chars_to_string(&attr);
        Nu16 {attr: u16::from_str(&n).unwrap()}
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

// Create u32 from chars.
pub struct Nu32{pub attr: u32}

impl Nu32 {
    pub fn new(attr: &Vec<char>) -> Nu32 {
        let n = chars_to_string(&attr);
        Nu32 {attr: u32::from_str(&n).unwrap()}
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

// Create u64 from chars.
pub struct Nu64{pub attr: u64}

impl Nu64 {
    pub fn new(attr: &Vec<char>) -> Nu64 {
        let n = chars_to_string(&attr);
        Nu64 {attr: u64::from_str(&n).unwrap()}
    }
}

// Create i128 from chars.
pub struct Ni128{pub attr: i128}

impl Ni128 {
    pub fn new(attr: &Vec<char>) -> Ni128 {
        let n = chars_to_string(&attr);
        Ni128 {attr: i128::from_str(&n).unwrap()}
    }
}

// Create u128 from chars.
pub struct Nu128{pub attr: u128}

impl Nu128 {
    pub fn new(attr: &Vec<char>) -> Nu128 {
        let n = chars_to_string(&attr);
        Nu128 {attr: u128::from_str(&n).unwrap()}
    }
}

// Create isize from chars.
pub struct Nisize{pub attr: isize}

impl Nisize {
    pub fn new(attr: &Vec<char>) -> Nisize {
        let n = chars_to_string(&attr);
        Nisize {attr: isize::from_str(&n).unwrap()}
    }
}

// Create usize from chars.
pub struct Nusize{pub attr: usize}

impl Nusize {
    pub fn new(attr: &Vec<char>) -> Nusize {
        let n = chars_to_string(&attr);
        Nusize {attr: usize::from_str(&n).unwrap()}
    }
}

// Create string literal from chars.
pub struct Literal{pub attr: String}

impl Literal {
    pub fn new(attr: &Vec<char>) -> Literal {
        Literal {attr: chars_to_string(&attr)}
    }
}
