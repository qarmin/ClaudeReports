use tempfile::Builder;

fn main() {
    let mut builder = Builder::new();
    let _ = builder.rand_bytes(10000000000000000000usize);
    let _ = builder.tempfile();
}
