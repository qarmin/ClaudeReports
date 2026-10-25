use regex_automata::util::primitives::StateID;

fn main() {
    let _ = <regex_automata::dfa::sparse::DFA<Vec<u8>> as regex_automata::dfa::Automaton>::next_state(&<regex_automata::dfa::sparse::DFA<Vec<u8>>>::new("").unwrap(), StateID::try_from(1u16).unwrap(), 0u8);
}
