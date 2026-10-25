use regex_automata::Anchored;
use regex_automata::dfa::dense::Builder;

fn main() {
    let _ = <regex_automata::dfa::dense::DFA<Vec<u32>> as regex_automata::dfa::Automaton>::next_state(&Builder::new().build_many::<String>(&[]).unwrap(), <regex_automata::dfa::dense::DFA<Vec<u32>> as regex_automata::dfa::Automaton>::universal_start_state(&Builder::new().build_many::<String>(&["".to_string()]).unwrap(), Anchored::No).unwrap(), 0u8);
}
