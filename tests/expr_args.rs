//! Arbitrary expressions being used in Display.

#[macro_use]
extern crate display_attr;

#[derive(Debug, DisplayAttr)]
#[display(fmt = "{} +/- {} = ({}, {})", _0, _1, arg = "_0 - _1", arg = "_0 + _1")]
struct PlusMinus(f32, f32);

#[test]
fn main() {
    assert_eq!(PlusMinus(5.0, 1.5).to_string(), "5 +/- 1.5 = (3.5, 6.5)");
}
