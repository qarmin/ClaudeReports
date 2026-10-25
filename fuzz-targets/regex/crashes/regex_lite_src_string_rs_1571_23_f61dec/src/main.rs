use regex_lite::Regex;

fn main() {
    let regex = Regex::try_from("".to_string()).unwrap();
    let string = "\u{c0aa7}".to_string();
    let match_v = regex.find_at(&string, 1usize).unwrap();
    let _ = match_v.as_str();
}
