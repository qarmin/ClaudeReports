use svgtypes::Error;

fn main() {
    let error = Error::InvalidString(vec![], 0usize);
    let _ = error.to_string();
}
