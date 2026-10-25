use regex_automata::meta::Cache;
use regex_automata::meta::Regex;

fn main() {
    let mut cache = Cache::new(&Regex::new("").unwrap());
    cache.reset(&Regex::builder().build_from_hir(&regex_syntax::parse("(1)").unwrap()).unwrap());
}
