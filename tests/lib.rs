extern crate onigiri;
use onigiri::{tools, validator};


#[test]
fn test_each_tools() {
    let test_text = "-123 + 456".to_string();
    let new_vvchar = tools::create_vvchar(&test_text);

    // "-123" is negative number.
    assert_eq!(validator::is_positive_number(&new_vvchar[0]), false);
    assert_eq!(validator::is_negative_number(&new_vvchar[0]), true);
    assert_eq!(validator::is_symbol(&new_vvchar[0]), false);

    // "+" is symbol.
    assert_eq!(validator::is_positive_number(&new_vvchar[1]), false);
    assert_eq!(validator::is_negative_number(&new_vvchar[1]), false);
    assert_eq!(validator::is_symbol(&new_vvchar[1]), true);

    // "456" is positive number.
    assert_eq!(validator::is_positive_number(&new_vvchar[2]), true);
    assert_eq!(validator::is_negative_number(&new_vvchar[2]), false);
    assert_eq!(validator::is_symbol(&new_vvchar[2]), false);

    // "-123" => -123_i32.
    let new_i32 = tools::Ni32::new(&new_vvchar[0]);
    assert_eq!(&new_i32.attr, &-123_i32);
    assert_eq!(&new_i32.attr + 123, 0_i32);
}

#[test]
fn test_create_new_vvc_and_test_iterate() {
    let test_text = "-123 + 456".to_string();
    let mut new_vvc = tools::Vvc::new(&test_text);
    assert_eq!(&new_vvc.next(), &Some("-123".to_string()));
    assert_eq!(&new_vvc.next(), &Some("+".to_string()));
    assert_eq!(&new_vvc.next(), &Some("456".to_string()));
    assert_eq!(&new_vvc.next(), &None);
    assert_eq!(&new_vvc.nth(1), &Some("+".to_string()));
    assert_eq!(&new_vvc.nth(3), &None);
}
