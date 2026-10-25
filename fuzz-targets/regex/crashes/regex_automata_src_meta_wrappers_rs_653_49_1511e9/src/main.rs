use regex_automata::Input;
use regex_automata::meta::Cache;
use regex_automata::meta::Regex;

fn main() {
    let regex = Regex::new("; hm4K]l&\u{3fec0}*f\u{934b4} ").unwrap();
    let mut cache = Cache::new(&Regex::builder().build("").unwrap());
    let vec = vec![108u8, 66u8, 0u8, 124u8, 0u8, 0u8, 0u8, 0u8, 0u8, 107u8, 0u8, 56u8, 47u8, 0u8, 0u8];
    let input = Input::new::<Vec<u8>>(&vec);
    let _ = regex.search_half_with(&mut cache, &input);
}
