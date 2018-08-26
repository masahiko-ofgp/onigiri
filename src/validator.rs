//! onigiri::validator contains 3 functions for validating `Vec<char>`.


pub fn is_positive_number(vc: &Vec<char>) -> bool {
    //! Validate `Vec<char>` whether it is positive number.
    //! ```
    //! let test_chars_1 = vec!['1', '2', '3'];
    //! let test_chars_2 = vec!['2', '+', '1'];
    //! let test_chars_3 = vec!['-', '1', '2'];
    //!
    //! assert_eq!(
    //!     onigiri::validator::is_positive_number(&test_chars_1), true);
    //! assert_eq!(
    //!     onigiri::validator::is_positive_number(&test_chars_2), false);
    //! assert_eq!(
    //!     onigiri::validator::is_positive_number(&test_chars_3), false);
    //! ```
    let mut stack: Vec<bool> = vec![];
    for v in vc {
        match &v {
            '0' ... '9' => stack.push(true),
            _ => stack.push(false)
        }
    }
    if stack.iter().all(|&x| x == true) { true }
    else { false }
}

pub fn is_negative_number(vc: &Vec<char>) -> bool {
    //! Validate `Vec<char>` whether it is negative number.
    //! ```
    //! let test_chars_1 = vec!['1', '2', '3'];
    //! let test_chars_2 = vec!['-', '2', '1'];
    //! let test_chars_3 = vec!['2', '-', '1'];
    //! assert_eq!(
    //!     onigiri::validator::is_negative_number(&test_chars_1), false);
    //! assert_eq!(
    //!     onigiri::validator::is_negative_number(&test_chars_2), true);
    //! assert_eq!(
    //!     onigiri::validator::is_negative_number(&test_chars_3), false);
    //! ```
    let head = &vc[0];
    let tail = &vc[1..];
    let mut stack: Vec<bool> = vec![];
    
    if head == &'-' {
        stack.push(true)
    } else {
        stack.push(false)
    }

    let result_tail = is_positive_number(&tail.to_vec());
    stack.push(result_tail);

    if stack.iter().all(|&x| x == true) { true }
    else { false }
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
