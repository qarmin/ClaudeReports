use syn::AttrStyle;
use syn::Attribute;
use syn::Meta;
use syn::Path;
use syn::token::Bracket;
use syn::token::Not;
use syn::token::Pound;

fn main() {
    let attribute = Attribute { pound_token: Pound::default(), style: AttrStyle::Inner(Not::default()), bracket_token: Bracket::default(), meta: Meta::from(Path { leading_colon: None, segments: <syn::punctuated::Punctuated<syn::PathSegment, syn::token::PathSep> as ::core::default::Default>::default() }) };
    let _ = attribute.parse_args::<syn::token::As>();
}
