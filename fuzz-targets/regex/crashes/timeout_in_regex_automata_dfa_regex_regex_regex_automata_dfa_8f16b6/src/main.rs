use regex_automata::dfa::regex::Builder;

fn main() {
    let regex = Builder::new().build_many_sparse::<String>(&[include_str!("string_0.txt").to_string(), "\u{200b}\u{200d}".to_string(), include_str!("string_1.txt").to_string(), include_str!("string_2.txt").to_string(), "<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"NaN\" height=\"166\" viewBox=\"0 0 16 37\"><defs><linearGradient id=\"g\"><stop offset=\"0\" stop-color=\"blue\"/><stop offset=\"1\" stop-color=\"yellow\" stop-opacity=\"0.5\"/></linearGradient><\"0.5\"/></linearGradient><".to_string(), include_str!("string_3.txt").to_string()]).unwrap();
    let _ = regex.reverse();
}
