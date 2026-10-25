use regex_automata::nfa::thompson::NFA;
use regex_syntax::hir::Hir;
use regex_syntax::hir::Repetition;

fn main() {
    let compiler = NFA::compiler();
    let _ = compiler.build_from_hir(&Hir::repetition(Repetition { min: 0u32, max: Some(100000000u32), greedy: false, sub: Box::new(regex_automata::util::syntax::parse("a").unwrap()) }));
}
