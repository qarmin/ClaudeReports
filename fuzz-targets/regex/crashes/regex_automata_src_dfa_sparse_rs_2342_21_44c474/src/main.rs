use regex_automata::util::primitives::StateID;

fn main() {
    let _ = <regex_automata::dfa::sparse::DFA<Vec<u8>> as regex_automata::dfa::Automaton>::match_pattern(&<regex_automata::dfa::sparse::DFA<Vec<u8>>>::never_match().unwrap(), StateID::from(0u8), 10000000000000000000usize);
}
