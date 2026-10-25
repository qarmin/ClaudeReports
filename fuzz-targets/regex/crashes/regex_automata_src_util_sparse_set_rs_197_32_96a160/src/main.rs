use regex_automata::Input;
use regex_automata::nfa::thompson::pikevm::Cache;
use regex_automata::nfa::thompson::pikevm::PikeVM;
use regex_automata::util::captures::Captures;
use regex_automata::util::captures::GroupInfo;

fn main() {
    let builder = PikeVM::builder();
    let pike_v_m = builder.build_many::<String>(&["".to_string(), "".to_string()]).unwrap();
    let mut cache = Cache::new(&PikeVM::new("").unwrap());
    let mut captures = Captures::all(GroupInfo::empty());
    let vec = vec![];
    pike_v_m.captures::<regex_automata::Input>(&mut cache, Input::new::<Vec<u8>>(&vec), &mut captures);
}
