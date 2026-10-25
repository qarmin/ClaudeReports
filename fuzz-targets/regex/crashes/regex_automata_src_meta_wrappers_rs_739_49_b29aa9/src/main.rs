use regex_automata::Input;
use regex_automata::PatternSet;
use regex_automata::meta::Builder;
use regex_automata::meta::Regex;
use regex_automata::util::captures::GroupInfo;

fn main() {
    let group_info = GroupInfo::default();
    let usize_v = group_info.explicit_slot_len();
    let regex = Regex::new_many::<String>(&["\u{ff0c6}\u{5c4ee}\u{200b}|R.".to_string()]).unwrap();
    let builder = Builder::new();
    let regex2 = builder.build_many::<String>(&[]).unwrap();
    let mut cache = regex2.create_cache();
    let mut pattern_set = PatternSet::new(usize_v);
    let vec = vec![73u8, 70u8];
    let input = Input::new::<Vec<u8>>(&vec);
    regex.which_overlapping_matches_with(&mut cache, &input, &mut pattern_set);
}
