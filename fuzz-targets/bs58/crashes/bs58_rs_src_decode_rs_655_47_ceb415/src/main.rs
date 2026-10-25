use bs58::decode::Error;

fn main() {
    let error = Error::InvalidCharacter { character: 'a', index: 0usize };
    let _ = error.unwrap_const();
}
