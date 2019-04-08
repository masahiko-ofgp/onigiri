// Copyright 2019 Masahiko Hamazawa
//
// Licensed under the MIT license <LICENSE or
//  http://opensource.org/licenses/MIT>.
// This file may not be copied, modified, on distributed except
//  according to those terms.

//! onigiri::validator contains some functions for validating `Vec<char>`.


/// Validate `Vec<char>` whether it is integer.
/// 
///     use onigiri::validator::is_integer;
///
///     let test_chars_1 = vec!['1', '2'];
///     let test_chars_2 = vec!['-', '1'];
///     let test_chars_3 = vec!['-', 'a'];
///     assert_eq!(
///         is_integer(&test_chars_1), 
///         true
///     );
///     assert_eq!(
///         is_integer(&test_chars_2),
///         true
///     );
///     assert_eq!(
///         is_integer(&test_chars_3),
///         false
///     );
///
pub fn is_integer(vc: &Vec<char>) -> bool {
    (is_positive_integer(&vc) == true)|
        (is_negative_integer(&vc) == true)
}

/// Validate `Vec<char>` whether it is positive integer.
/// 
///     use onigiri::validator::is_positive_integer;
///
///     let test_chars_1 = vec!['1', '2', '3'];
///     let test_chars_2 = vec!['2', '+', '1']; 
///     let test_chars_3 = vec!['-', '1', '2'];
///
///     assert_eq!(
///         is_positive_integer(&test_chars_1),
///         true
///     );
///     assert_eq!(
///         is_positive_integer(&test_chars_2),
///         false
///     );
///     assert_eq!(
///         is_positive_integer(&test_chars_3),
///         false
///     );
/// 
pub fn is_positive_integer(vc: &Vec<char>) -> bool {
    let mut iter = vc.iter().peekable();

    loop {
        match iter.next() {
            Some(p) => match p {
                '0'...'9' => continue,
                _ => return false
            },
            None => break
        }
    }
    true
}


/// Validate `Vec<char>` whether it is negative integer.
///  
///     use onigiri::validator::is_negative_integer;
///
///     let test_chars_1 = vec!['1', '2', '3'];
///     let test_chars_2 = vec!['-', '2', '1'];
///     let test_chars_3 = vec!['2', '-', '1'];
///
///     assert_eq!(
///         is_negative_integer(&test_chars_1),
///         false
///     );
///     assert_eq!(
///         is_negative_integer(&test_chars_2),
///         true
///     );
///     assert_eq!(
///         is_negative_integer(&test_chars_3),
///         false
///     );
/// 
pub fn is_negative_integer(vc: &Vec<char>) -> bool {
    let head = &vc[0];
    let tail = &vc[1..];

    if head == &'-' {
        if is_positive_integer(&tail.to_vec()) == true {
            true
        } else {
            false
        }
    } else {
        false
    }
}


/// Validate `Vec<char>` whether it is float.
///  
///     use onigiri::validator::is_float;
///
///     let test_float1 = vec!['1', '.', '2'];
///     let test_float2 = vec!['-', '1', '.', '2'];
///     let test_float3 = vec!['0', '.', '5', '.'];
/// 
///     assert_eq!(
///         is_float(&test_float1), 
///         true
///     );
///     assert_eq!(
///         is_float(&test_float2),
///         true
///     );
///     assert_eq!(
///         is_float(&test_float3),
///         false
///     );
/// 
pub fn is_float(vc: &Vec<char>) -> bool {
    (is_positive_float(&vc) == true)|
        (is_negative_float(&vc) == true)
}


/// Validate `Vec<char>` whether it is positive float.
/// 
///     use onigiri::validator::is_positive_float;
///
///     let test_float = vec!['0', '.', '1', '2'];
///     let test_float2 = vec!['0', '.', '.', '1'];
///     let test_float3 = vec!['-', '0', '.', '1'];
///
///     assert_eq!(
///         is_positive_float(&test_float),
///         true
///     );
///     assert_eq!(
///         is_positive_float(&test_float2),
///         false
///     );
///     assert_eq!(
///         is_positive_float(&test_float3),
///         false
///     );
/// 
pub fn is_positive_float(vc: &Vec<char>) -> bool {
    let mut iter = vc.iter().peekable();
    let mut count: usize = 0;
    let mut stack: Vec<bool> = vec![];

    loop {
        match iter.next() {
            Some(i) => match i {
                '.' => count += 1,
                '0'...'9' => stack.push(true),
                _ => stack.push(false)
            },
            None => break
        }
    }
    if count == 1_usize {
        if stack.iter().all(|&r| r == true) {
            true 
        } else {
            false
        }
    } else {
        false
    }
}


/// Validate `Vec<char>` whether it is negative float.
/// 
///     use onigiri::validator::is_negative_float;
///
///     let test_float = vec!['0', '.', '1', '2'];
///     let test_float2 = vec!['-', '0', '.', '1'];
///
///     assert_eq!(
///         is_negative_float(&test_float),
///         false
///     );
///     assert_eq!(
///         is_negative_float(&test_float2),
///         true
///     );
/// 
pub fn is_negative_float(vc: &Vec<char>) -> bool {
    let head = &vc[0];
    let tail = &vc[1..];

    if head == &'-' {
        if is_positive_float(&tail.to_vec()) == true {
            true
        } else {
            false
        }
    } else {
        false
    }
}


/// Validate `Vec<char>` whether it is punctuation.
/// 
///     use onigiri::validator::is_punctuation;
///
///     let test_vc = vec!['+'];
///
///     assert_eq!(
///         is_punctuation(&test_vc),
///         true
///     );
///
///     let test_vc_2 = vec!['-', '2'];
///
///     assert_eq!(
///         is_punctuation(&test_vc_2),
///         false
///     );
///
///     let test_vc_3 = vec!['(', ')'];
///
///     assert_eq!(
///         is_punctuation(&test_vc_3),
///         true
///     );
/// 
pub fn is_punctuation(vc: &Vec<char>) -> bool {
    let mut iter = vc.iter().peekable();

    loop {
        match iter.next() {
            Some(p) => if p.is_ascii_punctuation() {
                continue;
            } else {
                return false;
            },
            None => break
        }
    }
    true
}


/// Validate `Vec<char>` whether it is all ascii-lowercase.
///
///     use onigiri::validator::is_lower_ascii;
///     
///     let test_vc = vec!['h', 'e', 'l', 'l', 'o'];
///     let test_vc2 = vec!['H', 'e', 'l', 'l', 'o'];
///     let test_vc3 = vec!['1', '2', 'a'];
///     
///     assert_eq!(
///         is_lower_ascii(&test_vc),
///         true
///     );
///
///     assert_eq!(
///         is_lower_ascii(&test_vc2),
///         false
///     );
///
///     assert_eq!(
///         is_lower_ascii(&test_vc3),
///         false
///     );
///
pub fn is_lower_ascii(vc: &Vec<char>) -> bool {
    let mut iter = vc.iter().peekable();

    loop {
        match iter.next() {
            Some(l) => if l.is_ascii_lowercase() {
                continue;
            } else {
                return false;
            },
            None => break,
        }
    }
    true
}

/// Validate `Vec<char>` whether it is all ascii-uppercase.
///
///     use onigiri::validator::is_upper_ascii;
///     
///     let test_vc = vec!['h', 'e', 'l', 'l', 'o'];
///     let test_vc2 = vec!['H', 'E', 'L', 'L', 'O'];
///     let test_vc3 = vec!['1', '2', 'a'];
///     
///     assert_eq!(
///         is_upper_ascii(&test_vc),
///         false
///     );
///
///     assert_eq!(
///         is_upper_ascii(&test_vc2),
///         true
///     );
///
///     assert_eq!(
///         is_upper_ascii(&test_vc3),
///         false
///     );
///     
pub fn is_upper_ascii(vc: &Vec<char>) -> bool {
    let mut iter = vc.iter().peekable();

    loop {
        match iter.next() {
            Some(l) => if l.is_ascii_uppercase() {
                continue;
            } else {
                return false;
            },
            None => break,
        }
    }
    true
}


/// Validate `Vec<char>` whether it is title case.
///
///     use onigiri::validator::is_title;
///
///     let test_vc = vec!['H', 'e', 'l', 'l', 'o'];
///     let test_vc2 = vec!['h', 'E', 'l', 'L', 'O'];
///
///     assert_eq!(
///         is_title(&test_vc),
///         true
///     );
///
///     assert_eq!(
///         is_title(&test_vc2),
///         false
///     );
///
pub fn is_title(vc: &Vec<char>) -> bool {
    let head = &vc[0];
    let tail = &vc[1..];

    match head {
        'A' ... 'Z' => if is_lower_ascii(&tail.to_vec()) {
            true
        } else {
            false
        },
        _ => false
    }
}
