use regex_automata::Input;

fn main() {
    let vec = vec![];
    let input = Input::new::<Vec<u8>>(&vec);
    let _ = <regex_automata::dfa::sparse::DFA<Vec<u8>> as regex_automata::dfa::Automaton>::match_len(&<regex_automata::dfa::sparse::DFA<Vec<u8>>>::always_match().unwrap(), <regex_automata::dfa::dense::DFA<Vec<u32>> as regex_automata::dfa::Automaton>::start_state_forward(&<regex_automata::dfa::dense::DFA<Vec<u32>>>::new("مر").unwrap(), &input).unwrap());
}
