// Copyright 2019 Masahiko Hamazawa
//
// Licensed under the MIT license <LICENSE or
//  http://opensource.org/licenses/MIT>.
// This file may not be copied, modified, on distributed except
//  according to those terms.

//! onigiri::Onigiri is Factory. It can select either `Vvc` or `Btmvc`
//!  and generate it.
//!
//! `Vvc` is `Vec<Vec<char>>`.
//!
//! `Btmvc` is `BTreeMap<usize, Vec<char>>`.
//!
//! onigiri::utils is the utilities of handling `chars`.
//!
//! onigiri::validator is validate whether `Vec<char>` is valid.
//!
//! Some examples are as follows.
//!
//! 
//!     use onigiri::Onigiri;
//!     use onigiri::utils::cast;
//!     use onigiri::validator;
//!
//!     let test_text = "(13 + 2)".to_string();
//!     let oni = Onigiri::new(&test_text);
//!     let vvc = oni.create_vvc(' ');
//!     
//!     assert_eq!(
//!         &vvc.attr,
//!         &vec![vec!['(', '1','3'],vec!['+'],vec!['2', ')']]
//!     );
//!
//!     let thirteen = &vvc.attr[0][1..].to_vec();
//!     assert_eq!(validator::is_positive_integer(&thirteen), true);
//! 
//!     let num = cast::<u8>(&thirteen);
//!     assert_eq!(&num, &Some(13_u8));
//!     assert_eq!(&num.unwrap() + 2, 15_u8);
//!

pub mod vvc;
pub mod btmvc;
pub mod utils;
pub mod validator;

use crate::vvc::Vvc;
use crate::btmvc::Btmvc;

#[derive(Debug)]
pub struct Onigiri { pub attr: String }

impl Onigiri {
    pub fn new<'a>(attr: &'a str) -> Self {
        Onigiri { attr: attr.to_string() }
    }
    /// This function create `Vvc`.
    ///
    ///     use onigiri::Onigiri;
    ///     use onigiri::vvc::Vvc;
    ///
    ///     let test_text = "-123 456".to_string();
    ///     let oni = Onigiri::new(&test_text);
    ///     let vvc = oni.create_vvc(' ');
    ///
    ///     assert_eq!(
    ///         vvc,
    ///         Vvc {
    ///             attr: vec![
    ///                 vec!['-', '1', '2', '3'],
    ///                 vec!['4', '5', '6']
    ///                 ]
    ///         }
    ///         );
    ///
    pub fn create_vvc(&self, sep: char) -> Vvc {
        Vvc::new(&self.attr, sep)
    }
    /// This function create `Btmvc`.
    ///
    ///     use onigiri::Onigiri;
    ///
    ///     let test_text = "-123 456".to_string();
    ///     let oni = Onigiri::new(&test_text);
    ///     let btmvc = oni.create_btmvc(' ');
    ///
    ///     assert_eq!(
    ///         btmvc.attr.get(&0),
    ///         Some(&vec!['-', '1', '2', '3'])
    ///         );
    ///
    ///     assert_eq!(
    ///         btmvc.attr.get(&1),
    ///         Some(&vec!['4', '5', '6'])
    ///         );
    ///
    pub fn create_btmvc(&self, sep: char) -> Btmvc {
        Btmvc::new(&self.attr, sep)
    }
}
