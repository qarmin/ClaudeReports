use regex_automata::hybrid::dfa::DFA;
use regex_automata::hybrid::regex::Builder;

fn main() {
    let builder = Builder::default();
    let regex = builder.build_from_dfas(DFA::never_match().unwrap(), DFA::new("").unwrap());
    let _ = regex.pattern_len();
}
