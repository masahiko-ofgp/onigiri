//! `onigiri::tools` contains some tools of handling chars. 
///
/// Sep 13 2018, I removed some structs.
/// Instead of them, I added new function `cast`.

use std::str::FromStr;


pub fn chars_to_string(chars: &Vec<char>) -> String {
    //! Convert from `Vec<char>` to `String`.
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
#[derive(Debug, PartialEq)]
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
        //! assert_eq!(new_vvc.next(), Some("-123".to_string()));
        //! assert_eq!(new_vvc.next(), Some("+".to_string()));
        //! assert_eq!(new_vvc.next(), Some("456".to_string()));
        //! assert_eq!(new_vvc.next(), None);
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
        //! assert_eq!(new_vvc.nth(1), Some("+".to_string()));
        //! assert_eq!(new_vvc.nth(3), None);
        //! ```
        if n < self.attr.len() {
            Some(chars_to_string(&(self.attr[n])))
        } else {
            None
        }
    }
}

pub fn cast<T: FromStr>(vc: &Vec<char>) -> Option<T> {
    //! This function can cast from `Vec<char>` to some types.
    //! ```
    //! let test_vc = vec!['-','1','2'];
    //! assert_eq!(
    //!     onigiri::tools::cast::<i32>(&test_vc),
    //!     Some(-12_i32)
    //! );
    //! ```
    let vc2s = chars_to_string(&vc);
    match T::from_str(&vc2s) {
        Ok(n) => Some(n),
        _ => None
    }
}
