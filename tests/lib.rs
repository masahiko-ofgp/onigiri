#[cfg(test)]
mod test {
    use onigiri::Onigiri;
    use onigiri::vvc::Vvc;
    use onigiri::utils::*;
    use onigiri::validator::*;

    #[test]
    fn test_create_onigiri() {
        let s = "I eat an onigiri.".to_string();

        let oni = Onigiri::new(&s);

        let vvc = oni.create_vvc(' ');
        let btmvc = oni.create_btmvc(' ');

        assert_eq!(
            vvc,
            Vvc {
                attr: vec![
                    vec!['I'],
                    vec!['e', 'a', 't'],
                    vec!['a', 'n'],
                    vec!['o', 'n', 'i', 'g', 'i', 'r', 'i', '.']
                ]
            }
            );

        assert_eq!(btmvc.attr.get(&0), Some(&vec!['I']));
        assert_eq!(btmvc.attr.get(&1), Some(&vec!['e', 'a', 't']));
        assert_eq!(btmvc.attr.get(&2), Some(&vec!['a', 'n']));
        assert_eq!(btmvc.attr.get(&3), Some(
                &vec!['o', 'n', 'i', 'g', 'i', 'r', 'i', '.']
                ));
    }

    #[test]
    fn test_search_all() {
        let a = "A A a B A".to_string();
        let oni = Onigiri::new(&a);
        let btmvc = oni.create_btmvc(' ');

        assert_eq!(
            btmvc.search_all("A"),
            Some(vec![0, 1, 4])
            );
    }

    #[test]
    fn test_each_tools() {
        let test_text = "-123 + 456".to_string();
        let oni = Onigiri::new(&test_text);
        let x = oni.create_vvc(' ');

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
        let test_text = "Rust rust RUST".to_string();
        let oni = Onigiri::new(&test_text);
        let vvc = oni.create_vvc(' ');

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
        let test_text = "(1 - 23)".to_string();
        let oni = Onigiri::new(&test_text);
        let x = oni.create_btmvc(' ');

        let first = x.attr.get(&0).unwrap();
        let l_paren = &first[0];

        assert_eq!(l_paren, &'(');
    }
}
