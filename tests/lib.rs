use onigiri::tools::*;
use onigiri::validator::*;

#[test]
fn test_each_tools() {
    let test_text = "-123 + 456";
    let x = Vvc::new(&test_text, ' ');

    // "-123" is negative integer.
    assert_eq!(is_positive_integer(&x.attr[0]), false);
    assert_eq!(is_negative_integer(&x.attr[0]), true);
    assert_eq!(is_punctuation(&x.attr[0]), false);

    // "+" is punctuation.
    assert_eq!(is_positive_integer(&x.attr[1]), false);
    assert_eq!(is_negative_integer(&x.attr[1]), false);
    assert_eq!(is_punctuation(&x.attr[1]), true);

    // "456" is positive integer.
    assert_eq!(is_positive_integer(&x.attr[2]), true);
    assert_eq!(is_negative_integer(&x.attr[2]), false);
    assert_eq!(is_punctuation(&x.attr[2]), false);

    // "-123" => -123_i32.
    let value_i32 = cast::<i32>(&x.attr[0]);
    assert_eq!(&value_i32, &Some(-123_i32));
    assert_eq!(&value_i32.unwrap() + 123, 0_i32);
}

#[test]
fn test_some_kind_string() {
    let test_text = "Rust rust RUST";
    let vvc = Vvc::new(&test_text, ' ');

    assert_eq!(
        is_title(&vvc.attr[0]),
        true
        );

    assert_eq!(
        is_lower_ascii(&vvc.attr[1]),
        true
        );

    assert_eq!(
        is_upper_ascii(&vvc.attr[2]),
        true
        );
}

#[test]
fn test_handle_btreemap() {
    let test_text = "(1 - 23)";
    let x = Vvc::new(&test_text, ' ');
    let x_btm = &x.to_btm().unwrap();
    let first = x_btm.get(&0).unwrap();
    let l_paren = &first[0];
    assert_eq!(l_paren, &'(');
}

#[test]
fn test_search_word_all() {
    let test_text = "Hello Hallo Hollo Hello Hello";
    let x = Vvc::new(&test_text, ' ');
    let search_result = search_all(
        &x.to_btm().unwrap(), 
        "Hello".to_string());
    let count = search_result.unwrap().len();
    assert_eq!(3_usize, count);
}
