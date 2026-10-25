use regex_automata::util::primitives::StateID;

fn main() {
    let _ = <regex_automata::dfa::sparse::DFA<Vec<u8>> as regex_automata::dfa::Automaton>::match_pattern(&<regex_automata::dfa::sparse::DFA<Vec<u8>>>::new_many::<String>(&[]).unwrap(), StateID::new_unchecked(0usize), 0usize);
}
