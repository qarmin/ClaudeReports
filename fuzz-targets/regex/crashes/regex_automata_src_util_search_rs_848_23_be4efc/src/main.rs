use regex_automata::Span;

fn main() {
    let span = Span { start: 0usize, end: 0usize };
    let span2 = span.offset(1usize);
    let _ = span2.offset(usize::MAX);
}
