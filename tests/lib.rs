extern crate onigiri;
use onigiri::{tools, validator};

#[test]
fn test_each_tools() {
    let test_text = "-123 + 456";
    let new_vvchar = tools::create_vvchar(&test_text, ' ');

    // "-123" is negative integer.
    assert_eq!(validator::is_positive_integer(&new_vvchar[0]), false);
    assert_eq!(validator::is_negative_integer(&new_vvchar[0]), true);
    assert_eq!(validator::is_symbol(&new_vvchar[0]), false);

    // "+" is symbol.
    assert_eq!(validator::is_positive_integer(&new_vvchar[1]), false);
    assert_eq!(validator::is_negative_integer(&new_vvchar[1]), false);
    assert_eq!(validator::is_symbol(&new_vvchar[1]), true);

    // "456" is positive integer.
    assert_eq!(validator::is_positive_integer(&new_vvchar[2]), true);
    assert_eq!(validator::is_negative_integer(&new_vvchar[2]), false);
    assert_eq!(validator::is_symbol(&new_vvchar[2]), false);

    // "-123" => -123_i32.
    let new_i32 = tools::cast::<i32>(&new_vvchar[0]);
    assert_eq!(&new_i32, &Some(-123_i32));
    assert_eq!(&new_i32.unwrap() + 123, 0_i32);
}

#[test]
fn test_handle_btreemap() {
    let test_text = "(1 - 23)";
    let new_vvc = tools::Vvc::new(&test_text, ' ');
    let new_btm = &new_vvc.to_btm().unwrap();
    let first = new_btm.get(&0).unwrap();
    let l_paren = &first[0];
    assert_eq!(l_paren, &'(');
}

#[test]
fn test_search_word_all() {
    let test_text = "Hello Hallo Hollo Hello";
    let new_vvc = tools::Vvc::new(&test_text, ' ');
    let search_result = tools::search_all(
        &new_vvc.to_btm().unwrap(), 
        "Hello".to_string());
    assert_eq!(search_result, Some(vec![0, 3]));
    assert_eq!(tools::chars_to_string(&new_vvc.attr[0]), "Hello".to_string());
    assert_eq!(tools::chars_to_string(&new_vvc.attr[3]), "Hello".to_string());
}
