// Copyright 2019 Masahiko Hamazawa
//
// Licensed under the MIT license <LICENSE or
//  http://opensource.org/licenses/MIT>.
// This file may not be copied, modified, on distributed except
//  according to those terms.

//! `onigiri::tools` contains some tools of handling chars. 

use std::str::{FromStr, from_utf8};
use std::collections::BTreeMap;
use super::validator;


/// Convert from `Vec<char>` to `String`.
/// 
///     use onigiri::tools::chars_to_string;
///
///     let chars: Vec<char> = vec!['-', '1', '2', '3'];
///     assert_eq!(
///         chars_to_string(&chars), 
///         "-123".to_string()
///     );
/// 
pub fn chars_to_string(chars: &Vec<char>) -> String {
    let vec_str: Vec<u8> = chars.iter()
        .map(|&v| v as u8)
        .collect();
    from_utf8(&vec_str).unwrap().to_string()
}

/// This function can cast from `Vec<char>` to some types.
/// 
///     use onigiri::tools::cast;
///
///     let test_vc = vec!['-','1','2'];
///
///     assert_eq!(
///         cast::<i32>(&test_vc),
///         Some(-12_i32)
///     );
///
///     let test_float_vc = vec!['0', '.', '1', '2'];
///
///     assert_eq!(
///         cast::<f64>(&test_float_vc),
///         Some(0.12_f64)
///     );
///
pub fn cast<T: FromStr>(vc: &Vec<char>) -> Option<T> {
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

/// This function can search a word. 
/// And return index for `BTreeMap<usize, Vec<char>>`.
/// 
///     use onigiri::tools::{Vvc, search_all};
///
///     let new_vvc = Vvc::new(&"Hello world Hello", ' ');
///     let btm = &new_vvc.to_btm().unwrap();
///     let result = search_all(&btm, "Hello".to_string());
///
///     assert_eq!(result, Some(vec![0, 2]));
///
pub fn search_all(btmvc: &BTreeMap<usize, Vec<char>>, word: String) -> Option<Vec<usize>> {
    let word_vc: Vec<char> = word.chars().collect();
    let mut stack: Vec<usize> = vec![];
    for k in btmvc.keys() {
        if btmvc.get(&k).unwrap() == &word_vc {
            stack.push(*k);
        } else { continue; }
    }
    match stack.len() {
        0 => None,
        _ => Some(stack)
    }
}


/// Vvc is abbreviation of `Vec<Vec<char>>`.
#[derive(Debug, PartialEq, Clone)]
pub struct Vvc {
    pub attr: Vec<Vec<char>>
}

impl Vvc {

    /// This function create `Vvc` from `str`.
    ///
    ///     use onigiri::tools::Vvc;
    /// 
    ///     let test_text = "-123".to_string();
    ///     let mut new_vvc = Vvc::new(&test_text, ' ');
    ///     
    ///     assert_eq!(
    ///         &new_vvc.attr, 
    ///         &vec![vec!['-','1','2','3']]
    ///     );
    ///
    pub fn new<'a>(attr: &'a str, sep: char) -> Vvc { 
        let split_text: Vec<&str> = attr.split(sep).collect();
        let mut vvchar: Vec<Vec<char>> = split_text.iter()
            .map(|&x| x.chars().collect())
            .collect();
    
        vvchar.shrink_to_fit();
        Vvc { attr: vvchar }
    }

    /// This function is create BTreeMap of `Vec<char>`.
    /// 
    ///     use onigiri::tools::Vvc;
    /// 
    ///     let test_text = "-123 456".to_string();
    ///     let mut new_vvc = Vvc::new(&test_text, ' ');
    ///     let btm = &new_vvc.to_btm().unwrap();
    ///     
    ///     assert_eq!(
    ///          btm.get(&0).unwrap(),
    ///          &vec!['-', '1', '2', '3']
    ///     );
    ///     
    ///     assert_eq!(
    ///         btm.get(&1).unwrap(),
    ///         &vec!['4', '5', '6']
    ///     );
    ///  
    pub fn to_btm(&self) -> Option<BTreeMap<usize, Vec<char>>> {
        let mut bt = BTreeMap::new();
        for (k, v) in self.attr.iter().enumerate() {
            bt.insert(k, v.to_vec());
        }
        if bt.is_empty() {
            None
        } else { Some(bt) }
    }
}
