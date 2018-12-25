//! onigiri::validator contains some functions for validating `Vec<char>`.


pub fn is_integer(vc: &Vec<char>) -> bool {
    //! Validate `Vec<char>` whether it is integer.
    //! ```
    //! let test_chars_1 = vec!['1', '2'];
    //! let test_chars_2 = vec!['-', '1'];
    //! let test_chars_3 = vec!['-', 'a'];
    //! assert_eq!(
    //!     onigiri::validator::is_integer(&test_chars_1), 
    //!     true
    //! );
    //! assert_eq!(
    //!     onigiri::validator::is_integer(&test_chars_2),
    //!     true
    //! );
    //! assert_eq!(
    //!     onigiri::validator::is_integer(&test_chars_3),
    //!     false
    //! );
    let mut stack: Vec<bool> = vec![];

    if is_positive_integer(&vc) {
        stack.push(true);
    } else if is_negative_integer(&vc) {
        stack.push(true);
    } else {
        return false;
    }

    if stack.iter().all(|&r| r == true) {true}
    else {false}

}

pub fn is_positive_integer(vc: &Vec<char>) -> bool {
    //! Validate `Vec<char>` whether it is positive integer.
    //! ```
    //! let test_chars_1 = vec!['1', '2', '3'];
    //! let test_chars_2 = vec!['2', '+', '1'];
    //! let test_chars_3 = vec!['-', '1', '2'];
    //!
    //! assert_eq!(
    //!     onigiri::validator::is_positive_integer(&test_chars_1), true);
    //! assert_eq!(
    //!     onigiri::validator::is_positive_integer(&test_chars_2), false);
    //! assert_eq!(
    //!     onigiri::validator::is_positive_integer(&test_chars_3), false);
    //! ```
    let mut iter = vc.iter().peekable();

    loop {
        match iter.next() {
            Some(p) => match Some(p).unwrap() {
                '0'...'9' => continue,
                _ => return false
            },
            None => break
        }
    }
    true
}

pub fn is_negative_integer(vc: &Vec<char>) -> bool {
    //! Validate `Vec<char>` whether it is negative integer.
    //! ```
    //! let test_chars_1 = vec!['1', '2', '3'];
    //! let test_chars_2 = vec!['-', '2', '1'];
    //! let test_chars_3 = vec!['2', '-', '1'];
    //! assert_eq!(
    //!     onigiri::validator::is_negative_integer(&test_chars_1), false);
    //! assert_eq!(
    //!     onigiri::validator::is_negative_integer(&test_chars_2), true);
    //! assert_eq!(
    //!     onigiri::validator::is_negative_integer(&test_chars_3), false);
    //! ```
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

pub fn is_float(vc: &Vec<char>) -> bool {
    //! Validate `Vec<char>` whether it is float.
    //! ```
    //! let test_float1 = vec!['1', '.', '2'];
    //! let test_float2 = vec!['-', '1', '.', '2'];
    //! let test_float3 = vec!['0', '.', '5', '.'];
    //! assert_eq!(
    //!     onigiri::validator::is_float(&test_float1), 
    //!     true
    //! );
    //! assert_eq!(
    //!     onigiri::validator::is_float(&test_float2),
    //!     true
    //! );
    //! assert_eq!(
    //!     onigiri::validator::is_float(&test_float3),
    //!     false
    //! );
    //! ```
    let mut stack: Vec<bool> = vec![];

    if is_positive_float(&vc) {
        stack.push(true);
    } else if is_negative_float(&vc) {
        stack.push(true);
    } else {
        return false;
    }

    if stack.iter().all(|&r| r == true) {true}
    else {false}

}

pub fn is_positive_float(vc: &Vec<char>) -> bool {
    //! Validate `Vec<char>` whether it is positive float.
    //! ```
    //! let test_float = vec!['0', '.', '1', '2'];
    //! let test_float2 = vec!['0', '.', '.', '1'];
    //! let test_float3 = vec!['-', '0', '.', '1'];
    //! assert_eq!(
    //!     onigiri::validator::is_positive_float(&test_float),
    //!     true
    //! );
    //! assert_eq!(
    //!     onigiri::validator::is_positive_float(&test_float2),
    //!     false
    //! );
    //! assert_eq!(
    //!     onigiri::validator::is_positive_float(&test_float3),
    //!     false
    //! );
    //! ```
    let mut iter = vc.iter().peekable();
    let mut count: usize = 0;
    let mut stack: Vec<bool> = vec![];

    loop {
        match iter.next() {
            Some(i) => match Some(i).unwrap() {
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

pub fn is_negative_float(vc: &Vec<char>) -> bool {
    //! Validate `Vec<char>` whether it is negative float.
    //! ```
    //! let test_float = vec!['0', '.', '1', '2'];
    //! let test_float2 = vec!['-', '0', '.', '1'];
    //! assert_eq!(
    //!     onigiri::validator::is_negative_float(&test_float),
    //!     false
    //! );
    //! assert_eq!(
    //!     onigiri::validator::is_negative_float(&test_float2),
    //!     true
    //! );
    //! ```
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

pub fn is_symbol(vc: &Vec<char>) -> bool {
    //! Validate `Vec<char>` whether it is symbol.
    //! ```
    //! let test_vc = vec!['+'];
    //! assert_eq!(
    //!     onigiri::validator::is_symbol(&test_vc),
    //!     true
    //! );
    //! let test_vc_2 = vec!['2'];
    //! assert_eq!(
    //!     onigiri::validator::is_symbol(&test_vc_2),
    //!     false
    //! );
    //! let test_vc_3 = vec!['+', '+'];
    //! assert_eq!(
    //!     onigiri::validator::is_symbol(&test_vc_3),
    //!     false
    //! );
    //! ```
    if vc.len() == 1_usize {
        match &vc[0] {
            '0' ... '9' => false,
            'a' ... 'z' => false,
            'A' ... 'Z' => false,
            _ => true
        }
    } else {
        false
    }
}
