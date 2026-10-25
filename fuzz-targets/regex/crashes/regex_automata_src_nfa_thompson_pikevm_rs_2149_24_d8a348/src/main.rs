use regex_automata::Input;
use regex_automata::nfa::thompson::pikevm::PikeVM;

fn main() {
    let pike_v_m = PikeVM::always_match().unwrap();
    let mut cache = pike_v_m.create_cache();
    let vec = vec![];
    let input = Input::new::<Vec<u8>>(&vec);
    let _ = pike_v_m.search_slots(&mut cache, &input, &mut vec![None, None, None, None, None]);
}
