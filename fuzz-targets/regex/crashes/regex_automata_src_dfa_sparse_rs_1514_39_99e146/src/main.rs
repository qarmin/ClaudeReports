use regex_automata::util::primitives::StateID;

fn main() {
    let _ = <regex_automata::dfa::sparse::DFA<Vec<u8>> as regex_automata::dfa::Automaton>::next_eoi_state(&<regex_automata::dfa::sparse::DFA<Vec<u8>>>::new("").unwrap(), StateID::from_ne_bytes([0u8, 1u8, 0u8, 0u8]).unwrap());
}
