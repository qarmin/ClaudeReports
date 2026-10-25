fn main() {
    let d_f_a = <regex_automata::dfa::sparse::DFA<Vec<u8>>>::new_many::<String>(&[]).unwrap();
    let _ = d_f_a.write_to_big_endian(&mut include_bytes!("data_0.bin").to_vec());
}
