fn main() {
    let d_f_a = <regex_automata::dfa::sparse::DFA<Vec<u8>>>::always_match().unwrap();
    let _ = d_f_a.write_to_native_endian(&mut include_bytes!("data_0.bin").to_vec());
}
