use regex_automata::Anchored;
use regex_automata::dfa::dense::Builder;
use regex_automata::nfa::thompson::NFA;

fn main() {
    let _ = <regex_automata::dfa::sparse::DFA<Vec<u8>> as regex_automata::dfa::Automaton>::next_state(&<regex_automata::dfa::sparse::DFA<Vec<u8>>>::new("[[test]]\nname ").unwrap(), <regex_automata::dfa::dense::DFA<Vec<u32>> as regex_automata::dfa::Automaton>::universal_start_state(&Builder::default().build_from_nfa(&NFA::compiler().build("<svg xmtp:3.o182\"Bp-col\"/>iearG").unwrap()).unwrap(), Anchored::No).unwrap(), 0u8);
}
