// Copyright 2019 Masahiko Hamazawa
//
// Licensed under the MIT license <LICENSE or
//  http://opensource.org/licenses/MIT>.
// This file may not be copied, modified, on distributed except
//  according to those terms.

use crate::vvc::Vvc;
use std::collections::BTreeMap;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Btmvc { pub attr: BTreeMap<usize, Vec<char>>}

impl Btmvc {
    /// This function create new `Btmvc`.
    pub fn new<'a>(attr: &'a str, sep: char) -> Self {
        let vvchar = Vvc::new(attr, sep);

        let mut btm = BTreeMap::new();

        for (k, v) in vvchar.attr.iter().enumerate() {
            btm.insert(k, v.to_vec());
        }
        Btmvc { attr: btm }
    }
    /// This function can search a word.
    /// And return index for `BTreeMap<usize, Vec<char>>`.
    ///
    ///     use onigiri::Onigiri;
    ///
    ///     let test_text = "Hello hello Hello".to_string();
    ///     let oni = Onigiri::new(&test_text);
    ///     let btmvc = oni.create_btmvc(' ');
    ///
    ///     let result = btmvc.search_all("Hello");
    ///
    ///     assert_eq!(result, Some(vec![0, 2]));
    ///
    pub fn search_all<'a>(self, word: &'a str) -> Option<Vec<usize>> {
        let word_vc: Vec<char> = word.chars().collect();
        let mut stack: Vec<usize> = vec![];

        for k in self.attr.keys() {
            if self.attr.get(&k).unwrap() == &word_vc {
                stack.push(*k);
            } else {
                continue;
            }
        }
        match stack.len() {
            0 => None,
            _ => Some(stack),
        }
    }
}
