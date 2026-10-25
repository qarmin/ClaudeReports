use regex_automata::dfa::dense::Builder;
use regex_automata::nfa::thompson::backtrack::Config;

fn main() {
    let config = Config::new();
    let usize_v = config.get_visited_capacity();
    let _ = <regex_automata::dfa::dense::DFA<Vec<u32>> as regex_automata::dfa::Automaton>::match_pattern(&Builder::new().build_many::<String>(&["".to_string(), "".to_string()]).unwrap(), regex_automata::nfa::thompson::Builder::new().add_empty().unwrap(), usize_v);
}
