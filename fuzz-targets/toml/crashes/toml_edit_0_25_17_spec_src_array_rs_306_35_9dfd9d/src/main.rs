use toml_edit::Array;

fn main() {
    let _ = Array::default().remove(0usize);
}
