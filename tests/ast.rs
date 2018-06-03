//! An AST type that we want to define Display on.
//!
//! Note that precedence isn't implemented.

#[macro_use]
extern crate display_attr;

#[derive(Debug, DisplayAttr)]
enum Ast {
    #[display(fmt = "{} + {}", _0, _1)]
    Add(Box<Ast>, Box<Ast>),

    #[display(fmt = "{}", _0)]
    Const(isize),

    #[display(fmt = "-{}", _0)]
    Neg(Box<Ast>),
}

fn main() {
    let ast = Ast::Add(
        Box::new(Ast::Const(4)),
        Box::new(Ast::Neg(Box::new(Ast::Const(2)))),
    );
    assert_eq!(ast.to_string(), "4 + -2");
}
