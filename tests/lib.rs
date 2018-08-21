extern crate onigiri;
use onigiri::tools;


#[test]
fn test_a_series_of_usage() {
    let test_text = "-123 456".to_string();
    let new_vvchar = tools::create_vvchar(&test_text);

    let new_ni32 = tools::Ni32::new(&new_vvchar[0]);
    let new_ni32_2 = tools::Ni32::new(&new_vvchar[1]);

    let addition = new_ni32.attr + new_ni32_2.attr;
    assert_eq!(addition, 333_i32);
}
