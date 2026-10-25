use regex_automata::meta::Regex;

fn main() {
    let builder = Regex::builder();
    let regex = builder.build_many_from_hir::<regex_syntax::hir::Hir>(&[]).unwrap();
    let mut cache = regex.create_cache();
    cache.reset(&Regex::new_many::<String>(&["𲺶\0\0\0\0\0\0\0\0\0\0\0\0\0".to_string(), "".to_string(), "".to_string(), "".to_string()]).unwrap());
}
