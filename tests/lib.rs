extern crate onigiri;
use onigiri::{tools, validator};


#[test]
fn test_a_series_of_usage() {
    let test_text = "-123 456".to_string();
    let new_vvchar = tools::create_vvchar(&test_text);

    if validator::is_negative_number(&new_vvchar[0]) {
        let new_ni32 = tools::Ni32::new(&new_vvchar[0]);
        assert_eq!(new_ni32.attr + 123_i32, 0_i32);
    }
    
    if validator::is_positive_number(&new_vvchar[1]) {
        let new_ni32_2 = tools::Ni32::new(&new_vvchar[1]);
        assert_eq!(new_ni32_2.attr - 123_i32, 333_i32);
    }
}
