use tempfile::SpooledTempFile;

fn main() {
    let mut spooled_temp_file = SpooledTempFile::new(usize::MAX);
    let _ = spooled_temp_file.set_len(u64::MAX);
}
