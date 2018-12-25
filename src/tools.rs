//! `onigiri::tools` contains some tools of handling chars. 
///
/// This library is still under develop. 

use std::str::FromStr;
use std::collections::BTreeMap;
use super::validator;


pub fn chars_to_string(chars: &Vec<char>) -> String {
    //! Convert from `Vec<char>` to `String`.
    //! ```
    //! let chars: Vec<char> = vec!['-', '1', '2', '3'];
    //! assert_eq!(
    //!     onigiri::tools::chars_to_string(&chars), 
    //!     "-123".to_string());
    //! ```
    let vec_str: Vec<String> = chars.iter()
        .map(|ref v| v.to_string())
        .collect();
    let result = vec_str.concat();
    result
}

pub fn create_vvchar<'a>(text: &'a str, sep: char) -> Vec<Vec<char>> {
    //! This function convert from `str` to `Vec<Vec<char>>`.
    //! ```
    //! let text = "123 456";
    //! assert_eq!(
    //!     onigiri::tools::create_vvchar(&text, ' '),
    //!     vec![vec!['1','2','3'], vec!['4','5','6']]
    //! );
    //! ```
    let split_text: Vec<&str> = text.split(sep).collect();
    let mut vvchar: Vec<Vec<char>> = split_text.iter()
        .map(|&x| x.chars().collect())
        .collect();
    
    vvchar.shrink_to_fit();
    vvchar
}

pub fn cast<T: FromStr>(vc: &Vec<char>) -> Option<T> {
    //! This function can cast from `Vec<char>` to some types.
    //! ```
    //! let test_vc = vec!['-','1','2'];
    //! assert_eq!(
    //!     onigiri::tools::cast::<i32>(&test_vc),
    //!     Some(-12_i32)
    //! );
    //! let test_float_vc = vec!['0', '.', '1', '2'];
    //! assert_eq!(
    //!     onigiri::tools::cast::<f64>(&test_float_vc),
    //!     Some(0.12_f64)
    //! );
    //! ```
    if validator::is_integer(&vc)|validator::is_float(&vc) {
        let vc2s = chars_to_string(&vc);
        match T::from_str(&vc2s) {
            Ok(n) => Some(n),
            _ => None
        }
    } else {
        None
    }
}

pub fn search_all(btmvc: &BTreeMap<usize, Vec<char>>, word: String) -> Option<Vec<usize>> {
    //! This function can search a word. And return index for `BTreeMap<usize, Vec<char>>`.
    //! ```
    //! use onigiri::tools;
    //!
    //! let new_vvc = tools::Vvc::new(&"Hello world Hello", ' ');
    //! let btm = &new_vvc.to_btm().unwrap();
    //! let result = tools::search_all(&btm, "Hello".to_string());
    //! assert_eq!(result, Some(vec![0, 2]));
    //! ```
    let word_vc: Vec<char> = word.chars().collect();
    let mut stack: Vec<usize> = vec![];
    for k in 0..btmvc.len() {
        if btmvc.get(&k).unwrap() == &word_vc {
            stack.push(k);
        } else { continue; }
    }
    match stack.len() {
        0 => None,
        _ => Some(stack)
    }
}

// Vvc is abbreviation of Vec<Vec<char>>.
#[derive(Debug, PartialEq, Clone)]
pub struct Vvc {
    pub attr: Vec<Vec<char>>
}

impl Vvc {
    pub fn new<'a>(attr: &'a str, sep: char) -> Vvc {
        //! This function create `Vvc` from `str`.
        //! It is almost the same as `create_vvchar()`,
        //! 
        //! ```
        //! let test_text = "-123";
        //! let mut new_vvc = onigiri::tools::Vvc::new(&test_text, ' ');
        //! assert_eq!(&new_vvc.attr, &vec![vec!['-','1','2','3']]);
        //! ```
        Vvc { attr: create_vvchar(&attr, sep) }
    }
    pub fn to_btm(&self) -> Option<BTreeMap<usize, Vec<char>>> {
        //! This function is create BTreeMap of `Vec<char>`.
        //! ```
        //! let test_text = "-123 456";
        //! let mut new_vvc = onigiri::tools::Vvc::new(&test_text, ' ');
        //! let btm = &new_vvc.to_btm().unwrap();
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
        for (k, v) in self.attr.iter().enumerate() {
            bt.insert(k,v.to_vec());
        }
        if bt.is_empty() {
            None
        } else { Some(bt) }
    }
}
