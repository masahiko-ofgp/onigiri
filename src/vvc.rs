// Copyright 2019 Masahiko Hamazawa
//
// Licensed under the MIT license <LICENSE or
//  http://opensource.org/licenses/MIT>.
// This file may not be copied, modified, on distributed except
//  according to those terms.

use crate::utils::strcmp;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Vvc { pub attr: Vec<Vec<char>> }

impl Vvc {
    /// This function create new `Vvc`.
    pub fn new<'a>(attr: &'a str, sep: char) -> Self {
        let split_text: Vec<&str> = attr.split(sep).collect();
        let mut vvchar: Vec<Vec<char>> = split_text.iter()
            .map(|&x| x.chars().collect())
            .collect();
        vvchar.shrink_to_fit();
        Vvc { attr: vvchar }
    }
    /// This function find string in `Vvc.attr`.
    ///
    ///     use onigiri::Onigiri;
    ///
    ///     let test_text = "12 hello 34 abc".to_string();
    ///     let oni = Onigiri::new(&test_text);
    ///     let vvc = oni.create_vvc(' ');
    ///
    ///     assert_eq!(true, vvc.find("abc"));
    ///     assert_eq!(false, vvc.find("hollo"));
    ///
    pub fn find<'f>(&self, search_string: &'f str) -> bool {
        for vc in &self.attr {
            if strcmp(vc, search_string) {
                return true;
            }
        }
        false
    }
}
