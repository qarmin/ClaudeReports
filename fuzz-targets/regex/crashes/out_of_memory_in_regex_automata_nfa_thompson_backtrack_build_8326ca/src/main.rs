use regex_automata::nfa::thompson::Compiler;
use regex_automata::nfa::thompson::backtrack::Builder;
use regex_automata::util::syntax::Config;
use regex_syntax::hir::Hir;
use regex_syntax::hir::Repetition;

fn main() {
    let hir = Hir::repetition(Repetition { min: 0u32, max: Some(1000000000u32), greedy: false, sub: Box::new(regex_automata::util::syntax::parse_with("a", &Config::new()).unwrap()) });
    let builder = Builder::new();
    let _ = builder.build_from_nfa(Compiler::new().build_from_hir(&hir).unwrap());
}
