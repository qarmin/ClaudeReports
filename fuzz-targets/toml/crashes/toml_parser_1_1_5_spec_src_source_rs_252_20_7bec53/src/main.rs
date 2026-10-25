use toml_parser::Span;

fn main() {
    let _ = <toml_parser::Span as ::core::ops::Add<usize>>::add(Span::new_unchecked(10000000000000000000usize, 4294967295usize), 18446744073709551614usize);
}
