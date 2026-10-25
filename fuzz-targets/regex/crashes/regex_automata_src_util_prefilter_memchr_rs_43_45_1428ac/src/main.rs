use regex_automata::MatchKind;
use regex_automata::Span;
use regex_automata::util::prefilter::Prefilter;

fn main() {
    let prefilter = Prefilter::from_hir_prefix(MatchKind::LeftmostFirst, &regex_syntax::parse("a").unwrap()).unwrap();
    let _ = prefilter.find(&vec![], Span::from(1usize..0usize));
}
