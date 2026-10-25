fn main() {
    let mut spooled_temp_file = tempfile::spooled_tempfile(1000000000000000000usize);
    let _ = spooled_temp_file.set_len(1000000000000000000u64);
}
