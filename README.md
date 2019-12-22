# onigiri

[![Build Status](https://travis-ci.org/masahiko-ofgp/onigiri.svg?branch=master)](https://travis-ci.org/masahiko-ofgp/onigiri)

<img src="./imgs/onigiri2.png" width=60% alt="onigiri">

onigiri is the tool of handling `chars` in Rust.
The japanese name of rice ball is "Onigiri". 
In my image, a grain of rice is `char`. 
And what collected them is `chars`. 

## Dec 22, 2019

onigiri 0.2.0 released.

- Add struct `Onigiri`. This is Factory. It can select either `Vvc` or
 `Btmvc` and generate it.
- Remove `tools.rs`.
- Add `vvc.rs`, `btmvc.rs`, 'utils.rs'.

## Usage

You add onigiri in Cargo.toml.

```
[dependencies]
onigiri = "0.2.0"
```
example is as follows.

```
use onigiri::Onigiri;
use onigiri::vvc::Vvc;

fn main() {
    let s = "I eat an onigiri.".to_string();

    let oni = Onigiri::new(&s);

    // Create Vec<Vec<char>>
    let vvc = oni.create_vvc(' ');

    // Create BTreeMap<usize, Vec<char>>
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
```
