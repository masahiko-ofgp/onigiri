//! onigiri::tools is the tool of handling `chars`.
//!
//! onigiri::validator is validate whether `Vec<char>` is valid.
//!
//! Some examples are as follows.
//!
//! 
//!     use onigiri::tools::{Vvc, cast};
//!     use onigiri::validator;
//!
//!     let test_text = "(13 + 2)".to_string();
//!
//!     let new_vvchar = Vvc::new(&test_text, ' ');
//!     
//!     assert_eq!(
//!         &new_vvchar.attr,
//!         &vec![vec!['(', '1','3'],vec!['+'],vec!['2', ')']]
//!     );
//!
//!     let thirteen = &new_vvchar.attr[0][1..].to_vec();
//!     assert_eq!(validator::is_positive_integer(&thirteen), true);
//! 
//!     let num = cast::<u8>(&thirteen);
//!     assert_eq!(&num, &Some(13_u8));
//!     assert_eq!(&num.unwrap() + 2, 15_u8);
//!

pub mod tools;
pub mod validator;
