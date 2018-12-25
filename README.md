# onigiri

[![Build Status](https://travis-ci.org/masahiko-ofgp/onigiri.svg?branch=master)](https://travis-ci.org/masahiko-ofgp/onigiri)

<img src="./imgs/onigiri2.png" width=60% alt="onigiri">

onigiri is the tool of handling `chars` in Rust.
The japanese name of rice ball is "Onigiri". 
In my image, a grain of rice is `char`. 
And what collected them is `chars`. 

##### Dec 24 2018

I removed what seems useless from `tools.rs`.

Then I renamed `is_number`, `is_positive_number`, `is_negative_number` to `is_integer`, `is_positive_integer`, `is_negative_integer`.

Accordingly, I added `is_float`, `is_positive_float` and `is_negative_float` newly.

And I renamed `create_btm` to `to_btm`.

##### Dec 9 2018

Vvc make it possible to specify a separator.

##### Nov 5 2018

I add new function `search_all`.


## Usage

You add onigiri in Cargo.toml.

```
[dependencies]
onigiri = "0.1.11"
```
example is as follows.

```
extern crate onigiri;
use onigiri::tools;

fn main() {
    let test_text = "(13 + 2)";
    
    let new_vvchar = onigiri::tools::create_vvchar(&test_text, ' ');
    assert_eq!(&new_vvchar, &vec![vec!['(', '1','3'],vec!['+'],vec!['2', ')']]);
    
    let thirteen = &new_vvchar[0][1..].to_vec();
    assert_eq!(onigiri::validator::is_positive_integer(&thirteen), true);
   
    let num = onigiri::tools::cast::<u8>(&thirteen);
    assert_eq!(&num, &Some(13_u8));
    assert_eq!(&num.unwrap() + 2, 15_u8);
}
```
