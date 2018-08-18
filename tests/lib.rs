extern crate onigiri;
use onigiri::tools::{Ni32, Literal};

#[test]
fn test_create_new_number_struct() {
    let test_vec = vec!['-', '2', '1'];

    let new_number = Ni32::new(&test_vec);
    assert_eq!(&new_number.attr, &-21_i32);

    let new_literal = Literal::new(&test_vec);
    assert_eq!(&new_literal.attr, &"-21".to_string());
}
