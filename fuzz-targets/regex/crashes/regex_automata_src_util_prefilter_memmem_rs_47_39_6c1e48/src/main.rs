use regex_automata::MatchKind;
use regex_automata::Span;
use regex_automata::util::prefilter::Prefilter;
use regex_syntax::Parser;

fn main() {
    let prefilter = Prefilter::from_hir_prefix(MatchKind::All, &Parser::new().parse("sv").unwrap()).unwrap();
    let _ = prefilter.find(&vec![], Span { start: 0usize, end: 1usize });
}
