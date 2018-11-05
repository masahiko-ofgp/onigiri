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
    let new_i32 = tools::cast::<i32>(&new_vvchar[0]);
    assert_eq!(&new_i32, &Some(-123_i32));
    assert_eq!(&new_i32.unwrap() + 123, 0_i32);
}

#[test]
fn test_create_new_vvc_and_test_iterate() {
    let test_text = "-123 + 456".to_string();
    let mut new_vvc = tools::Vvc::new(&test_text);
    assert_eq!(new_vvc.next(), Some("-123".to_string()));
    assert_eq!(new_vvc.next(), Some("+".to_string()));
    assert_eq!(new_vvc.next(), Some("456".to_string()));
    assert_eq!(new_vvc.next(), None);
    assert_eq!(new_vvc.nth(1), Some("+".to_string()));
    assert_eq!(new_vvc.nth(3), None);
}

#[test]
fn test_create_and_handle_btreemap() {
    let test_text = "(1 - 23)".to_string();
    let new_vvc = tools::Vvc::new(&test_text);
    let new_btm = &new_vvc.create_btm().unwrap();
    let first = new_btm.get(&0).unwrap();
    let l_paren = &first[0];
    assert_eq!(l_paren, &'(');
}

#[test]
fn test_search_word_all() {
    let test_text = "Hello Hallo Hollo Hello".to_string();
    let new_vvc = tools::Vvc::new(&test_text);
    let search_result = new_vvc.clone().search_all("Hello".to_string());
    assert_eq!(search_result, Some(vec![0, 3]));
    assert_eq!(&new_vvc.attr[0], &vec!['H','e','l','l','o']);
}