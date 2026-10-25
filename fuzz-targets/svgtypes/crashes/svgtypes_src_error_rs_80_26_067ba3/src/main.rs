use svgtypes::Error;

fn main() {
    let error = Error::InvalidChar(vec![], 0usize);
    let _ = error.to_string();
}
