//! Display being derived on enum types.

#[macro_use]
extern crate display_attr;

#[derive(Debug, DisplayAttr)]
#[allow(dead_code)]
enum Empty {}

#[derive(Debug, DisplayAttr)]
#[display(where_bounds = "T: ::std::fmt::Display")]
enum Just<T> {
    #[display(fmt = "{}", _0)]
    Value(T),
}

#[derive(Debug, DisplayAttr)]
enum Foo {
    #[display(fmt = "bar")]
    Bar,

    #[display(fmt = "baz[{} ~ {}]", _0, _1)]
    Baz(usize, usize),

    #[display(fmt = "{} -> {}", xyzzy, tl)]
    Quux { xyzzy: &'static str, tl: Box<Foo> },
}

fn main() {
    assert_eq!(Just::Value(0).to_string(), "0");
    assert_eq!(Foo::Bar.to_string(), "bar");
    assert_eq!(Foo::Baz(42, 137).to_string(), "baz[42 ~ 137]");
    assert_eq!(
        Foo::Quux {
            xyzzy: "first",
            tl: Box::new(Foo::Quux {
                xyzzy: "middle",
                tl: Box::new(Foo::Bar),
            }),
        }.to_string(),
        "first -> middle -> bar",
    );
}
