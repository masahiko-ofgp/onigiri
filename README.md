# onigiri

![onigiri](./imgs/onigiri2.png)

onigiri is the tool of handling `chars` in Rust.
The japanese name of rice ball is "Onigiri". 
In my image, a grain of rice is `char`. 
And what collected them is `chars`. 
And the seasoning of it is each structure of `onigiri::tools`.

Sep 3 2018, I add new struct Vvc.
However, I didn't changed the previous functions.
Although it may be a few, someone may already be using it.
I'm doing my best to develop that it can be used more easily and clearly.

## Usage

You add onigiri in Cargo.toml.

```
[dependencies]
onigiri = "0.1.5"
```
example is as follows.

```
extern crate onigiri;
use onigiri::tools;

fn main() {
    let test_text = "-123 456".to_string();
    let new_vvchar = tools::create_vvchar(&test_text);

    let new_ni32 = tools::Ni32::new(&new_vvchar[0]);
    let new_ni32_2 = tools::Ni32::new(&new_vvchar[1]);

    let addition = new_ni32.attr + new_ni32_2.attr;
    assert_eq!(addition, 333_i32);
}
```
