use toml_parser::Span;

fn main() {
    let _ = <toml_parser::Span as ::core::ops::Add<usize>>::add(Span::new_unchecked(0usize, usize::MAX), 1usize);
}
