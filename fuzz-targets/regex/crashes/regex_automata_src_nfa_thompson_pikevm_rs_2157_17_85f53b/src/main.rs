use regex_automata::Input;
use regex_automata::meta::Builder;
use regex_automata::nfa::thompson::pikevm::PikeVM;
use regex_syntax::hir::Dot;
use regex_syntax::hir::Hir;

fn main() {
    let pike_v_m = PikeVM::never_match().unwrap();
    let mut cache = pike_v_m.create_cache();
    let builder = Builder::new();
    let regex = builder.build_from_hir(&Hir::dot(Dot::AnyCharExcept('a'))).unwrap();
    let vec = vec![];
    let captures_matches = regex.captures_iter::<regex_automata::Input>(Input::new::<Vec<u8>>(&vec));
    let input = captures_matches.input();
    let builder2 = regex_automata::dfa::onepass::Builder::new();
    let d_f_a = builder2.build("").unwrap();
    let mut captures = d_f_a.create_captures();
    pike_v_m.search(&mut cache, input, &mut captures);
}
