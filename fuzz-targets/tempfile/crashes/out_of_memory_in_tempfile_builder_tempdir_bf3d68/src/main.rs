use tempfile::Builder;

fn main() {
    let mut builder = Builder::new();
    let _ = builder.rand_bytes(4294967295usize);
    let _ = builder.tempdir();
}
