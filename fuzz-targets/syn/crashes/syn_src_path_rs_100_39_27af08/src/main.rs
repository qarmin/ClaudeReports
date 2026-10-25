use syn::Path;

fn main() {
    let path = Path { leading_colon: None, segments: <syn::punctuated::Punctuated<syn::PathSegment, syn::token::PathSep>>::new() };
    let _ = path.require_ident();
}
