extern crate onigiri;
use onigiri::tools;
use onigiri::tools::{Nu8, Ni32, Ni64, Literal};

#[test]
fn test_chars_to_string() {
    let test_chars = vec!['*', 'h', 'e', '1', '1', '0', '*'];
    assert_eq!(tools::chars_to_string(&test_chars), "*he110*".to_string());
}

#[test]
fn test_create_new_number_struct() {
    let test_pos_vec = vec!['2', '5', '5'];
    let test_neg_vec = vec!['-', '2', '1'];

    let new_u8 = Nu8::new(&test_pos_vec);
    // If 256, error! u8 max_value is 255.
    assert_eq!(&new_u8.attr, &255_u8);
    
    let new_i32 = Ni32::new(&test_neg_vec);
    assert_eq!(&new_i32.attr, &-21_i32);
    assert_eq!(&new_i32.attr + 21_i32, 0);

    let new_i64 = Ni64::new(&test_neg_vec);
    assert_eq!(&new_i64.attr, &-21_i64);
    assert_eq!(!&new_i64.attr, 20);

    let new_literal = Literal::new(&test_neg_vec);
    assert_eq!(&new_literal.attr, &"-21".to_string());
    assert_eq!(&new_literal.attr.replace("-", "+"), &"+21".to_string());
}
