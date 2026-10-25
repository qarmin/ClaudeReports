use regex_automata::Input;
use regex_automata::hybrid::dfa::Builder;
use regex_automata::hybrid::dfa::Cache;
use regex_automata::meta::Regex;

fn main() {
    let regex = Regex::new_many::<String>(&[]).unwrap();
    let captures = regex.create_captures();
    let string = captures.interpolate_string("", "a");
    let builder = Builder::new();
    let d_f_a = builder.build(&string).unwrap();
    let mut cache = Cache::new(&Builder::new().build_many::<String>(&["".to_string()]).unwrap());
    let vec = vec![];
    let find_matches = regex.find_iter::<regex_automata::Input>(Input::new::<Vec<u8>>(&vec));
    let input = find_matches.input();
    let _ = d_f_a.start_state_reverse(&mut cache, input);
}
