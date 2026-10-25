fn main() {
    let _ = triple_accel::levenshtein_search(&include_bytes!("data_0.bin").to_vec(), &include_bytes!("data_1.bin").to_vec());
}
