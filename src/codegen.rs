use proc_macro2::{Span, TokenStream};
use syn::{Data, DataEnum, DataStruct, DeriveInput, Fields, Ident, Path};

use attr::DisplayAttribute;
use util::{path_from_ident, path_segment_from_ident};

pub fn gen_impl(ast: DeriveInput) -> TokenStream {
    let DeriveInput {
        attrs,
        ident,
        generics,
        data,
        ..
    } = ast;
    let path = path_from_ident(ident.clone());
    let attr = DisplayAttribute::from_attrs(path.clone(), attrs);
    let body = match data {
        Data::Struct(data) => gen_struct(&path, &attr, data),
        Data::Enum(data) => gen_enum(&path, data),
        Data::Union(_) => panic!("Can't derive DisplayAttr on a union"),
    };
    let where_bounds = attr.where_bounds();
    quote! {
        #[allow(unused_variables)]
        impl #generics ::std::fmt::Display for #ident #generics
            #where_bounds
        {
            fn fmt(&self, fmt: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                #body
            }
        }
    }
}

fn gen_struct(path: &Path, attr: &DisplayAttribute, data: DataStruct) -> TokenStream {
    let branch = gen_fields(path, data.fields, &attr);
    quote! { match *self { #branch } }
}

fn gen_enum(path: &Path, data: DataEnum) -> TokenStream {
    let branches = data.variants.into_iter().map(|var| {
        let mut path = path.clone();
        path.segments.push(path_segment_from_ident(var.ident));
        let attr = DisplayAttribute::from_attrs(path.clone(), var.attrs);
        gen_fields(&path, var.fields, &attr)
    });
    quote! { match *self { #(#branches),* } }
}

fn gen_fields(path: &Path, fields: Fields, attr: &DisplayAttribute) -> TokenStream {
    let pat = match fields {
        Fields::Named(fields) => {
            let iter = fields.named.into_iter().map(|f| f.ident.unwrap());
            quote! { #path { #(ref #iter),* } }
        }
        Fields::Unnamed(fields) => {
            let iter = fields
                .unnamed
                .into_iter()
                .enumerate()
                .map(|(i, _)| Ident::new(&format!("_{}", i), Span::call_site()));
            quote! { #path(#(ref #iter),*) }
        }
        Fields::Unit => quote! { #path },
    };
    let fmt_call = attr.fmt_call();
    quote! { #pat => #fmt_call }
}
