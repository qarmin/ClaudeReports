use regex_automata::util::start::Config;

fn main() {
    let _ = <regex_automata::dfa::dense::DFA<Vec<u32>> as regex_automata::dfa::Automaton>::next_eoi_state(&<regex_automata::dfa::dense::DFA<Vec<u32>>>::never_match().unwrap(), <regex_automata::dfa::sparse::DFA<Vec<u8>> as regex_automata::dfa::Automaton>::start_state(&<regex_automata::dfa::sparse::DFA<Vec<u8>>>::new_many::<String>(&[]).unwrap(), &Config::new()).unwrap());
}
