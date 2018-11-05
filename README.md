# onigiri

[![Build Status](https://travis-ci.org/masahiko-ofgp/onigiri.svg?branch=master)](https://travis-ci.org/masahiko-ofgp/onigiri)

<img src="./imgs/onigiri2.png" width=60% alt="onigiri">

onigiri is the tool of handling `chars` in Rust.
The japanese name of rice ball is "Onigiri". 
In my image, a grain of rice is `char`. 
And what collected them is `chars`. 

##### Nov 5 2018

I add new function `search_all`.

##### Oct 20 2018

I add new function `create_btm`.

##### Sep 28 2018

I add new validator `is_calc_operator` and add derive `PartialEq` to 
struct `Vvc`

##### Sep 13 2018

I remove some structs.
Instead of them, I added new function `cast`.

##### Sep 3 2018

I add new struct Vvc.
However, I didn't changed the previous functions.
Although it may be a few, someone may already be using it.
I'm doing my best to develop that it can be used more easily and clearly.

## Usage

You add onigiri in Cargo.toml.

```
[dependencies]
onigiri = "0.1.9"
```
example is as follows.

```
extern crate onigiri;
use onigiri::tools;

fn main() {
    let test_text = "(13 + 2)".to_string();
    
    let new_vvchar = onigiri::tools::create_vvchar(&test_text);
    assert_eq!(&new_vvchar, &vec![vec!['(', '1','3'],vec!['+'],vec!['2', ')']]);
    
    let thirteen = &new_vvchar[0][1..].to_vec();
    assert_eq!(onigiri::validator::is_positive_number(&thirteen), true);
   
    let num = onigiri::tools::cast::<u8>(&thirteen);
    assert_eq!(&num, &Some(13_u8));
    assert_eq!(&num.unwrap() + 2, 15_u8);
}
```
