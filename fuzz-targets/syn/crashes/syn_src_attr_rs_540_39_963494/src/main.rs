use syn::Meta;
use syn::Path;

fn main() {
    let meta = Meta::Path(Path { leading_colon: None, segments: <syn::punctuated::Punctuated<syn::PathSegment, syn::token::PathSep>>::new() });
    let _ = meta.require_list();
}
