use toml_parser::Span;
use toml_parser::parser::Event;
use toml_parser::parser::EventKind;

fn main() {
    let event = Event::new_unchecked(EventKind::InlineTableOpen, None, Span::new_unchecked(1usize, 0usize));
    let span = event.span();
    let _ = span.len();
}
