//! Display being derived on struct types.

#[macro_use]
extern crate display_attr;

#[derive(Debug, DisplayAttr)]
#[display(fmt = "foo = {}, bar = {}", foo, bar)]
struct Baz {
    foo: usize,
    bar: bool,
}

#[derive(Debug, DisplayAttr)]
#[display(fmt = "[{}, {}]", _0, _1)]
struct Quux(usize, isize);

#[derive(Debug, DisplayAttr)]
#[display(fmt = "xyzzy")]
struct Xyzzy;

#[test]
fn main() {
    assert_eq!(
        Baz { foo: 42, bar: true }.to_string(),
        "foo = 42, bar = true"
    );
    assert_eq!(Quux(1, -1).to_string(), "[1, -1]");
    assert_eq!(Xyzzy.to_string(), "xyzzy");
}
