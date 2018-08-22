//! onigiri::tools is the tool of handling `chars`.
//!
//! onigiri::validator is validate whether `Vec<char>` is valid.
//!
//! Some examples are as follows.
//!
//! ```
//! let test_text = "-123".to_string();
//!
//! let new_vvchar = onigiri::tools::create_vvchar(&test_text);
//! assert_eq!(&new_vvchar, &vec![vec!['-', '1', '2', '3']]);
//! assert_eq!(onigiri::validator::is_negative_number(&new_vvchar[0]), true);
//!
//! let num = onigiri::tools::Ni32::new(&new_vvchar[0]);
//! assert_eq!(&num.attr, &-123_i32);
//!
//! let literal = onigiri::tools::Literal::new(&new_vvchar[0]);
//! assert_eq!(&literal.attr, &"-123".to_string());
//!
//! let mut new_isize = onigiri::tools::Nisize::new(&new_vvchar[0]);
//! assert_eq!(&new_isize.attr, &-123_isize);
//! assert_eq!(&new_isize.attr + 23_isize, -100_isize);
//!
//! let new_i128 = onigiri::tools::Ni128::new(&new_vvchar[0]);
//! assert_eq!(&new_i128.attr, &-123_i128);
//! ```
//!

pub mod tools;
pub mod validator;
