use regex_automata::util::primitives::StateID;

fn main() {
    let _ = <regex_automata::dfa::dense::DFA<Vec<u32>> as regex_automata::dfa::Automaton>::match_len(&<regex_automata::dfa::dense::DFA<Vec<u32>>>::new_many::<String>(&[]).unwrap(), StateID::from(0u8));
}
