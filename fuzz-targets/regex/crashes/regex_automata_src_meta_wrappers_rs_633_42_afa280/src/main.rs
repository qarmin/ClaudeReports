use regex_automata::Input;
use regex_automata::meta::Cache;
use regex_automata::meta::Regex;

fn main() {
    let builder = Regex::builder();
    let regex = builder.build("q/+�\u{9f633} S\u{8753d}\u{69001}").unwrap();
    let mut cache = Cache::new(&builder.build("").unwrap());
    let vec = vec![0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8];
    let input = Input::new::<Vec<u8>>(&vec);
    let _ = regex.search_slots_with(&mut cache, &input, &mut vec![]);
}
