//! A custom derive for implementing Display more easily.
//!
//! [![Build Status](https://travis-ci.org/remexre/display_attr.svg?branch=master)](https://travis-ci.org/remexre/display_attr)
//! [![Crates.io](https://img.shields.io/crates/v/display_attr.svg)](https://crates.io/crates/display_attr)
//! [![Documentation](https://docs.rs/display_attr/badge.svg)](https://docs.rs/display_attr/*/display_attr/)
//! ![License](https://img.shields.io/crates/l/display_attr.svg)

extern crate proc_macro;
extern crate proc_macro2;
#[macro_use]
extern crate quote;
extern crate syn;

mod attr;
mod codegen;
mod util;

use proc_macro::TokenStream;

#[proc_macro_derive(DisplayAttr, attributes(display))]
pub fn display_attr(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    codegen::gen_impl(ast).into()
}
