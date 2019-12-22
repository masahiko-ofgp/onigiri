// Copyright 2019 Masahiko Hamazawa
//
// Licensed under the MIT license <LICENSE or
//  http://opensource.org/licenses/MIT>.
// This file may not be copied, modified, on distributed except
//  according to those terms.

//! `onigiri::utils` contains some utilities of handling chars.

use std::str::{FromStr, from_utf8};
use crate::validator as vld;

/// Convert from `Vec<char>` to `String`.
///
///     use onigiri::utils::chars_to_string;
///
///     let vc: Vec<char> = vec!['-', '1', '2', '3'];
///
///     assert_eq!(
///         chars_to_string(&vc),
///         "-123".to_string()
///         );
pub fn chars_to_string(chars: &[char]) -> String {
    let vec_str: Vec<u8> = chars.iter()
        .map(|&v| v as u8)
        .collect();
    from_utf8(&vec_str).unwrap().to_string()
}

/// This function can cast from `Vec<char>` to some types.
///
///     use onigiri::utils::cast;
///
///     let test_vc = vec!['0', '.', '1', '2'];
///
///     assert_eq!(
///         cast::<f64>(&test_vc),
///         Some(0.12_f64)
///         );
///
pub fn cast<T: FromStr>(vc: &[char]) -> Option<T> {
    if vld::is_integer(&vc)|vld::is_float(&vc) {
        let vc2s = chars_to_string(&vc);

        match T::from_str(&vc2s) {
            Ok(n) => Some(n),
            _ => None
        }
    } else {
        None
    }
}

/// This function can compare `Vec<char>` with string.
///
///     use onigiri::utils::strcmp;
///
///     let test_vc = vec!['-', '2'];
///
///     assert_eq!(true, strcmp(&test_vc, "-2"));
///
pub fn strcmp<'s>(vc: &[char], cmp_string: &'s str) -> bool {
    let _chars = cmp_string.chars()
        .collect::<Vec<char>>();
    vc == &*_chars
}
