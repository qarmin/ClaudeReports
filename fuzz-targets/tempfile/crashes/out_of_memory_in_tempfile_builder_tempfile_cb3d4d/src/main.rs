use tempfile::Builder;

fn main() {
    let mut builder = Builder::new();
    let _ = builder.rand_bytes(1000000000000000000usize);
    let _ = builder.tempfile();
}
