use svgtypes::Error;

fn main() {
    let error = Error::from(Error::InvalidChar(vec![32u8, 197u8], 0usize));
    let _ = error.to_string();
}
