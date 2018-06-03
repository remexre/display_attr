//! Handling the `display` attribute.

use proc_macro2::{Span, TokenStream};
use quote::ToTokens;
use syn::{parse_str, Attribute, Expr, ExprPath, Ident, Lit, LitStr, Meta, NestedMeta, Path,
          WhereClause};

use util::path_from_ident;

/// The parsed form of the `display` attribute.
#[derive(Clone, Debug)]
pub struct DisplayAttribute {
    pub fmt: Option<(LitStr, Vec<Expr>)>,
    pub where_bounds: Option<LitStr>,
    pub path: Path,
}

impl DisplayAttribute {
    /// Returns tokens corresponding to the appropriate formatting call.
    /// Panics if no formatting attribute is present.
    pub fn fmt_call(&self) -> TokenStream {
        let &(ref fmt, ref args) = self.fmt.as_ref().unwrap_or_else(|| {
            panic!(
                "Invalid #[display(...)] attribute on {}: No formatting specification found",
                self.path.clone().into_token_stream()
            )
        });
        quote! {
            write!(fmt, #fmt, #(#args),*)
        }
    }

    /// Tries to convert a series of attributes to a DisplayAttribute,
    /// panicking on error.
    pub fn from_attrs(path: Path, attrs: Vec<Attribute>) -> DisplayAttribute {
        let display_path = path_from_ident(Ident::new("display", Span::call_site()));

        let mut display_attr = DisplayAttribute {
            fmt: None,
            path: path.clone(),
            where_bounds: None,
        };
        for attr in attrs {
            if let Some(meta) = attr.interpret_meta() {
                if let Some(attr) = DisplayAttribute::from_meta(path.clone(), meta) {
                    display_attr.merge(attr);
                }
            } else if attr.path == display_path {
                panic!(
                    "Invalid #[display(...)] attribute on {}: Not a valid meta",
                    path.into_token_stream()
                )
            }
        }
        display_attr
    }

    /// Tries to convert a Meta to a DisplayAttribute, returning None if the
    /// meta is not a `display` attribute and panicking on an invalid `display`
    /// attribute.
    pub fn from_meta(path: Path, meta: Meta) -> Option<DisplayAttribute> {
        match meta {
            Meta::List(meta) => {
                if meta.ident == "display" {
                    Some(DisplayAttribute::from_nested_metas(
                        path,
                        meta.nested.into_iter(),
                    ))
                } else {
                    None
                }
            }
            Meta::NameValue(ref meta) if meta.ident == "display" => panic!(
                "Invalid #[display = ...] attribute on {}: Not a meta list",
                path.into_token_stream()
            ),
            Meta::Word(ref meta) if meta == "display" => panic!(
                "Invalid #[display] attribute on {}: Not a meta list",
                path.into_token_stream()
            ),
            _ => None,
        }
    }

    fn from_nested_metas<I>(path: Path, metas: I) -> DisplayAttribute
    where
        I: Iterator<Item = NestedMeta>,
    {
        let mut fmt = None;
        let mut args = Vec::new();
        let mut where_bounds = None;
        for meta in metas {
            match meta {
                NestedMeta::Meta(Meta::NameValue(meta)) => if meta.ident == "fmt" {
                    if let Lit::Str(lit) = meta.lit {
                        if fmt.is_some() {
                            panic!(
                                "Invalid #[display(...)] attribute on {}: Duplicate fmt",
                                path.into_token_stream()
                            )
                        }
                        fmt = Some(lit);
                    } else {
                        panic!(
                            "Invalid #[display(...)] attribute on {}: Format string not a string",
                            path.into_token_stream()
                        )
                    }
                } else if meta.ident == "where_bounds" {
                    if let Lit::Str(lit) = meta.lit {
                        if where_bounds.is_some() {
                            panic!(
                                "Invalid #[display(...)] attribute on {}: Duplicate where_bounds",
                                path.into_token_stream()
                            )
                        }
                        where_bounds = Some(lit);
                    } else {
                        panic!(
                            "Invalid #[display(...)] attribute on {}: Where bounds not a string",
                            path.into_token_stream()
                        )
                    }
                } else if meta.ident == "arg" {
                    if let Lit::Str(lit) = meta.lit {
                        match lit.parse() {
                            Ok(expr) => args.push(expr),
                            Err(err) => panic!(
                                "Invalid #[display(...)] attribute on {}: When parsing argument \
                                 string: {}",
                                path.into_token_stream(),
                                err,
                            ),
                        }
                    } else {
                        panic!(
                            "Invalid #[display(...)] attribute on {}: Argument is not a format \
                             string, where bounds declaration, field name, or expression",
                            path.into_token_stream(),
                        )
                    }
                } else {
                    panic!(
                        "Invalid #[display(...)] attribute on {}: Invalid named meta item",
                        path.into_token_stream()
                    )
                },
                NestedMeta::Meta(Meta::Word(meta)) => args.push(Expr::Path(ExprPath {
                    attrs: Vec::new(),
                    qself: None,
                    path: path_from_ident(meta),
                })),
                NestedMeta::Literal(Lit::Str(lit)) => match lit.parse() {
                    Ok(expr) => args.push(expr),
                    Err(err) => panic!(
                        "Invalid #[display(...)] attribute on {}: When parsing argument string: {}",
                        path.into_token_stream(),
                        err,
                    ),
                },
                _ => panic!(
                    "Invalid #[display(...)] attribute on {}: Argument is not a format string, \
                     where bounds declaration, field name, or expression",
                    path.into_token_stream(),
                ),
            }
        }

        DisplayAttribute {
            fmt: fmt.map(|fmt| (fmt, args)),
            path,
            where_bounds,
        }
    }

    fn merge(&mut self, other: DisplayAttribute) {
        assert_eq!(self.path, other.path);
        match (&mut self.fmt, other.fmt) {
            (_, None) => {}
            (&mut None, Some(fmt)) => self.fmt = Some(fmt),
            _ => panic!(
                "Invalid #[display(...)] attribute on {}: Duplicate formatting attribute",
                self.path.clone().into_token_stream()
            ),
        }
        match (&mut self.where_bounds, other.where_bounds) {
            (_, None) => {}
            (&mut None, Some(where_bounds)) => self.where_bounds = Some(where_bounds),
            _ => panic!(
                "Invalid #[display(...)] attribute on {}: Duplicate where bounds attribute",
                self.path.clone().into_token_stream()
            ),
        }
    }

    /// Returns tokens corresponding to the appropriate where bounds, returning an empty
    /// TokenStream if no bounds exist.
    pub fn where_bounds(&self) -> TokenStream {
        if let Some(where_bounds) = self.where_bounds.as_ref() {
            let where_bounds: WhereClause = parse_str(&format!("where {}", where_bounds.value()))
                .unwrap_or_else(|err| {
                    panic!(
                        "Invalid #[display(...)] attribute on {}: When parsing where bounds: {}",
                        self.path.clone().into_token_stream(),
                        err,
                    )
                });
            quote! { #where_bounds }
        } else {
            quote!{}
        }
    }
}
