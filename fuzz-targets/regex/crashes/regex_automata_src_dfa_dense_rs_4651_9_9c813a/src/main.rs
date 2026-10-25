use regex_automata::util::primitives::StateID;

fn main() {
    let _ = <regex_automata::dfa::dense::DFA<Vec<u32>> as regex_automata::dfa::Automaton>::match_pattern(&<regex_automata::dfa::dense::DFA<Vec<u32>>>::new_many::<String>(&["H+".to_string(), "Zażółć gęślą j".to_string()]).unwrap(), StateID::new_unchecked(255usize), 1usize);
}
