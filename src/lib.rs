/// onigiri::tools is the tool of handling `chars`.
///
/// ```
/// let test_text = "-123".to_string();
/// let new_vvchar = onigiri::tools::create_vvchar(&test_text);
/// assert_eq!(&new_vvchar, &vec![vec!['-', '1', '2', '3']]);
/// 
/// let num = onigiri::tools::Ni32::new(&new_vvchar[0]);
/// assert_eq!(&num.attr, &-123_i32);
///
/// let literal = onigiri::tools::Literal::new(&new_vvchar[0]);
/// assert_eq!(&literal.attr, &"-123".to_string());
///
/// let mut new_isize = onigiri::tools::Nisize::new(&new_vvchar[0]);
/// assert_eq!(&new_isize.attr, &-123_isize);
/// assert_eq!(&new_isize.attr + 23_isize, -100_isize);
/// ```
///

pub mod tools;
