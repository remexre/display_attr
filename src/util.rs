use syn::{punctuated::Punctuated, Ident, Path, PathArguments, PathSegment};

pub fn path_from_ident(ident: Ident) -> Path {
    Path {
        leading_colon: None,
        segments: {
            let mut segments = Punctuated::new();
            segments.push(path_segment_from_ident(ident));
            segments
        },
    }
}

pub fn path_segment_from_ident(ident: Ident) -> PathSegment {
    PathSegment {
        ident,
        arguments: PathArguments::None,
    }
}
