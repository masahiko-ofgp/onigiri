//! `onigiri::tools` contains some tools of handling chars. 
///
/// 2018 Nov 5, I add new function "search_all"

use std::str::FromStr;
use std::collections::BTreeMap;
use onigiri::validator;

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
#[derive(Debug, PartialEq, Clone)]
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
    // TODO: I think this function is useful.
    // But this one may be wasteful.
    pub fn create_btm(self) -> Option<BTreeMap<usize, Vec<char>>> {
        //! This function is create BTreeMap of `Vec<char>`.
        //! Perhaps, this one may be more convenient.
        //! ```
        //! let test_text = "-123 456".to_string();
        //! let mut new_vvc = onigiri::tools::Vvc::new(&test_text);
        //! let btm = &new_vvc.create_btm().unwrap();
        //! assert_eq!(
        //!     btm.get(&0).unwrap(),
        //!     &vec!['-', '1', '2', '3']
        //! );
        //! assert_eq!(
        //!     btm.get(&1).unwrap(),
        //!     &vec!['4', '5', '6']
        //! );
        //! ``` 
        let mut bt = BTreeMap::new();
        for (k, v) in self.attr.into_iter().enumerate() {
            bt.insert(k,v);
        }
        if bt.is_empty() {
            None
        } else { Some(bt) }
    }
    pub fn search_all(self, word: String) -> Option<Vec<usize>> {
        //! This function can search a word. And return index.
        //! ```
        //! extern crate onigiri;
        //! use onigiri::tools::Vvc;
        //! let result = Vvc::search_all(
        //!     Vvc::new(&"Hello world Hello".to_string()),
        //!     "Hello".to_string()
        //! );
        //! assert_eq!(result, Some(vec![0, 2]));
        //! ```
        let base_btm = &self.create_btm().unwrap();
        let word_vc: Vec<char> = word.chars().collect();
        let mut stack: Vec<usize> = vec![];
        for k in 0..base_btm.len() {
            if base_btm.get(&k).unwrap() == &word_vc {
                stack.push(k);
            } else { continue; }
        }
        match stack.len() {
            0 => None,
            _ => Some(stack)
        }
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
    if validator::is_number(&vc) {
        let vc2s = chars_to_string(&vc);
        match T::from_str(&vc2s) {
            Ok(n) => Some(n),
            _ => None
        }
    } else {
        None
    }
}


