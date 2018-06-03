//! Arbitrary expressions being used in Display.

#[macro_use]
extern crate display_attr;

#[derive(Debug, DisplayAttr)]
#[display(
    fmt = "{}",
    _0,
    /*
    "if _0 % 15 == 0 { \"fizzbuzz\".to_string() } \
     else if _0 % 3 == 0 { \"fizz\".to_string() } \
     else if _0 % 5 == 0 { \"buzz\".to_string() } \
     else { _0.to_string() }"
     */
)]
struct FizzBuzz(usize);

fn main() {
    for i in 0..100 {
        assert_eq!(FizzBuzz(i).to_string(), fizzbuzz(i));
    }
}

fn fizzbuzz(i: usize) -> String {
    if i % 15 == 0 {
        "fizzbuzz".to_string()
    } else if i % 3 == 0 {
        "fizz".to_string()
    } else if i % 5 == 0 {
        "buzz".to_string()
    } else {
        i.to_string()
    }
}
