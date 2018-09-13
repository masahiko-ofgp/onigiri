//! onigiri::tools is the tool of handling `chars`.
//!
//! onigiri::validator is validate whether `Vec<char>` is valid.
//!
//! Some examples are as follows.
//!
//! ```
//! let test_text = "(13 + 2)".to_string();
//!
//! let new_vvchar = onigiri::tools::create_vvchar(&test_text);
//! assert_eq!(&new_vvchar, &vec![vec!['(', '1','3'],vec!['+'],vec!['2', ')']]);
//! let thirteen = &new_vvchar[0][1..].to_vec();
//! assert_eq!(onigiri::validator::is_positive_number(&thirteen), true);
//!
//! let num = onigiri::tools::cast::<u8>(&thirteen);
//! assert_eq!(&num, &Some(13_u8));
//! assert_eq!(&num.unwrap() + 2, 15_u8);
//! ```
//!

pub mod tools;
pub mod validator;
