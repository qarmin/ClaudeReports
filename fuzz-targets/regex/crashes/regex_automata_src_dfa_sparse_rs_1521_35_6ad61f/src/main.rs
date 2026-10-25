use regex_automata::util::primitives::StateID;

fn main() {
    let _ = <regex_automata::dfa::sparse::DFA<Vec<u8>> as regex_automata::dfa::Automaton>::match_pattern(&<regex_automata::dfa::sparse::DFA<Vec<u8>>>::new_many::<String>(&[include_str!("string_0.txt").to_string(), " ".to_string(), "\u{200b}\u{200d}".to_string()]).unwrap(), StateID::try_from(2049u32).unwrap(), 1usize);
}
