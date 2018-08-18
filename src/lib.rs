/// onigiri-tools is the tool of handling `chars`.
///
/// ```
/// let test_chars = vec!['-', '1', '2', '3'];
/// 
/// let num = onigiri::tools::Ni32::new(&test_chars);
/// assert_eq!(&num.attr, &-123_i32);
///
/// let literal = onigiri::tools::Literal::new(&test_chars);
/// assert_eq!(&literal.attr, &"-123".to_string());
/// ```
///

pub mod tools;
